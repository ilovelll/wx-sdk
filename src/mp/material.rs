use crate::SdkResult;
use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
    wechat::{WxApiRequestBuilder, WxSdk},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaRes {
    #[serde(rename = "type")]
    pub type_: String,
    pub media_id: String,
    pub created_at: i64,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub enum MediaType {
//     image,
//     voice,
//     video,
//     thumb,
// }

#[derive(Serialize, Deserialize)]
pub struct MediaId {
    pub media_id: String,
}

// pub type MediaId = String;

#[derive(Serialize, Deserialize)]
struct MediaUrl {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Material {
    Video(Video),
    NewsItemList(NewsItemList),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Video {
    pub title: String,
    pub description: String,
    pub down_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsItemList {
    pub news_item: Vec<NewsItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsItem {
    pub title: String,
    pub thumb_media_id: String,
    /// 是否显示封面，0为false，即不显示，1为true，即显示
    pub show_cover_pic: i8,
    pub author: String,
    pub digest: String,
    pub content: String,
    pub url: String,
    pub content_source_url: String,
    #[serde(default)]
    pub thumb_url: String,
    #[serde(default)]
    pub need_open_comment: i8,
    #[serde(default)]
    pub only_fans_can_comment: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Articles {
    pub title: String,
    pub thumb_media_id: String,
    pub author: String,
    pub digest: String,
    /// 是否显示封面，0为false，即不显示，1为true，即显示
    pub show_cover_pic: i8,
    pub content: String,
    pub content_source_url: String,
    pub need_open_comment: Option<i8>,
    pub only_fans_can_comment: Option<i8>,
}

pub async fn add_news<T: AccessTokenProvider>(
    articles: &[Articles],
    sdk: &WxSdk<T>,
) -> SdkResult<MediaId> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/add_news";

    let builder = sdk.wx_post(base_url).await?;
    let res: CommonResponse<MediaId> = builder
        .json(&serde_json::json!({ "articles": articles }))
        .send()
        .await?
        .json()
        .await?;
    res.into()
}

#[derive(Debug, Serialize)]
pub struct VideoDesc {
    pub title: String,
    pub introduction: String,
}

pub struct FileStruct {
    pub filetype: String,
    pub file: Vec<u8>,
    pub filename: String,
    pub conten_type: String,
    pub video_desc: Option<VideoDesc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddMaterialRes {
    pub media_id: String,
    /// 新增的图片素材的图片URL（仅新增图片素材时会返回该字段）
    pub url: Option<String>,
}

/// 新增其他类型永久素材
pub async fn add_material<T: AccessTokenProvider>(
    file: FileStruct,
    sdk: &WxSdk<T>,
) -> SdkResult<AddMaterialRes> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/add_material";
    let part = reqwest::multipart::Part::bytes(file.file)
        .file_name(file.filename)
        .mime_str(file.conten_type.as_ref());

    let mut form = reqwest::multipart::Form::new().part("media", part.unwrap());

    if file.filetype == "video" {
        form = form.text(
            "description",
            serde_json::to_string(&file.video_desc).unwrap(),
        );
    }

    let builder = sdk.wx_post(base_url).await?;
    let builder = builder.query(&[("type", file.filetype)]);
    let res: CommonResponse<AddMaterialRes> = builder.multipart(form).send().await?.json().await?;
    res.into()
}

pub async fn get_material<T: AccessTokenProvider>(
    media_id: &str,
    sdk: &WxSdk<T>,
) -> SdkResult<Material> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/get_material";
    let builder = sdk.wx_post(base_url).await?;
    let res: CommonResponse<Material> = builder
        .json(&serde_json::json!({ "media_id": media_id }))
        .send()
        .await?
        .json()
        .await?;

    res.into()
}

pub async fn get_material_other<T: AccessTokenProvider>(
    media_id: &str,
    sdk: &WxSdk<T>,
) -> SdkResult<Vec<u8>> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/get_material";

    let res = sdk
        .wx_post(base_url)
        .await?
        .json(&serde_json::json!({ "media_id": media_id }))
        .send()
        .await?
        .bytes()
        .await?;

    Ok(res.to_vec())
}

pub async fn del_material<T: AccessTokenProvider>(media_id: &str, sdk: &WxSdk<T>) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/del_material";
    let builder = sdk.wx_post(base_url).await?;

    let res: CommonError = builder
        .json(&serde_json::json!({ "media_id": media_id }))
        .send()
        .await?
        .json()
        .await?;

    res.into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNews {
    pub media_id: String,
    /// 要更新的文章在图文消息中的位置（多图文消息时，此字段才有意义），第一篇为0
    pub index: i32,
    /// 在开发文档中这两个字段: `need_open_comment`, `only_fans_can_comment` 在更新时是没有提到的
    pub articles: Articles,
}

/// 修改永久图文素材
pub async fn update_news<T: AccessTokenProvider>(
    news: &UpdateNews,
    sdk: &WxSdk<T>,
) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/update_news";
    let builder = sdk.wx_post(base_url).await?;

    let res: CommonError = builder.json(news).send().await?.json().await?;

    res.into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialCount {
    pub voice_count: i32,
    pub video_count: i32,
    pub image_count: i32,
    pub news_count: i32,
}

pub async fn get_materialcount<T: AccessTokenProvider>(sdk: &WxSdk<T>) -> SdkResult<MaterialCount> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/get_materialcount";

    let res: CommonResponse<MaterialCount> =
        sdk.wx_get(base_url).await?.send().await?.json().await?;
    res.into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Batch {
    #[serde(rename = "type")]
    pub type_: String,
    /// 从全部素材的该偏移位置开始返回，0表示从第一个素材 返回
    pub offset: i32,
    /// 返回素材的数量，取值在1到20之间
    pub count: u8,
}

pub async fn batchget_material<T: AccessTokenProvider>(
    batch: &Batch,
    sdk: &WxSdk<T>,
) -> SdkResult<MaterialList> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/material/batchget_material";
    let builder = sdk.wx_post(base_url).await?;
    let res: CommonResponse<MaterialList> = builder.json(batch).send().await?.json().await?;

    res.into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MaterialList {
    pub total_count: u32,
    pub item_count: u32,
    pub item: MaterialItemList,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaterialItemList {
    MediaInfo(Vec<MediaInfo>),
    News(Vec<NewsInfo>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsInfo {
    pub media_id: String,
    pub content: NewsItemList,
    pub update_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaInfo {
    pub media_id: String,
    pub name: String,
    pub update_time: i64,
    pub url: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}
