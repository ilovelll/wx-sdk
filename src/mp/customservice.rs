use serde::{Deserialize, Serialize};

use crate::{
    error::{CommonError, CommonResponse},
    wechat::WxApiRequestBuilder,
    SdkResult,
};

use super::material::FileStruct;

#[derive(Debug, Serialize, Deserialize)]
pub struct KFList {
    kf_list: Vec<KFAccount>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct KFAccount {
    pub kf_account: String,
    pub kf_headimgurl: String,
    pub kf_id: String,
    pub kf_nick: String,
    #[serde(alias = "invite_wx")]
    pub kf_wx: String,
    pub invite_expire_time: Option<i64>,
    pub invite_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KFOnlineList {
    kf_online_list: Vec<KFOnlineAccount>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KFOnlineAccount {
    pub kf_account: String,
    pub status: i8,
    pub kf_id: String,
    pub accepted_case: i8,
}

/// Custom service module 客服模块
pub struct CustomServiceModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);
impl<'a, T: WxApiRequestBuilder> CustomServiceModule<'a, T> {
    /// 获取客服基本信息
    pub async fn getkflist(&self) -> SdkResult<KFList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/getkflist";
        let sdk = self.0;
        let res: CommonResponse<KFList> = sdk.wx_get(base_url).await?.send().await?.json().await?;
        res.into()
    }

    /// 获取在线客服信息
    pub async fn getonlinekflist(&self) -> SdkResult<KFOnlineList> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/getonlinekflist";
        let sdk = self.0;

        let res: CommonResponse<KFOnlineList> =
            sdk.wx_get(base_url).await?.send().await?.json().await?;
        res.into()
    }

    /// 添加客服帐号
    pub async fn kfaccount_add(&self, kf_account: String, nickname: String) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/add";
        let sdk = self.0;
        let msg: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({
                "kf_account": kf_account,
                "nickname": nickname
            }))
            .send()
            .await?
            .json()
            .await?;

        msg.into()
    }

    /// 邀请绑定客服帐号
    pub async fn kfaccount_inviteworker(
        &self,
        kf_account: String,
        invite_wx: String,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/inviteworker";
        let sdk = self.0;
        let msg: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({
                "kf_account": kf_account,
                "invite_wx": invite_wx
            }))
            .send()
            .await?
            .json()
            .await?;

        msg.into()
    }

    /// 设置客服信息
    pub async fn kfaccount_update(&self, kf_account: String, nickname: String) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/update";
        let sdk = self.0;
        let msg: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({
                "kf_account": kf_account,
                "nickname": nickname
            }))
            .send()
            .await?
            .json()
            .await?;

        msg.into()
    }

    /// 上传客服头像
    pub async fn kfaccount_uploadheadimg(
        &self,
        kf_account: String,
        file: FileStruct,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/uploadheadimg";
        let sdk = self.0;
        let builder = sdk.wx_post(base_url).await?;
        let builder = builder.query(&[("kf_account", kf_account)]);

        let part = reqwest::multipart::Part::bytes(file.file)
            .file_name(file.filename)
            .mime_str(file.conten_type.as_ref());

        // form-data 中媒体文件标识，有filename、filelength、content-type 等信息，文件大小为5M 以内
        let form = reqwest::multipart::Form::new().part("media", part.unwrap());

        let res: CommonError = builder.multipart(form).send().await?.json().await?;

        res.into()
    }

    /// 删除客服帐号
    pub async fn kfaccount_del(&self, kf_account: String) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/del";
        let sdk = self.0;
        let builder = sdk.wx_get(base_url).await?;
        let builder = builder.query(&[("kf_account", kf_account)]);

        let res: CommonError = builder.send().await?.json().await?;

        res.into()
    }

    /// 创建会话
    pub async fn kfsession_create(&self, kf_account: String, openid: String) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfsession/create";
        let sdk = self.0;
        let msg: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({
                "kf_account": kf_account,
                "openid": openid
            }))
            .send()
            .await?
            .json()
            .await?;

        msg.into()
    }

    /// 关闭会话
    pub async fn kfsession_close(&self, kf_account: String, openid: String) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfsession/close";
        let sdk = self.0;
        let msg: CommonError = sdk
            .wx_post(base_url)
            .await?
            .json(&serde_json::json!({
                "kf_account": kf_account,
                "openid": openid
            }))
            .send()
            .await?
            .json()
            .await?;

        msg.into()
    }
}
