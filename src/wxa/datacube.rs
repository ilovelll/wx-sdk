use super::{post_send, DateRange, ListRes, TimestampRange};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};
// use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitRetain {
    /// 日期
    pub ref_date: String,
    /// 新增用户留存
    pub visit_uv_new: Vec<KeyValue>,
    /// 活跃用户留存
    pub visit_uv: Vec<KeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    pub key: i64,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailySummary {
    /// 日期，格式为 yyyymmdd
    pub ref_date: String,
    /// 累计用户数
    pub visit_total: i64,
    /// 转发次数
    pub share_pv: i64,
    /// 转发人数
    pub share_uv: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyVisitTrend {
    /// 日期，格式为 yyyymmdd
    pub ref_date: String,
    /// 打开次数
    pub session_cnt: i64,
    /// 访问次数
    pub visit_pv: i64,
    /// 访问人数
    pub visit_uv: i64,
    /// 新用户数
    pub visit_uv_new: f64,
    /// 人均停留时长
    pub stay_time_uv: f64,
    /// 次均停留时长
    pub stay_time_session: f64,
    /// 平均访问深度
    pub visit_depth: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitTrend {
    /// 时间，格式为 yyyymm，如："201702"
    pub ref_date: String,
    /// 打开次数（汇总）
    pub session_cnt: i64,
    /// 访问次数（汇总）
    pub visit_pv: i64,
    /// 访问人数（去重）
    pub visit_uv: i64,
    /// 新用户数（去重）
    pub visit_uv_new: f64,
    /// 人均停留时长
    pub stay_time_uv: f64,
    /// 次均停留时长
    pub stay_time_session: f64,
    /// 平均访问深度
    pub visit_depth: f64,
}

pub mod performance_data {
    use super::TimestampRange;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Query {
        /// 开始和结束日期的时间戳，时间跨度不能超过30天
        pub time: TimestampRange,
        /// 查询数据的类型
        /// - `10016`, 打开率, params字段可传入网络类型和机型
        /// - `10017`, 启动各阶段耗时，params字段可传入网络类型和机型
        /// - `10021`, 页面切换耗时，params数组字段可传入机型
        /// - `10022`, 内存指标，params数组字段可传入机型
        /// - `10023`, 内存异常，params数组字段可传入机型
        pub module: String,
        /// 查询条件，比如机型，网络类型等等
        pub params: Vec<Param>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Param {
        /// 查询条件
        /// - `"networktype"`,
        /// 网络类型作为查询条件，value=“-1,3g,4g,wifi”分别表示 全部网络类型，3G，4G，WIFI,不传networktype默认为全部网络类型
        /// - `"device_level"`,
        /// 机型作为查询条件，此时value=“-1,1,2,3”分别表示 全部机型，高档机，中档机，低档机,不传device_level默认为全部机型
        /// - `"device"`,
        /// 平台作为查询条件，此时value="-1,1,2"分别表示 全部平台，IOS平台，安卓平台,不传device默认为全部平台
        pub field: String,
        /// 查询条件值
        pub value: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Result {
        /// 返回的性能数据
        pub body: Data,
        /// 错误码
        pub errcode: i32,
        /// 错误信息
        pub errmsg: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Data {
        pub body: Body,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Body {
        /// 返回的数据数组
        pub tables: Vec<Table>,
        /// 数组大小
        pub count: i64,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Table {
        /// 性能数据指标id
        pub id: String,
        /// 按时间排列的性能数据
        pub lines: Vec<Line>,
        /// 性能数据指标中文名
        pub zh: String,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Line {
        /// 单天的性能数据
        pub fields: Vec<Field>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Field {
        /// 日期
        pub refdate: String,
        /// 性能数据值
        pub value: String,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPortraitResult {
    /// 时间范围，如："20170611-20170617"
    pub ref_date: String,
    /// 新用户画像
    pub visit_uv_new: UserPortrait,
    /// 活跃用户画像
    pub visit_uv: UserPortrait,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPortrait {
    /// 分布类型
    pub index: i64,
    /// 省份，如北京、广东等
    pub province: Vec<NameValue>,
    /// 城市，如北京、广州等
    pub city: Vec<NameValue>,
    /// 性别，包括男、女、未知
    pub genders: Vec<NameValue>,
    /// 终端类型，包括 iPhone，android，其他
    pub platforms: Vec<NameValue>,
    /// 机型，如苹果 iPhone 6，OPPO R9 等
    pub devices: Vec<NameValue>,
    /// 年龄，包括17岁以下、18-24岁等区间
    pub ages: Vec<NameValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameValue {
    pub id: i64,
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitDistribution {
    /// 日期，格式为 yyyymmdd
    pub ref_date: String,
    /// 数据列表
    pub list: Vec<VisitDistributionItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitDistributionItem {
    /// 分布类型
    pub index: String,
    /// 分布数据列表
    pub item_list: Vec<KeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitPage {
    /// 日期，格式为 yyyymmdd
    pub ref_date: String,
    /// 数据列表
    pub list: Vec<VisitPageItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisitPageItem {
    /// 页面路径
    pub page_path: String,
    /// 访问次数
    pub page_visit_pv: i64,
    /// 访问人数
    pub page_visit_uv: i64,
    /// 次均停留时长
    pub page_staytime_pv: i64,
    /// 进入页次数
    pub entrypage_pv: i64,
    /// 退出页次数
    pub exitpage_pv: i64,
    /// 转发次数
    pub page_share_pv: i64,
    /// 转发人数
    pub page_share_uv: i64,
}

pub struct DataCubeModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> DataCubeModule<'a, T> {
    /// 获取用户访问小程序日留存
    pub async fn get_daily_retain(&self, query: &DateRange) -> SdkResult<VisitRetain> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappiddailyretaininfo";
        post_send(self.0, url, query).await
    }

    /// 获取用户访问小程序月留存
    pub async fn get_monthly_retain(&self, query: &DateRange) -> SdkResult<VisitRetain> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappidmonthlyretaininfo";
        post_send(self.0, url, query).await
    }

    /// 获取用户访问小程序周留存
    pub async fn get_weekly_retain(&self, query: &DateRange) -> SdkResult<VisitRetain> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappidweeklyretaininfo";
        post_send(self.0, url, query).await
    }

    /// 获取用户访问小程序数据概况
    pub async fn get_daily_summary(&self, query: &DateRange) -> SdkResult<ListRes<DailySummary>> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappiddailysummarytrend";
        post_send(self.0, url, query).await
    }

    /// 获取用户访问小程序数据日趋势
    pub async fn get_daily_visit_trend(
        &self,
        query: &DateRange,
    ) -> SdkResult<ListRes<DailyVisitTrend>> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappiddailyvisittrend";
        post_send(self.0, url, query).await
    }

    /// 获取用户访问小程序数据月趋势(能查询到的最新数据为上一个自然月的数据)
    pub async fn get_monthly_visit_trend(&self, query: &DateRange) -> SdkResult<VisitTrend> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappidmonthlyvisittrend";
        post_send(self.0, url, query).await
    }

    /// 获取用户访问小程序数据周趋势
    pub async fn get_weekly_visit_trend(&self, query: &DateRange) -> SdkResult<VisitTrend> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappidweeklyvisittrend";
        post_send(self.0, url, query).await
    }

    /// 获取小程序启动性能，运行性能等数据。
    pub async fn get_performance_data(
        &self,
        query: &performance_data::Query,
    ) -> SdkResult<performance_data::Result> {
        let url = "https://api.weixin.qq.com/wxa/business/performance/boot";
        post_send(self.0, url, query).await
    }

    /// 获取小程序新增或活跃用户的画像分布数据。时间范围支持昨天、最近7天、最近30天。<br/>
    /// 其中，新增用户数为时间范围内首次访问小程序的去重用户数，活跃用户数为时间范围内访问过小程序的去重用户数。
    pub async fn get_user_portrait(&self, query: &DateRange) -> SdkResult<UserPortraitResult> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappiduserportrait";
        post_send(self.0, url, query).await
    }

    /// 获取用户小程序访问分布数据
    pub async fn get_visit_distribution(&self, query: &DateRange) -> SdkResult<VisitDistribution> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappidvisitdistribution";
        post_send(self.0, url, query).await
    }

    /// 访问页面。目前只提供按 page_visit_pv 排序的 top200。
    pub async fn get_visit_page(&self, query: &DateRange) -> SdkResult<VisitPage> {
        let url = "https://api.weixin.qq.com/datacube/getweanalysisappidvisitpage";
        post_send(self.0, url, query).await
    }
}
