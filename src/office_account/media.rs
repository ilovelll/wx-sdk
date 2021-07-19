use crate::SdkResult;
use crate::{
    access_token::AccessTokenProvider,
    error::CommonResponse,
    wechat::{WxApiRequestBuilder, WxSdk},
};
use serde::{Deserialize, Serialize};

use super::material::Articles;

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResponse {
    #[serde(rename = "type")]
    pub type_: String,
    pub media_id: String,
    pub created_at: i32,
}

pub async fn uploadvideo<T: AccessTokenProvider>(
    media_id: String,
    title: Option<String>,
    description: Option<String>,
    sdk: &WxSdk<T>,
) -> SdkResult<UploadResponse> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/media/uploadvideo";

    let builder = sdk.wx_post(base_url).await?;
    let res: CommonResponse<UploadResponse> = builder
        .json(&serde_json::json!({
            "media_id": media_id,
            "title": title,
            "description": description,
        }))
        .send()
        .await?
        .json()
        .await?;

    res.into()
}

/// A single "part" of a multipart/form-data body.
///
/// Yielded from the `FormData` stream.

pub struct Part {
    pub name: String,
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct PicUrl {
    pub url: String,
}

/// 上传永久素材，上传图文消息内的图片
pub async fn uploadimg<T: AccessTokenProvider>(form: Part, sdk: &WxSdk<T>) -> SdkResult<PicUrl> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/media/uploadimg";

    let part = reqwest::multipart::Part::bytes(form.data)
        .file_name(form.filename)
        .mime_str(form.content_type.as_ref());

    let form = reqwest::multipart::Form::new().part(form.name, part.unwrap());
    let builder = sdk.wx_post(base_url).await?;
    let res: CommonResponse<PicUrl> = builder.multipart(form).send().await?.json().await?;

    res.into()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaRes {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(alias = "thumb_media_id")]
    pub media_id: String,
    pub created_at: i64,
}

/// 新增临时素材，媒体文件在微信后台保存时间为3天，即3天后media_id失效
pub async fn upload<T: AccessTokenProvider>(
    m_type: &str,
    media: &[i8],
    sdk: &WxSdk<T>,
) -> SdkResult<MediaRes> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/media/upload";

    let builder = sdk.wx_post(base_url).await?;
    let builder = builder.query(&[("type", m_type)]);
    let res: CommonResponse<MediaRes> = builder.form(media).send().await?.json().await?;
    res.into()
}

///公众号可以使用本接口获取临时素材（即下载临时的多媒体文件）。
///本接口即为原“下载多媒体文件”接口。
pub async fn get<T: AccessTokenProvider>(
    media_id: &str,
    sdk: &WxSdk<T>,
) -> SdkResult<reqwest::Response> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/media/get";

    let builder = sdk.wx_get(base_url).await?;
    let builder = builder.query(&[("media_id", media_id)]);
    // 直接返回给前端处理了 https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_temporary_materials.html
    Ok(builder.send().await?)
}

/// 上传图文消息素材【订阅号与服务号认证后均可用】效
pub async fn uploadnews<T: AccessTokenProvider>(
    articles: &[Articles],
    sdk: &WxSdk<T>,
) -> SdkResult<MediaRes> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/media/uploadnews";

    let builder = sdk.wx_post(base_url).await?;
    let res: CommonResponse<MediaRes> = builder
        .json(&serde_json::json!({ "articles": articles }))
        .send()
        .await?
        .json()
        .await?;
    res.into()
}
