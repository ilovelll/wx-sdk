use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumber {
    phone_info: PhoneInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneInfo {
    /// 用户绑定的手机号（国外手机号会有区号）
    pub phone_number: String,
    /// 没有区号的手机号
    pub pure_phone_number: String,
    /// 区号
    pub country_code: String,
    /// 数据水印
    pub watermark: Watermark,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Watermark {
    /// 小程序appid
    pub appid: String,
    /// 用户获取手机号操作的时间戳
    pub timestamp: i64,
}

pub struct PhoneNumberModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> PhoneNumberModule<'a, T> {
    /// code换取用户手机号。 每个code只能使用一次，code的有效期为5min
    pub async fn get_user_phone_number(&self, code: &str) -> SdkResult<PhoneNumber> {
        let data = &serde_json::json!({ "code": code });
        let url = "https://api.weixin.qq.com/wxa/business/getuserphonenumber";
        post_send(self.0, url, data).await
    }
}
