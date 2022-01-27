use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InvokeService {
    /// 服务 ID
    pub service: String,
    /// 接口名
    pub api: String,
    /// 服务提供方接口定义的 JSON 格式的数据
    pub data: String,
    /// 随机字符串 ID，调用方请求的唯一标识
    pub client_msg_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrData {
    /// 回包信息
    pub data: String,
}

pub struct ServiceMarketModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> ServiceMarketModule<'a, T> {
    /// 调用服务平台提供的服务
    pub async fn invoke_service(&self, data: &InvokeService) -> SdkResult<StrData> {
        let url = "https://api.weixin.qq.com/wxa/servicemarket";
        post_send(self.0, url, data).await
    }
}
