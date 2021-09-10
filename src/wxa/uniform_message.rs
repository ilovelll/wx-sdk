use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct UniformMsg {
    /// 用户openid，可以是小程序的openid，也可以是mp_template_msg.appid对应的公众号的openid
    pub touser: String,
    /// 小程序模板消息相关的信息，可以参考小程序模板消息接口; 有此节点则优先发送小程序模板消息；<br/>
    ///（小程序模板消息已下线，不用传此节点）
    #[serde(default)]
    pub weapp_template_msg: Option<WeappTemplateMsg>,
    /// 公众号模板消息相关的信息，可以参考公众号模板消息接口；<br/>
    /// 有此节点并且没有weapp_template_msg节点时，发送公众号模板消息
    pub mp_template_msg: MpTemplateMsg,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeappTemplateMsg {
    /// 小程序模板ID
    pub template_id: String,
    /// 小程序页面路径
    pub page: String,
    /// 小程序模板消息formid
    pub form_id: String,
    /// 小程序模板数据
    pub data: HashMap<String, Value>,
    /// 小程序模板放大关键词
    pub emphasis_keyword: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Value {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MpTemplateMsg {
    /// 公众号appid，要求与小程序有绑定且同主体
    pub appid: String,
    /// 公众号模板id
    pub template_id: String,
    /// 公众号模板消息所要跳转的url
    pub url: String,
    /// 公众号模板消息所要跳转的小程序，小程序的必须与公众号具有绑定关系
    pub miniprogram: String,
    /// 公众号模板消息的数据
    pub data: HashMap<String, ValueColor>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValueColor {
    pub value: String,
    pub color: String,
}

pub struct UniformMessageModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> UniformMessageModule<'a, T> {
    /// 下发小程序和公众号统一的服务消息
    pub async fn send(&self, query: &UniformMsg) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/message/wxopen/template/uniform_send";
        post_send(self.0, url, query).await
    }
}
