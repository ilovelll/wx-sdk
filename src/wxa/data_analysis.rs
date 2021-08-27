use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDailyRetain {
    /// 接口调用凭证
    pub access_token: String,
    /// 开始日期。格式为 yyyymmdd
    pub begin_date: String,
    /// 结束日期，限定查询1天数据，允许设置的最大值为昨日。格式为 yyyymmdd
    pub end_date: String,
}


pub struct DataAnalysisModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);
impl<'a, T: WxApiRequestBuilder> DataAnalysisModule<'a, T> {
    
    pub async fn get_daily_retain(
        &self,
        encrypted_msg_hash: &str,
    ) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/business/checkencryptedmsg";
        let post_data = &json!({ "encrypted_msg_hash": encrypted_msg_hash });
        crate::wxa::get_send(self.0, url, post_data).await
    }
    
}