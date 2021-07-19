use serde::{Deserialize, Serialize};

use crate::{
    access_token::AccessTokenProvider,
    error::CommonResponse,
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

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

/// 获取客服基本信息
pub async fn getkflist<T: AccessTokenProvider>(sdk: &WxSdk<T>) -> SdkResult<KFList> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/getkflist";

    let res: CommonResponse<KFList> = sdk.wx_get(base_url).await?.send().await?.json().await?;
    res.into()
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

/// 获取在线客服信息
pub async fn getonlinekflist<T: AccessTokenProvider>(sdk: &WxSdk<T>) -> SdkResult<KFOnlineList> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/getonlinekflist";

    let res: CommonResponse<KFOnlineList> =
        sdk.wx_get(base_url).await?.send().await?.json().await?;
    res.into()
}

pub mod kfaccount {
    use crate::{
        access_token::AccessTokenProvider,
        error::CommonError,
        office_account::material::FileStruct,
        wechat::{WxApiRequestBuilder, WxSdk},
        SdkResult,
    };

    /// 添加客服帐号
    pub async fn add<T: AccessTokenProvider>(
        kf_account: String,
        nickname: String,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/add";

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
    pub async fn inviteworker<T: AccessTokenProvider>(
        kf_account: String,
        invite_wx: String,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/inviteworker";

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
    pub async fn update<T: AccessTokenProvider>(
        kf_account: String,
        nickname: String,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/update";

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
    pub async fn uploadheadimg<T: AccessTokenProvider>(
        kf_account: String,
        file: FileStruct,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/uploadheadimg";
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
    pub async fn del<T: AccessTokenProvider>(kf_account: String, sdk: &WxSdk<T>) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfaccount/del";

        let builder = sdk.wx_get(base_url).await?;
        let builder = builder.query(&[("kf_account", kf_account)]);

        let res: CommonError = builder.send().await?.json().await?;

        res.into()
    }
}
pub mod kfsession {
    use crate::{
        access_token::AccessTokenProvider,
        error::CommonError,
        wechat::{WxApiRequestBuilder, WxSdk},
        SdkResult,
    };

    /// 创建会话
    pub async fn create<T: AccessTokenProvider>(
        kf_account: String,
        openid: String,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfsession/create";

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
    pub async fn close<T: AccessTokenProvider>(
        kf_account: String,
        openid: String,
        sdk: &WxSdk<T>,
    ) -> SdkResult<()> {
        let base_url = "https://api.weixin.qq.com/cgi-bin/customservice/kfsession/close";

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
