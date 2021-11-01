use super::post_send;
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryAbnormal {
    /// 商家id，由配送公司分配的appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 商家门店编号，在配送公司登记，闪送必填，值为店铺id
    pub shop_no: String,
    /// 用配送公司提供的appSecret加密的校验串说明
    pub delivery_sign: String,
    /// 配送单id
    pub waybill_id: String,
    /// 备注
    #[serde(default)]
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultCode {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrder {
    /// 预下单接口返回的参数，配送公司可保证在一段时间内运费不变
    #[serde(default)]
    pub delivery_token: Option<String>,
    /// 商家id，由配送公司分配的appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成, 不超过128字节
    pub shop_order_id: String,
    /// 商家门店编号，在配送公司登记，如果只有一个门店，美团闪送必填, 值为店铺id
    pub shop_no: String,
    /// 用配送公司提供的appSecret加密的校验串说明
    pub delivery_sign: String,
    /// 配送公司ID
    pub delivery_id: String,
    /// 下单用户的openid
    pub openid: String,
    /// 发件人信息，顺丰同城急送必须填写，美团配送、达达、闪送，若传了`shop_no` 的值可不填该字段
    #[serde(default)]
    pub sender: Option<Address>,
    /// 收件人信息
    pub receiver: Address,
    /// 货物信息
    pub cargo: Cargo,
    /// 订单信息
    pub order_info: AddOrderInfo,
    /// 商品信息，会展示到物流通知消息中
    pub shop: Shop,
    /// 子商户id，区分小程序内部多个子商户
    #[serde(default)]
    pub sub_biz_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    /// 姓名，最长不超过256个字符
    pub name: String,
    /// 城市名称，如广州市
    pub city: String,
    /// 地址(街道、小区、大厦等，用于定位)
    pub address: String,
    /// 地址详情(楼号、单元号、层号)
    pub address_detail: String,
    /// 电话/手机号，最长不超过64个字符
    pub phone: String,
    /// 经度（火星坐标或百度坐标，和 `coordinate_type` 字段配合使用，确到小数点后6位
    pub lng: f64,
    /// 纬度（火星坐标或百度坐标，和 `coordinate_type` 字段配合使用，精确到小数点后6位）
    pub lat: f64,
    /// 坐标类型，0：火星坐标（高德，腾讯地图均采用火星坐标） 1：百度坐标 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub coordinate_type: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cargo {
    /// 货物价格，单位为元，精确到小数点后两位（如果小数点后位数多于两位，则四舍五入保留两位小数），范围为(0-5000]
    pub goods_value: f32,
    /// 货物高度，单位为cm，精确到小数点后两位（如果小数点后位数多于两位，则四舍五入保留两位小数），范围为(0-45]
    #[serde(default)]
    pub goods_height: Option<f32>,
    /// 货物长度，单位为cm，精确到小数点后两位（如果小数点后位数多于两位，则四舍五入保留两位小数），范围为(0-65]
    #[serde(default)]
    pub goods_length: Option<f32>,
    /// 货物宽度，单位为cm，精确到小数点后两位（如果小数点后位数多于两位，则四舍五入保留两位小数），范围为(0-50]
    #[serde(default)]
    pub goods_width: Option<f32>,
    /// 货物重量，单位为kg，精确到小数点后两位（如果小数点后位数多于两位，则四舍五入保留两位小数），范围为(0-50]
    pub goods_weight: f32,
    /// 货物详情，最长不超过10240个字符
    #[serde(default)]
    pub goods_detail: Option<GoodsList>,
    /// 货物取货信息，用于骑手到店取货，最长不超过100个字符
    #[serde(default)]
    pub goods_pickup_info: Option<String>,
    /// 货物交付信息，最长不超过100个字符
    #[serde(default)]
    pub goods_delivery_info: Option<String>,
    /// 品类一级类目, 详见品类表
    pub cargo_first_class: String,
    /// 品类二级类目
    pub cargo_second_class: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsList {
    /// 货物列表
    pub goods: Vec<Goods>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Goods {
    /// 货物数量
    pub good_count: i32,
    /// 货品名称
    pub good_name: String,
    /// 货品单价，精确到小数点后两位（如果小数点后位数多于两位，则四舍五入保留两位小数）
    #[serde(default)]
    pub good_price: Option<f32>,
    /// 货品单位，最长不超过20个字符
    #[serde(default)]
    pub good_unit: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddOrderInfo {
    /// 配送服务代码 不同配送公司自定义, 顺丰和达达不填
    #[serde(default)]
    pub delivery_service_code: Option<String>,
    /// 订单类型, 0: 即时单 1 预约单， <br/>
    /// 如预约单，需要设置 `expected_delivery_time` 或 `expected_finish_time` 或 `expected_pick_time` <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub order_type: Option<i32>,
    /// 期望派单时间(达达支持，表示达达系统调度时间, 到那个时间才会有状态更新的回调通知)，unix-timestamp, 比如1586342180 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub expected_delivery_time: Option<i64>,
    /// 期望送达时间(美团、顺丰同城急送支持），unix-timestamp, 比如1586342180 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub expected_finish_time: Option<i64>,
    /// 期望取件时间，unix-timestamp，比如1586342180  <br/>
    /// （闪送、顺丰同城急送支持，闪送需要设置两个小时后的时间，
    /// 顺丰同城急送只需传expected_finish_time或expected_pick_time其中之一即可，
    /// 同时都传则以expected_finish_time为准）<br/>
    /// 默认值：`0`
    #[serde(default)]
    pub expected_pick_time: Option<i64>,
    /// 门店订单流水号，建议提供，方便骑手门店取货，最长不超过32个字符
    #[serde(default)]
    pub poi_seq: Option<String>,
    /// 备注，最长不超过200个字符
    #[serde(default)]
    pub note: Option<String>,
    /// 用户下单付款时间, 顺丰必填, 比如1555220757
    #[serde(default)]
    pub order_time: Option<i64>,
    /// 是否保价，0，非保价，1.保价 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub is_insured: Option<i32>,
    /// 保价金额，单位为元，精确到分
    #[serde(default)]
    pub declared_value: Option<f32>,
    /// 小费，单位为元, 下单一般不加小费
    #[serde(default)]
    pub tips: Option<f32>,
    /// 是否选择直拿直送 <br/>
    /// 0：不需要；1：需要。 <br/>
    /// 选择直拿直送后，同一时间骑手只能配送此订单至完成，配送费用也相应高一些， <br/>
    /// 闪送必须选1，达达可选0或1，其余配送公司不支持直拿直送 <br/>
    #[serde(default)]
    pub is_direct_delivery: Option<i32>,
    /// 骑手应付金额，单位为元，精确到分
    #[serde(default)]
    pub cash_on_delivery: Option<f32>,
    /// 骑手应收金额，单位为元，精确到分
    #[serde(default)]
    pub cash_on_pickup: Option<f32>,
    /// 物流流向，1：从门店取件送至用户；2：从用户取件送至门店
    #[serde(default)]
    pub rider_pick_method: Option<i32>,
    /// 收货码（0：不需要；1：需要。收货码的作用是：骑手必须输入收货码才能完成订单妥投）
    #[serde(default)]
    pub is_finish_code_needed: Option<i32>,
    /// 取货码（0：不需要；1：需要。取货码的作用是：骑手必须输入取货码才能从商家取货）
    #[serde(default)]
    pub is_pickup_code_needed: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shop {
    /// 商家小程序的路径，建议为订单页面
    pub wxa_path: String,
    /// 商品缩略图 url
    pub img_url: String,
    /// 商品名称
    pub goods_name: String,
    /// 商品数量
    pub goods_count: i32,
    /// 若结算方式为：第三方向配送公司统一结算，商户后续和第三方结算，则该参数必填； <br/>
    /// 在该结算模式下，第三方用自己的开发小程序替授权商户发起下单，并将授权小程序的appid给平台， <br/>
    /// 后续配送通知中可回流授权商户小程序。
    #[serde(default)]
    pub wxa_appid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddOrderRes {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
    /// 实际运费(单位：元)，运费减去优惠券费用
    pub fee: f32,
    /// 运费(单位：元)
    pub deliverfee: f32,
    /// 优惠券费用(单位：元)
    pub couponfee: f32,
    /// 小费(单位：元)
    pub tips: f32,
    /// 保价费(单位：元)
    pub insurancefee: f32,
    /// 配送距离(单位：米)
    pub distance: f32,
    /// 配送单号
    pub waybill_id: String,
    /// 配送状态
    pub order_status: i32,
    /// 收货码
    pub finish_code: i32,
    /// 取货码
    pub pickup_code: i32,
    /// 预计骑手接单时间，单位秒，比如5分钟，就填300, 无法预计填0
    pub dispatch_duration: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tip {
    /// 商家id， 由配送公司分配的 appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 商家门店编号，在配送公司登记，如果只有一个门店，闪送 shop_no 必填，值为店铺id
    pub shop_no: String,
    /// 用配送公司提供的 appSecret 加密的校验串说明
    pub delivery_sign: String,
    /// 配送单id
    pub waybill_id: String,
    /// 下单用户的 openid
    pub openid: String,
    /// 小费金额(单位：元) 各家配送公司最大值不同
    pub tips: f32,
    /// 备注
    pub remark: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrder {
    /// 商家id， 由配送公司分配的appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 商家门店编号，如果只有一个门店，闪送shop_no必填，值为店铺id
    pub shop_no: String,
    /// 用配送公司提供的appSecret加密的校验串说明
    pub delivery_sign: String,
    /// 快递公司ID
    pub delivery_id: String,
    /// 配送单id
    #[serde(default)]
    pub waybill_id: Option<String>,
    /// 取消原因Id
    /// - `1`, 暂时不需要邮寄
    /// - `2`, 价格不合适
    /// - `3`, 订单信息有误，重新下单
    /// - `4`, 骑手取货不及时
    /// - `5`, 骑手配送不及时
    /// - `6`, 其他原因( 如果选择6，需要填写取消原因，否则不需要填写 )
    pub cancel_reason_id: i32,
    /// 取消原因
    #[serde(default)]
    pub cancel_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelOrderRes {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
    /// 扣除的违约金(单位：元)，精确到分
    pub deduct_fee: i32,
    /// 说明
    pub desc: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImmeDeliveryList {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
    /// 配送公司列表
    pub list: Vec<DeliveryInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryInfo {
    /// 配送公司Id
    pub delivery_id: String,
    /// 配送公司名称
    pub delivery_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindAccountRes {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
    /// 绑定的商家签约账号列表
    pub shop_list: Vec<BindShop>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BindShop {
    /// 配送公司Id
    pub delivery_id: String,
    /// 商家id
    pub shopid: String,
    /// 审核状态
    /// - `0`, 审核通过
    /// - `1`, 审核中
    /// - `2`, 审核不通过
    pub audit_result: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryOrder {
    /// 商家id， 由配送公司分配的appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 商家门店编号， 在配送公司登记，如果只有一个门店，可以不填
    pub shop_no: String,
    /// 用配送公司提供的appSecret加密的校验串说明
    pub delivery_sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
    /// 配送状态，枚举值
    pub order_status: i32,
    /// 配送单号
    pub waybill_id: String,
    /// 骑手姓名
    pub rider_name: String,
    /// 骑手电话
    pub rider_phone: String,
    /// 骑手位置经度, 配送中时返回
    pub rider_lng: f64,
    /// 骑手位置纬度, 配送中时返回
    pub rider_lat: f64,
    /// 预计还剩多久送达时间, 配送中时返回，单位秒， 已取货配送中需返回，比如5分钟后送达，填300
    pub reach_time: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MockUpdateOrder {
    /// 商家id, 必须是 "test_shop_id"
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 状态变更时间点，Unix秒级时间戳
    pub action_time: i64,
    /// 配送状态，枚举值
    pub order_status: i32,
    /// 附加信息
    #[serde(default)]
    pub action_msg: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreAddOrder {
    /// 商家id， 由配送公司分配的appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成, 不超过128字节
    pub shop_order_id: String,
    /// 商家门店编号，在配送公司登记，美团、闪送必填
    pub shop_no: String,
    /// 用配送公司提供的appSecret加密的校验串说明
    pub delivery_sign: String,
    /// 配送公司ID
    pub delivery_id: String,
    /// 下单用户的openid
    pub openid: String,
    /// 发件人信息，闪送、顺丰同城急送必须填写，美团配送、达达，若传了shop_no的值可不填该字段
    pub sender: Option<Address>,
    /// 收件人信息
    pub receiver: Address,
    /// 货物信息
    pub cargo: Cargo,
    /// 订单信息
    pub order_info: AddOrderInfo,
    /// 商品信息，会展示到物流通知消息中
    pub shop: Shop,
    /// 子商户id，区分小程序内部多个子商户
    #[serde(default)]
    pub sub_biz_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreAddOrderRes {
    /// 运力返回的错误码
    pub resultcode: i32,
    /// 运力返回的错误描述
    pub resultmsg: String,
    /// 实际运费(单位：元)，运费减去优惠券费用
    pub fee: i32,
    /// 运费(单位：元)
    pub deliverfee: i32,
    /// 优惠券费用(单位：元)
    pub couponfee: i32,
    /// 小费(单位：元)
    pub tips: i32,
    /// 保价费(单位：元)
    pub insurancefee: i32,
    /// 配送距离(单位：米)
    pub distance: i32,
    /// 预计骑手接单时间，单位秒，比如5分钟，就填300, 无法预计填0
    pub dispatch_duration: i32,
    /// 配送公司可以返回此字段，当用户下单时候带上这个字段，保证在一段时间内运费不变
    pub delivery_token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealMockUpdateOrder {
    /// 商家id
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 状态变更时间点，Unix秒级时间戳
    pub action_time: i32,
    /// 配送状态，枚举值
    pub order_status: i32,
    /// 附加信息
    #[serde(default)]
    pub action_msg: Option<String>,
    /// 用配送公司提供的appSecret加密的校验串说明
    pub delivery_sign: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateOrder {
    /// 下单事件中推送的wx_token字段
    pub wx_token: String,
    /// 商家id， 由配送公司分配，可以是dev_id或者appkey
    pub shopid: String,
    /// 唯一标识订单的 ID，由商户生成
    pub shop_order_id: String,
    /// 商家门店编号， 在配送公司侧登记
    #[serde(default)]
    pub shop_no: Option<String>,
    /// 配送单id
    pub waybill_id: String,
    /// 状态变更时间点，Unix秒级时间戳
    pub action_time: i64,
    /// 订单状态，枚举值
    pub order_status: i32,
    /// 附加信息
    #[serde(default)]
    pub action_msg: Option<String>,
    /// 配送公司小程序跳转路径，用于用户收到消息会间接跳转到这个页面
    pub wxa_path: String,
    /// 骑手信息, 骑手接单时需返回
    #[serde(default)]
    pub agent: Option<Agent>,
    /// 预计送达时间戳， 骑手接单时需返回
    #[serde(default)]
    pub expected_delivery_time: Option<i64>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Agent {
    /// 骑手姓名
    pub name: String,
    /// 骑手电话
    pub phone: String,
    /// 电话是否加密 <br/>
    /// 默认值：`0`
    #[serde(default)]
    pub is_phone_encrypted: Option<i32>,
}

pub struct ImmediateDeliveryModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> ImmediateDeliveryModule<'a, T> {
    /// 异常件退回商家商家确认收货接口
    pub async fn abnormal_confirm(&self, data: &QueryAbnormal) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/confirm_return";
        post_send(self.0, url, data).await
    }

    /// 下配送单接口
    pub async fn add_order(&self, data: &AddOrder) -> SdkResult<AddOrderRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/add";
        post_send(self.0, url, data).await
    }

    /// 可以对待接单状态的订单增加小费。 <br/>
    /// 需要注意：订单的小费，以最新一次加小费动作的金额为准，故下一次增加小费额必须大于上一次小费额
    pub async fn add_tip(&self, data: &Tip) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/addtips";
        post_send(self.0, url, data).await
    }

    /// 第三方代商户发起绑定配送公司帐号的请求
    pub async fn bind_account(&self, delivery_id: &str) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/shop/add";
        let data = &serde_json::json!({ "delivery_id": delivery_id });
        post_send(self.0, url, data).await
    }

    /// 取消配送单接口
    pub async fn cancel_order(&self, data: &CancelOrder) -> SdkResult<CancelOrderRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/cancel";
        post_send(self.0, url, data).await
    }

    /// 获取已支持的配送公司列表接口
    pub async fn get_all_imme_delivery(&self) -> SdkResult<ImmeDeliveryList> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/delivery/getall";
        post_send(self.0, url, &()).await
    }

    /// 拉取已绑定账号
    pub async fn get_bind_account(&self) -> SdkResult<BindAccountRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/shop/get";
        post_send(self.0, url, &()).await
    }

    /// 拉取配送单信息
    pub async fn get_order(&self, data: &QueryOrder) -> SdkResult<OrderInfo> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/get";
        post_send(self.0, url, data).await
    }

    /// 模拟配送公司更新配送单状态, 该接口只用于沙盒环境，即订单并没有真实流转到运力方
    pub async fn mock_update_order(&self, data: &MockUpdateOrder) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/test_update_order";
        post_send(self.0, url, data).await
    }

    /// 第三方代商户发起开通即时配送权限
    pub async fn open_delivery(&self) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/open";
        post_send(self.0, url, &()).await
    }

    /// 预下配送单接口
    pub async fn pre_add_order(&self, data: &PreAddOrder) -> SdkResult<PreAddOrderRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/pre_add";
        post_send(self.0, url, data).await
    }

    /// 预取消配送单接口
    pub async fn pre_cancel_order(&self, data: &CancelOrder) -> SdkResult<CancelOrderRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/precancel";
        post_send(self.0, url, data).await
    }

    /// 模拟配送公司更新配送单状态, 该接口用于测试账户下的单，将请求转发到运力测试环境
    pub async fn real_mock_update_order(
        &self,
        data: &RealMockUpdateOrder,
    ) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/realmock_update_order";
        post_send(self.0, url, data).await
    }

    /// 重新下单
    pub async fn re_order(&self, data: &AddOrder) -> SdkResult<AddOrderRes> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/business/order/readd";
        post_send(self.0, url, data).await
    }

    /// 配送公司更新配送单状态
    pub async fn update_order(&self, data: &UpdateOrder) -> SdkResult<ResultCode> {
        let url = "https://api.weixin.qq.com/cgi-bin/express/local/delivery/update_order";
        post_send(self.0, url, data).await
    }
}
