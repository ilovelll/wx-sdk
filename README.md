# wx-sdk
[![CI](https://github.com/ilovelll/wx-sdk/workflows/CI/badge.svg)](https://github.com/ilovelll/wx-sdk/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/wx-sdk)](https://crates.io/crates/wx-sdk)
[![docs](https://docs.rs/wx-sdk/badge.svg)](https://docs.rs/wx-sdk)

Warning 🚧 (WIP), this crate is undering develop, api may be changed.
## `wx-sdk` is a [WeChat SDK](https://mp.weixin.qq.com/) written in [Rust](https://www.rust-lang.org/).
## QuickStart

First, please refer to this [page](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Access_Overview.html) to provide these values: `token`, `EncodingAESKey`,`EncodingMode`.
```rust
use wx_sdk::wechat::WxSdk;
let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret", config);
```
Then, you can use the sdk functions, like get current menu info:
```rust
use wx_sdk::mp::{ServerConfig, EncodingMode};
let config = ServerConfig::new("token", EncodingMode::Plain);
let mpsdk = WxSdk::mp(sdk, config); // or sdk.mp(config);
let menu = mpsdk.menu().get_current_selfmenu_info().await;
```
## Features
There're `mp`, `pay`, `wxa` features gates, we have only implemented the `mp` feature now. Please check [`FEATURES.md`](https://github.com/ilovelll/wx-sdk/blob/main/FEATURES.md)

## Contributing

Issue reports and Pull Requests are always welcome!

## License

wx-sdk is available under the [_MIT License_](https://github.com/ilovelll/wx-sdk/blob/main/LICENSE)