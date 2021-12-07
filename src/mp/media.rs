//! Media Module （临时）素材文件接口相关
use crate::SdkResult;
use crate::{error::CommonResponse, wechat::WxApiRequestBuilder};
use serde::{Deserialize, Serialize};

use super::material::Articles;
#[derive(Serialize, Deserialize)]
pub struct MediaId {
    pub media_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResponse {
    #[serde(rename = "type")]
    pub type_: String,
    pub media_id: String,
    pub created_at: i32,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaRes {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(alias = "thumb_media_id")]
    pub media_id: String,
    pub created_at: i64,
}
/// Media Module （临时）素材文件接口相关
pub struct MediaModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);
impl<'a, T: WxApiRequestBuilder> MediaModule<'a, T> {
    /// 群发消息中的视频需要经过此接口再次上传得到 media_id 再群发
    pub async fn uploadvideo<S: AsRef<str>>(
        &self,
        media_id: S,
        title: Option<String>,
        description: Option<String>,
    ) -> SdkResult<UploadResponse> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/media/uploadvideo";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<UploadResponse> = builder
            .json(&serde_json::json!({
                "media_id": media_id.as_ref().to_string(),
                "title": title,
                "description": description,
            }))
            .send()
            .await?
            .json()
            .await?;

        res.into()
    }

    /// 上传永久素材，上传图文消息内的图片
    /// 本接口所上传的图片不占用公众号的素材库中图片数量的5000个的限制。图片仅支持jpg/png格式，大小必须在1MB以下。
    pub async fn uploadimg(&self, form: Part) -> SdkResult<PicUrl> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/media/uploadimg";
        let sdk = self.0;
        let part = reqwest::multipart::Part::bytes(form.data)
            .file_name(form.filename)
            .mime_str(form.content_type.as_ref());

        let form = reqwest::multipart::Form::new().part(form.name, part.unwrap());
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<PicUrl> = builder.multipart(form).send().await?.json().await?;

        res.into()
    }

    /// 新增临时素材，媒体文件在微信后台保存时间为3天，即3天后media_id失效
    pub async fn upload(&self, media_type: &str, media: &[i8]) -> SdkResult<MediaRes> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/media/upload";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let builder = builder.query(&[("type", media_type)]);
        let res: CommonResponse<MediaRes> = builder.form(media).send().await?.json().await?;
        res.into()
    }

    ///公众号可以使用本接口获取临时素材（即下载临时的多媒体文件）。
    ///本接口即为原“下载多媒体文件”接口。
    pub async fn get<S: AsRef<str>>(&self, media_id: S) -> SdkResult<reqwest::Response> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/media/get";
        let sdk = self.0;
        let builder = sdk.wx_get(base_url).await?;
        let builder = builder.query(&[("media_id", media_id.as_ref())]);
        // 直接返回给前端处理了 https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/Get_temporary_materials.html
        Ok(builder.send().await?)
    }

    /// 上传图文消息素材【订阅号与服务号认证后均可用】效
    pub async fn uploadnews(&self, articles: &[Articles]) -> SdkResult<MediaRes> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/media/uploadnews";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let res: CommonResponse<MediaRes> = builder
            .json(&serde_json::json!({ "articles": articles }))
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
}
