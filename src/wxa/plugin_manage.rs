use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyList {
    /// 插件使用方列表
    pub apply_list: Vec<ApplyItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplyItem {
    /// 使用者的appid
    pub appid: String,
    /// 插件状态
    /// - `1` 申请中
    /// - `2` 申请通过
    /// - `3` 已拒绝
    /// - `4` 已超时
    pub status: i32,
    /// 使用者的昵称
    pub nickname: String,
    /// 使用者的头像
    pub headimgurl: String,
    //  文档不全，暂时使用 HashMap
    //  https://developers.weixin.qq.com/miniprogram/dev/api-backend/open-api/plugin-management/pluginManager.getPluginDevApplyList.html
    /// 使用者的类目
    pub categories: Vec<HashMap<String, String>>,
    /// 使用者的申请时间
    pub create_time: String,
    /// 使用者的小程序码
    pub apply_url: String,
    /// 使用者的申请说明
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginList {
    /// 申请或使用中的插件列表
    pub plugin_list: Vec<PluginItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginItem {
    /// 插件 appId
    pub appid: String,
    /// 插件状态
    /// - `1` 申请中
    /// - `2` 申请通过
    /// - `3` 已拒绝
    /// - `4` 已超时
    pub status: i32,
    /// 插件昵称
    pub nickname: String,
    /// 插件头像
    pub headimgurl: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetDevPluginStatus {
    /// 修改操作
    pub action: SetAction,
    /// 使用者的 appid。同意申请时填写。
    #[serde(default)]
    pub appid: Option<String>,
    /// 拒绝理由。拒绝申请时填写。
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetAction {
    /// 同意申请
    DevAgree,
    /// 拒绝申请
    DevRefuse,
    /// 删除已拒绝的申请者
    DevDelete,
}

pub struct PluginManageModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> PluginManageModule<'a, T> {
    /// 向插件开发者发起使用插件的申请
    pub async fn apply_plugin(&self, plugin_appid: &str, reason: Option<String>) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/plugin";
        let data = &serde_json::json!({
            "action": "apply",
            "plugin_appid": plugin_appid,
            "reason": reason,
        });
        post_send(self.0, url, data).await
    }

    /// 获取当前所有插件使用方（供插件开发者调用）
    pub async fn get_plugin_dev_apply_list(&self, page: i32, num: i32) -> SdkResult<ApplyList> {
        let url = "https://api.weixin.qq.com/wxa/devplugin";
        let data = &serde_json::json!({
            "action": "dev_apply_list",
            "page": page,
            "num": num,
        });
        post_send(self.0, url, data).await
    }

    /// 查询已添加的插件
    pub async fn get_plugin_list(&self) -> SdkResult<PluginList> {
        let url = "https://api.weixin.qq.com/wxa/plugin";
        post_send(self.0, url, &serde_json::json!({"action": "list"})).await
    }

    /// 修改插件使用申请的状态（供插件开发者调用）
    pub async fn set_dev_plugin_apply_status(&self, data: &SetDevPluginStatus) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/devplugin";
        post_send(self.0, url, data).await
    }

    /// 删除已添加的插件
    pub async fn unbind_plugin(&self, plugin_appid: &str) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/plugin";
        let data = &serde_json::json!({"action": "unbind", "plugin_appid": plugin_appid});
        post_send(self.0, url, data).await
    }
}
