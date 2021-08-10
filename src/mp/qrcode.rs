use serde::{Deserialize, Serialize};

use crate::error::SdkResult;
use crate::wechat::ServerConfig;
use crate::{
    access_token::AccessTokenProvider,
    error::CommonResponse,
    wechat::{WxApiRequestBuilder, WxSdk},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct QRStruct {
    action_name: String,
    action_info: ActionInfo,

    #[serde(skip_serializing_if = "Option::is_none")]
    expire_seconds: Option<u32>,
}

impl QRStruct {
    pub fn new(name: String, expire_seconds: Option<u32>, scene_str: String) -> Self {
        QRStruct {
            action_name: name,
            expire_seconds,
            action_info: ActionInfo {
                scene: Scene {
                    scene_id: None,
                    scene_str: Some(scene_str),
                },
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionInfo {
    scene: Scene,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    scene_id: Option<i64>,
    scene_str: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QRValue {
    pub ticket: String,
    pub expire_seconds: Option<i32>,
    pub url: Option<String>,
}

pub struct QrcodeModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> QrcodeModule<'a, T> {
    pub async fn create_qrcode(&self, qr: QRStruct) -> SdkResult<QRValue> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/qrcode/create";
        let sdk = self.0;
        let res = sdk.wx_post(base_url).await?;
        let res = res
            .json(&qr)
            .send()
            .await?
            .json::<CommonResponse<QRValue>>()
            .await?;
        res.into()
    }
}
