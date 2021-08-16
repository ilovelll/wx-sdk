use serde::{Deserialize, Serialize};

use crate::{
    access_token::AccessTokenProvider,
    error::CommonResponse,
    wechat::{WxApiRequestBuilder, WxSdk},
    SdkResult,
};

// pub mod ad;
#[derive(Debug, Serialize, Deserialize)]
pub struct ListRes<T> {
    pub list: Vec<T>,
}

// 冲突的实现，
// impl<T> Into<WxResult<Vec<T>>> for WxResult<ListRes<T>> {
//     fn into(self) -> WxResult<Vec<T>> {
//         self.map(|ls| ls.list)
//     }
// }

impl<T> From<ListRes<T>> for Vec<T> {
    fn from(t: ListRes<T>) -> Self {
        t.list
    }
}

#[derive(Debug, Serialize)]
pub struct TimeSpan {
    pub begin_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserNewCancel {
    pub ref_date: String,
    pub user_source: i32,
    pub new_user: i32,
    pub cancel_user: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCumulate {
    pub ref_date: String,
    pub cumulate_user: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleSummary {
    pub ref_date: String,
    pub msgid: String,
    pub title: String,
    pub int_page_read_user: i32,
    pub int_page_read_count: i32,
    pub ori_page_read_user: i32,
    pub ori_page_read_count: i32,
    pub share_user: i32,
    pub share_count: i32,
    pub add_to_fav_user: i32,
    pub add_to_fav_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleDetail {
    pub stat_date: String,
    pub target_user: i64,
    pub int_page_read_user: i32,
    pub int_page_read_count: i32,
    pub ori_page_read_user: i32,
    pub ori_page_read_count: i32,
    pub share_user: i32,
    pub share_count: i32,
    pub add_to_fav_user: i32,
    pub add_to_fav_count: i32,
    pub int_page_from_session_read_user: i32,
    pub int_page_from_session_read_count: i32,
    pub int_page_from_hist_msg_read_user: i32,
    pub int_page_from_hist_msg_read_count: i32,
    pub int_page_from_feed_read_user: i32,
    pub int_page_from_feed_read_count: i32,
    pub int_page_from_friends_read_user: i32,
    pub int_page_from_friends_read_count: i32,
    pub int_page_from_other_read_user: i32,
    pub int_page_from_other_read_count: i32,
    pub feed_share_from_session_user: i32,
    pub feed_share_from_session_cnt: i32,
    pub feed_share_from_feed_user: i32,
    pub feed_share_from_feed_cnt: i32,
    pub feed_share_from_other_user: i32,
    pub feed_share_from_other_cnt: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleTotal {
    pub ref_date: String,
    pub msgid: String,
    pub title: String,
    pub details: Vec<ArticleDetail>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRead {
    pub ref_date: String,
    pub user_source: i32,
    pub int_page_read_user: i32,
    pub int_page_read_count: i32,
    pub ori_page_read_user: i32,
    pub ori_page_read_count: i32,
    pub share_user: i32,
    pub share_count: i32,
    pub add_to_fav_user: i32,
    pub add_to_fav_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReadHour {
    pub ref_date: String,
    pub ref_hour: i32,
    pub user_source: i32,
    pub int_page_read_user: i32,
    pub int_page_read_count: i32,
    pub ori_page_read_user: i32,
    pub ori_page_read_count: i32,
    pub share_user: i32,
    pub share_count: i32,
    pub add_to_fav_user: i32,
    pub add_to_fav_count: i32,
}

// pub async fn get_analyze<T: AccessTokenProvider, U: DeserializeOwned>(time: &TimeSpan, sdk: &WxSdk<T>, url: &'static str) -> SdkResult<ListRes<U>> {
//      let res: CommonResponse<ListRes<U>> = sdk.wx_post(url).await?.json(time).send().await?.json().await?;
//     res.into()
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct UserShare {
    pub ref_date: String,
    pub share_scene: i32,
    pub share_count: i32,
    pub share_user: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserShareHour {
    pub ref_date: String,
    pub ref_hour: i32,
    pub share_scene: i32,
    pub share_count: i32,
    pub share_user: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamMsg {
    pub ref_date: String,
    pub msg_type: i32,
    pub msg_user: i32,
    pub msg_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamMsgHour {
    pub ref_date: String,
    pub ref_hour: i32,
    pub msg_type: i32,
    pub msg_user: i32,
    pub msg_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpstreamMsgDist {
    pub ref_date: String,
    pub count_interval: i32,
    pub msg_user: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiAnalysis {
    pub ref_date: String,
    pub callback_count: i32,
    pub fail_count: i32,
    pub total_time_cost: i64,
    pub max_time_cost: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiAnalysisHour {
    pub ref_date: String,
    pub ref_hour: i32,
    pub callback_count: i32,
    pub fail_count: i32,
    pub total_time_cost: i64,
    pub max_time_cost: i64,
}

pub struct DataCubeModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);
impl<'a, T: WxApiRequestBuilder> DataCubeModule<'a, T> {
    pub async fn get_user_summary(&self, time: &TimeSpan) -> SdkResult<ListRes<UserNewCancel>> {
        let base_url = "https://api.weixin.qq.com/datacube/getusersummary";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UserNewCancel>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_user_cumulate(&self, time: &TimeSpan) -> SdkResult<ListRes<UserCumulate>> {
        let base_url = "https://api.weixin.qq.com/datacube/getusercumulate";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UserCumulate>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_article_summary(&self, time: &TimeSpan) -> SdkResult<ListRes<ArticleSummary>> {
        let base_url = "https://api.weixin.qq.com/datacube/getarticlesummary";
        let sdk = self.0;
        let res: CommonResponse<ListRes<ArticleSummary>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_article_total(&self, time: &TimeSpan) -> SdkResult<ListRes<ArticleTotal>> {
        let base_url = "https://api.weixin.qq.com/datacube/getarticletotal";
        let sdk = self.0;
        let res: CommonResponse<ListRes<ArticleTotal>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_user_read(&self, time: &TimeSpan) -> SdkResult<ListRes<UserRead>> {
        let base_url = "https://api.weixin.qq.com/datacube/getuserread";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UserRead>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_user_share(&self, time: &TimeSpan) -> SdkResult<ListRes<UserShare>> {
        let base_url = "https://api.weixin.qq.com/datacube/getusershare";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UserShare>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_user_read_hour(&self, time: &TimeSpan) -> SdkResult<ListRes<UserReadHour>> {
        let base_url = "https://api.weixin.qq.com/datacube/getuserreadhour";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UserReadHour>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_user_share_hour(&self, time: &TimeSpan) -> SdkResult<ListRes<UserShareHour>> {
        let base_url = "https://api.weixin.qq.com/datacube/getusersharehour";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UserShareHour>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_upstream_msg_hour(
        &self,
        time: &TimeSpan,
    ) -> SdkResult<ListRes<UpstreamMsgHour>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsghour";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsgHour>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    pub async fn get_upstream_msg_week(&self, time: &TimeSpan) -> SdkResult<ListRes<UpstreamMsg>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsgweek";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsg>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    pub async fn get_upstream_msg_month(&self, time: &TimeSpan) -> SdkResult<ListRes<UpstreamMsg>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsgmonth";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsg>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    pub async fn get_upstream_msg(&self, time: &TimeSpan) -> SdkResult<ListRes<UpstreamMsg>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsg";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsg>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_upstream_msg_dist(
        &self,
        time: &TimeSpan,
    ) -> SdkResult<ListRes<UpstreamMsgDist>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsgdist";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsgDist>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    pub async fn get_upstream_msg_dist_week(
        &self,
        time: &TimeSpan,
    ) -> SdkResult<ListRes<UpstreamMsgDist>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsgdistweek";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsgDist>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }

    pub async fn get_upstream_msg_dist_month(
        &self,
        time: &TimeSpan,
    ) -> SdkResult<ListRes<UpstreamMsgDist>> {
        let base_url = "https://api.weixin.qq.com/datacube/getupstreammsgdistmonth";
        let sdk = self.0;
        let res: CommonResponse<ListRes<UpstreamMsgDist>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_interface_summary(&self, time: &TimeSpan) -> SdkResult<ListRes<ApiAnalysis>> {
        let base_url = "https://api.weixin.qq.com/datacube/getinterfacesummary";
        let sdk = self.0;
        let res: CommonResponse<ListRes<ApiAnalysis>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
    pub async fn get_interface_summary_hour(
        &self,
        time: &TimeSpan,
    ) -> SdkResult<ListRes<ApiAnalysisHour>> {
        let base_url = "https://api.weixin.qq.com/datacube/getinterfacesummaryhour";
        let sdk = self.0;
        let res: CommonResponse<ListRes<ApiAnalysisHour>> = sdk
            .wx_post(base_url)
            .await?
            .json(time)
            .send()
            .await?
            .json()
            .await?;
        res.into()
    }
}
#[cfg(test)]
mod tests {
    // use super::*;

    // #[tokio::test]
    // async fn test_summary() -> SdkResult<()> {
    //     let time = &TimeSpan {
    //         begin_date: "2020-04-02".to_string(),
    //         end_date: "2020-04-02".to_string(),
    //     };
    //     let res = get_user_cumulate(time).await?;
    //     println!("{:#?}", res); // --nocapture
    //     assert_eq!(res.list[0].ref_date.as_str(), time.begin_date.as_str());
    //     Ok(())
    // }

    // #[tokio::test]
    // async fn test_list() -> SdkResult<()> {
    //     let time = &TimeSpan {
    //         begin_date: "2020-04-02".to_string(),
    //         end_date: "2020-04-02".to_string(),
    //     };
    //     let res = get_interface_summary_hour(time).await?;
    //     println!("{:#?}", res); // --nocapture
    //     let res = get_interface_summary(time).await?;
    //     println!("{:#?}", res); // --nocapture
    //     let res = get_upstream_msg_dist(time).await?;
    //     println!("{:#?}", res); // --nocapture
    //     Ok(())
    // }
}
