use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Generate {
    /// 通过 URL Link 进入的小程序页面路径，必须是已经发布的小程序存在的页面，不可携带 query 。
    /// path 为空时会跳转小程序主页
    #[serde(default)]
    pub path: Option<String>,
    /// 通过 URL Link 进入小程序时的query，最大1024个字符，
    /// 只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~%
    #[serde(default)]
    pub query: Option<String>,
    /// 生成的 URL Link 类型，到期失效：true，永久有效：false。
    /// 注意，永久有效 Link 和有效时间超过31天的到期失效 Link 的总数上限为10万个，
    /// 详见获取 URL Link，生成 Link 前请仔细确认。 <br/>
    /// 默认值：`false`
    #[serde(default)]
    pub is_expire: Option<bool>,
    /// 小程序 URL Link 失效类型，失效时间：0，失效间隔天数：1 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub expire_type: Option<i32>,
    /// 到期失效的 URL Link 的失效时间，为 Unix 时间戳。生成的到期失效 URL Link 在该时间前有效。
    /// 最长有效期为1年。expire_type 为 0 必填
    #[serde(default)]
    pub expire_time: Option<i64>,
    /// 到期失效的URL Link的失效间隔天数。生成的到期失效URL Link在该间隔时间到达前有效。
    /// 最长间隔天数为365天。expire_type 为 1 必填
    #[serde(default)]
    pub expire_interval: Option<i64>,
    /// 云开发静态网站自定义 H5 配置参数，可配置中转的云开发 H5 页面。不填默认用官方 H5 页面
    #[serde(default)]
    pub cloud_base: Option<CloudBase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloudBase {
    /// 云开发环境
    pub env: String,
    /// 静态网站自定义域名，不填则使用默认域名
    #[serde(default)]
    pub domain: Option<String>,
    /// 云开发静态网站 H5 页面路径，不可携带 query <br/>
    /// 默认值：`/`
    #[serde(default)]
    pub path: Option<String>,
    /// 云开发静态网站 H5 页面 query 参数，最大 1024 个字符，
    /// 只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~%
    #[serde(default)]
    pub query: Option<String>,
    /// 第三方批量代云开发时必填，表示创建该 env 的 appid （小程序/第三方平台）
    #[serde(default)]
    pub resource_appid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlLink {
    openlink: String,
    errcode: i32,
    errmsg: String,
}

pub struct UrlLinkModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> UrlLinkModule<'a, T> {
    /// 获取小程序 URL Link，适用于短信、邮件、网页、微信内等拉起小程序的业务场景。通过该接口，可以选择生成到期失效和永久有效的小程序链接，有数量限制，目前仅针对国内非个人主体的小程序开放，详见获取 URL Link。
    pub async fn generate(&self, data: &Generate) -> SdkResult<UrlLink> {
        let url = "https://api.weixin.qq.com/wxa/generate_urllink";
        post_send(self.0, url, data).await
    }
}
