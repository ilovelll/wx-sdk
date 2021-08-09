use serde::{Deserialize, Serialize};

use crate::{
    access_token::AccessTokenProvider,
    error::{CommonError, CommonResponse},
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct IndustryPost {
    pub industry_id1: String,
    pub industry_id2: String,
}

/// 设置所属行业
/// 设置行业可在微信公众平台后台完成，每月可修改行业1次，帐号仅可使用所属行业中相关的模板，为方便第三方开发者，提供通过接口调用的方式来修改账号所属行业，具体如下：
pub async fn api_set_industry<T: AccessTokenProvider>(
    form: IndustryPost,
    sdk: &WxSdk<T>,
) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/template/api_set_industry";

    let msg: CommonError = sdk
        .wx_post(base_url)
        .await?
        .json(&serde_json::json!(form))
        .send()
        .await?
        .json()
        .await?;

    msg.into()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndustryInfo {
    pub primary_industry: IndustryItem,
    pub secondary_industry: IndustryItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndustryItem {
    first_class: String,
    second_class: String,
}

/// 获取设置的行业信息
/// 获取帐号设置的行业信息。可登录微信公众平台，在公众号后台中查看行业信息。为方便第三方开发者，提供通过接口调用的方式来获取帐号所设置的行业信息，具体如下:
pub async fn get_industry<T: AccessTokenProvider>(sdk: &WxSdk<T>) -> SdkResult<IndustryInfo> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/template/get_industry";

    let res: CommonResponse<IndustryInfo> =
        sdk.wx_get(base_url).await?.send().await?.json().await?;

    res.into()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddTplResponse {
    pub template_id: Option<String>,
    pub errcode: i32,
    pub errmsg: String,
}

/// 获得模板ID
/// 从行业模板库选择模板到帐号后台，获得模板ID的过程可在微信公众平台后台完成。为方便第三方开发者，提供通过接口调用的方式来获取模板ID，具体如下：
pub async fn api_add_template<T: AccessTokenProvider>(
    template_id_short: String,
    sdk: &WxSdk<T>,
) -> SdkResult<AddTplResponse> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/template/api_add_template";

    let msg: AddTplResponse = sdk
        .wx_post(base_url)
        .await?
        .json(&serde_json::json!({
            "template_id_short": template_id_short
        }))
        .send()
        .await?
        .json()
        .await?;

    Ok(msg)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateList {
    pub template_list: Vec<TemplateItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateItem {
    pub template_id: String,
    pub title: String,
    pub primary_industry: String,
    pub deputy_industry: String,
    pub content: String,
    pub example: String,
}

/// 获取模板列表
/// 获取已添加至帐号下所有模板列表，可在微信公众平台后台中查看模板列表信息。为方便第三方开发者，提供通过接口调用的方式来获取帐号下所有模板信息，具体如下:
pub async fn get_all_private_template<T: AccessTokenProvider>(
    sdk: &WxSdk<T>,
) -> SdkResult<TemplateList> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/template/get_all_private_template";

    let res: CommonResponse<TemplateList> =
        sdk.wx_get(base_url).await?.send().await?.json().await?;

    res.into()
}

/// 删除模板
/// 删除模板可在微信公众平台后台完成，为方便第三方开发者，提供通过接口调用的方式来删除某帐号下的模板，具体如下：
pub async fn del_private_template<T: AccessTokenProvider>(
    template_id: String,
    sdk: &WxSdk<T>,
) -> SdkResult<()> {
    let base_url = "https://api.weixin.qq.com/cgi-bin/template/del_private_template";

    let msg: CommonError = sdk
        .wx_post(base_url)
        .await?
        .json(&serde_json::json!({ "template_id": template_id }))
        .send()
        .await?
        .json()
        .await?;

    msg.into()
}
