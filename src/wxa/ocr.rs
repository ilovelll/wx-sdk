use crate::wxa::img::{post_img_data, Pos, WH};
use crate::{error::SdkError::InvalidParams, wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

pub use crate::wxa::img::ImgData;

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleLicense {
    /// 车辆类型
    pub vehicle_type: String,
    /// 所有人
    pub owner: String,
    /// 住址
    pub addr: String,
    /// 使用性质
    pub use_character: String,
    /// 品牌型号
    pub model: String,
    /// 车辆识别代
    pub vin: String,
    /// 发动机号码
    pub engine_num: String,
    /// 注册日期
    pub register_date: String,
    /// 发证日期
    pub issue_date: String,
    /// 车牌号码
    pub plate_num_b: String,
    /// 号牌
    pub record: String,
    /// 核定载人数
    pub passengers_num: String,
    /// 总质量
    pub total_quality: String,
    /// 整备质量
    pub totalprepare_quality_quality: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BusinessLicense {
    /// 注册号
    pub reg_num: String,
    /// 编号
    pub serial: String,
    /// 法定代表人姓名
    pub legal_representative: String,
    /// 企业名称
    pub enterprise_name: String,
    /// 组成形式
    pub type_of_organization: String,
    /// 经营场所/企业住所
    pub address: String,
    /// 公司类型
    pub type_of_enterprise: String,
    /// 经营范围
    pub business_scope: String,
    /// 注册资本
    pub registered_capital: String,
    /// 实收资本
    pub paid_in_capital: String,
    /// 营业期限
    pub valid_period: String,
    /// 注册日期/成立日期
    pub registered_date: String,
    /// 营业执照位置
    pub cert_position: Pos,
    /// 图片大小
    pub img_size: WH,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DriverLicense {
    /// 证号
    pub id_num: String,
    /// 姓名
    pub name: String,
    /// 性别
    pub sex: String,
    /// 地址
    pub address: String,
    /// 出生日期
    pub birth_date: String,
    /// 初次领证日期
    pub issue_date: String,
    /// 准驾车型
    pub car_class: String,
    /// 有效期限起始日
    pub valid_from: String,
    /// 有效期限终止日
    pub valid_to: String,
    /// 印章文构
    pub official_seal: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Idcard {
    Front(IdcardFront),
    Back(IdcardBack),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdcardFront {
    pub name: String,
    pub id: String,
    pub addr: String,
    pub gender: String,
    pub nationality: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdcardBack {
    /// 有效期
    pub valid_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintedText {
    /// 识别结果
    pub items: Vec<PrintedTextItem>,
    /// 图片大小
    pub img_size: WH,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintedTextItem {
    pub text: String,
    pub pos: Pos,
}

pub struct OcrModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> OcrModule<'a, T> {
    /// 本接口提供基于小程序的银行卡 OCR 识别
    pub async fn bankcard(&self, data: ImgData) -> SdkResult<Id> {
        let url = "https://api.weixin.qq.com/cv/ocr/bankcard";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的营业执照 OCR 识别
    pub async fn business_license(&self, data: ImgData) -> SdkResult<BusinessLicense> {
        let url = "https://api.weixin.qq.com/cv/ocr/bizlicense";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的驾驶证 OCR 识别
    pub async fn driver_license(&self, data: ImgData) -> SdkResult<DriverLicense> {
        let url = "https://api.weixin.qq.com/cv/ocr/drivinglicense";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的身份证 OCR 识别
    pub async fn idcard(&self, data: ImgData) -> SdkResult<Idcard> {
        let url = "https://api.weixin.qq.com/cv/ocr/idcard";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的通用印刷体 OCR 识别
    pub async fn printed_text(&self, data: ImgData) -> SdkResult<PrintedText> {
        let url = "https://api.weixin.qq.com/cv/ocr/comm";
        post_img_data(self.0, url, data).await
    }

    /// 本接口提供基于小程序的行驶证 OCR 识别
    pub async fn vehicle_license(&self, data: ImgData, type_: &str) -> SdkResult<VehicleLicense> {
        let url = match type_ {
            "photo" => "https://api.weixin.qq.com/cv/ocr/driving?type=photo",
            "scan" => "https://api.weixin.qq.com/cv/ocr/driving?type=scan",
            _ => {
                return Err(InvalidParams(format!(
                    "Unknown `type` parameter: {}",
                    type_
                )))
            }
        };
        post_img_data(self.0, url, data).await
    }
}

// #[test]
// fn test_enum_tag() {
//     let input = r#"{
//   "errcode": "0",
//   "errmsg": "ok",
//   "type": "Front",
//   "name": "张三",
//   "id": "123456789012345678",
//   "addr": "广东省广州市",
//   "gender": "男",
//   "nationality": "汉"
// }"#;
//     let data = &serde_json::from_str::<crate::error::CommonResponse<Idcard>>(input).unwrap();
//     println!("{:?}", data);
//     println!("{:?}", serde_json::to_string(data));
// }
