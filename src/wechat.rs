//! This module provides the most important struct [WxSdk], almost every funcition in `wx_func` take it as a parameter.
//!
//! You can construct it with a [ServerConfig].
//!
//! Example
//! ```rust
//! use wx_func::wechat::{WxSdk, ServerConfig, EncodingMode};
//! let config = ServerConfig::new("token", Some("aes_key"), EncodingMode::Plain);
//! let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret", config);
//! ```
//! above example use the default token client, you could implement one that impl trait [AccessTokenProvider] by yourself.
//! ```ignore
//! let token_clinet = MyTokenClient{};
//! let sdk = WxSdk::new("app_id", "app_sercret", config, token_client);
//! ```


use async_trait::async_trait;
use reqwest::Client;

use crate::{access_token::AccessTokenProvider, AccessToken, SdkResult, TokenClient};
pub struct WxSdk<T: AccessTokenProvider> {
    pub app_id: String,
    app_secret: String,
    server_config: ServerConfig,
    http_client: Client,
    token_client: T,
}

pub struct ServerConfig {
    pub token: String,
    pub encoding_aes_key: Option<String>,
    pub encoding_mode: EncodingMode
}

pub enum EncodingMode {
    Plain,
    Compat,
    Security,
}

impl ServerConfig {
    pub fn new<S: AsRef<str>>(token: S, aes_key: Option<S>, encoding_mode: EncodingMode) -> Self {
        ServerConfig {
            token: token.as_ref().to_owned(),
            encoding_aes_key: aes_key.map(|x| x.as_ref().to_owned()),
            encoding_mode
        }
    }
}

impl<T: AccessTokenProvider> WxSdk<T> {
    pub fn new<S: AsRef<str>>(app_id: S, app_secret: S, server_config: ServerConfig, token_client: T) -> Self {
        WxSdk {
            http_client: Client::new(),
            app_id: app_id.as_ref().to_owned(),
            app_secret: app_secret.as_ref().to_owned(),
            server_config,
            token_client,
        }
    }

    pub fn get_server_config(&self) -> &ServerConfig {
        &self.server_config
    }
}

impl WxSdk<TokenClient> {
    pub fn new_with_default_token_client<S: AsRef<str>>(app_id: S, app_secret: S, server_config: ServerConfig) -> Self {
        let app_id = app_id.as_ref().to_owned();
        let app_secret = app_secret.as_ref().to_owned();
        let token_client = TokenClient::new(app_id.clone(), app_secret.clone());
        WxSdk {
            http_client: Client::new(),
            app_id,
            app_secret,
            server_config,
            token_client,
        }
    }
}

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
