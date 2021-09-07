//! 短key托管
//!
//! 短key托管类似于短链API，开发者可以通过GenShorten将不超过4KB的长信息转成短key，再通过FetchShorten将短key还原为长信息。
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::error::SdkResult;
use crate::{error::CommonResponse, wechat::WxApiRequestBuilder};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortKey {
    pub short_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LongData {
    pub long_data: String,
    pub create_time: u64,
    pub expire_seconds: u32,
}
/// 短key托管
///
/// 短key托管类似于短链API，开发者可以通过GenShorten将不超过4KB的长信息转成短key，再通过FetchShorten将短key还原为长信息。
pub struct ShortenModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> ShortenModule<'a, T> {
    /// 短key生成
    pub async fn gen<S: AsRef<str>>(
        &self,
        long_data: S,
        expire_seconds: Option<u32>,
    ) -> SdkResult<ShortKey> {
        let mut expire_seconds = expire_seconds.unwrap_or(2592000);
        if expire_seconds > 2592000 {
            expire_seconds = 2592000;
        }
        let base_url = "https://api.weixin.qq.com/cgi-bin/shorten/gen";
        let sdk = self.0;
        let res = sdk.wx_post(base_url).await?;
        let res = res
                .json(&json!({"long_data": long_data.as_ref().to_owned(), "expire_seconds": expire_seconds}))
                .send()
                .await?
                .json::<CommonResponse<ShortKey>>()
                .await?;
        res.into()
    }

    /// 短key获取
    pub async fn fetch<S: AsRef<str>>(&self, short_key: S) -> SdkResult<LongData> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/shorten/fetch";
        let sdk = self.0;
        let res = sdk.wx_post(base_url).await?;
        let res = res
            .json(&json!({"short_key": short_key.as_ref().to_owned()}))
            .send()
            .await?
            .json::<CommonResponse<LongData>>()
            .await?;
        res.into()
    }
}
