use serde::{Deserialize, Serialize};

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
    wechat::WxApiRequestBuilder,
    SdkResult, WxSdk,
};

use super::draft::NewsItemList;

/// 发布能力接口模块
pub struct FreePublishModule<'a, T: AccessTokenProvider>(pub(crate) &'a WxSdk<T>);

#[derive(Serialize, Deserialize)]
pub struct PublishId {
    publish_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublishResult {
    pub publish_id: String,
    pub publish_status: i8,
    pub article_id: String,
    pub article_detail: Option<ArticleDetail>,
    pub fail_idx: Vec<i8>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleDetail {
    pub count: i8,
    pub item: Vec<ArticleDetailItem>,
}

#[derive(Serialize, Deserialize)]
pub struct ArticleDetailItem {
    pub idx: i8,
    pub article_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct BatchList {
    pub total_count: i32,
    pub item_count: i32,
    pub item: Vec<PublishListItem>,
}

#[derive(Serialize, Deserialize)]
pub struct PublishListItem {
    pub article_id: String,
    pub content: NewsItemList,
}

impl<'a, T: AccessTokenProvider> FreePublishModule<'a, T> {
    /// 发布草稿
    /// 开发者需要先将图文素材以草稿的形式保存（见“草稿箱/新建草稿”，如需从已保存的草稿中选择，见“草稿箱/获取草稿列表”），选择要发布的草稿 media_id 进行发布
    pub async fn submit(&self, media_id: &str) -> SdkResult<PublishId> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/freepublish/submit";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<PublishId> = builder
            .json(&serde_json::json!({ "media_id": media_id }))
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    /// 开发者可以尝试通过下面的发布状态轮询接口获知发布情况。
    pub async fn get(&self, publish_id: &str) -> SdkResult<PublishResult> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/freepublish/get";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<PublishResult> = builder
            .json(&serde_json::json!({ "publish_id": publish_id }))
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    /// 发布成功之后，随时可以通过该接口删除。此操作不可逆，请谨慎操作。
    pub async fn delete(&self, article_id: &str, index: i8) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/freepublish/delete";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonError = builder
            .json(&serde_json::json!({ "article_id": article_id, "index": index }))
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    /// 开发者可以通过 article_id 获取已发布的图文信息。
    pub async fn get_article(&self, article_id: &str) -> SdkResult<NewsItemList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/freepublish/getarticle";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<NewsItemList> = builder
            .json(&serde_json::json!({ "article_id": article_id }))
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    /// 开发者可以获取已成功发布的消息列表。
    pub async fn batchget(&self, offset: i64, count: i64, no_content: i8) -> SdkResult<BatchList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/freepublish/batchget";
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
