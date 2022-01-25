use super::{get_send, post_send};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTemplate {
    /// 模板标题 id，可通过接口获取，也可登录小程序后台查看获取
    pub tid: String,
    /// 开发者自行组合好的模板关键词列表，关键词顺序可以自由搭配（例如 [3,5,4] 或 [4,5,3]），最多支持5个，最少2个关键词组合
    pub kid_list: Vec<i64>,
    /// 服务场景描述，15个字以内
    #[serde(default)]
    pub scene_desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriTmplId {
    /// 添加至帐号下的模板id，发送小程序订阅消息时所需
    pub pri_tmpl_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryData {
    /// 添加至帐号下的模板id，发送小程序订阅消息时所需
    pub data: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    /// 类目id，查询公共库模版时需要
    pub id: i32,
    /// 类目的中文名
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateKeywords {
    /// 模版标题列表总数
    pub count: i64,
    /// 关键词列表
    pub data: Vec<TemplateKeyword>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateKeyword {
    /// 关键词 id，选用模板时需要
    pub kid: i32,
    /// 关键词内容
    pub name: String,
    /// 关键词内容对应的示例
    pub example: String,
    /// 参数类型
    pub rule: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryTemplateTitleList {
    /// 类目 id，多个用逗号隔开
    pub ids: String,
    /// 用于分页，表示从 start 开始。从 0 开始计数。
    pub start: i32,
    /// 用于分页，表示拉取 limit 条记录。最大为 30。
    pub limit: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateTitleList {
    /// 模版标题列表总数
    pub count: i64,
    /// 模板标题列表
    pub data: Vec<TemplateTitle>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TemplateTitle {
    /// 模版标题 id
    pub tid: i64,
    /// 模版标题
    pub title: String,
    /// 模版类型，2 为一次性订阅，3 为长期订阅
    #[serde(rename = "type")]
    pub type_: i32,
    /// 模版所属类目 id
    pub category_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateList {
    /// 个人模板列表
    pub data: Vec<Template>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Template {
    /// 添加至帐号下的模板 id，发送小程序订阅消息时所需
    pub pri_tmpl_id: String,
    /// 模版标题
    pub title: String,
    /// 模版内容
    pub content: String,
    /// 模板内容示例
    pub example: String,
    /// 模版类型，2 为一次性订阅，3 为长期订阅
    #[serde(rename = "type")]
    pub type_: i32,
    /// 枚举参数值范围
    pub keyword_enum_value_list: Vec<KeywordEnum>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeywordEnum {
    /// 枚举参数的 key
    pub keyword_code: String,
    /// 枚举参数值范围列表
    pub enum_value_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendData {
    /// 接收者（用户）的 openid
    pub touser: String,
    /// 所需下发的订阅模板id
    pub template_id: String,
    /// 点击模板卡片后的跳转页面，仅限本小程序内的页面。支持带参数,（示例index?foo=bar）。该字段不填则模板无跳转。
    #[serde(default)]
    pub page: Option<String>,
    /// 模板内容，格式形如 { "key1": { "value": any }, "key2": { "value": any } }
    pub data: HashMap<String, StrValue>,
    /// 跳转小程序类型：developer为开发版；trial为体验版；formal为正式版；默认为正式版
    #[serde(default)]
    pub miniprogram_state: Option<String>,
    /// 进入小程序查看”的语言类型，支持zh_CN(简体中文)、en_US(英文)、zh_HK(繁体中文)、zh_TW(繁体中文)，默认为zh_CN
    #[serde(default)]
    pub lang: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StrValue {
    pub value: String,
}

pub struct SubscribeMessageModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> SubscribeMessageModule<'a, T> {
    /// 组合模板并添加至帐号下的个人模板库
    pub async fn add_template(&self, data: &AddTemplate) -> SdkResult<PriTmplId> {
        let url = "https://api.weixin.qq.com/wxaapi/newtmpl/addtemplate";
        post_send(self.0, url, data).await
    }

    /// 删除帐号下的个人模板
    pub async fn delete_template(&self, data: &PriTmplId) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/newtmpl/deltemplate";
        post_send(self.0, url, data).await
    }

    /// 获取小程序账号的类目
    pub async fn get_category(&self) -> SdkResult<CategoryData> {
        let url = "https://api.weixin.qq.com/wxaapi/newtmpl/getcategory";
        get_send(self.0, url, &()).await
    }

    /// 获取模板标题下的关键词列表
    pub async fn get_pub_template_keywords_by_id(&self, tid: &str) -> SdkResult<TemplateKeywords> {
        let query = &serde_json::json!({ "tid": tid });
        let url = "https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatekeywords";
        get_send(self.0, url, query).await
    }

    /// 获取帐号所属类目下的公共模板标题
    pub async fn get_pub_template_title_list(
        &self,
        query: &QueryTemplateTitleList,
    ) -> SdkResult<TemplateTitleList> {
        let url = "https://api.weixin.qq.com/wxaapi/newtmpl/getpubtemplatetitles";
        get_send(self.0, url, query).await
    }

    /// 获取当前帐号下的个人模板列表
    pub async fn get_template_list(&self) -> SdkResult<TemplateList> {
        let url = "https://api.weixin.qq.com/wxaapi/newtmpl/gettemplate";
        get_send(self.0, url, &()).await
    }

    /// 发送订阅消息
    pub async fn send(&self, data: &SendData) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/message/subscribe/send";
        post_send(self.0, url, data).await
    }
}
