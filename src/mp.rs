//! This module define the api set of wechat office account.
//!
//! It seperates those apis to different mods by url path.

use std::collections::HashMap;

use roxmltree::Document;

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, SdkError},
    mp::event::signature::Signature,
};
use crate::{
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

use self::{
    customservice::CustomServiceModule, datacube::DataCubeModule, draft::DraftModule,
    freepublish::FreePublishModule, material::MaterialModule, media::MediaModule, menu::MenuModule,
    message::MessageModule, qrcode::QrcodeModule, reply::Reply, shorten::ShortenModule,
    sns::SnsModule, tags::TagsModule, template::TemplateModule, ticket::TicketModule,
    user::UserModule,
};
pub mod customservice;
pub mod datacube;
pub mod draft;
pub mod event;
pub mod freepublish;
pub mod material;
pub mod media;
pub mod menu;
pub mod message;
pub mod qrcode;
pub mod reply;
pub mod shorten;
pub mod sns;
pub mod tags;
pub mod template;
pub mod ticket;
pub mod user;

/// The configuration of your app server.
#[derive(Clone)]
pub struct ServerConfig {
    pub token: String,
    pub encoding_mode: EncodingMode,
}

type AesKey = String;

/// Encoding mode of message getting or sending with wechat.
/// [EncodingMode::Compat] or [EncodingMode::Security] mode has a aes-key.
#[derive(Clone)]
pub enum EncodingMode {
    Plain,
    Compat(AesKey),
    Security(AesKey),
}

impl ServerConfig {
    pub fn new<S: AsRef<str>>(token: S, encoding_mode: EncodingMode) -> Self {
        ServerConfig {
            token: token.as_ref().to_owned(),
            encoding_mode,
        }
    }
}

/// 公众号接口SDK，由于 Rust Doc 中还无法搜索中文，请直接搜索相关请求 url 中的关键信息，例如 `clear_quota`为接口限额清零接口。
#[derive(Clone)]
pub struct MpSdk<T: AccessTokenProvider> {
    pub(crate) sdk: WxSdk<T>,
    pub(crate) server_config: ServerConfig,
}

impl<T: AccessTokenProvider> MpSdk<T> {
    /// 接口限额清零
    ///
    /// 公众号调用接口并不是无限制的。
    /// 每个帐号每月共10次清零操作机会，清零生效一次即用掉一次机会（10次包括了平台上的清零和调用接口API的清零）。
    pub async fn clear_quota(&self) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/clear_quota";
        let sdk = &self.sdk;
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
        QrcodeModule(&self.sdk)
    }
    /// Short key generator module 短key生成模块
    pub fn shorten(&self) -> ShortenModule<WxSdk<T>> {
        ShortenModule(&self.sdk)
    }
    /// Tag module 标签模块
    pub fn tags(&self) -> TagsModule<WxSdk<T>> {
        TagsModule(&self.sdk)
    }
    /// User module 用户模块
    pub fn user(&self) -> UserModule<WxSdk<T>> {
        UserModule(&self.sdk)
    }
    /// Message send module 消息（发送）相关模块
    pub fn message(&self) -> MessageModule<WxSdk<T>> {
        MessageModule(&self.sdk)
    }
    /// Menu module 自定义菜单模块
    pub fn menu(&self) -> MenuModule<WxSdk<T>> {
        MenuModule(&self.sdk)
    }
    /// Template message module 模板消息模块
    pub fn template(&self) -> TemplateModule<WxSdk<T>> {
        TemplateModule(&self.sdk)
    }

    /// Media module （临时）素材文件模块
    pub fn media(&self) -> MediaModule<WxSdk<T>> {
        MediaModule(&self.sdk)
    }

    /// Material module （永久）素材模块
    pub fn material(&self) -> MaterialModule<WxSdk<T>> {
        MaterialModule(&self.sdk)
    }

    /// Datacube module 分析中心模块
    pub fn datacube(&self) -> DataCubeModule<WxSdk<T>> {
        DataCubeModule(&self.sdk)
    }

    /// Custom Service module 客服模块
    pub fn customservice(&self) -> CustomServiceModule<WxSdk<T>> {
        CustomServiceModule(&self.sdk)
    }

    /// 获取jsapi ticket 或者 wx_card ticket
    pub fn ticket(&self) -> TicketModule<T> {
        TicketModule(&self.sdk)
    }

    /// 网页授权模块
    pub fn sns(&self) -> SnsModule<T> {
        SnsModule(&self.sdk)
    }

    /// 草稿箱模块
    pub fn draft(&self) -> DraftModule<T> {
        DraftModule(&self.sdk)
    }

    /// 草稿箱模块
    pub fn freepublish(&self) -> FreePublishModule<T> {
        FreePublishModule(&self.sdk)
    }

    /// 解析微信推送消息
    pub fn parse_received_msg<S: AsRef<str>>(
        &self,
        msg: S,
        url_params: Option<HashMap<String, String>>,
    ) -> SdkResult<event::ReceivedEvent> {
        let server_config = &self.server_config;
        let msg = match server_config.encoding_mode {
            EncodingMode::Plain => event::ReceivedEvent::parse(msg.as_ref()),
            EncodingMode::Compat(_) => event::ReceivedEvent::parse(msg.as_ref()),
            EncodingMode::Security(ref aes_key) => {
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
                if app_id != self.sdk.app_id {
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
        let server_config = &self.server_config;
        let mut reply_xml = reply::reply_to_xml(reply, from, to)?;
        if let EncodingMode::Security(ref aes_key) = server_config.encoding_mode {
            let ref app_id = self.sdk.app_id;
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
