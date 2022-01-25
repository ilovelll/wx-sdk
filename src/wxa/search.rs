use super::{post_send, Part};
use crate::{error::CommonResponse, wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgSearchRes {
    items: Vec<ImgSearchResItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImgSearchResItem {
    /// 小程序商品页面标题
    pub title: String,
    /// 小程序商品页面主图url
    pub img_url: String,
    /// 小程序商品页面价格
    pub price: String,
    /// 小程序商品页面地址
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteSearch {
    /// 关键词
    pub keyword: String,
    /// 请求下一页的参数，开发者无需理解。为空时查询的是第一页内容，如需查询下一页，把返回参数的next_page_info填充到这里即可
    pub next_page_info: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteSearchRes {
    /// 搜索结果列表
    pub items: Vec<SiteSearchResItem>,
    /// 是否有下一页
    pub has_next_page: bool,
    /// 请求下一页的参数，开发者无需理解，如需查询下一页结果，把该参数填充到下页请求参数中的next_page_info即可
    pub next_page_info: String,
    /// 估算索引文档数
    pub hit_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteSearchResItem {
    /// 小程序页面标题
    pub title: String,
    /// 小程序页面摘要
    pub description: String,
    /// 小程序页面代表图
    pub image: String,
    /// 小程序页面路径
    pub path: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitPages {
    /// 小程序页面信息列表
    pub pages: Vec<SubmitPagesItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubmitPagesItem {
    /// 页面路径
    pub path: String,
    /// 页面参数
    pub query: String,
}

pub struct SearchModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> SearchModule<'a, T> {
    /// 基于小程序的站内搜商品图片搜索能力
    pub async fn image_search(&self, data: Vec<Part>) -> SdkResult<ImgSearchRes> {
        let url = "https://api.weixin.qq.com/wxa/imagesearch";

        let builder = self.0.wx_post(url).await?;
        let form =
            data.into_iter()
                .try_fold(reqwest::multipart::Form::new(), |form, data_part| {
                    let part = reqwest::multipart::Part::bytes(data_part.data)
                        .file_name(data_part.filename)
                        .mime_str(&data_part.content_type)?;
                    Ok::<_, reqwest::Error>(form.part("img", part))
                })?;
        let builder = builder.multipart(form);

        let res: CommonResponse<ImgSearchRes> = builder.send().await?.json().await?;

        res.into()
    }

    /// 小程序内部搜索API提供针对页面的查询能力，小程序开发者输入搜索词后，将返回自身小程序和搜索词相关的页面。
    /// 因此，利用该接口，开发者可以查看指定内容的页面被微信平台的收录情况；
    /// 同时，该接口也可供开发者在小程序内应用，给小程序用户提供搜索能力。
    pub async fn site_search(&self, data: &SiteSearch) -> SdkResult<SiteSearchRes> {
        let url = "https://api.weixin.qq.com/wxa/sitesearch";
        post_send(self.0, url, data).await
    }

    /// 小程序开发者可以通过本接口提交小程序页面url及参数信息(不要推送webview页面)，
    /// 让微信可以更及时的收录到小程序的页面信息，开发者提交的页面信息将可能被用于小程序搜索结果展示。
    pub async fn submit_pages(&self, data: &SubmitPages) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/search/wxaapi_submitpages";
        post_send(self.0, url, data).await
    }
}
