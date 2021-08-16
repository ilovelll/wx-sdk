//! # wx-sdk API Documentation
//! ## `wx-sdk` is a [WeChat SDK](https://mp.weixin.qq.com/) written in [Rust](https://www.rust-lang.org/).
//! ## Features
//! Fealtures can be checked at [README page](https://github.com/ilovelll/wx-sdk/blob/main/README.md)

//! ## QuickStart

//! First, please refer to this [page](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Access_Overview.html) to provide these values: `token`, `EncodingAESKey`,`EncodingMode`.
//! ```ignore
//! use wx_sdk::wechat::{WxSdk, ServerConfig, EncodingMode};
//!
//! let config = ServerConfig::new("token", EncodingMode::Plain);
//! let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret", config);
//!
//! // Then, you can use the sdk functions, like get current menu info:
//! let mpsdk = WxSdk::mp(&sdk);
//! let menu = mpsdk.menu().get_current_selfmenu_info().await;
//! ```

//! ## Contributing

//! Issue reports and Pull Requests are always welcome!

//! ## License

//! wx-sdk is available under the [_MIT License_](https://github.com/ilovelll/wx-sdk/blob/main/LICENSE)

pub mod access_token;

pub use access_token::AccessToken;

pub mod error;
pub use error::SdkResult;
pub mod wechat;
pub use access_token::TokenClient;

pub use wechat::WxSdk;
#[cfg(feature = "mp")]
pub mod mp;

// #[tokio::test]
// async fn test_sdk() -> SdkResult<()> {
//     use crate::wechat::{WxSdk, ServerConfig, EncodingMode};

//     let config = ServerConfig::new("token", EncodingMode::Plain);
//     let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret", config);
//     let mpsdk = sdk.mp();
//     let menu = mpsdk.menu().get_current_selfmenu_info().await;
//     Ok(())
// }
