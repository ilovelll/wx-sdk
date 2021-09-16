use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    /// 可领取用户的openid
    pub openid: String,
    /// 在红包封面平台获取发放ctoken（需要指定可以发放的appid）
    pub ctoken: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResult {
    pub data: DataUrl,
    pub errcode: i32,
    pub errmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataUrl {
    pub url: String,
}

pub struct RedpacketCoverModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> RedpacketCoverModule<'a, T> {
    /// 微信红包封面 鉴权接口。本接口用于获得指定用户可以领取的红包封面链接，获取参数ctoken参考微信红包封面开放平台
    pub async fn get_authentication_url(&self, data: &Auth) -> SdkResult<AuthResult> {
        let url = "https://api.weixin.qq.com/redpacketcover/wxapp/cover_url/get_by_token";
        post_send(self.0, url, data).await
    }
}
