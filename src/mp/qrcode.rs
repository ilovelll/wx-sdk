//! 生成带参数的二维码
//!
//! 目前有2种类型的二维码：
//! 1、临时二维码，是有过期时间的，最长可以设置为在二维码生成后的30天（即2592000秒）后过期，但能够生成较多数量。临时二维码主要用于帐号绑定等不要求二维码永久保存的业务场景 
//! 2、永久二维码，是无过期时间的，但数量较少（目前为最多10万个）。永久二维码主要用于适用于帐号绑定、用户来源统计等场景。
use serde::{Deserialize, Serialize};

use crate::error::SdkResult;
use crate::{
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
    /// 生成带参数的二维码
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
