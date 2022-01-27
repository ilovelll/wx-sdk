use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateLink {
    /// 通过 Short Link 进入的小程序页面路径，必须是已经发布的小程序存在的页面，可携带 query，最大1024个字符
    pub page_url: String,
    /// 页面标题，不能包含违法信息，超过20字符会用... 截断代替
    pub page_title: String,
    /// 生成的 Short Link 类型，短期有效：false，永久有效：true <br/>
    /// 默认值：`false`
    #[serde(default)]
    pub is_permanent: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    /// 回包信息
    pub link: String,
}

pub struct ShortLinkModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> ShortLinkModule<'a, T> {
    /// 获取小程序 Short Link，适用于微信内拉起小程序的业务场景。
    /// 目前只开放给电商类目(具体包含以下一级类目：电商平台、商家自营、跨境电商)。
    /// 通过该接口，可以选择生成到期失效和永久有效的小程序短链，详见获取 Short Link。
    pub async fn generate(&self, data: &GenerateLink) -> SdkResult<Link> {
        let url = "https://api.weixin.qq.com/wxa/genwxashortlink";
        post_send(self.0, url, data).await
    }
}
