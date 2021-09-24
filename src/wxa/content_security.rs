use super::{post_send, Part};
use crate::{error::CommonError, wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    /// 要检测的图片或音频的url，支持图片格式包括jpg, jepg, png, bmp, gif（取首帧），<br/>
    /// 支持的音频格式包括mp3, aac, ac3, wma, flac, vorbis, opus, wav
    pub media_url: String,
    /// 1:音频;2:图片
    pub media_type: i32,
    /// 接口版本号，2.0版本为固定值2
    pub version: f64,
    /// 用户的openid（用户需在近两小时访问过小程序）
    pub openid: String,
    /// 场景枚举值（1 资料；2 评论；3 论坛；4 社交日志）
    pub scene: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaCheckResult {
    /// 唯一请求标识，标记单次请求，用于匹配异步推送结果
    pub trace_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    /// 接口版本号，2.0版本为固定值2
    pub version: f64,
    /// 用户的openid（用户需在近两小时访问过小程序）
    pub openid: String,
    /// 场景枚举值（1 资料；2 评论；3 论坛；4 社交日志）
    pub scene: i32,
    /// 需检测的文本内容，文本字数的上限为2500字
    pub content: String,
    /// 用户昵称
    #[serde(default)]
    pub nickname: Option<String>,
    /// 文本标题
    #[serde(default)]
    pub title: Option<String>,
    /// 个性签名，该参数仅在资料类场景有效(scene=1)
    #[serde(default)]
    pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgCheckRes {
    /// 唯一请求标识，标记单次请求
    pub trace_id: String,
    /// 综合结果
    pub result: MsgCheckResult,
    /// 详细检测结果
    pub detail: Vec<MsgCheckDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgCheckResult {
    /// 建议，有risky、pass、review三种值
    pub suggust: String,
    /// 命中标签枚举值，100 正常；10001 广告；20001 时政；20002 色情；20003 辱骂；<br/>
    /// 20006 违法犯罪；20008 欺诈；20012 低俗；20013  版权；21000 其他
    pub label: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgCheckDetail {
    /// 策略类型
    pub strategy: String,
    /// 错误码，仅当该值为0时，该项结果有效
    pub errcode: i32,
    /// 建议，有risky、pass、review三种值
    pub suggest: String,
    /// 命中标签枚举值，100 正常；10001 广告；20001 时政；20002 色情；20003 辱骂；20006 违法犯罪；20008 欺诈；20012 低俗；20013  版权；21000 其他
    #[serde(default)]
    pub label: Option<i32>,
    /// 0-100，代表置信度，越高代表越有可能属于当前返回的标签（label）
    #[serde(default)]
    pub prob: Option<i32>,
    /// 命中的自定义关键词
    #[serde(default)]
    pub keyword: Option<String>,
}

pub struct ContentSecurityModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> ContentSecurityModule<'a, T> {
    /// 校验一张图片是否含有违法违规内容。详见内容安全解决方案
    pub async fn img_sec_check(&self, data: Part) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/img_sec_check";
        let part = reqwest::multipart::Part::bytes(data.data)
            .file_name(data.filename)
            .mime_str(&data.content_type);

        let form = reqwest::multipart::Form::new().part(data.name, part.unwrap());
        let builder = self.0.wx_post(url).await?.multipart(form);
        let res: CommonError = builder.send().await?.json().await?;

        res.into()
    }

    /// 异步校验图片/音频是否含有违法违规内容。
    pub async fn media_check_async(&self, data: &Media) -> SdkResult<MediaCheckResult> {
        let url = "https://api.weixin.qq.com/wxa/media_check_async";
        post_send(self.0, url, data).await
    }

    /// 检查一段文本是否含有违法违规内容。
    pub async fn msg_sec_check(&self, data: &Msg) -> SdkResult<MsgCheckRes> {
        let url = "https://api.weixin.qq.com/wxa/msg_sec_check";
        post_send(self.0, url, data).await
    }
}
