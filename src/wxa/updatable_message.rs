use super::{get_send, post_send};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateActivityId {
    /// 为私密消息创建activity_id时，指定分享者为 `unionid` 用户。其余用户不能用此activity_id分享私密消息。<br/>
    /// openid与unionid填一个即可。私密消息暂不支持云函数生成activity id。
    #[serde(default)]
    pub unionid: Option<String>,
    /// 为私密消息创建activity_id时，指定分享者为 `openid` 用户。其余用户不能用此activity_id分享私密消息。<br/>
    /// openid与unionid填一个即可。私密消息暂不支持云函数生成activity id。
    #[serde(default)]
    pub openid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivityId {
    /// 动态消息的 ID
    pub activity_id: String,
    /// activity_id 的过期时间戳。默认24小时后过期。
    pub expiration_time: i64,
    /// 错误码
    pub errcode: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetUpdatableMsg {
    /// 动态消息的 ID，通过 `create_activity_id` 接口获取
    pub activity_id: String,
    /// 动态消息修改后的状态
    /// - `0`, 未开始
    /// - `1`, 已开始
    pub target_state: i32,
    /// 动态消息对应的模板信息
    pub template_info: TemplateInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateInfo {
    /// 模板中需要修改的参数
    pub parameter_list: Vec<Param>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Param {
    /// 要修改的参数名
    pub name: ParamName,
    /// 修改后的参数值
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParamName {
    /// target_state = 0 时必填，文字内容模板中 member_count 的值
    MemberCount,
    /// target_state = 0 时必填，文字内容模板中 room_limit 的值
    RoomLimit,
    /// target_state = 1 时必填，点击「进入」启动小程序时使用的路径。<br/>
    /// 对于小游戏，没有页面的概念，可以用于传递查询字符串（query），如 "?foo=bar"
    Path,
    /// target_state = 1 时必填，点击「进入」启动小程序时使用的版本。<br/>
    /// 有效参数值为：`develop`（开发版），`trial`（体验版），`release`（正式版）
    VersionType,
}

pub struct UpdatableMessageModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> UpdatableMessageModule<'a, T> {
    /// 创建被分享动态消息或私密消息的 activity_id。详见动态消息。
    pub async fn create_activity_id(&self, query: &CreateActivityId) -> SdkResult<ActivityId> {
        let url = "https://api.weixin.qq.com/cgi-bin/message/wxopen/activityid/create";
        get_send(self.0, url, query).await
    }

    /// 修改被分享的动态消息。详见动态消息。
    pub async fn set_updatable_msg(&self, query: &SetUpdatableMsg) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/message/wxopen/updatablemsg/send";
        post_send(self.0, url, query).await
    }
}

// #[test]
// fn test_json_snake_case() {
//     let input = r#"["member_count"]"#;
//     let data = &serde_json::from_str::<Vec<ParamName>>(input).unwrap();
//     println!("{:?}", data);
//     println!("{:?}", serde_json::to_string(data));
// }
