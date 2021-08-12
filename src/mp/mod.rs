//! This module define the api set of wechat office account.
//!
//! It seperates those apis to different mods by url path.
use serde_json::json;

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
};
use crate::{
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

use self::{
    menu::MenuModule, message::MessageModule, qrcode::QrcodeModule, shorten::ShortenModule,
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
}
