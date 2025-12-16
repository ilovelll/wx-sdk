use crate::{access_token::AccessTokenProvider, error::CommonResponse, SdkResult, WxSdk};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GrantAccessToken {
    pub access_token: String,
    pub expires_in: u32,
    pub refresh_token: String,
    pub openid: String,
    pub scope: String,
    pub unionid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub openid: String,
    pub nickname: String,
    pub sex: u8,
    pub province: String,
    pub city: String,
    pub country: String,
    pub headimgurl: String,
    pub privilege: Option<Vec<String>>,
    pub unionid: Option<String>,
}

pub struct SnsModule<'a, T: AccessTokenProvider>(pub(crate) &'a WxSdk<T>);

impl<'a, T: AccessTokenProvider> SnsModule<'a, T> {
    /// 微信网页授权
    /// 详情请参考[微信官方文档](https://developers.weixin.qq.com/doc/offiaccount/OA_Web_Apps/Wechat_webpage_authorization.html)
    /// 第二步，通过code换取access token
    pub async fn oauth_access_token(&self, code: &str) -> SdkResult<GrantAccessToken> {
        let base_url = "https://api.weixin.qq.com/sns/oauth2/access_token";
        let client = self.0.http_client.get(base_url);

        let app_id = self.0.app_id.clone();
        let app_secret = self.0.app_secret.clone();
        let client = client.query(&[
            ("appid", app_id.as_str()),
            ("secret", app_secret.as_str()),
            ("code", code),
            ("grant_type", "authorization_code"),
        ]);
        let res: CommonResponse<GrantAccessToken> = client.send().await?.json().await?;
        res.into()
    }
    /// 微信网页授权
    /// 第三步：刷新access_token（如果需要）
    pub async fn oauth_refresh_token(&self, refresh_token: String) -> SdkResult<GrantAccessToken> {
        let base_url = "https://api.weixin.qq.com/sns/oauth2/refresh_token";
        let client = self.0.http_client.get(base_url);

        let app_id = self.0.app_id.clone();
        let client = client.query(&[
            ("appid", app_id),
            ("refresh_token", refresh_token),
            ("grant_type", "refresh_token".to_owned()),
        ]);
        let res: CommonResponse<GrantAccessToken> = client.send().await?.json().await?;
        res.into()
    }

    /// 微信网页授权
    /// 拉取用户信息(需scope为 snsapi_userinfo)
    pub async fn userinfo(
        &self,
        openid: String,
        access_token: String,
        lang: String,
    ) -> SdkResult<UserInfo> {
        let base_url = "https://api.weixin.qq.com/sns/userinfo";
        let client = self.0.http_client.get(base_url);

        let client = client.query(&[
            ("access_token", access_token),
            ("openid", openid),
            ("lang", lang),
        ]);
        let res: CommonResponse<UserInfo> = client.send().await?.json().await?;
        res.into()
    }
}
