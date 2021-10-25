use super::{post_send, Part};
use crate::{error::CommonResponse, wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

pub enum ImgData {
    /// 要检测的图片 url
    ImgUrl(String),
    /// form-data 中媒体文件标识，有filename、filelength、content-type等信息
    Img(Part),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgCrop {
    pub results: Vec<CorpResult>,
    pub img_size: WH,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CorpResult {
    pub crop_left: i32,
    pub crop_top: i32,
    pub crop_right: i32,
    pub crop_bottom: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WH {
    pub w: i32,
    pub h: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanQRCode {
    pub code_results: Vec<CodeResult>,
    pub img_size: WH,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeResult {
    pub type_name: String,
    pub data: String,
    pub pos: Option<Pos>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pos {
    pub left_top: XY,
    pub right_top: XY,
    pub right_bottom: XY,
    pub left_bottom: XY,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct XY {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaId {
    pub media_id: String,
}

pub(crate) async fn post_img_data<'a, A: WxApiRequestBuilder, R: serde::de::DeserializeOwned>(
    api_builder: &'a A,
    url: &'static str,
    data: ImgData,
) -> SdkResult<R> {
    match data {
        ImgData::ImgUrl(img_url) => {
            let data = &serde_json::json!({ "img_url": img_url });
            post_send(api_builder, url, data).await
        }
        ImgData::Img(data) => {
            let part = reqwest::multipart::Part::bytes(data.data)
                .file_name(data.filename)
                .mime_str(&data.content_type);

            let form = reqwest::multipart::Form::new().part(data.name, part.unwrap());
            let builder = api_builder.wx_post(url).await?.multipart(form);
            let res: CommonResponse<R> = builder.send().await?.json().await?;

            res.into()
        }
    }
}

pub struct ImgModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> ImgModule<'a, T> {
    /// 本接口提供基于小程序的图片智能裁剪能力。
    pub async fn ai_crop(&self, data: ImgData) -> SdkResult<ImgCrop> {
        let url = "https://api.weixin.qq.com/cv/img/aicrop";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的条码/二维码识别的API。
    pub async fn scan_qrcode(&self, data: ImgData) -> SdkResult<ScanQRCode> {
        let url = "https://api.weixin.qq.com/cv/img/qrcode";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的图片高清化能力。
    pub async fn superresolution(&self, data: ImgData) -> SdkResult<MediaId> {
        let url = "https://api.weixin.qq.com/cv/img/superresolution";
        post_img_data(self.0, url, data).await
    }
}
