use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUserEncryptKey {
    /// 用户的openid
    pub openid: String,
    /// 用sessionkey对空字符串签名得到的结果
    pub signature: String,
    /// 签名方法，只支持 hmac_sha256
    pub sig_method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfoList {
    /// 用户最近三次的加密key列表
    pub key_info_list: Vec<KeyInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyInfo {
    /// 加密key
    pub encrypt_key: String,
    /// 加密iv
    pub iv: String,
    /// key的版本号
    pub version: f64,
    /// 剩余有效时间
    pub expire_in: i64,
    /// 创建key的时间戳
    pub create_time: i64,
}

pub struct InternetModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> InternetModule<'a, T> {
    /// 获取用户encryptKey。会获取用户最近3次的key，每个key的存活时间为3600s。
    pub async fn get_user_encrypt_key(&self, data: &QueryUserEncryptKey) -> SdkResult<KeyInfoList> {
        let url = "https://api.weixin.qq.com/wxa/business/getuserencryptkey";
        post_send(self.0, url, data).await
    }
}
