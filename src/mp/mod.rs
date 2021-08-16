//! This module define the api set of wechat office account.
//!
//! It seperates those apis to different mods by url path.

use std::collections::HashMap;

use roxmltree::Document;

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse, SdkError},
    mp::event::signature::Signature,
    wechat,
};
use crate::{
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

use self::{
    datacube::DataCubeModule, material::MaterialModule, media::MediaModule, menu::MenuModule,
    message::MessageModule, qrcode::QrcodeModule, reply::Reply, shorten::ShortenModule,
    tags::TagsModule, template::TemplateModule, user::UserModule,
};
pub mod customservice;
pub mod datacube;
pub mod event;
pub mod material;
pub mod media;
pub mod menu;
pub mod message;
pub mod qrcode;
pub mod reply;
pub mod shorten;
pub mod tags;
pub mod template;
pub mod user;

/// 公众号接口SDK，由于 Rust Doc 中还无法搜索中文，请直接搜索相关请求 url 中的关键信息，例如 `clear_quota`为接口限额清零接口。
pub struct MpSdk<'a, T: AccessTokenProvider>(pub(crate) &'a WxSdk<T>);

impl<'a, T: AccessTokenProvider> MpSdk<'a, T> {
    /// 接口限额清零
    ///
    /// 公众号调用接口并不是无限制的。
    /// 每个帐号每月共10次清零操作机会，清零生效一次即用掉一次机会（10次包括了平台上的清零和调用接口API的清零）。
    pub async fn clear_quota(&self) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/clear_quota";
        let sdk = self.0;
        let app_id = sdk.app_id.clone();
        let res = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({ "appid": app_id }))
            .send()
            .await?
            .json::<CommonError>()
            .await?;

        res.into()
    }

    /// Qrcode generator module 生成二维码模块
    pub fn qrcode(&self) -> QrcodeModule<WxSdk<T>> {
        QrcodeModule(self.0)
    }
    /// Short key generator module 短key生成模块
    pub fn shorten(&self) -> ShortenModule<WxSdk<T>> {
        ShortenModule(self.0)
    }
    /// Tag module 标签模块
    pub fn tags(&self) -> TagsModule<WxSdk<T>> {
        TagsModule(self.0)
    }
    /// User module 用户模块
    pub fn user(&self) -> UserModule<WxSdk<T>> {
        UserModule(self.0)
    }
    /// Message send module 消息（发送）相关模块
    pub fn message(&self) -> MessageModule<WxSdk<T>> {
        MessageModule(self.0)
    }
    /// Menu module 自定义菜单模块
    pub fn menu(&self) -> MenuModule<WxSdk<T>> {
        MenuModule(self.0)
    }
    /// Template message module 模板消息模块
    pub fn template(&self) -> TemplateModule<WxSdk<T>> {
        TemplateModule(self.0)
    }

    /// Media module （临时）素材文件模块
    pub fn media(&self) -> MediaModule<WxSdk<T>> {
        MediaModule(self.0)
    }

    /// Material module （永久）素材模块
    pub fn material(&self) -> MaterialModule<WxSdk<T>> {
        MaterialModule(self.0)
    }

    /// Datacube module 分析中心模块
    pub fn datacube(&self) -> DataCubeModule<WxSdk<T>> {
        DataCubeModule(self.0)
    }

    /// 解析微信推送消息
    pub fn parse_received_msg<S: AsRef<str>>(
        &self,
        msg: S,
        url_params: Option<HashMap<String, String>>,
    ) -> SdkResult<event::ReceivedEvent> {
        let server_config = self.0.get_server_config();
        let msg = match server_config.encoding_mode {
            crate::wechat::EncodingMode::Plain => event::ReceivedEvent::parse(msg.as_ref()),
            crate::wechat::EncodingMode::Compat(_) => event::ReceivedEvent::parse(msg.as_ref()),
            crate::wechat::EncodingMode::Security(ref aes_key) => {
                let url_params = url_params
                    .ok_or_else(|| SdkError::InvalidParams("needs url_params".to_owned()))?;
                let signature = url_params
                    .get("msg_signature")
                    .ok_or_else(|| SdkError::InvalidParams("msg_signature".to_owned()))?;
                let timestamp = url_params
                    .get("timestamp")
                    .ok_or_else(|| SdkError::InvalidParams("timestamp".to_owned()))?;
                let nonce = url_params
                    .get("nonce")
                    .ok_or_else(|| SdkError::InvalidParams("nonce".to_owned()))?;
                let token = server_config.token.clone();
                let root = Document::parse(msg.as_ref())?;
                let encrypt_msg = root
                    .descendants()
                    .find(|n| n.has_tag_name("Encrypt"))
                    .map(|n| n.text())
                    .flatten()
                    .unwrap();

                let check_sign = vec![
                    token,
                    timestamp.clone(),
                    nonce.clone(),
                    encrypt_msg.to_owned(),
                ];
                let sign = event::signature::Signature::new(signature, check_sign);
                if !sign.is_ok() {
                    return Err(SdkError::InvalidSignature);
                }
                // decrpyted_text = [random(16) + content_len(4) + content + appid]
                let (msg, app_id) = event::crypto::decrypt_message(encrypt_msg, aes_key)?;
                if app_id != self.0.app_id {
                    return Err(SdkError::InvalidAppid);
                }
                event::ReceivedEvent::parse(msg.as_ref())
            }
        };
        msg
    }

    /// 得到回复消息 XML
    pub fn reply_to_xml<S: Into<String>>(
        &self,
        reply: Reply,
        from: S,
        to: S,
        url_params: Option<HashMap<String, String>>,
    ) -> SdkResult<String> {
        let server_config = self.0.get_server_config();
        let mut reply_xml = reply::reply_to_xml(reply, from, to)?;
        if let wechat::EncodingMode::Security(ref aes_key) = server_config.encoding_mode {
            let ref app_id = self.0.app_id;
            let encrypt_msg = event::crypto::encrypt_message(&reply_xml, aes_key, app_id)?;
            let url_params =
                url_params.ok_or_else(|| SdkError::InvalidParams("needs url_params".to_owned()))?;

            let timestamp = url_params
                .get("timestamp")
                .ok_or_else(|| SdkError::InvalidParams("timestamp".to_owned()))?;
            let nonce = url_params
                .get("nonce")
                .ok_or_else(|| SdkError::InvalidParams("nonce".to_owned()))?;
            let token = server_config.token.clone();
            let check_sign = vec![
                token,
                timestamp.clone(),
                nonce.clone(),
                encrypt_msg.to_owned(),
            ];
            let msg_signaturet = Signature::generate_signature(check_sign);
            reply_xml = format!(
                "<xml>
<Encrypt><![CDATA[{}]]></Encrypt>
<MsgSignature><![CDATA[{}]]></MsgSignature>
<TimeStamp>{}</TimeStamp>
<Nonce><![CDATA[{}]]></Nonce>
</xml>",
                encrypt_msg, msg_signaturet, timestamp, nonce
            );
        }
        Ok(reply_xml)
    }
}
