# wx_func
![CI](https://github.com/ilovelll/wx_func/workflows/CI.yml/badge.svg)

## `wx_func` is a [WeChat SDK](https://mp.weixin.qq.com/) written in [Rust](https://www.rust-lang.org/).
## QuickStart

First, please refer to this [page](https://developers.weixin.qq.com/doc/offiaccount/Basic_Information/Access_Overview.html) to provide these values: `token`, `EncodingAESKey`,`EncodingMode`.
```rust
use wx_func::wechat::{ServerConfig, EncodingMode};

let config = ServerConfig::new(token, Some("aes_key"), EncodingMode::Plain);
let sdk = WxSdk::new_with_default_token_client("app_id", "app_secret", config);
```
Then, you can use the sdk functions, like get current menu info:
```rust
let mpsdk = WxSdk::mp(&sdk);
let menu = mpsdk.menu().get_current_selfmenu_info().await;
```
## Features

- [x] get access token
- [x] clear quota
- [ ] custom menu
  - [x] [create custom defined menu](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Creating_Custom-Defined_Menu.html)
  - [x] [get current menu info](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Querying_Custom_Menus.html)
  - [x] [delete custom defined menu](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Deleting_Custom-Defined_Menu.html)
  - [x] [add conditional menu ](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Personalized_menu_interface.html#0)
  - [x] [delete conditional menu](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Personalized_menu_interface.html#1)
  - [ ] [try match menu](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Personalized_menu_interface.html#2)
  - [ ] [get all menu info](https://developers.weixin.qq.com/doc/offiaccount/Custom_Menus/Getting_Custom_Menu_Configurations.html)
- [ ] parse received event
- [ ] customer service
- [x] assert managerment
  - [x] [add temporary material](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html)
  - [x] [get temporary materials](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_temporary_materials.html)
  - [x] [add permanent asset](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Adding_Permanent_Assets.html)
  - [x] [get permanent asset](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Getting_Permanent_Assets.html)
  - [x] [delete permanent asset](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Deleting_Permanent_Assets.html)
  - [x] [edit permanent rich media asset](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Editing_Permanent_Rich_Media_Assets.html)
  - [x] [get the total count of all materials](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_the_total_of_all_materials.html)
  - [x] [get materials list](https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_materials_list.html)
- [ ] [comments management](https://developers.weixin.qq.com/doc/offiaccount/Comments_management/Image_Comments_Management_Interface.html)
- [x] [user management]
  - [x] [user tag management](https://developers.weixin.qq.com/doc/offiaccount/User_Management/User_Tag_Management.html)
    - [x] add user tag
    - [x] get user tags
    - [x] edit user tag
    - [x] delete user tag
    - [x] get user by tag
    - [x] batch tagging users
  - [x] [update user remark](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Configuring_user_notes.html)
  - [x] [get user info](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Get_users_basic_information_UnionID.html#UinonId)
  - [x] [batch get user info](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Get_users_basic_information_UnionID.html#UinonId)
  - [x] [get subscribe openid list](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Getting_a_User_List.html)
  - [x] [get blocklist](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Manage_blacklist.html)
  - [x] [batch add blocklist](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Manage_blacklist.html)
  - [x] [batch undo blocklist](https://developers.weixin.qq.com/doc/offiaccount/User_Management/Manage_blacklist.html)

## Contributing

Issue reports and Pull Requests are always welcome!

## License

wx_func is available under the [_MIT License_](https://github.com/ilovelll/wx_func/blob/main/LICENSE)