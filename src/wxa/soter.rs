use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifySignature {
    /// 用户 openid
    pub openid: String,
    /// 通过 wx.startSoterAuthentication 成功回调获得的 resultJSON 字段
    pub json_string: String,
    /// 通过 wx.startSoterAuthentication 成功回调获得的 resultJSONSignature 字段
    pub json_signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IsOk {
    /// 验证结果
    pub is_ok: bool,
}

pub struct SoterModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> SoterModule<'a, T> {
    /// SOTER 生物认证秘钥签名验证
    pub async fn verify_signature(&self, data: &VerifySignature) -> SdkResult<IsOk> {
        let url = "https://api.weixin.qq.com/cgi-bin/soter/verify_signature";
        post_send(self.0, url, data).await
    }
}
