#![allow(non_camel_case_types)]

use crate::{
    error::{CommonError, CommonResponse, SdkError},
    wechat::{WxApiRequestBuilder, WxSdk},
};

use super::SdkResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BtnKeyType {
    click,
    scancode_waitmsg,
    scancode_push,
    pic_sysphoto,
    pic_photo_or_album,
    pic_weixin,
    location_select,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtnKey {
    #[serde(rename = "type")]
    pub type_: BtnKeyType,
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BtnUrlType {
    view,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtnUrl {
    #[serde(rename = "type")]
    pub type_: BtnUrlType,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BtnMediaType {
    /// 图片
    media_id,
    /// 图文消息
    view_limited,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtnMedia {
    #[serde(rename = "type")]
    pub type_: BtnMediaType,
    pub name: String,
    #[serde(alias = "value")]
    pub media_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtnValue {
    #[serde(rename = "type")]
    pub type_: BtnMediaType,
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BtnMiniprogram {
    pub type_: String,
    pub name: String,
    pub url: String,
    pub appid: String,
    pub pagepath: String,
}

/// 层级菜单
#[derive(Debug, Serialize, Deserialize)]
pub struct SubBtn {
    pub name: String,
    pub sub_button: Vec<Btn>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Btn {
    url(BtnUrl),
    key(BtnKey),
    media(BtnMedia),
    miniprogram(BtnMiniprogram),
    sub(SubBtn),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuInfo {
    pub is_menu_open: i8,
    pub selfmenu_info: SelfmenuInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfmenuInfo {
    pub button: Vec<ButtonInfo2>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ButtonInfo {
    url(BtnUrl),
    key(BtnKey),
    media(BtnMedia),
    miniprogram(BtnMiniprogram),
    sub(SubButtonList),
}
/// 查询接口是这种结构，但是创建接口不是
#[derive(Debug, Serialize, Deserialize)]
pub struct SubButtonList {
    name: String,
    sub_button: SubButtonInfo,
}

/// 查询接口是这种结构，但是创建接口不是
#[derive(Debug, Serialize, Deserialize)]
pub struct SubButtonInfo {
    list: Vec<ButtonInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonInfo2 {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub name: String,
    pub value: Option<String>,
    pub url: Option<String>,
    pub key: Option<String>,
    pub appid: Option<String>,
    pub pagepath: Option<String>,
    pub sub_button: Option<SubButtonInfo2>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubButtonInfo2 {
    pub list: Vec<ButtonInfo2>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_platform_type: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl MatchRule {
    fn is_valid(&self) -> bool {
        match (
            self.tag_id,
            self.sex,
            self.country.as_ref(),
            self.province.as_ref(),
            self.city.as_ref(),
            self.client_platform_type,
            self.language.as_ref(),
        ) {
            (None, None, None, None, None, None, None) => false,
            (_, _, None, Some(_), _, _, _) => false,
            (_, _, _, None, Some(_), _, _) => false,
            _ => true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum MenuButton {
    button(ButtonItem),
    root_button(RootButton),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ButtonItem {
    view(ButtonView),
    click(ButtonClick),
    miniprogram(ButtonMiniProgram),
    scancode_waitmsg(ButtonClick),
    scancode_push(ButtonClick),
    pic_sysphoto(ButtonClick),
    pic_photo_or_album(ButtonClick),
    pic_weixin(ButtonClick),
    location_select(ButtonClick),
    media_id(ButtonMedia),
    view_limited(ButtonMedia),
}

impl From<ButtonItem> for MenuButton {
    fn from(btns: ButtonItem) -> Self {
        MenuButton::button(btns)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootButton {
    pub name: String,
    pub sub_button: Vec<ButtonItem>,
}

impl From<RootButton> for MenuButton {
    fn from(r_btn: RootButton) -> Self {
        MenuButton::root_button(r_btn)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonView {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonClick {
    pub name: String,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonMiniProgram {
    pub name: String,
    pub url: String,
    pub appid: String,
    pub pagepath: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ButtonMedia {
    pub name: String,
    pub media_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuId {
    pub menuid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MatchButtons {
    pub button: Vec<MenuButton>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub menuid: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub matchrule: Option<MatchRule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllButtons {
    pub menu: MatchButtons,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditionalmenu: Option<Vec<MatchButtons>>,
}

/// 菜单模块
pub struct MenuModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> MenuModule<'a, T> {
    /// 创建自定义菜单
    pub async fn create(&self, menu: Vec<Btn>) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/create";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonError = builder
            .json(&serde_json::json!({ "button": menu }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    /// 创建自定义菜单（通过自定义的json数据）
    pub async fn create_by_json<U: Serialize + ?Sized>(&self, menu_json: &U) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/create";
        let sdk = self.0;
        let res: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(menu_json)
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }
    /// 获取当前菜单信息
    pub async fn get_current_selfmenu_info(&self) -> SdkResult<MenuInfo> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/get_current_selfmenu_info";
        let sdk = self.0;
        let res: CommonResponse<MenuInfo> =
            sdk.wx_get(base_url).await?.send().await?.json().await?;

        res.into()
    }

    /// 删除自定义菜单
    /// 调用此接口会删除默认菜单及全部个性化菜单
    pub async fn delete(&self) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/delete";
        let sdk = self.0;
        let res: CommonError = sdk.wx_get(base_url).await?.send().await?.json().await?;
        res.into()
    }

    /// 添加个性化菜单
    pub async fn addconditional(
        &self,
        rules: MatchRule,
        menu_json: Vec<MenuButton>,
    ) -> SdkResult<MenuId> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/addconditional";

        if !rules.is_valid() {
            return Err(SdkError::InvalidParams(
                "add conditional menu match rules invalid.".to_string(),
            ));
        }
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<MenuId> = builder
            .json(&serde_json::json!({
                "button": &menu_json,
                "matchrule": rules
            }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    /// 删除个性化菜单
    pub async fn delconditional(&self, menuid: MenuId) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/delconditional";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let msg: CommonError = builder.json(&menuid).send().await?.json().await?;

        msg.into()
    }

    /// 测试个性化菜单匹配结果
    /// user_id可以是粉丝的OpenID，也可以是粉丝的微信号。
    pub async fn trymatch(&self, user_id: String) -> SdkResult<MatchButtons> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/trymatch";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let msg: CommonResponse<MatchButtons> = builder
            .json(&serde_json::json!({ "user_id": &user_id }))
            .send()
            .await?
            .json()
            .await?;

        msg.into()
    }
    /// 获取自定义菜单配置
    /// 在设置了个性化菜单后，使用本自定义菜单查询接口可以获取默认菜单和全部个性化菜单信息
    pub async fn get(&self) -> SdkResult<AllButtons> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/menu/get";
        let sdk = self.0;
        let res: CommonResponse<AllButtons> =
            sdk.wx_get(base_url).await?.send().await?.json().await?;

        res.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::discriminant;
    // use serde::{Serialize, Deserialize};

    #[test]
    fn deserialize_menu() -> std::result::Result<(), &'static str> {
        let menu_json = "
    [{
        \"type\": \"view\",
        \"name\": \"今日歌曲\",
        \"url\": \"V1001_TODAY_MUSIC\"
    }, {
      \"name\": \"菜单\",
      \"sub_button\": [
        {
          \"type\": \"scancode_push\",
          \"name\": \"扫码推事件\",
          \"key\": \"rselfmenu_0_1\"
        }, {
          \"type\": \"media_id\",
          \"name\": \"图片\",
          \"media_id\": \"V1001_GOOD\"
        }]
    }]
       ";
        // }, {
        //     \"type\": \"miniprogram\",
        //     \"name\": \"wxa\",
        //     \"url\": \"http://mp.weixin.qq.com\",
        //     \"appid\": \"wx286b93c14bbf93aa\",
        //     \"pagepath\": \"pages/lunar/index\"

        let menu: Vec<Btn> = serde_json::from_str(&menu_json).unwrap();
        // println!("{:#?}", &menu);

        match &menu[0] {
            Btn::url(btn) => {
                assert_eq!(discriminant(&btn.type_), discriminant(&BtnUrlType::view));
            }
            _ => return Err("match &menu[0]"),
        }
        match &menu[1] {
            Btn::sub(btn) => {
                assert_eq!(btn.name.as_str(), "菜单");
                match &btn.sub_button[0] {
                    Btn::key(btn) => {
                        assert_eq!(
                            discriminant(&btn.type_),
                            discriminant(&BtnKeyType::scancode_push)
                        );
                    }
                    _ => return Err("match &btn.sub_button[0]"),
                }
                match &btn.sub_button[1] {
                    Btn::media(btn) => {
                        assert_eq!(
                            discriminant(&btn.type_),
                            discriminant(&BtnMediaType::media_id)
                        );
                    }
                    _ => return Err("match &btn.sub_button[1]"),
                }
            }
            _ => return Err("match &menu[1]"),
        }

        Ok(())
    }

    #[test]
    fn deserialize_allmenu1() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let input = r#"{
    "menu": {
        "button": [
            {
                "type": "click", 
                "name": "今日歌曲", 
                "key": "V1001_TODAY_MUSIC", 
                "sub_button": [ ]
            }, 
            {
                "type": "click", 
                "name": "歌手简介", 
                "key": "V1001_TODAY_SINGER", 
                "sub_button": [ ]
            }, 
            {
                "name": "菜单", 
                "sub_button": [
                    {
                        "type": "view", 
                        "name": "搜索", 
                        "url": "http://www.soso.com/", 
                        "sub_button": [ ]
                    }, 
                    {
                        "type": "view", 
                        "name": "视频", 
                        "url": "http://v.qq.com/", 
                        "sub_button": [ ]
                    }, 
                    {
                        "type": "click", 
                        "name": "赞一下我们", 
                        "key": "V1001_GOOD", 
                        "sub_button": [ ]
                    }
                ]
            }
        ]
    }
}"#;
        let menu: AllButtons = serde_json::from_str(&input).unwrap();
        // println!("{:#?}", &menu);
        Ok(())
    }

    #[test]
    fn deserialize_allmenu2() -> std::result::Result<(), Box<dyn std::error::Error>> {
        let input = r#"{"menu": {
                    "button": [
                        {
                            "type": "click", 
                            "name": "今日歌曲", 
                            "key": "V1001_TODAY_MUSIC", 
                            "sub_button": [ ]
                        }
                    ], 
                    "menuid": 208396938
                }, 
                "conditionalmenu": [
                    {
                        "button": [
                            {
                                "type": "click", 
                                "name": "今日歌曲", 
                                "key": "V1001_TODAY_MUSIC", 
                                "sub_button": [ ]
                            }, 
                            {
                                "name": "菜单", 
                                "sub_button": [
                                    {
                                        "type": "view", 
                                        "name": "搜索", 
                                        "url": "http://www.soso.com/", 
                                        "sub_button": [ ]
                                    }, 
                                    {
                                        "type": "view", 
                                        "name": "视频", 
                                        "url": "http://v.qq.com/", 
                                        "sub_button": [ ]
                                    }, 
                                    {
                                        "type": "click", 
                                        "name": "赞一下我们", 
                                        "key": "V1001_GOOD", 
                                        "sub_button": [ ]
                                    }
                                ]
                            }
                        ], 
                        "matchrule": {
                            "group_id": 2, 
                            "sex": 1, 
                            "country": "中国", 
                            "province": "广东", 
                            "city": "广州", 
                            "client_platform_type": 2
                        }, 
                        "menuid": 208396993
                    }
                ]
}"#;
        let menu: AllButtons = serde_json::from_str(&input).unwrap();
        // println!("{:#?}", &menu);
        Ok(())
    }
}
