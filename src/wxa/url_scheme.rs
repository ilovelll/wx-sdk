use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Generate {
    /// 跳转到的目标小程序信息。
    #[serde(default)]
    pub jump_wxa: Option<JumpWxa>,
    /// 生成的 scheme 码类型，到期失效：true，永久有效：false。 <br/>
    /// 注意，永久有效 scheme 和有效时间超过31天的到期失效 scheme 的总数上限为10万个，
    /// 详见获取 URL scheme，生成 scheme 码前请仔细确认。 <br/>
    /// 默认值：`false`
    #[serde(default)]
    pub is_expire: Option<bool>,
    /// 到期失效的 scheme 码失效类型，失效时间：0，失效间隔天数：1 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub expire_type: Option<i32>,
    /// 到期失效的 scheme 码的失效时间，为 Unix 时间戳。生成的到期失效 scheme 码在该时间前有效。 <br/>
    /// 最长有效期为1年。is_expire 为 true 且 expire_type 为 0 时必填
    #[serde(default)]
    pub expire_time: Option<i64>,
    /// 到期失效的 scheme 码的失效间隔天数。生成的到期失效 scheme 码在该间隔时间到达前有效。 <br/>
    /// 最长间隔天数为365天。is_expire 为 true 且 expire_type 为 1 时必填
    #[serde(default)]
    pub expire_interval: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JumpWxa {
    /// 通过 scheme 码进入的小程序页面路径，必须是已经发布的小程序存在的页面，不可携带 query。
    /// path 为空时会跳转小程序主页。
    #[serde(default)]
    pub path: Option<String>,
    /// 通过 scheme 码进入小程序时的 query，最大1024个字符，
    /// 只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~%
    #[serde(default)]
    pub query: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Openlink {
    openlink: String,
}

pub struct UrlSchemeModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> UrlSchemeModule<'a, T> {
    /// 获取小程序 scheme 码，适用于短信、邮件、外部网页、微信内等拉起小程序的业务场景。
    /// 通过该接口，可以选择生成到期失效和永久有效的小程序码，有数量限制，
    /// 目前仅针对国内非个人主体的小程序开放，详见获取 URL scheme。
    pub async fn generate(&self, data: &Generate) -> SdkResult<Openlink> {
        let url = "https://api.weixin.qq.com/wxa/generatescheme";
        post_send(self.0, url, data).await
    }
}
