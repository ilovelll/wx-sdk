use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserRiskRank {
    /// 小程序appid
    pub appid: String,
    /// 用户的openid
    pub openid: String,
    /// 场景值，0:注册，1:营销作弊
    pub scene: i32,
    /// 用户手机号
    #[serde(default)]
    pub mobile_no: Option<String>,
    /// 用户访问源ip
    pub client_ip: String,
    /// 用户邮箱地址
    #[serde(default)]
    pub email_address: Option<String>,
    /// 额外补充信息
    #[serde(default)]
    pub extended_info: Option<String>,
    /// false：正式调用，true：测试调用
    #[serde(default)]
    pub is_test: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRiskRank {
    /// 唯一请求标识，标记单次请求
    pub unoin_id: i32,
    /// 用户风险等级
    /// - `0`, 风险等级0
    /// - `1`, 风险等级1
    /// - `2`, 风险等级2
    /// - `3`, 风险等级3
    /// - `4`, 风险等级4
    pub risk_rank: i32,
}

pub struct RiskControlModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> RiskControlModule<'a, T> {
    /// 根据提交的用户信息数据获取用户的安全等级 risk_rank，无需用户授权。
    pub async fn get_user_risk_rank(&self, data: &QueryUserRiskRank) -> SdkResult<UserRiskRank> {
        let url = "https://api.weixin.qq.com/wxa/getuserriskrank";
        post_send(self.0, url, &data).await
    }
}
