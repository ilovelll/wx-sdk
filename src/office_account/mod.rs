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

// pub async fn clear_quota<T: AccessTokenGetter + WxAPIRequest>(
//     app_id: String,
//     token_client: &T,
// ) -> Result<()> {
//     let url = url::Url::parse("https://api.weixin.qq.com/cgi-bin/clear_quota")?;
//     let res = token_client
//         .wx_post(url, &serde_json::json!({ "appid": app_id }))
//         .await;
//     res
// }

pub async fn clear_quota<T: AccessTokenProvider>(app_id: String, sdk: &WxSdk<T>) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/clear_quota";

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
