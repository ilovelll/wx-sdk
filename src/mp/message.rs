use serde::{Deserialize, Serialize};

use crate::wechat::WxApiRequestBuilder;

use self::{mass::MassModule, template::TemplateModule};

pub mod template {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use crate::{
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

    pub struct TemplateModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

    impl<'a, T: WxApiRequestBuilder> TemplateModule<'a, T> {
        /// 发送模板消息
        pub async fn send(&self, params: SendTplMsgParams) -> SdkResult<SendTplMsgResponse> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/message/template/send";
            let sdk = self.0;
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
}

pub mod mass {

    use serde::{Deserialize, Serialize};

    use crate::{error::CommonError, wechat::WxApiRequestBuilder, SdkResult};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SendAllFilter {
        pub is_to_all: bool,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub tag_id: Option<i32>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(untagged)]
    pub enum SendContent {
        Text(Text),
        Images(Images),
        Image(Image),
        Voice(Voice),
        MPNews(MPNews),
        MPVideo(MPVideo),
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Text {
        pub msgtype: String,
        pub text: TextContent,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct TextContent {
        pub content: String,
    }

    impl Text {
        pub fn new<S: AsRef<str>>(text: S) -> Self {
            Text {
                msgtype: "text".to_owned(),
                text: TextContent {
                    content: text.as_ref().to_owned(),
                },
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MPVideo {
        pub msgtype: String,
        pub mpvideo: MediaID,
    }

    impl MPVideo {
        pub fn new<S: AsRef<str>>(media_id: S) -> Self {
            MPVideo {
                msgtype: "mpvideo".to_owned(),
                mpvideo: MediaID {
                    media_id: media_id.as_ref().to_owned(),
                },
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Voice {
        pub msgtype: String,
        pub voice: MediaID,
    }

    impl Voice {
        pub fn new<S: AsRef<str>>(media_id: S) -> Self {
            Voice {
                msgtype: "voice".to_owned(),
                voice: MediaID {
                    media_id: media_id.as_ref().to_owned(),
                },
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Images {
        pub msgtype: String,
        pub images: ImagesContent,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Image {
        pub msgtype: String,
        pub image: MediaID,
    }

    impl Image {
        pub fn new<S: AsRef<str>>(media_id: S, _recommend: Option<String>) -> Self {
            Image {
                msgtype: "image".to_owned(),
                image: MediaID {
                    media_id: media_id.as_ref().to_owned(),
                },
            }
        }
    }

    impl Images {
        pub fn new(media_ids: Vec<String>, recommend: Option<String>) -> Self {
            Images {
                msgtype: "image".to_owned(),
                images: ImagesContent {
                    media_ids,
                    recommend,
                },
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ImagesContent {
        pub media_ids: Vec<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub recommend: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MPNews {
        pub msgtype: String,
        pub mpnews: MediaID,
        pub send_ignore_reprint: i8,
    }

    impl MPNews {
        pub fn new<S: AsRef<str>>(media_id: S, send_ignore_reprint: i8) -> Self {
            MPNews {
                msgtype: "mpnews".to_owned(),
                mpnews: MediaID {
                    media_id: media_id.as_ref().to_owned(),
                },
                send_ignore_reprint,
            }
        }
    }

    // #[derive(Serialize, Deserialize, Debug)]
    // pub struct SendVideo {
    //     pub media_id: String,
    //     pub title: Option<String>,
    //     pub description: Option<String>,
    // }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MediaID {
        media_id: String,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct SendAll {
        filter: SendAllFilter,

        #[serde(flatten)]
        content: SendContent,

        #[serde(skip_serializing_if = "Option::is_none")]
        clientmsgid: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Send {
        touser: Vec<String>,

        #[serde(flatten)]
        content: SendContent,

        #[serde(skip_serializing_if = "Option::is_none")]
        clientmsgid: Option<String>,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct SendPreview {
        touser: String,

        #[serde(flatten)]
        content: SendContent,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct SendResponse {
        pub msg_id: Option<i64>,
        pub msg_data_id: Option<i64>,
        pub errcode: i32,
        pub errmsg: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DeleteSend {
        pub msg_id: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        article_idx: Option<u8>,
    }

    pub struct MassModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);
    impl<'a, T: WxApiRequestBuilder> MassModule<'a, T> {
        /// 根据标签进行群发
        pub async fn send_all(
            &self,
            filter: SendAllFilter,
            content: SendContent,
            client_msg_id: Option<String>,
        ) -> SdkResult<SendResponse> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/message/mass/sendall";

            let send = SendAll {
                filter,
                content,
                clientmsgid: client_msg_id,
            };
            let sdk = self.0;
            let msg: SendResponse = sdk
                .wx_post(base_url)
                .await?
                .json(&send)
                .send()
                .await?
                .json()
                .await?;

            Ok(msg)
        }

        /// 根据OpenID列表群发
        pub async fn send(
            &self,
            touser: Vec<String>,
            content: SendContent,
            client_msg_id: Option<String>,
        ) -> SdkResult<SendResponse> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/message/mass/send";

            let send = Send {
                touser,
                content,
                clientmsgid: client_msg_id,
            };
            let sdk = self.0;
            // println!("{:#?}", serde_json::to_string(&send));
            let res: SendResponse = sdk
                .wx_post(base_url)
                .await?
                .json(&send)
                .send()
                .await?
                .json()
                .await?;

            Ok(res)
        }

        /// 删除群发
        pub async fn delete(&self, msg_id: String, article_idx: Option<u8>) -> SdkResult<()> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/message/mass/delete";

            let send = DeleteSend {
                msg_id,
                article_idx,
            };
            let sdk = self.0;
            // println!("{:#?}", serde_json::to_string(&send));
            let res: CommonError = sdk
                .wx_post(base_url)
                .await?
                .json(&send)
                .send()
                .await?
                .json()
                .await?;

            res.into()
        }

        ///预览接口
        pub async fn preview(
            &self,
            touser: String,
            content: SendContent,
        ) -> SdkResult<SendResponse> {
            let base_url = "https://api.weixin.qq.com/cgi-bin/message/mass/preview";

            let send = SendPreview { touser, content };
            let sdk = self.0;
            // println!("{:#?}", serde_json::to_string(&send));
            let res: SendResponse = sdk
                .wx_post(base_url)
                .await?
                .json(&send)
                .send()
                .await?
                .json()
                .await?;

            Ok(res)
        }
    }
}
pub struct MessageModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> MessageModule<'a, T> {
    pub fn template(&self) -> TemplateModule<T> {
        TemplateModule(self.0)
    }

    pub fn mass(&self) -> MassModule<T> {
        MassModule(self.0)
    }
}
