use serde::{Deserialize, Serialize};

use crate::{
    error::{CommonError, CommonResponse},
    wechat::WxApiRequestBuilder,
    SdkResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserList {
    pub total: Option<i32>,
    pub data: OpenidList,
    pub count: i32,
    pub next_openid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenidList {
    pub openid: Vec<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserInfo {
    pub openid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub subscribe: i8,
    pub openid: String,
    pub nickname: String,
    pub sex: i8,
    pub city: String,
    pub country: String,
    pub province: String,
    pub language: String,
    pub headimgurl: String,
    pub subscribe_time: i64,
    pub unionid: Option<String>,
    pub remark: Option<String>,
    pub groupid: i32,
    pub tagid_list: Vec<i32>,
    pub subscribe_scene: String,
    pub qr_scene: Option<i32>,
    pub qr_scene_str: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfoList {
    pub user_info_list: Vec<UserInfoItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserInfoItem {
    Subscribe(UserInfo),
    Unsubscribe { openid: String },
}

pub struct UserModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> UserModule<'a, T> {
    pub async fn get(&self, next_openid: Option<String>) -> SdkResult<UserList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/user/get";
        let sdk = self.0;
        let mut builder = sdk.wx_get(base_url).await?;
        if let Some(next) = next_openid {
            builder = builder.query(&[("next_openid", next)])
        }

        let res: CommonResponse<UserList> = builder.send().await?.json().await?;

        res.into()
    }

    pub async fn info(&self, openid: &str, lang: &str) -> SdkResult<UserInfo> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/user/info";
        let sdk = self.0;
        let builder = sdk.wx_get(base_url).await?;
        let builder = builder.query(&[("openid", openid), ("lang", lang)]);
        let res: CommonResponse<UserInfo> = builder.send().await?.json().await?;

        res.into()
    }

    pub async fn tag_get(&self, tagid: i32, next_openid: &str) -> SdkResult<UserList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/user/tag/get";
        let sdk = self.0;
        let res: CommonResponse<UserList> = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({ "tagid": tagid, "next_openid": next_openid }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    pub async fn info_updateremark(&self, openid: &str, remark: &str) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/user/info/updateremark";
        let sdk = self.0;
        let res: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({ "openid": openid, "remark": remark }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    pub async fn info_batchget(&self, query: &[QueryUserInfo]) -> SdkResult<UserInfoList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/user/info/batchget";
        let sdk = self.0;
        let res: CommonResponse<UserInfoList> = sdk
            .wx_post(base_url)
            .await?
            .json(query)
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_user_info_item() -> Result<(), &'static str> {
        let json = r#"
        [
            {
                "subscribe": 1, 
                "openid": "otvxTs4dckWG7imySrJd6jSi0CWE", 
                "nickname": "iWithery", 
                "sex": 1, 
                "language": "zh_CN", 
                "city": "揭阳", 
                "province": "广东", 
                "country": "中国", 
                "headimgurl": "http://thirdwx.qlogo.cn/mmopen/xbIQx1GRqdvyqkMMhEaGOX802l1CyqMJNgUzKP8MeAeHFicRDSnZH7FY4XB7p8XHXIf6uJA2SCunTPicGKezDC4saKISzRj3nz/0",
                "subscribe_time": 1434093047, 
                "unionid": "oR5GjjgEhCMJFyzaVZdrxZ2zRRF4", 
                "remark": "", 
                "groupid": 0,
                "tagid_list":[128,2],
                "subscribe_scene": "ADD_SCENE_QR_CODE",
                "qr_scene": 98765,
                "qr_scene_str": ""
            }, {
                "subscribe": 0, 
                "openid": "otvxTs_JZ6SEiP0imdhpi50fuSZg"
            }
        ]
        "#;
        let users: Vec<UserInfoItem> = serde_json::from_str(&json).unwrap();
        // println!("{:#?}", &users);
        match &users[0] {
            UserInfoItem::Subscribe(user) => assert_eq!(user.tagid_list, vec![128, 2]),
            _ => return Err("match &users[0]"),
        }
        match &users[1] {
            UserInfoItem::Unsubscribe { openid } => {
                assert_eq!(openid, "otvxTs_JZ6SEiP0imdhpi50fuSZg")
            }
            _ => return Err("match &users[1]"),
        }

        Ok(())
    }
}
