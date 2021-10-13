use super::{get_send, post_send};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrder {
    /// 订单来源，0为小程序订单，2为App或H5订单，填2则不发送物流服务通知
    pub add_source: i32,
    /// App或H5的appid，add_source=2时必填，需和开通了物流助手的小程序绑定同一open帐号
    #[serde(default)]
    pub wx_appid: Option<String>,
    /// 订单ID，须保证全局唯一，不超过512字节
    pub order_id: String,
    /// 用户openid，当add_source=2时无需填写（不发送物流服务通知）
    #[serde(default)]
    pub openid: Option<String>,
    /// 快递公司ID，参见getAllDelivery
    pub delivery_id: String,
    /// 快递客户编码或者现付编码
    pub biz_id: String,
    /// 快递备注信息，比如"易碎物品"，不超过1024字节
    #[serde(default)]
    pub custom_remark: Option<String>,
    /// 订单标签id，用于平台型小程序区分平台上的入驻方，tagid须与入驻方账号一一对应，非平台型小程序无需填写该字段
    #[serde(default)]
    pub tagid: Option<i64>,
    /// 发件人信息
    pub sender: UserInfo,
    /// 收件人信息
    pub receiver: UserInfo,
    /// 包裹信息，将传递给快递公司
    pub cargo: Cargo,
    /// 商品信息，会展示到物流服务通知和电子面单中
    pub shop: Shop,
    /// 保价信息
    pub insured: Insured,
    /// 服务类型
    pub service: ServiceType,
    /// Unix 时间戳, 单位秒，顺丰必须传。 预期的上门揽件时间，0表示已事先约定取件时间；否则请传预期揽件时间戳，需大于当前时间，收件员会在预期时间附近上门。例如expect_time为“1557989929”，表示希望收件员将在2019年05月16日14:58:49-15:58:49内上门取货。说明：若选择 了预期揽件时间，请不要自己打单，由上门揽件的时候打印。如果是下顺丰散单，则必传此字段，否则不会有收件员上门揽件。
    #[serde(default)]
    pub expect_time: Option<i64>,
    /// 分单策略，【0：线下网点签约，1：总部签约结算】，不传默认线下网点签约。目前支持圆通。
    #[serde(default)]
    pub take_mode: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    /// 发件人姓名，不超过64字节
    pub name: String,
    /// 发件人座机号码，若不填写则必须填写 mobile，不超过32字节
    #[serde(default)]
    pub tel: Option<String>,
    /// 发件人手机号码，若不填写则必须填写 tel，不超过32字节
    #[serde(default)]
    pub mobile: Option<String>,
    /// 发件人公司名称，不超过64字节
    #[serde(default)]
    pub company: Option<String>,
    /// 发件人邮编，不超过10字节
    #[serde(default)]
    pub post_code: Option<String>,
    /// 发件人国家，不超过64字节
    #[serde(default)]
    pub country: Option<String>,
    /// 发件人省份，比如："广东省"，不超过64字节
    pub province: String,
    /// 发件人市/地区，比如："广州市"，不超过64字节
    pub city: String,
    /// 发件人区/县，比如："海珠区"，不超过64字节
    pub area: String,
    /// 发件人详细地址，比如："XX路XX号XX大厦XX"，不超过512字节
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cargo {
    /// 包裹数量, 需要和detail_list size保持一致
    pub count: i32,
    /// 包裹总重量，单位是千克(kg)
    pub weight: f32,
    /// 包裹长度，单位厘米(cm)
    pub space_x: f32,
    /// 包裹宽度，单位厘米(cm)
    pub space_y: f32,
    /// 包裹高度，单位厘米(cm)
    pub space_z: f32,
    /// 包裹中商品详情列表
    pub detail_list: Vec<Goods>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goods {
    /// 商品名，不超过128字节
    pub name: String,
    /// 商品数量
    pub count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    /// 商家小程序的路径，建议为订单页面
    pub wxa_path: String,
    /// 商品缩略图 url
    pub img_url: String,
    /// 商品名称, 不超过128字节
    pub goods_name: String,
    /// 商品数量
    pub goods_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Insured {
    /// 是否保价，0 表示不保价，1 表示保价
    pub use_insured: i32,
    /// 保价金额，单位是分，比如: 10000 表示 100 元
    pub insured_value: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceType {
    /// 服务类型ID，详见已经支持的快递公司基本信息
    pub service_type: i32,
    /// 服务名称，详见已经支持的快递公司基本信息
    pub service_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrderRes {
    /// 订单ID，下单成功时返回
    pub order_id: String,
    /// 运单ID，下单成功时返回
    pub waybill_id: String,
    /// 运单信息，下单成功时返回
    pub waybill_data: Vec<KeyValue>,
    /// 快递侧错误码，下单失败时返回
    pub delivery_resultcode: i32,
    /// 快递侧错误信息，下单失败时返回
    pub delivery_resultmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyValue {
    /// 运单信息 key
    pub key: String,
    /// 运单信息 value
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOrderList {
    /// 订单列表, 最多不能超过100个
    pub order_list: Vec<QueryOrderItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOrderItem {
    /// 订单ID
    pub order_id: String,
    /// 快递公司ID，参见getAllDelivery
    pub delivery_id: String,
    /// 运单ID
    #[serde(default)]
    pub waybill_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderListRes {
    /// 运单列表
    pub order_list: Vec<OrderRes>,
    /// 运单状态, 0正常，1取消
    pub order_status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderRes {
    /// 错误码
    pub errcode: i32,
    /// 错误信息
    pub errmsg: String,
    /// 订单ID
    pub order_id: String,
    /// 快递公司ID，参见getAllDelivery
    pub delivery_id: String,
    /// 运单ID
    pub waybill_id: String,
    /// 运单 html 的 BASE64 结果
    pub print_html: String,
    /// 运单信息
    pub waybill_data: Vec<KeyValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindAccount {
    /// bind表示绑定，unbind表示解除绑定
    #[serde(rename = "type")]
    pub type_: String,
    /// 快递公司客户编码
    pub biz_id: String,
    /// 快递公司ID
    pub delivery_id: String,
    /// 快递公司客户密码, ems，顺丰，京东非必填
    #[serde(default)]
    pub password: Option<String>,
    /// 备注内容（提交EMS审核需要）
    #[serde(default)]
    pub remark_content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    /// 订单 ID，需保证全局唯一
    pub order_id: String,
    /// 用户openid，当add_source=2时无需填写（不发送物流服务通知）
    #[serde(default)]
    pub openid: Option<String>,
    /// 快递公司ID，参见getAllDelivery
    pub delivery_id: String,
    /// 运单ID
    pub waybill_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DResultCode {
    /// 运力返回的错误码
    pub delivery_resultcode: i32,
    /// 运力返回的错误信息
    pub delivery_resultmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountList {
    /// 账号数量
    pub count: i64,
    /// 账号列表
    pub list: Vec<AccountInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfo {
    /// 快递公司客户编码
    pub biz_id: String,
    /// 快递公司ID
    pub delivery_id: String,
    /// 账号绑定时间
    pub create_time: i64,
    /// 账号更新时间
    pub update_time: i64,
    /// 绑定状态
    /// - `0`, 绑定成功
    /// - `1`, 审核中
    /// - `2`, 绑定失败
    /// - `3`, 已解绑
    pub status_code: i32,
    /// 账号别名
    pub alias: String,
    /// 账号绑定失败的错误信息（EMS审核结果）
    pub remark_wrong_msg: String,
    /// 账号绑定时的备注内容（提交EMS审核需要）
    pub remark_content: String,
    /// 电子面单余额
    pub quota_num: f64,
    /// 电子面单余额更新时间
    pub quota_update_time: i64,
    /// 该绑定帐号支持的服务类型
    #[serde(default)]
    pub service_type: Option<Vec<ServiceType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryList {
    /// 快递公司数量
    pub count: i64,
    /// 快递公司信息列表
    pub data: Vec<DeliveryInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryInfo {
    /// 快递公司 ID
    pub delivery_id: String,
    /// 快递公司名称
    pub delivery_name: String,
    /// 是否支持散单, 1表示支持
    #[serde(default)]
    pub can_use_cash: Option<i32>,
    /// 是否支持查询面单余额, 1表示支持
    #[serde(default)]
    pub can_get_quota: Option<i32>,
    /// 散单对应的bizid，当can_use_cash=1时有效
    #[serde(default)]
    pub cash_biz_id: Option<String>,
    /// 支持的服务类型
    #[serde(default)]
    pub service_type: Option<Vec<ServiceType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOrder {
    /// 订单 ID，需保证全局唯一
    pub order_id: String,
    /// 用户openid，当add_source=2时无需填写（不发送物流服务通知）
    #[serde(default)]
    pub openid: Option<String>,
    /// 快递公司ID，参见getAllDelivery, 必须和waybill_id对应
    pub delivery_id: String,
    /// 运单ID
    #[serde(default)]
    pub waybill_id: Option<String>,
    /// 获取打印面单类型（`1`：一联单，`0`：二联单），默认获取二联单
    #[serde(default)]
    pub print_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    /// 运单 html 的 BASE64 结果
    pub print_html: String,
    /// 运单信息
    pub waybill_data: Vec<KeyValue>,
    /// 快递公司ID
    pub delivery_id: String,
    /// 订单ID
    pub order_id: String,
    /// 运单号
    pub waybill_id: String,
    /// 运单状态, 0正常，1取消
    pub order_status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryPath {
    /// 订单 ID，需保证全局唯一
    pub order_id: String,
    /// 用户openid，当add_source=2时无需填写（不发送物流服务通知）
    #[serde(default)]
    pub openid: Option<String>,
    /// 快递公司ID，参见getAllDelivery
    pub delivery_id: String,
    /// 运单ID
    pub waybill_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathList {
    /// 用户openid
    pub openid: String,
    /// 快递公司 ID
    pub delivery_id: String,
    /// 运单 ID
    pub waybill_id: String,
    /// 轨迹节点数量
    pub path_item_num: i64,
    /// 轨迹节点列表
    pub path_item_list: Vec<PathItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PathItem {
    /// 轨迹节点 Unix 时间戳
    pub action_time: i64,
    /// 轨迹节点类型
    /// - `100001`, 揽件阶段-揽件成功
    /// - `100002`, 揽件阶段-揽件失败
    /// - `100003`, 揽件阶段-分配业务员
    /// - `200001`, 运输阶段-更新运输轨迹
    /// - `300002`, 派送阶段-开始派送
    /// - `300003`, 派送阶段-签收成功
    /// - `300004`, 派送阶段-签收失败
    /// - `400001`, 异常阶段-订单取消
    /// - `400002`, 异常阶段-订单滞留
    pub action_type: i32,
    /// 轨迹节点详情
    pub action_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinterList {
    pub count: i64,
    pub openid: Vec<String>,
    pub tagid_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryQuota {
    /// 快递公司ID，参见getAllDelivery
    pub delivery_id: String,
    /// 快递公司客户编码
    pub biz_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuotaNum {
    /// 电子面单余额
    pub quota_num: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestUpdateOrder {
    /// 商户id,需填test_biz_id
    pub biz_id: String,
    /// 订单号
    pub order_id: String,
    /// 快递公司id,需填TEST
    pub delivery_id: String,
    /// 运单号
    pub waybill_id: String,
    /// 轨迹变化 Unix 时间戳
    pub action_time: i64,
    /// 轨迹变化类型
    /// - `100001`, 揽件阶段-揽件成功
    /// - `100002`, 揽件阶段-揽件失败
    /// - `100003`, 揽件阶段-分配业务员
    /// - `200001`, 运输阶段-更新运输轨迹
    /// - `300002`, 派送阶段-开始派送
    /// - `300003`, 派送阶段-签收成功
    /// - `300004`, 派送阶段-签收失败
    /// - `400001`, 异常阶段-订单取消
    /// - `400002`, 异常阶段-订单滞留
    pub action_type: i32,
    /// 轨迹变化具体信息说明,使用UTF-8编码
    pub action_msg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePrinter {
    /// 打印员 openid
    pub openid: String,
    /// 更新类型
    /// - `"bind"`，绑定
    /// - `"unbind"`，解除绑定
    pub update_type: String,
    /// 用于平台型小程序设置入驻方的打印员面单打印权限，同一打印员最多支持10个tagid，
    /// 使用半角逗号分隔，中间不加空格，如填写123,456，表示该打印员可以拉取到tagid为123和456的下的单，
    /// 非平台型小程序无需填写该字段
    #[serde(default)]
    pub tagid_list: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryContact {
    /// 商户侧下单事件中推送的 Token 字段
    pub token: String,
    /// 运单 ID
    pub waybill_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Contact {
    /// 运单 ID
    pub waybill_id: String,
    /// 发件人信息
    pub sender: ContactInfo,
    /// 收件人信息
    pub receiver: ContactInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactInfo {
    /// 地址，已经将省市区信息合并
    pub address: String,
    /// 用户姓名
    pub name: String,
    /// 座机号码
    pub tel: String,
    /// 手机号码
    pub mobile: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewTemplate {
    /// 运单 ID
    pub waybill_id: String,
    /// 面单 HTML 模板内容（需经 Base64 编码）
    pub waybill_template: String,
    /// 面单数据。详情参考下单事件返回值中的 WaybillData
    pub waybill_data: String,
    /// 商户下单数据，格式是商户侧下单 API 中的请求体
    pub custom: AddOrder,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Custom {
    pub order_id: String,
    pub openid: String,
    pub delivery_id: String,
    pub biz_id: String,
    #[serde(default)]
    pub custom_remark: String,
    pub sender: UserInfo,
    pub receiver: UserInfo,
    pub shop: Shop,
    pub cargo: Cargo,
    pub insured: Insured,
    pub service: ServiceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewTemplateRes {
    /// 运单 ID
    pub waybill_id: String,
    /// 渲染后的面单 HTML 文件（已经过 Base64 编码）
    pub rendered_waybill_template: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateBusiness {
    /// 商户的小程序AppID，即审核商户事件中的 ShopAppID
    pub shop_app_id: String,
    /// 商户账户
    pub biz_id: String,
    /// 审核结果，0 表示审核通过，其他表示审核失败
    pub result_code: i32,
    /// 审核错误原因，仅 result_code 不等于 0 时需要设置
    #[serde(default)]
    pub result_msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePath {
    /// 商户侧下单事件中推送的 Token 字段
    pub token: String,
    /// 运单 ID
    pub waybill_id: String,
    /// 轨迹变化 Unix 时间戳
    pub action_time: i64,
    /// 轨迹变化类型
    /// - `100001`, 揽件阶段-揽件成功
    /// - `100002`, 揽件阶段-揽件失败
    /// - `100003`, 揽件阶段-分配业务员
    /// - `200001`, 运输阶段-更新运输轨迹
    /// - `300002`, 派送阶段-开始派送
    /// - `300003`, 派送阶段-签收成功
    /// - `300004`, 派送阶段-签收失败
    /// - `400001`, 异常阶段-订单取消
    /// - `400002`, 异常阶段-订单滞留
    pub action_type: i32,
    /// 轨迹变化具体信息说明，展示在快递轨迹详情页中。若有手机号码，则直接写11位手机号码。使用UTF-8编码。
    pub action_msg: String,
}

pub struct LogisticsModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> LogisticsModule<'a, T> {
    /// 生成运单
    pub async fn add_order(&self, data: &AddOrder) -> SdkResult<AddOrderRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/order/add";
        post_send(self.0, url, data).await
    }

    /// 批量获取运单数据
    pub async fn batch_get_order(&self, data: &QueryOrderList) -> SdkResult<OrderListRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/order/batchget";
        post_send(self.0, url, data).await
    }

    /// 绑定、解绑物流账号
    pub async fn bind_account(&self, data: &BindAccount) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/account/bind";
        post_send(self.0, url, data).await
    }

    /// 取消运单
    pub async fn cancel_order(&self, data: &CancelOrder) -> SdkResult<DResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/order/cancel";
        post_send(self.0, url, data).await
    }

    /// 获取所有绑定的物流账号
    pub async fn get_all_account(&self) -> SdkResult<AccountList> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/account/getall";
        get_send(self.0, url, &()).await
    }

    /// 获取支持的快递公司列表
    pub async fn get_all_delivery(&self) -> SdkResult<DeliveryList> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/delivery/getall";
        get_send(self.0, url, &()).await
    }

    /// 获取运单数据
    pub async fn get_order(&self, data: &QueryOrder) -> SdkResult<Order> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/order/get";
        post_send(self.0, url, data).await
    }

    /// 查询运单轨迹
    pub async fn get_path(&self, data: &QueryPath) -> SdkResult<PathList> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/path/get";
        post_send(self.0, url, data).await
    }

    /// 获取打印员。若需要使用微信打单 PC 软件，才需要调用。
    pub async fn get_printer(&self) -> SdkResult<PrinterList> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/printer/getall";
        get_send(self.0, url, &()).await
    }

    /// 获取电子面单余额。仅在使用加盟类快递公司时，才可以调用。
    pub async fn get_quota(&self, data: &QueryQuota) -> SdkResult<QuotaNum> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/quota/get";
        post_send(self.0, url, data).await
    }

    /// 模拟快递公司更新订单状态, 该接口只能用户测试
    pub async fn test_update_order(&self, data: &TestUpdateOrder) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/test_update_order";
        post_send(self.0, url, data).await
    }

    /// 配置面单打印员，可以设置多个，若需要使用微信打单 PC 软件，才需要调用。
    pub async fn update_printer(&self, data: &UpdatePrinter) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/business/printer/update";
        post_send(self.0, url, data).await
    }

    /// 获取面单联系人信息
    pub async fn get_contact(&self, data: &QueryContact) -> SdkResult<Contact> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/delivery/contact/get";
        post_send(self.0, url, data).await
    }

    /// 预览面单模板。用于调试面单模板使用。
    pub async fn preview_template(&self, data: &PreviewTemplate) -> SdkResult<PreviewTemplateRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/delivery/template/preview";
        post_send(self.0, url, data).await
    }

    /// 更新商户审核结果
    pub async fn update_business(&self, data: &UpdateBusiness) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/delivery/service/business/update";
        post_send(self.0, url, data).await
    }

    /// 更新运单轨迹
    pub async fn update_path(&self, data: &UpdatePath) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/delivery/path/update";
        post_send(self.0, url, data).await
    }
}
