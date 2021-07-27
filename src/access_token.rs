//! The [access_token](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Get_access_token.html) releated module.
//!
//! The purpose for this module is providing a [AccessTokenProvider] trait with a method [get_access_token][AccessTokenProvider], return a struct [AccessToken].
//!
//! We also provide a default [TokenClient](by [reqwest](https://crates.io/crates/reqwest) crate) for the users didn't want to implement one themselves.
use std::time::SystemTime;
use crate::error::{CommonResponse, SdkError, SdkResult};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::RwLock;

/// [WxSdk][crate::wechat::WxSdk] take a strust which impl [AccessTokenProvider].
#[async_trait]
pub trait AccessTokenProvider: Sync + Send + Sized {
    /// This trait derive [async_trait], it return a [std::future] of [AccessToken].
    async fn get_access_token(&self) -> SdkResult<AccessToken>;
}

/// Access token with a expires time.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: i32,
}

/// That is a cache struct for [TokenClient].
#[derive(Clone)]
struct AccessTokenCache {
    access_token: String,
    expires_in: i32,
    expires_at: u64,
}

impl From<AccessToken> for AccessTokenCache {
    fn from(at: AccessToken) -> Self {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
        let secs = timestamp.as_secs();
        let expires_at = secs + at.expires_in as u64;
        AccessTokenCache {
            access_token: at.access_token,
            expires_in: at.expires_in,
            expires_at
        }
    }
}

impl From<AccessTokenCache> for AccessToken {
    fn from(c: AccessTokenCache) -> Self {
        AccessToken {
            access_token: c.access_token,
            expires_in: c.expires_in
        }
    }
}

impl AccessTokenCache {
    pub fn is_expires(&self) -> bool {
        let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
        self.expires_at <= timestamp.as_secs() - 5 // more 5 seconds for buffer
    }
}

/// That's a default token client implement [AccessTokenProvider].
pub struct TokenClient {
    app_id: String,
    app_secret: String,
    cache_token: RwLock<Option<AccessTokenCache>>,
}

impl TokenClient {
    pub fn new(app_id: String, app_secret: String) -> Self {
        TokenClient { app_id, app_secret, cache_token: RwLock::new(None) }
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
        let locked = self.cache_token.read().await;
        
        if let Some(cache) = &*locked {
            if !cache.is_expires() {
                let cloned = cache.clone();
                return Ok(cloned.into());
            }
        };
        
        let msg = reqwest::get(&url)
            .await?
            .json::<CommonResponse<AccessToken>>()
            .await?;
        
        match msg {
            CommonResponse::Ok(at) => {
                let mut locked = self.cache_token.write().await;
                *locked = Some(at.clone().into());
                Ok(at)
            },
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

#[tokio::test]
async fn test_get_from_cache() {
    use std::time::Duration;
    use std::thread::sleep;

    let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default();
    let token_client = TokenClient{
        app_id: "app_id".to_owned(),
        app_secret: "app_secret".to_owned(),
        cache_token: RwLock::new(Some(AccessTokenCache{access_token: "ACCESS_TOKEN".to_owned(), expires_in: 7200, expires_at: timestamp.as_secs() + 7200}))
    };
    sleep(Duration::new(2, 0));
    let res = token_client.get_access_token().await.unwrap();
    assert_eq!(res, AccessToken { access_token: "ACCESS_TOKEN".to_owned(), expires_in: 7200,});
}
