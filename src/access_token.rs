//! The [access_token](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Get_access_token.html) releated module.
//!
//! The purpose for this module is providing a [AccessTokenProvider] trait with a method [get_access_token][AccessTokenProvider], return a struct [AccessToken]. 
//!
//! We also provide a default [TokenClient](by [reqwest](https://crates.io/crates/reqwest) crate) for the users didn't want to implement one themselves.

use crate::error::{CommonResponse, SdkError, SdkResult};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[async_trait]
pub trait AccessTokenProvider: Sync + Send + Sized {
    async fn get_access_token(&self) -> SdkResult<AccessToken>;
}

// #[async_trait]
// pub trait WxAPIRequest: Sync + Send {
//     async fn wx_get_url<R: DeserializeOwned>(&self, mut url: reqwest::Url) -> SdkResult<R>;
//     async fn wx_post_url<R: DeserializeOwned, D: Serialize + Sync + Send>(
//         &self,
//         mut url: reqwest::Url,
//         post_data: &D,
//     ) -> SdkResult<R>;
// }

// #[async_trait]
// pub trait WxAPIRequestBuilder {
//     async fn wx_get(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder>;
//     async fn wx_post(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder>;
// }

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct AccessToken {
    pub access_token: String,
    expires_in: i32,
}

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

// #[async_trait]
// impl WxAPIRequest for TokenClient {
//     async fn wx_get_url<R: DeserializeOwned>(&self, mut url: reqwest::Url) -> SdkResult<R> {
//         let at = self.get_access_token().await?;
//         url.query_pairs_mut()
//             .append_pair("access_token", at.access_token.as_ref());

//         let res = self
//             .http_client
//             .get(url)
//             .send()
//             .await?
//             .json::<CommonResponse<R>>()
//             .await?;
//         res.into()
//     }

//     async fn wx_post_url<R: DeserializeOwned, D: Serialize + Sync + Send>(
//         &self,
//         mut url: reqwest::Url,
//         post_data: &D,
//     ) -> SdkResult<R> {
//         let at = self.get_access_token().await?;
//         url.query_pairs_mut()
//             .append_pair("access_token", at.access_token.as_ref());

//         let res = self
//             .http_client
//             .post(url)
//             .json(post_data)
//             .send()
//             .await?
//             .json::<CommonResponse<R>>()
//             .await?;

//         res.into()
//     }
// }

// #[async_trait]
// impl WxAPIRequestBuilder for TokenClient {
//     async fn wx_get(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder> {
//         let at = self.get_access_token().await?;
//         let mut url = reqwest::Url::parse(url)?;
//         url.query_pairs_mut()
//             .append_pair("access_token", at.access_token.as_ref());

//         let builder = self.http_client.get(url);
//         Ok(builder)
//     }

//     async fn wx_post(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder> {
//         let at = self.get_access_token().await?;
//         let mut url = reqwest::Url::parse(url)?;
//         url.query_pairs_mut()
//             .append_pair("access_token", at.access_token.as_ref());

//         let builder = self.http_client.post(url);
//         Ok(builder)
//     }
// }

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
