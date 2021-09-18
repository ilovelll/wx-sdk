// use super::{post_send, Part};
// use crate::{SdkResult, error::CommonError, wechat::WxApiRequestBuilder};
// use serde::{Deserialize, Serialize};

// pub struct AdModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

// impl<'a, T: WxApiRequestBuilder> AdModule<'a, T> {
    
//     pub async fn AddUserAction(&self, data: Part) -> SdkResult<()> {
//         let url = "https://api.weixin.qq.com/marketing/user_actions/add";
//         let part = reqwest::multipart::Part::bytes(data.data)
//             .file_name(data.filename)
//             .mime_str(&data.content_type);

//         let form = reqwest::multipart::Form::new().part(data.name, part.unwrap());
//         let builder = self.0.wx_post(url).await?.multipart(form);
//         let res: CommonError = builder.send().await?.json().await?;

//         res.into()
//     }
// }
