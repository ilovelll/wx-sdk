use serde::{Deserialize, Serialize};

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
    wechat::WxApiRequestBuilder,
    SdkResult, WxSdk,
};

use super::media::MediaId;

pub struct DraftModule<'a, T: AccessTokenProvider>(pub(crate) &'a WxSdk<T>);

#[derive(Serialize, Deserialize)]
pub struct Articles {
    pub title: String,
    pub author: String,
    pub digest: String,
    pub content: String,
    pub content_source_url: String,
    pub thumb_media_id: String,
    /// 是否显示封面，0为false，即不显示，1为true，即显示
    pub show_cover_pic: i8,
    pub need_open_comment: Option<i8>,
    pub only_fans_can_comment: Option<i8>,
    #[serde(skip_serializing)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct NewsItemList {
    pub news_item: Vec<Articles>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateNews {
    pub media_id: String,
    /// 要更新的文章在图文消息中的位置（多图文消息时，此字段才有意义），第一篇为0
    pub index: i32,
    /// 在开发文档中这两个字段: `need_open_comment`, `only_fans_can_comment` 在更新时是没有提到的
    pub articles: Articles,
}

#[derive(Serialize, Deserialize)]
pub struct TotalCount {
    pub total_count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewsItem {
    pub media_id: String,
    pub content: NewsItemList,
    pub update_time: i64,
}
#[derive(Serialize, Deserialize)]
pub struct BatchList {
    pub total_count: i32,
    pub item_count: i32,
    pub item: Vec<NewsItem>,
}

impl<'a, T: AccessTokenProvider> DraftModule<'a, T> {
    /// 新增草稿
    pub async fn add(&self, articles: &[Articles]) -> SdkResult<MediaId> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/draft/add";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<MediaId> = builder
            .json(&serde_json::json!({ "articles": articles }))
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    /// 获取草稿
    pub async fn get(&self, media_id: &str) -> SdkResult<NewsItemList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/draft/get";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<NewsItemList> = builder
            .json(&serde_json::json!({ "media_id": media_id }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    /// 删除草稿
    pub async fn delete(&self, media_id: &str) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/draft/delete";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonError = builder
            .json(&serde_json::json!({ "media_id": media_id }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    /// 修改草稿
    pub async fn update(&self, news: &UpdateNews) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/draft/update";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;

        let res: CommonError = builder.json(news).send().await?.json().await?;

        res.into()
    }

    /// 获取草稿数量
    pub async fn count(&self) -> SdkResult<TotalCount> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/draft/count";
        let sdk = self.0;
        let res: CommonResponse<TotalCount> =
            sdk.wx_get(base_url).await?.send().await?.json().await?;
        res.into()
    }

    /// 批量获取草稿
    pub async fn batchget(&self, offset: u32, count: u32, no_content: u32) -> SdkResult<BatchList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/draft/batchget";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<BatchList> = builder
            .json(
                &serde_json::json!({ "offset": offset, "count": count, "no_content": no_content }),
            )
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }
}
