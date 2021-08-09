use serde::{Deserialize, Serialize};

pub mod template {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use crate::{
        access_token::AccessTokenProvider,
        wechat::{WxApiRequestBuilder, WxSdk},
        SdkResult,
    };

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SendTplMsgResponse {
        pub msgid: Option<i64>,
        pub errcode: i32,
        pub errmsg: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SendTplMsgParams {
        pub touser: String,
        pub template_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub miniprogram: Option<MiniProgramData>,
        pub data: HashMap<String, TplMsgData>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TplMsgData {
        pub value: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub color: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MiniProgramData {
        pub appid: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pagepath: Option<String>,
    }

    /// 发送模板消息
    pub async fn send<T: AccessTokenProvider>(
        params: SendTplMsgParams,
        sdk: &WxSdk<T>,
    ) -> SdkResult<SendTplMsgResponse> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/message/template/send";

        let msg: SendTplMsgResponse = sdk
            .wx_post(base_url)
            .await?
            .json(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(msg)
    }
}
