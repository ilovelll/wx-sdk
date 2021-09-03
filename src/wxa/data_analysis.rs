use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};
// use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryDailyRetain {
    /// 开始日期。格式为 yyyymmdd
    pub begin_date: String,
    /// 结束日期，限定查询1天数据，允许设置的最大值为昨日。格式为 yyyymmdd
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitRetain {
    /// 日期
    pub ref_date: String,
    /// 新增用户留存
    pub visit_uv_new: Vec<VisitUv>,
    /// 活跃用户留存
    pub visit_uv: Vec<VisitUv>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitUv {
    /// 标识，0开始，表示当天，1表示1天后。依此类推，key取值分别是：0,1,2,3,4,5,6,7,14,30
    pub key: i32,
    /// key对应日期的新增用户数/活跃用户数（key=0时）或留存用户数（k>0时）
    pub value: i32,
}

pub struct DataAnalysisModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> DataAnalysisModule<'a, T> {
    pub async fn get_daily_retain(&self, query: &QueryDailyRetain) -> SdkResult<VisitRetain> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappiddailyretaininfo";
        crate::wxa::post_common_send(self.0, url, query).await
    }
}
