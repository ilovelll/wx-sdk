use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DelayFn {
    /// 环境ID
    pub env: String,
    /// 函数名称
    pub function_name: String,
    /// 延迟时间，单位：秒，合法范围：6s-30天
    pub delay_time: i32,
    /// 发送的数据包，格式必须为JSONString
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PressureTask {
    /// 页面地址
    pub task_url: String,
    /// worker数量 合法范围：0-500
    pub run_count: i32,
    /// 持续时间，单位：秒，合法范围：0-600s
    pub run_time: i32,
    /// 压测版本 （4：线上版，9：体验版）
    pub source_type: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PressureId {
    /// 压测ID
    pub pressure_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsTask {
    /// 环境 ID
    pub env: String,
    /// 短信 CSV 文件地址CodeUri
    pub file_url: String,
    /// 短信模版 ID 默认值：844110（销类短信模版 ID)
    pub template_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryId {
    /// 查询 ID
    pub query_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FilesInfo {
    /// 待上传的文件列表
    pub extension_files: Vec<FileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileInfo {
    /// 文件类型。枚举值FUNCTION：函数代码STATIC：静态托管代码SMS：短信文件
    pub file_type: String,
    /// 文件名，长度不超过24
    pub file_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilesData {
    /// 待上传文件的信息数组
    #[serde(rename = "FilesData")]
    pub files_data: Vec<FileData>,
    /// 唯一请求 ID，每次请求都会返回。定位问题时需要提供该次请求的 RequestId。
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileData {
    /// 模板里使用的地址
    pub code_uri: String,
    /// 上传文件的临时地址，含签名
    pub upload_url: String,
    /// 自定义密钥。如果为空，则表示不需要加密
    pub custom_key: String,
    /// 文件大小限制，单位M，客户端上传前需要主动检查文件大小，超过限制的文件会被删除。
    pub max_size: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct QuerySmsRecord {
    /// 环境 ID
    pub env_id: String,
    /// 开始日期, 如:2021-01-01
    pub start_date: String,
    /// 结束日期, 如2021-01-07
    pub end_date: String,
    /// 电话号码
    #[serde(default)]
    pub mobile: Option<String>,
    /// 查询ID
    #[serde(default)]
    pub query_id: Option<String>,
    /// 页码(1开始)
    #[serde(default)]
    pub page_number: Option<i32>,
    /// 每页条目数
    #[serde(default)]
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmsRecords {
    /// 发送记录列表
    #[serde(rename = "SmsRecords")]
    pub sms_records: Vec<SmsRecord>,
    /// 记录总数
    #[serde(rename = "TotalCount")]
    pub total_count: i64,
    /// 唯一请求 ID，每次请求都会返回。定位问题时需要提供该次请求的 RequestId。
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SmsRecord {
    /// 手机号码
    pub mobile: String,
    /// 短信内容
    pub content: String,
    /// 短信内容长度
    pub content_size: i32,
    /// 计费条数
    pub fee: i32,
    /// 发送时间
    pub create_time: String,
    /// 用户接收时间
    pub received_time: String,
    /// sent(成功), error(失败)
    pub status: String,
    /// 备注
    pub remarks: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOpenData {
    /// 用户唯一标识符
    pub openid: String,
    /// CloudID 列表
    pub cloudid_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenDataList {
    /// 开放数据列表
    pub data_list: Vec<OpenData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenData {
    pub cloud_id: String,
    pub json: OpenDataJson,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenDataJson {
    #[serde(rename = "cloudID")]
    pub cloud_id: String,
    pub data: StepInfoList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepInfoList {
    pub step_info_list: Vec<StepInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepInfo {
    pub timestamp: i64,
    pub step: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PressureTestReport {
    /// 压测报告
    pub report: PTestReport,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PTestReport {
    /// 打开页面的白页率
    pub blankpage_pencent: f64,
    /// 打开页面的平均耗时, ms
    pub aver_time_cost: i32,
    /// 打开页面的最大耗时, ms
    pub max_time_cost: i32,
    /// 共打开了多少次该页面
    pub total_launch_cnt: i32,
    /// 共发起多少次网络请求
    pub total_request_cnt: i32,
    /// 各网络请求的统计
    pub network_list: Vec<Network>,
    /// 小程序id
    pub appid: String,
    /// 小程序页面URL
    pub task_url: String,
    /// 压测时间, s
    pub run_time: i32,
    /// 压测ID
    pub pressure_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Network {
    /// 网络请求path
    pub path: String,
    /// 该path的平均耗时, ms
    pub aver_time_cost: f64,
    /// 该path的最大耗时, ms
    pub max_time_cost: i32,
    /// 共请求了多少多次该path
    pub total_request_cnt: i32,
    /// 请求该path的成功率
    pub succ_percent: f64,
    /// top耗时的请求
    pub top_time_cost_list: Vec<TopTimeCost>,
    /// 超过800ms的请求的数量
    pub over_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopTimeCost {
    pub time_cost: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PressureTestStatus {
    /// 压测任务状态，当压测状态为 `"Done"` 时，才可以获取报告
    pub status: String,
    /// 压测开始时间
    pub beg_time: i64,
    /// 压测结束时间
    pub end_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryStatistics {
    /// 获取动作，目前支持（smsMarketingOverviewData：短信营销概览数据；
    /// smsMarketingConversionData：短信营销转化数据；smsMarketingRealTimeData：短信营销实时数据）
    pub action: String,
    /// 开始时间戳
    pub begin_date: i64,
    /// 结束时间戳
    pub end_date: i64,
    /// 分页 offset【action 取 smsMarketingOverviewData、smsMarketingConversionData 时必填】
    #[serde(default)]
    pub page_offset: Option<i32>,
    /// 分页 limit【action 取 smsMarketingOverviewData、smsMarketingConversionData 时必填】
    #[serde(default)]
    pub page_limit: Option<i32>,
    /// 查询条件
    pub condition: StatisticCondition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatisticCondition {
    pub env_id: String,
    pub activity_id: String,
    pub by_channel_id: String,
    pub act_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Statistics {
    /// 数据列定义
    pub data_column: Vec<DataColumn>,
    /// 数据行
    pub data_value: Vec<DataValue>,
    /// 总行数
    pub total_num: i32,
    /// 环境 ID
    pub env_id: Option<String>,
    /// 活动 ID
    pub activity_id: Option<String>,
    /// 是否按渠道划分（0：返回概览数据；1：返回各个渠道的数据，可以用于饼图）【action 取 smsMarketingOverviewData 时必填】
    #[serde(default)]
    pub by_channel_id: Option<String>,
    /// 渠道 ID（云开发 CMS 使用 _cms_sms_）【action 取 smsMarketingConversionData、smsMarketingRealTimeData 时必填】
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 行为数据（h5：打开 H5；wxapp：跳转小程序）【action == smsMarketingRealTimeData 时必填】
    #[serde(default)]
    pub act_type: Option<String>,
    /// 错误码
    pub errcode: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataColumn {
    /// 列 id
    pub col_id: String,
    /// 列名
    pub col_name: String,
    /// 数据类型（0:string；1:number；2:double）
    pub col_data_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataValue {
    /// 数据值
    pub data_value: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryVoIPSign {
    /// 用户唯一标识符
    pub openid: String,
    /// 游戏房间的标识
    pub group_id: String,
    /// 随机字符串，长度应小于 128
    pub nonce: String,
    /// 生成这个随机字符串的 UNIX 时间戳（精确到秒）
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoIPSign {
    /// 签名
    pub signature: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Report {
    /// 上报动作，目前支持（sendSmsTask：发送短信；openH5：H5 打开）
    pub report_action: String,
    /// 环境 ID
    pub env_id: String,
    /// 活动 ID
    pub activity_id: String,
    /// 任务 ID【report_action 取 sendSmsTask 时必填】
    #[serde(default)]
    pub task_id: Option<String>,
    /// 下发手机号数量【report_action 取 sendSmsTask 时必填】
    #[serde(default)]
    pub phone_count: Option<String>,
    /// 渠道 ID（云开发 CMS 使用 _cms_sms_）【report_action 取 openH5 时必填】
    #[serde(default)]
    pub channel_id: Option<String>,
    /// 会话 ID【report_action 取 openH5 时必填】
    #[serde(default)]
    pub session_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendSms {
    /// 环境 ID
    pub env: String,
    /// 手机号列表，单次请求最多支持 1000 个境内手机号，手机号必须以+86开头
    pub phone_number_list: Vec<String>,
    /// 短信类型，营销类短信：Marketing；通知类短信：Notification <br/>
    /// 默认值：`Marketing`
    pub sms_type: String,
    /// sms_type="Marketing" 时必填，自定义短信内容，一条短信最多为70个字。可自定义内容最多为 30 个字符，详情参考短信规则
    #[serde(default)]
    pub content: Option<String>,
    /// sms_type="Marketing" 时必填，云开发静态网站 path，不需要指定域名，例如/index.html
    #[serde(default)]
    pub path: Option<String>,
    /// sms_type="Notification" 时必填，模版 ID
    #[serde(default)]
    pub template_id: Option<String>,
    /// sms_type="Notification" 时必填，短信模版变量数组
    #[serde(default)]
    pub template_param_list: Option<Vec<String>>,
    /// 是否使用小程序简称 <br/>
    /// 默认值：`false`
    #[serde(default)]
    pub use_short_name: Option<bool>,
    /// 资源方appid，第三方代开发时可填第三方appid或小程序appid，应为所填环境所属的账号APPID
    #[serde(default)]
    pub resource_appid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendStatusList {
    /// 开放数据列表
    pub send_status_list: Vec<SendStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendStatus {
    /// 发送流水号
    pub serial_no: String,
    /// 手机号码
    pub phone_number: String,
    /// 短信请求错误码
    pub code: String,
    /// 短信请求错误码描述
    pub message: String,
    /// 国家码或地区码
    pub iso_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendSmsV2 {
    /// 环境 ID
    pub env: String,
    /// URL Link
    pub url_link: String,
    /// 短信模版 ID。(844110: 营销类短信模版 ID)
    pub template_id: String,
    /// 短信模版变量数组
    pub template_param_list: Vec<String>,
    /// 手机号列表，单次请求最多支持 1000 个境内手机号，手机号必须以+86开头
    pub phone_number_list: Vec<String>,
    /// 是否使用小程序简称 <br/>
    /// 默认值：`false`
    #[serde(default)]
    pub use_short_name: Option<bool>,
    /// 资源方appid，第三方代开发时可填第三方appid或小程序appid，应为所填环境所属的账号APPID
    #[serde(default)]
    pub resource_appid: Option<String>,
}

pub struct CloudbaseModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> CloudbaseModule<'a, T> {
    /// 延时调用云函数
    pub async fn add_delayed_function_task(&self, data: &DelayFn) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/tcb/adddelayedfunctiontask";
        post_send(self.0, url, data).await
    }

    /// 创建压测任务
    pub async fn create_pressure_test(&self, data: &PressureTask) -> SdkResult<PressureId> {
        let url = "https://api.weixin.qq.com/tcb/createpressuretesttask";
        post_send(self.0, url, data).await
    }

    /// 创建发短信任务。发送的短信支持打开云开发静态网站 H5，进而在 H5 里可以打开小程序。<br/>
    /// 详情可参考静态网站 H5 跳小程序
    pub async fn create_send_sms_task(&self, data: &SmsTask) -> SdkResult<QueryId> {
        let url = "https://api.weixin.qq.com/tcb/createsendsmstask";
        post_send(self.0, url, data).await
    }

    /// 描述扩展上传文件信息
    pub async fn describe_extension_upload_info(&self, data: &FilesInfo) -> SdkResult<FilesData> {
        let url = "https://api.weixin.qq.com/tcb/describeextensionuploadinfo";
        post_send(self.0, url, data).await
    }

    /// 查询 2 个月内的短信记录
    pub async fn describe_sms_records(&self, data: &QuerySmsRecord) -> SdkResult<SmsRecords> {
        let url = "https://api.weixin.qq.com/tcb/describesmsrecords";
        post_send(self.0, url, data).await
    }

    /// 换取 cloudID 对应的开放数据
    pub async fn get_open_data(&self, data: &QueryOpenData) -> SdkResult<OpenDataList> {
        let url = "https://api.weixin.qq.com/wxa/getopendata";
        post_send(self.0, url, data).await
    }

    /// 获取压测报告
    pub async fn get_pressure_test_report(
        &self,
        pressure_id: i64,
    ) -> SdkResult<PressureTestReport> {
        let url = "https://api.weixin.qq.com/tcb/getpressuretestreport";
        let data = &serde_json::json!({ "pressure_id": pressure_id });
        post_send(self.0, url, data).await
    }

    /// 获取压测状态
    pub async fn get_pressure_test_status(
        &self,
        pressure_id: i64,
    ) -> SdkResult<PressureTestStatus> {
        let url = "https://api.weixin.qq.com/tcb/getpressureteststatus";
        let data = &serde_json::json!({ "pressure_id": pressure_id });
        post_send(self.0, url, data).await
    }

    /// 获取云开发数据接口
    pub async fn get_statistics(&self, data: &QueryStatistics) -> SdkResult<Statistics> {
        let url = "https://api.weixin.qq.com/tcb/getstatistics";
        post_send(self.0, url, data).await
    }

    /// 获取实时语音签名
    pub async fn get_voipsign(&self, data: &QueryVoIPSign) -> SdkResult<VoIPSign> {
        let url = "https://api.weixin.qq.com/wxa/getvoipsign";
        post_send(self.0, url, data).await
    }

    /// 云开发通用上报接口
    pub async fn report(&self, data: &Report) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/tcb/cloudbasereport";
        post_send(self.0, url, data).await
    }

    /// 发送支持打开云开发静态网站的短信，该 H5 可以打开小程序。详情可参考静态网站 H5 跳小程序
    pub async fn send_sms(&self, data: &SendSms) -> SdkResult<SendStatusList> {
        let url = "https://api.weixin.qq.com/tcb/sendsms";
        post_send(self.0, url, data).await
    }

    /// 发送携带 URL Link 的短信
    pub async fn send_sms_v2(&self, data: &SendSmsV2) -> SdkResult<SendStatusList> {
        let url = "https://api.weixin.qq.com/tcb/sendsmsv2";
        post_send(self.0, url, data).await
    }
}

// #[test]
// fn test_rename_pascal_case() {
//     let input = r#"{"FileType": "t", "FileName": "name"}"#;
//     let msg_type = &serde_json::from_str::<FileInfo>(input).unwrap();
//     println!("{:?}", msg_type);
//     println!("{:?}", serde_json::to_string(msg_type));
// }

// #[test]
// fn test_rename_sdk_error() {
//     let input = r#"
// {
//   "errcode": 0,
//   "errmsg": "ok",
//   "signature": null
// }"#;
//     let data = &serde_json::from_str::<crate::error::CommonResponse<Signature>>(input).unwrap();
//     println!("{:?}", data);
//     println!("{:?}", serde_json::to_string(data));
// }
