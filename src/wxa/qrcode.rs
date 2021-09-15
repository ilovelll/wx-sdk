use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQRCode {
    /// 扫码进入的小程序页面路径，最大长度 128 字节，不能为空；对于小游戏，可以只传入 query 部分，来实现传参效果 <br/>
    /// 如：传入 "?foo=bar"，即可在 wx.getLaunchOptionsSync 接口中的 query 参数获取到 {foo:"bar"}。
    pub path: String,
    /// 二维码的宽度，单位 px。最小 280px，最大 1280px <br/>
    /// 默认值： `430`
    #[serde(default)]
    pub width: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryQrc {
    /// 扫码进入的小程序页面路径，最大长度 128 字节，不能为空；对于小游戏，可以只传入 query 部分，来实现传参效果 <br/>
    /// 如：传入 "?foo=bar"，即可在 wx.getLaunchOptionsSync 接口中的 query 参数获取到 {foo:"bar"}。
    pub path: String,
    /// 二维码的宽度，单位 px。最小 280px，最大 1280px <br/>
    /// 默认值： `430`
    #[serde(default)]
    pub width: Option<i32>,
    /// 自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调 <br/>
    /// 默认值： `false`
    #[serde(default)]
    pub auto_color: Option<bool>,
    /// auto_color 为 false 时生效，使用 rgb 设置颜色 <br/>
    // 例如 {"r":"xxx","g":"xxx","b":"xxx"} 十进制表示
    #[serde(default)]
    pub line_color: Option<Color>,
    /// 是否需要透明底色，为 true 时，生成透明底色的小程序码 <br/>
    /// 默认值： `false`
    #[serde(default)]
    pub is_hyaline: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryQrcUnlimited {
    /// 最大32个可见字符，只支持数字，大小写英文以及部分特殊字符：!#$&'()*+,/:;=?@-._~， <br/>
    /// 其它字符请自行编码为合法字符（因不支持%，中文无法使用 urlencode 处理，请使用其他编码方式）
    pub scene: String,
    /// 必须是已经发布的小程序存在的页面（否则报错），例如 pages/index/index, 根路径前不要填加 /,
    /// 不能携带参数（参数请放在scene字段里），如果不填写这个字段，默认跳主页面
    #[serde(default)]
    pub page: Option<String>,
    /// 二维码的宽度，单位 px，最小 280px，最大 1280px <br/>
    /// 默认值： `430`
    #[serde(default)]
    pub width: Option<i64>,
    /// 自动配置线条颜色，如果颜色依然是黑色，则说明不建议配置主色调 <br/>
    /// 默认值： `false`
    #[serde(default)]
    pub auto_color: Option<bool>,
    /// auto_color 为 false 时生效，使用 rgb 设置颜色
    #[serde(default)]
    pub line_color: Option<Color>,
    /// 是否需要透明底色，为 true 时，生成透明底色的小程序 <br/>
    /// 默认值： `false`
    #[serde(default)]
    pub is_hyaline: Option<bool>,
}

pub struct QrcodeModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> QrcodeModule<'a, T> {
    pub async fn create(&self, data: &CreateQRCode) -> SdkResult<Vec<u8>> {
        let url = "https://api.weixin.qq.com/cgi-bin/wxaapp/createwxaqrcode";
        let builder = self.0.wx_post(url).await?.json(data);
        let bytes = builder.send().await?.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// 获取小程序码，适用于需要的码数量较少的业务场景。通过该接口生成的小程序码，永久有效，有数量限制，详见获取二维码。
    pub async fn get(&self, data: &QueryQrc) -> SdkResult<Vec<u8>> {
        let url = "https://api.weixin.qq.com/wxa/getwxacode";
        let builder = self.0.wx_post(url).await?.json(data);
        let bytes = builder.send().await?.bytes().await?;
        Ok(bytes.to_vec())
    }

    /// 获取小程序码，适用于需要的码数量极多的业务场景。通过该接口生成的小程序码，永久有效，数量暂无限制。
    /// 更多用法详见获取二维码。
    pub async fn get_unlimited(&self, data: &QueryQrcUnlimited) -> SdkResult<Vec<u8>> {
        let url = "https://api.weixin.qq.com/wxa/getwxacodeunlimit";
        let builder = self.0.wx_post(url).await?.json(data);
        let bytes = builder.send().await?.bytes().await?;
        Ok(bytes.to_vec())
    }
}
