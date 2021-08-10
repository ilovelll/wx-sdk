use serde::{Deserialize, Serialize};

use crate::{
    error::{CommonError, CommonResponse},
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

use self::members::MembersModule;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "tag")]
pub struct Tag {
    pub tag: TagValue,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TagValue {
    // #[serde(skip_serializing)]
    pub id: i32,
    pub name: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Tags {
    pub tags: Vec<TagsValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagsValue {
    pub id: i32,
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagidList {
    pub tagid_list: Vec<String>,
}

pub struct TagsModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> TagsModule<'a, T> {
    pub fn members(&self) -> MembersModule<T> {
        MembersModule(self.0)
    }
    pub async fn create(&self, name: &str) -> SdkResult<Tag> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/create";
        let sdk = self.0;
        let tag: CommonResponse<Tag> = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({ "tag": { "name" : name } }))
            .send()
            .await?
            .json()
            .await?;

        tag.into()
    }

    pub async fn update(&self, tag: TagValue) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/update?";
        let sdk = self.0;
        let res: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&Tag { tag })
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    pub async fn delete(&self, tag_id: i32) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/delete";
        let sdk = self.0;
        let res: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({ "tag": { "id": tag_id } }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    pub async fn get(&self) -> SdkResult<Tags> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/get";
        let sdk = self.0;
        let builder = sdk.wx_get(base_url).await?;
        let tag: CommonResponse<Tags> = builder.send().await?.json().await?;
        tag.into()
    }

    pub async fn getidlist(&self, openid: &str) -> SdkResult<TagidList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/getidlist";
        let sdk = self.0;
        let res: CommonResponse<TagidList> = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({ "openid": openid }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }
}

pub mod members {
    use crate::{
        error::{CommonError, CommonResponse},
        mp::user::UserList,
        wechat::WxApiRequestBuilder,
        SdkResult,
    };
    pub struct MembersModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

    impl<'a, T: WxApiRequestBuilder> MembersModule<'a, T> {
        pub async fn batchtagging(&self, list: Vec<String>, tagid: i32) -> SdkResult<()> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchtagging";
            let sdk = self.0;
            let res: CommonError = sdk
                .wx_post(base_url)
                .await?
                .json(&serde_json::json!({"openid_list": list,"tagid": tagid}))
                .send()
                .await?
                .json()
                .await?;

            res.into()
        }

        pub async fn batchuntagging(&self, list: Vec<String>, tagid: i32) -> SdkResult<()> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchuntagging";
            let sdk = self.0;
            let res: CommonError = sdk
                .wx_post(base_url)
                .await?
                .json(&serde_json::json!({"openid_list": list,"tagid": tagid}))
                .send()
                .await?
                .json()
                .await?;

            res.into()
        }

        pub async fn getblacklist(&self, begin_openid: &str) -> SdkResult<UserList> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/getblacklist";
            let sdk = self.0;
            let res: CommonResponse<UserList> = sdk
                .wx_post(base_url)
                .await?
                .json(&serde_json::json!({ "begin_openid": begin_openid }))
                .send()
                .await?
                .json()
                .await?;
            res.into()
        }

        pub async fn batchblacklist(&self, openid_list: &[String]) -> SdkResult<()> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchblacklist";
            let sdk = self.0;
            let res: CommonError = sdk
                .wx_post(base_url)
                .await?
                .json(&serde_json::json!({ "openid_list": openid_list }))
                .send()
                .await?
                .json()
                .await?;

            res.into()
        }

        pub async fn batchunblacklist(&self, openid_list: &[String]) -> SdkResult<()> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchunblacklist";
            let sdk = self.0;
            let res: CommonError = sdk
                .wx_post(base_url)
                .await?
                .json(&serde_json::json!({ "openid_list": openid_list }))
                .send()
                .await?
                .json()
                .await?;

            res.into()
        }
    }
}
