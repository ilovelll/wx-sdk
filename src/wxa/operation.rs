use super::{get_send, post_send};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    /// 查询配置域名的类型, 可选值如下：
    /// - `getbizdomain` 返回业务域名 <br/>
    /// - `getserverdomain` 返回服务器域名 <br/>
    /// - 不指明返回全部 <br/>
    #[serde(default)]
    pub action: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainInfo {
    /// request合法域名列表
    pub requestdomain: Vec<String>,
    /// socket合法域名列表
    pub wsrequestdomain: Vec<String>,
    /// uploadFile合法域名列表
    pub uploaddomain: Vec<String>,
    /// downloadFile合法域名列表
    pub downloaddomain: Vec<String>,
    /// udp合法域名列表
    pub udpdomain: Vec<String>,
    /// 业务域名列表
    pub bizdomain: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryFeedback {
    /// 反馈的类型，默认拉取全部类型
    /// - `1` 无法打开小程序
    /// - `2` 小程序闪退
    /// - `3` 卡顿
    /// - `4` 黑屏白屏
    /// - `5` 死机
    /// - `6` 界面错位
    /// - `7` 界面加载慢
    /// - `8` 其他异常
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<i32>,
    /// 分页的页数，从1开始
    pub page: i32,
    /// 分页拉取的数据数量
    pub num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeedbackList {
    /// 反馈列表
    pub list: Vec<Feedback>,
    /// 总条数
    pub total_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Feedback {
    pub record_id: i64,
    pub create_time: i64,
    pub content: String,
    pub phone: i64,
    pub openid: String,
    pub nickname: String,
    pub head_url: String,
    #[serde(rename = "type")]
    pub type_: i32,
    #[serde(rename = "mediaIds")]
    pub media_ids: Vec<String>,
    #[serde(rename = "systemInfo")]
    pub system_info: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct GrayReleasePlanRes {
    /// 分阶段发布计划详情
    pub gray_release_plan: GrayReleasePlan,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrayReleasePlan {
    /// 0:初始状态 1:执行中 2:暂停中 3:执行完毕 4:被删除
    pub status: i32,
    /// 当前的灰度比例
    pub gray_percentage: f64,
    /// 分阶段发布计划的创建事件
    pub create_timestamp: i64,
    /// 预计全量时间
    pub default_finish_timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryJsErrDetail {
    /// 开始时间， 格式 "xxxx-xx-xx"
    pub start_time: String,
    /// 结束时间，格式 “xxxx-xx-xx”
    pub end_time: String,
    /// 错误列表查询 接口 返回的 errorMsgMd5 字段
    pub error_msg_md5: String,
    /// 错误列表查询 接口 返回的 errorStackMd5 字段
    pub error_stack_md5: String,
    /// 小程序版本 "0"代表全部， 例如：“2.0.18”
    pub app_version: String,
    /// 基础库版本 "0"表示所有版本，例如 "2.14.1"
    pub sdk_version: String,
    /// 系统类型 "0"【全部】，"1" 【安卓】，"2" 【IOS】，"3"【其他】
    pub os_name: String,
    /// 客户端版本 "0"表示所有版本， 例如 "7.0.22"
    pub client_version: String,
    /// 发生错误的用户 openId
    pub openid: String,
    /// 排序规则 "0" 升序, "1" 降序
    pub desc: String,
    /// 分页起始值
    pub offset: i32,
    /// 一次拉取最大值
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataList<T> {
    pub openid: String,
    pub data: Vec<T>,
    #[serde(rename = "totalCount")]
    pub total_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsErrDetail {
    #[serde(rename = "Count")]
    pub count: String,
    #[serde(rename = "sdkVersion")]
    pub sdk_version: String,
    #[serde(rename = "ClientVersion")]
    pub client_version: String,
    #[serde(rename = "errorStackMd5")]
    pub error_stack_md5: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "appVersion")]
    pub app_version: String,
    #[serde(rename = "errorMsgMd5")]
    pub error_msg_md5: String,
    #[serde(rename = "errorMsg")]
    pub error_msg: String,
    #[serde(rename = "errorStack")]
    pub error_stack: String,
    #[serde(rename = "Ds")]
    pub ds: String,
    #[serde(rename = "OsName")]
    pub os_name: String,
    #[serde(rename = "openId")]
    pub open_id: String,
    pub pluginversion: String,
    #[serde(rename = "appId")]
    pub app_id: String,
    #[serde(rename = "DeviceModel")]
    pub device_model: String,
    pub source: String,
    pub route: String,
    #[serde(rename = "Uin")]
    pub uin: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryJsErrList {
    /// 小程序版本 "0"代表全部， 例如：“2.0.18”
    pub app_version: String,
    /// 错误类型 "0"【全部】，"1"【业务代码错误】，"2"【插件错误】，"3"【系统框架错误】
    pub err_type: String,
    /// 开始时间， 格式 "xxxx-xx-xx"
    pub start_time: String,
    /// 结束时间，格式 “xxxx-xx-xx”
    pub end_time: String,
    /// 从错误中搜索关键词，关键词过滤
    pub keyword: String,
    /// 发生错误的用户 openId
    pub openid: String,
    /// 排序字段 "uv", "pv" 二选一
    pub orderby: String,
    /// 排序规则 "1" orderby字段降序，"2" orderby字段升序
    pub desc: String,
    /// 分页起始值
    pub offset: i32,
    /// 一次拉取最大值， 最大 30
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsErr {
    pub error_msg_md5: String,
    pub error_msg: String,
    pub uv: i64,
    pub pv: i64,
    pub error_stack_md5: String,
    pub error_stack: String,
    pub pv_percent: String,
    pub uv_percent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryJsErrSearch {
    /// 错误关键字
    pub errmsg_keyword: String,
    /// 查询类型，1 为客户端， 2为服务直达
    #[serde(rename = "type")]
    pub type_: i32,
    /// 客户端版本，可以通过 getVersionList 接口拉取, 不传或者传空代表所有版本
    pub client_version: String,
    /// 开始时间
    pub start_time: i64,
    /// 结束时间
    pub end_time: i64,
    /// 分页起始值
    pub start: i32,
    /// 一次拉取最大值
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsErrSearchRes {
    pub results: Vec<JsErrSearch>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsErrSearch {
    pub time: i64,
    pub client_version: String,
    pub app_version: String,
    pub version_error_cnt: i64,
    pub total_error_cnt: i64,
    pub errmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPerformance {
    /// 可选值 1（启动总耗时）， 2（下载耗时），3（初次渲染耗时）
    pub cost_time_type: i32,
    /// 查询开始时间
    pub default_start_time: i64,
    /// 查询结束时间
    pub default_end_time: i64,
    /// 系统平台，可选值 "@_all"（全部），1（IOS）， 2（android）
    pub device: String,
    /// 是否下载代码包，当 type 为 1 的时候才生效，可选值 "@_all"（全部），1（是）， 2（否）
    pub is_download_code: String,
    /// 访问来源，当 type 为 1 或者 2 的时候才生效，通过 getSceneList 接口获取
    pub scene: String,
    /// 网络环境, 当 type 为 2 的时候才生效，可选值 "@_all"，wifi, 4g, 3g, 2g
    pub networktype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Performance {
    /// 错误查询数据(json字符串，结构如下所述的 strbody)
    pub default_time_data: String,
    /// 比较数据
    pub compare_time_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SceneList {
    pub scene: Vec<NameValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameValue {
    pub name: String,
    pub value: StrI32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StrI32 {
    String(String),
    I32(i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionList {
    pub cvlist: Vec<CvList>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CvList {
    /// 查询类型，1 为客户端， 2为服务直达
    #[serde(rename = "type")]
    pub type_: i32,
    pub client_version_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryRealtimelog {
    /// YYYYMMDD格式的日期，仅支持最近7天
    pub date: String,
    /// 开始时间，必须是date指定日期的时间
    pub begintime: i64,
    /// 结束时间，必须是date指定日期的时间
    pub endtime: i64,
    /// 开始返回的数据下标，用作分页 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub start: Option<i32>,
    /// 返回的数据条数，用作分页 <br/>
    /// 默认值：`20`
    #[serde(default)]
    pub limit: Option<i32>,
    /// 小程序启动的唯一ID，按TraceId查询会展示该次小程序启动过程的所有页面的日志。
    #[serde(default)]
    pub trace_id: Option<String>,
    /// 小程序页面路径，例如pages/index/index
    #[serde(default)]
    pub url: Option<String>,
    /// 用户微信号或者OpenId
    #[serde(default)]
    pub id: Option<String>,
    /// 开发者通过setFileterMsg/addFilterMsg指定的filterMsg字段
    #[serde(default)]
    pub filter_msg: Option<String>,
    /// 日志等级，返回大于等于level等级的日志，level的定义为2（Info）、4（Warn）、8（Error） <br/>
    /// 如果指定为4，则返回大于等于4的日志，即返回Warn和Error日志。
    #[serde(default)]
    pub level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Realtimelog {
    /// 返回的日志数据和日志条数总量
    pub data: RealtimelogList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimelogList {
    pub list: Vec<RealtimelogItem>,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealtimelogItem {
    /// 日志等级，是msg数组里面的所有level字段的或操作得到的结果。
    /// 例如msg数组里有两条日志，Info（值为2）和Warn（值为4），则level值为6
    pub level: i32,
    /// 基础库版本
    pub library_version: String,
    /// 客户端版本
    pub client_version: String,
    /// 微信用户OpenID
    pub id: String,
    /// 打日志的Unix时间戳
    pub timestamp: i64,
    /// 1 安卓 2 IOS
    pub platform: i32,
    /// 小程序页面链接
    pub url: String,
    /// 日志内容数组，log.info等的内容存在这里
    pub msg: Vec<RealtimelogMsg>,
    /// 小程序启动的唯一ID，按TraceId查询会展示该次小程序启动过程的所有页面的日志。
    pub traceid: String,
    /// 微信用户OpenID
    pub filter_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealtimelogMsg {
    /// log.info调用的时间
    pub time: i64,
    /// log.info调用的内容，每个参数分别是数组的一项
    pub msg: Vec<String>,
    /// log.info调用的日志等级
    pub level: i32,
}

pub struct OperationModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> OperationModule<'a, T> {
    /// 查询域名配置
    pub async fn get_domain_info(&self, query: &Action) -> SdkResult<DomainInfo> {
        let url = "https://api.weixin.qq.com/wxa/getwxadevinfo";
        get_send(self.0, url, query).await
    }

    /// 获取用户反馈列表
    pub async fn get_feedback(&self, query: &QueryFeedback) -> SdkResult<FeedbackList> {
        let url = "https://api.weixin.qq.com/wxaapi/feedback/list";
        get_send(self.0, url, query).await
    }

    /// 获取 mediaId 图片
    pub async fn get_feedbackmedia(&self, record_id: &str, media_id: &str) -> SdkResult<Vec<u8>> {
        let url = "https://api.weixin.qq.com/cgi-bin/media/getfeedbackmedia";
        let query = &[("media_id", media_id), ("record_id", record_id)];
        let builder = self.0.wx_get(url).await?.query(query);
        let bytes = builder.send().await?.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// 查询当前分阶段发布详情
    pub async fn get_gray_release_plan(&self) -> SdkResult<GrayReleasePlanRes> {
        let url = "https://api.weixin.qq.com/wxa/getgrayreleaseplan";
        get_send(self.0, url, &()).await
    }

    /// 错误查询详情
    pub async fn get_js_err_detail(
        &self,
        data: &QueryJsErrDetail,
    ) -> SdkResult<DataList<JsErrDetail>> {
        let url = "https://api.weixin.qq.com/wxaapi/log/jserr_detail";
        post_send(self.0, url, data).await
    }

    /// 错误查询列表
    pub async fn get_js_err_list(&self, data: &QueryJsErrList) -> SdkResult<DataList<JsErr>> {
        let url = "https://api.weixin.qq.com/wxaapi/log/jserr_list";
        post_send(self.0, url, data).await
    }

    /// 错误查询, 接口即将废弃，请采用新接口 getJsErrList
    pub async fn get_js_err_search(&self, data: &QueryJsErrSearch) -> SdkResult<JsErrSearchRes> {
        let url = "https://api.weixin.qq.com/wxaapi/log/jserr_search";
        post_send(self.0, url, data).await
    }

    /// 性能监控
    pub async fn get_performance(&self, data: &QueryPerformance) -> SdkResult<Performance> {
        let url = "https://api.weixin.qq.com/wxaapi/log/get_performance";
        post_send(self.0, url, data).await
    }

    /// 获取访问来源
    pub async fn get_scene_list(&self) -> SdkResult<SceneList> {
        let url = "https://api.weixin.qq.com/wxaapi/log/get_scene";
        get_send(self.0, url, &()).await
    }

    /// 获取客户端版本
    pub async fn get_version_list(&self) -> SdkResult<VersionList> {
        let url = "https://api.weixin.qq.com/wxaapi/log/get_client_version";
        get_send(self.0, url, &()).await
    }

    /// 实时日志查询
    pub async fn realtimelog_search(&self, query: &QueryRealtimelog) -> SdkResult<Realtimelog> {
        let url = "https://api.weixin.qq.com/wxaapi/userlog/userlog_search";
        get_send(self.0, url, query).await
    }
}
