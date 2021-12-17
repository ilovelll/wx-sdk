//! This module provides the most important struct [WxSdk], almost every funcition in `wx-sdk` take it as a parameter.
//!
//! You can construct it with a [ServerConfig].
//!
//! Example
//! ```rust
//! use wx_sdk::wechat::WxSdk;
//! let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret");
//! ```
//! above example use the default token client, you could implement one that impl trait [AccessTokenProvider] by yourself.
//! ```ignore
//! let token_clinet = MyTokenClient{};
//! let sdk = WxSdk::new("app_id", "app_sercret", config, token_client);
//! ```

use std::sync::Arc;

use async_trait::async_trait;
use reqwest::Client;

#[cfg(feature = "mp")]
use crate::mp::{MpSdk, ServerConfig};

#[cfg(feature = "wxa")]
use crate::wxa::WxaSdk;

use crate::{access_token::AccessTokenProvider, cache::Cache, SdkResult, TokenClient};

/// This is the sdk object. We provide a `new` method to construct it.
#[derive(Clone)]
pub struct WxSdk<T: AccessTokenProvider> {
    pub app_id: String,
    pub(crate) app_secret: String,
    pub(crate) http_client: Client,
    pub(crate) token_client: T,
    pub(crate) cache: Arc<Cache<String, String>>,
}

impl<T: AccessTokenProvider> WxSdk<T> {
    pub fn new<S: AsRef<str>>(app_id: S, app_secret: S, token_client: T) -> Self {
        WxSdk {
            http_client: Client::new(),
            app_id: app_id.as_ref().to_owned(),
            app_secret: app_secret.as_ref().to_owned(),
            token_client,
            cache: Arc::new(Cache::new()),
        }
    }

    /// Official account(Media Press) module
    #[cfg(feature = "mp")]
    pub fn mp(self, server_config: ServerConfig) -> MpSdk<T> {
        MpSdk {
            sdk: self,
            server_config,
        }
    }

    /// miniprogram module
    #[cfg(feature = "wxa")]
    pub fn wxa(self) -> WxaSdk<T> {
        WxaSdk { sdk: self }
    }
}

impl WxSdk<TokenClient> {
    pub fn new_with_default_token_client<S: AsRef<str>>(app_id: S, app_secret: S) -> Self {
        let app_id = app_id.as_ref().to_owned();
        let app_secret = app_secret.as_ref().to_owned();
        let token_client = TokenClient::new(app_id.clone(), app_secret.clone());
        WxSdk {
            http_client: Client::new(),
            app_id,
            app_secret,
            token_client,
            cache: Arc::new(Cache::new()),
        }
    }
}

/// This trait warps two common http request method that [wx_get][WxApiRequestBuilder::wx_get] and [wx_post][WxApiRequestBuilder::wx_post] with wechat api server.
#[async_trait]
pub trait WxApiRequestBuilder {
    async fn wx_get(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder>;
    async fn wx_post(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder>;
}

#[async_trait]
impl<T: AccessTokenProvider> WxApiRequestBuilder for WxSdk<T> {
    async fn wx_get(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder> {
        let at = self.token_client.get_access_token().await?;
        let mut url = reqwest::Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("access_token", at.access_token.as_ref());

        let builder = self.http_client.get(url);
        Ok(builder)
    }

    async fn wx_post(&self, url: &'static str) -> SdkResult<reqwest::RequestBuilder> {
        let at = self.token_client.get_access_token().await?;
        let mut url = reqwest::Url::parse(url)?;
        url.query_pairs_mut()
            .append_pair("access_token", at.access_token.as_ref());

        let builder = self.http_client.post(url);
        Ok(builder)
    }
}
