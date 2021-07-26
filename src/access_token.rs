//! The [access_token](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Get_access_token.html) releated module.
//!
//! The purpose for this module is providing a [AccessTokenProvider] trait with a method [get_access_token][AccessTokenProvider], return a struct [AccessToken].
//!
//! We also provide a default [TokenClient](by [reqwest](https://crates.io/crates/reqwest) crate) for the users didn't want to implement one themselves.

use crate::error::{CommonResponse, SdkError, SdkResult};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// [WxSdk][crate::wechat::WxSdk] take a strust which impl [AccessTokenProvider].
#[async_trait]
pub trait AccessTokenProvider: Sync + Send + Sized {
    /// This trait derive [async_trait], it return a [std::future] of [AccessToken].
    async fn get_access_token(&self) -> SdkResult<AccessToken>;
}

/// Access token with a expires time.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AccessToken {
    pub access_token: String,
    expires_in: i32,
}

impl AccessToken {
    pub fn new<S: AsRef<str>>(token: S, expires_in: i32) -> Self {
        AccessToken {
            access_token: token.as_ref().to_owned(),
            expires_in,
        }
    }

    pub fn get_expires(&self) -> i32 {
        self.expires_in
    }
}

/// That's a default token client implement [AccessTokenProvider].
pub struct TokenClient {
    app_id: String,
    app_secret: String,
}

impl TokenClient {
    pub fn new(app_id: String, app_secret: String) -> Self {
        TokenClient { app_id, app_secret }
    }
}

#[async_trait]
impl AccessTokenProvider for TokenClient {
    async fn get_access_token(&self) -> SdkResult<AccessToken> {
        let url = format!(
            "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
            self.app_id.clone(),
            self.app_secret.clone()
        );
        let msg = reqwest::get(&url)
            .await?
            .json::<CommonResponse<AccessToken>>()
            .await?;
        match msg {
            CommonResponse::Ok(at) => Ok(at),
            CommonResponse::Err(e) => Err(SdkError::AccessTokenError(e)),
        }
    }
}

#[test]
fn test() {
    let input = r#"{"access_token":"ACCESS_TOKEN","expires_in":7200}"#;
    let expected = CommonResponse::Ok(AccessToken {
        access_token: "ACCESS_TOKEN".to_string(),
        expires_in: 7200,
    });
    assert_eq!(expected, serde_json::from_str(input).unwrap());

    let input = r#"{"errcode":40013,"errmsg":"invalid appid"}"#;
    let expected = CommonResponse::<AccessToken>::Err(crate::error::CommonError {
        errcode: 40013,
        errmsg: "invalid appid".to_string(),
    });
    assert_eq!(expected, serde_json::from_str(input).unwrap());
}
