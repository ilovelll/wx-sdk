//! The [access_token](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Get_access_token.html) releated module.
//!
//! The purpose for this module is providing a [AccessTokenProvider] trait with a method [get_access_token][AccessTokenProvider], return a struct [AccessToken].
//!
//! We also provide a default [TokenClient](by [reqwest](https://crates.io/crates/reqwest) crate) for the users didn't want to implement one themselves.
use crate::{
    cache,
    error::{CommonResponse, SdkError, SdkResult},
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// [WxSdk][crate::wechat::WxSdk] take a struct which impl [AccessTokenProvider].
/// You need to use [async_trait](https://crates.io/crates/async-trait) to implement [AccessTokenProvider].
#[async_trait]
pub trait AccessTokenProvider: Sync + Send + Sized {
    /// This trait derive [async_trait](https://crates.io/crates/async-trait), it return a [std::future] of [AccessToken].
    async fn get_access_token(&self) -> SdkResult<AccessToken>;
}

/// Access token with a expires time.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct AccessToken {
    pub access_token: String,
    pub expires_in: i32,
}

impl From<cache::Item<String>> for AccessToken {
    fn from(c: cache::Item<String>) -> Self {
        AccessToken {
            access_token: c.object,
            expires_in: 0,
        }
    }
}

impl From<AccessToken> for cache::Item<String> {
    fn from(t: AccessToken) -> Self {
        let duration = Duration::from_secs((t.expires_in - 5) as u64);
        cache::Item::new(t.access_token, Some(duration))
    }
}

/// That's a default token client implement [AccessTokenProvider].
pub struct TokenClient {
    app_id: String,
    app_secret: String,
    cache_token: Arc<RwLock<Option<cache::Item<String>>>>,
}

impl TokenClient {
    pub fn new(app_id: String, app_secret: String) -> Self {
        TokenClient {
            app_id,
            app_secret,
            cache_token: Arc::new(RwLock::new(None)),
        }
    }

    fn get_cache_token(&self) -> Option<AccessToken> {
        let locked = self.cache_token.read().unwrap();
        match &*locked {
            Some(i) if !i.expired() => Some(i.clone().into()),
            _ => None,
        }
    }

    fn set_cache_token(&self, token: AccessToken) {
        let mut locked = self.cache_token.write().unwrap();
        *locked = Some(token.into())
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
        let cache_token = self.get_cache_token();
        match cache_token {
            Some(token) => Ok(token),
            None => {
                let msg = reqwest::get(&url)
                    .await?
                    .json::<CommonResponse<AccessToken>>()
                    .await?;

                match msg {
                    CommonResponse::Ok(at) => {
                        self.set_cache_token(at.clone());
                        Ok(at)
                    }
                    CommonResponse::Err(e) => Err(SdkError::AccessTokenError(e)),
                }
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use tokio::time::sleep;

    use crate::{
        access_token::AccessTokenProvider, cache, error::CommonResponse, AccessToken, TokenClient,
    };

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

        let token_client = TokenClient {
            app_id: "app_id".to_owned(),
            app_secret: "secret".to_owned(),
            cache_token: std::sync::Arc::new(std::sync::RwLock::new(Some(cache::Item::new(
                "ACCESS_TOKEN".to_owned(),
                Some(Duration::from_secs(2)),
            )))),
        };
        sleep(Duration::new(1, 0)).await;
        let res = token_client.get_access_token().await.unwrap();
        let token = res.access_token;
        let new_t = token_client.get_access_token().await.unwrap();
        assert_eq!(
            new_t,
            AccessToken {
                access_token: token,
                expires_in: 0
            }
        );
    }
}
