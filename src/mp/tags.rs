use serde::{Deserialize, Serialize};

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

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

pub async fn create<T: AccessTokenProvider>(name: &str, sdk: &WxSdk<T>) -> SdkResult<Tag> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/tags/create";

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

pub async fn update<T: AccessTokenProvider>(tag: TagValue, sdk: &WxSdk<T>) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/tags/update?";

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

pub async fn delete<T: AccessTokenProvider>(tag_id: i32, sdk: &WxSdk<T>) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/tags/delete";

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

pub async fn get<T: AccessTokenProvider>(sdk: &WxSdk<T>) -> SdkResult<Tags> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/tags/get";

    let builder = sdk.wx_get(base_url).await?;
    let tag: CommonResponse<Tags> = builder.send().await?.json().await?;
    tag.into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TagidList {
    pub tagid_list: Vec<String>,
}

pub async fn getidlist<T: AccessTokenProvider>(
    openid: &str,
    sdk: &WxSdk<T>,
) -> SdkResult<TagidList> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/tags/getidlist";

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

pub mod members {
    use crate::{
        access_token::AccessTokenProvider,
        error::{CommonError, CommonResponse},
        mp::user::UserList,
        wechat::{WxApiRequestBuilder, WxSdk},
        SdkResult,
    };

    pub async fn batchtagging<T: AccessTokenProvider>(
        list: Vec<String>,
        tagid: i32,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchtagging";

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

    pub async fn batchuntagging<T: AccessTokenProvider>(
        list: Vec<String>,
        tagid: i32,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchuntagging";

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

    pub async fn getblacklist<T: AccessTokenProvider>(
        begin_openid: &str,
        sdk: &WxSdk<T>,
    ) -> SdkResult<UserList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/getblacklist";

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

    pub async fn batchblacklist<T: AccessTokenProvider>(
        openid_list: &[String],
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchblacklist";

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

    pub async fn batchunblacklist<T: AccessTokenProvider>(
        openid_list: &[String],
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/tags/members/batchunblacklist";

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
