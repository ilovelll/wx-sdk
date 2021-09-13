use super::{post_send, Part};
use crate::{error::CommonResponse, wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMsg {
    /// 用户的 OpenID
    pub touser: String,
    /// 消息类型
    pub msgtype: MsgType,
    /// 文本消息，msgtype="text" 时必填
    #[serde(default)]
    pub text: Option<TextContent>,
    /// 图片消息，msgtype="image" 时必填
    #[serde(default)]
    pub image: Option<MediaId>,
    /// 图文链接，msgtype="link" 时必填
    #[serde(default)]
    pub link: Option<Link>,
    /// 小程序卡片，msgtype="miniprogrampage" 时必填
    #[serde(default)]
    pub miniprogrampage: Option<MiniProgrampage>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgType {
    /// 文本消息
    Text,
    /// 图片消息
    Image,
    /// 图文链接
    Link,
    /// 小程序卡片
    Miniprogrampage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextContent {
    /// 文本消息内容
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaId {
    /// 发送的图片的媒体ID，通过 新增素材接口 上传图片文件获得。
    pub media_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Link {
    /// 消息标题
    pub title: String,
    /// 图文链接消息
    pub description: String,
    /// 图文链接消息被点击后跳转的链接
    pub url: String,
    /// 图文链接消息的图片链接，支持 JPG、PNG 格式，较好的效果为大图 640 X 320，小图 80 X 80
    pub thumb_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniProgrampage {
    /// 消息标题
    pub title: String,
    /// 小程序的页面路径，跟app.json对齐，支持参数，比如pages/index/index?foo=bar
    pub pagepath: String,
    /// 小程序消息卡片的封面， image 类型的 media_id，通过 新增素材接口 上传图片文件获得，建议大小为 520*416
    pub thumb_media_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetTyping {
    /// 用户的 OpenID
    pub touser: String,
    /// 命令
    pub command: Command,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    /// 对用户下发"正在输入"状态
    Typing,
    /// 取消对用户的"正在输入"状态
    CancelTyping,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaResult {
    /// 文件类型
    #[serde(rename = "type")]
    pub type_: String,
    /// 媒体文件上传后，获取标识，3天内有效。
    pub media_id: String,
    /// 媒体文件上传时间戳
    pub created_at: i64,
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
}

pub struct CustomerMessageModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> CustomerMessageModule<'a, T> {
    /// 获取客服消息内的临时素材。即下载临时的多媒体文件。
    pub async fn get_temp_media(&self, media_id: &str) -> SdkResult<Vec<u8>> {
        let url = "https://api.weixin.qq.com/cgi-bin/media/get";
        let builder = self.0.wx_get(url).await?.query(&("media_id", media_id));
        let bytes = builder.send().await?.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// 发送客服消息给用户。详细规则见 发送客服消息
    pub async fn send(&self, data: &SendMsg) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/message/custom/send";
        post_send(self.0, url, data).await
    }

    /// 下发客服当前输入状态给用户。详见 客服消息输入状态
    pub async fn set_typing(&self, data: &SetTyping) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/message/custom/typing";
        post_send(self.0, url, data).await
    }

    /// 把媒体文件上传到微信服务器。目前仅支持图片。用于发送客服消息或被动回复用户消息。
    pub async fn upload_temp_media(&self, data: Part) -> SdkResult<MediaResult> {
        let url = "https://api.weixin.qq.com/cgi-bin/media/upload?type=image";

        let part = reqwest::multipart::Part::bytes(data.data)
            .file_name(data.filename)
            .mime_str(&data.content_type);

        let form = reqwest::multipart::Form::new().part(data.name, part.unwrap());
        let builder = self.0.wx_post(url).await?.multipart(form);
        let res: CommonResponse<MediaResult> = builder.send().await?.json().await?;

        res.into()
    }
}

// #[test]
// fn test_rename_all_lowercase() {
//     let input = r#"["text"]"#;
//     let msg_type = &serde_json::from_str::<Vec<MsgType>>(input).unwrap();
//     println!("{:?}", msg_type);
//     println!("{:?}", serde_json::to_string(msg_type));
// }

// #[test]
// fn test_enum() {
//     let input = r#"{ "touser": "ab", "command": "Typing" }"#;
//     let data = &serde_json::from_str::<SetTyping>(input).unwrap();
//     println!("{:?}", data);
//     println!("{:?}", serde_json::to_string(data));
// }
