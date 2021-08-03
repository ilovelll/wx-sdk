//! This module define the api set of wechat office account.
//!
//! It seperates those apis to different mods by url path.
use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
};
use crate::{
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};
pub mod customservice;
pub mod datacube;
pub mod event;
pub mod material;
pub mod media;
pub mod menu;
pub mod message;
pub mod qrcode;
pub mod tags;
pub mod template;
pub mod user;
pub mod events;

/// 接口限额清零
///
/// 公众号调用接口并不是无限制的。
/// 每个帐号每月共10次清零操作机会，清零生效一次即用掉一次机会（10次包括了平台上的清零和调用接口API的清零）。
pub async fn clear_quota<T: AccessTokenProvider>(sdk: &WxSdk<T>) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/clear_quota";
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
