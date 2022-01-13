use super::{get_send, post_send};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAssistant {
    /// 房间ID
    pub room_id: i64,
    /// 用户数组
    pub users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// 用户微信号
    pub username: String,
    /// 用户昵称
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddRoomGoods {
    /// 房间ID
    pub room_id: i64,
    /// 数组列表，可传入多个，里面填写 商品 ID
    pub ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRole {
    /// 微信号
    pub username: String,
    /// 取值[1-管理员，2-主播，3-运营者]，设置超级管理员将无效
    pub role: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoom {
    /// 直播间名字，最短3个汉字，最长17个汉字，1个汉字相当于2个字符
    pub name: String,
    /// 背景图，填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取，请参考以下文档： https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html；直播间背景图，图片规则：建议像素1080*1920，大小不超过2M
    pub cover_img: String,
    /// 直播计划开始时间（开播时间需要在当前时间的10分钟后 并且 开始时间不能在 6 个月后）
    pub start_time: i64,
    /// 直播计划结束时间（开播时间和结束时间间隔不得短于30分钟，不得超过24小时）
    pub end_time: i64,
    /// 主播昵称，最短2个汉字，最长15个汉字，1个汉字相当于2个字符
    pub anchor_name: String,
    /// 主播微信号，如果未实名认证，需要先前往“小程序直播”小程序进行实名验证, 小程序二维码链接：https://res.wx.qq.com/op_res/9rSix1dhHfK4rR049JL0PHJ7TpOvkuZ3mE0z7Ou_Etvjf-w1J_jVX0rZqeStLfwh
    pub anchor_wechat: String,
    /// 主播副号微信号，如果未实名认证，需要先前往“小程序直播”小程序进行实名验证, 小程序二维码链接：https://res.wx.qq.com/op_res/9rSix1dhHfK4rR049JL0PHJ7TpOvkuZ3mE0z7Ou_Etvjf-w1J_jVX0rZqeStLfwh
    #[serde(default)]
    pub sub_anchor_wechat: Option<String>,
    /// 创建者微信号，不传入则此直播间所有成员可见。传入则此房间仅创建者、管理员、超管、直播间主播可见
    #[serde(default)]
    pub creater_wechat: Option<String>,
    /// 分享图，填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取，
    pub share_img: String,
    /// 购物直播频道封面图，填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取
    pub feeds_img: String,
    /// 是否开启官方收录 【1: 开启，0：关闭】，默认开启收录
    #[serde(default)]
    pub is_feeds_public: Option<i32>,
    /// 直播间类型 【1: 推流，0：手机直播】
    #[serde(rename = "type")]
    pub type_: i32,
    /// 是否关闭点赞 【0：开启，1：关闭】（若关闭，观众端将隐藏点赞按钮，直播开始后不允许开启）
    pub close_like: i32,
    /// 是否关闭货架 【0：开启，1：关闭】（若关闭，观众端将隐藏商品货架，直播开始后不允许开启）
    pub close_goods: i32,
    /// 是否关闭评论 【0：开启，1：关闭】（若关闭，观众端将隐藏评论入口，直播开始后不允许开启）
    pub close_comment: i32,
    /// 是否关闭回放 【0：开启，1：关闭】默认关闭回放（直播开始后允许开启）
    #[serde(default)]
    pub close_replay: Option<i32>,
    /// 是否关闭分享 【0：开启，1：关闭】默认开启分享（直播开始后不允许修改）
    #[serde(default)]
    pub close_share: Option<i32>,
    /// 是否关闭客服 【0：开启，1：关闭】 默认关闭客服（直播开始后允许开启）
    #[serde(default)]
    pub close_kf: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoomRes {
    /// 房间ID
    pub room_id: i64,
    /// "小程序直播" 小程序码
    pub qrcode_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryLiveInfo {
    pub start: i64,
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfos {
    pub total: i64,
    pub room_info: Vec<RoomInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomInfo {
    /// 直播间名称
    pub name: String,
    /// 直播间ID
    pub roomid: i64,
    /// 直播间背景图链接
    pub cover_img: String,
    /// 直播间分享图链接
    pub share_img: String,
    /// 直播间状态。101：直播中，102：未开始，103已结束，104禁播，105：暂停，106：异常，107：已过期
    pub live_status: i32,
    /// 直播间开始时间，列表按照start_time降序排列
    pub start_time: i64,
    /// 直播计划结束时间
    pub end_time: i64,
    /// 主播名
    pub anchor_name: String,
    ///
    pub goods: Vec<RoomGoodsInfo>,
    /// 直播类型，1 推流 0 手机直播
    pub live_type: i32,
    /// 是否关闭点赞 【0：开启，1：关闭】（若关闭，观众端将隐藏点赞按钮，直播开始后不允许开启）
    pub close_like: i32,
    /// 是否关闭货架 【0：开启，1：关闭】（若关闭，观众端将隐藏商品货架，直播开始后不允许开启）
    pub close_goods: i32,
    /// 是否关闭评论 【0：开启，1：关闭】（若关闭，观众端将隐藏评论入口，直播开始后不允许开启）
    pub close_comment: i32,
    /// 是否关闭客服 【0：开启，1：关闭】 默认关闭客服（直播开始后允许开启）
    pub close_kf: i32,
    /// 是否关闭回放 【0：开启，1：关闭】默认关闭回放（直播开始后允许开启）
    pub close_replay: i32,
    /// 是否开启官方收录，1 开启，0 关闭
    pub is_feeds_public: i32,
    /// 创建者openid
    pub creater_openid: String,
    /// 官方收录封面
    pub feeds_img: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoomGoodsInfo {
    /// 商品封面图链接
    pub cover_img: String,
    /// 商品小程序路径
    pub url: String,
    /// 商品名称
    pub name: String,
    /// 商品价格（分）
    pub price: f64,
    /// 商品价格，使用方式看price_type
    pub price2: f64,
    /// 价格类型，
    /// 1：一口价（只需要传入price，price2不传）
    /// 2：价格区间（price字段为左边界，price2字段为右边界，price和price2必传）
    /// 3：显示折扣价（price字段为原价，price2字段为现价， price和price2必传）
    pub price_type: i32,
    /// 商品id
    pub goods_id: i64,
    /// 第三方商品appid ,当前小程序商品则为空
    pub third_party_appid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryLiveReplay {
    /// 获取回放
    pub action: String,
    /// 直播间ID
    pub room_id: i64,
    /// 起始拉取视频，0表示从第一个视频片段开始拉取
    pub start: i64,
    /// 每次拉取的数量，建议100以内
    pub limit: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveReplayRes {
    pub live_replay: Vec<LiveReplay>,
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LiveReplay {
    /// 回放视频url过期时间
    pub expire_time: String,
    /// 回放视频创建时间
    pub create_time: String,
    /// 回放视频链接
    pub media_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRoom {
    /// 直播间id
    pub id: i64,
    /// 直播间名字，最短3个汉字，最长17个汉字，1个汉字相当于2个字符
    pub name: String,
    /// 背景图，填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取，请参考以下文档： https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html；直播间背景图，图片规则：建议像素1080*1920，大小不超过2M
    pub cover_img: String,
    /// 直播计划开始时间（开播时间需要在当前时间的10分钟后 并且 开始时间不能在 6 个月后）
    pub start_time: i64,
    /// 直播计划结束时间（开播时间和结束时间间隔不得短于30分钟，不得超过24小时）
    pub end_time: i64,
    /// 主播昵称，最短2个汉字，最长15个汉字，1个汉字相当于2个字符
    pub anchor_name: String,
    /// 主播微信号，如果未实名认证，需要先前往“小程序直播”小程序进行实名验证, 小程序二维码链接：https://res.wx.qq.com/op_res/9rSix1dhHfK4rR049JL0PHJ7TpOvkuZ3mE0z7Ou_Etvjf-w1J_jVX0rZqeStLfwh
    pub anchor_wechat: String,
    /// 分享图，填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取，请参考以下文档： https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html；直播间分享图，图片规则：建议像素800*640，大小不超过1M；
    pub share_img: String,
    /// 购物直播频道封面图，填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取，请参考以下文档： https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html; 购物直播频道封面图，图片规则：建议像素800*800，大小不超过100KB；
    pub feeds_img: String,
    /// 是否开启官方收录 【1: 开启，0：关闭】，默认开启收录
    #[serde(default)]
    pub is_feeds_public: Option<i32>,
    /// 是否关闭点赞 【0：开启，1：关闭】（若关闭，观众端不展示点赞入口，直播开始后不允许开启）
    pub close_like: i32,
    /// 是否关闭货架 【0：开启，1：关闭】（若关闭，观众端不展示商品货架，直播开始后不允许开启）
    pub close_goods: i32,
    /// 是否关闭评论 【0：开启，1：关闭】（若关闭，观众端不展示评论入口，直播开始后不允许开启）
    pub close_comment: i32,
    /// 是否关闭回放 【0：开启，1：关闭】默认关闭回放（直播开始后允许开启）
    #[serde(default)]
    pub close_replay: Option<i32>,
    /// 是否关闭分享 【0：开启，1：关闭】默认开启分享（直播开始后不允许修改）
    #[serde(default)]
    pub close_share: Option<i32>,
    /// 是否关闭客服 【0：开启，1：关闭】 默认关闭客服（直播开始后允许开启）
    #[serde(default)]
    pub close_kf: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushAddr {
    pub push_addr: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuerySharedCode {
    /// 房间ID
    pub room_id: i64,
    /// 自定义参数
    #[serde(default)]
    pub params: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedCodeRes {
    pub cdn_url: String,
    pub page_path: String,
    pub poster_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyAssistant {
    /// 房间ID
    pub room_id: i64,
    /// 用户微信号
    pub username: String,
    /// 用户微信昵称
    pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoveAssistant {
    /// 房间ID
    pub room_id: i32,
    /// 用户微信号
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssistantList {
    /// 小助手列表
    pub list: Vec<Assistant>,
    /// 小助手个数
    pub count: i64,
    /// 小助手最大个数
    pub max_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Assistant {
    /// 修改时间
    pub timestamp: i64,
    /// 头像
    pub headimg: String,
    /// 昵称
    pub nickname: String,
    /// 微信号
    pub alias: String,
    /// openid
    pub openid: String,
}

/// 主播副号
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subanchor {
    /// 房间ID
    pub room_id: i32,
    /// 用户微信号
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Username {
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomFnFeedPublic {
    /// 房间ID
    pub room_id: i64,
    /// 是否开启官方收录 【1: 开启，0：关闭】
    pub is_feeds_public: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomFnReplay {
    /// 房间ID
    pub room_id: i64,
    /// 是否关闭回放 【0：开启，1：关闭】
    pub close_replay: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomFnKf {
    /// 房间ID
    pub room_id: i64,
    /// 是否关闭客服 【0：开启，1：关闭】
    pub close_kf: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomFnComment {
    /// 房间ID
    pub room_id: i64,
    /// 1-禁言，0-取消禁言
    pub ban_comment: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomFnGoodsOnsale {
    /// 房间ID
    pub room_id: i64,
    /// 商品ID
    pub goods_id: i64,
    /// 上下架 【0：下架，1：上架】
    pub on_sale: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomGoods {
    /// 房间ID
    pub room_id: i64,
    /// 商品ID
    pub goods_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoomGoodsList {
    /// 房间ID
    pub room_id: i64,
    /// 商品ID列表
    pub goods: Vec<GoodsId>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsId {
    /// 商品ID
    pub goods_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UrlRes {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddGoods {
    /// 填入mediaID（mediaID获取后，三天内有效）；图片mediaID的获取，请参考以下文档： https://developers.weixin.qq.com/doc/offiaccount/Asset_Management/New_temporary_materials.html；图片规则：图片尺寸最大300像素*300像素；
    pub cover_img_url: String,
    /// 商品名称，最长14个汉字，1个汉字相当于2个字符
    pub name: String,
    /// 价格类型，1：一口价（只需要传入price，price2不传） 2：价格区间（price字段为左边界，price2字段为右边界，price和price2必传） 3：显示折扣价（price字段为原价，price2字段为现价， price和price2必传）
    pub price_type: i32,
    /// 数字，最多保留两位小数，单位元
    pub price: f64,
    /// 数字，最多保留两位小数，单位元
    #[serde(default)]
    pub price2: Option<f64>,
    /// 商品详情页的小程序路径，路径参数存在 url 的，该参数的值需要进行 encode 处理再填入
    pub url: String,
    /// 当商品为第三方小程序的商品则填写为对应第三方小程序的appid，自身小程序商品则为''
    #[serde(default)]
    pub third_party_appid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsInfo {
    /// 商品封面图链接
    pub cover_img: String,
    /// 商品小程序路径
    pub url: String,
    /// 商品名称
    pub name: String,
    /// 商品价格（分）
    pub price: f64,
    /// 商品价格，使用方式看price_type
    pub price2: f64,
    /// 价格类型，
    /// 1：一口价（只需要传入price，price2不传）
    /// 2：价格区间（price字段为左边界，price2字段为右边界，price和price2必传）
    /// 3：显示折扣价（price字段为原价，price2字段为现价， price和price2必传）
    pub price_type: i32,
    /// 商品id
    pub goods_id: i64,
    /// 第三方商品appid ,当前小程序商品则为空
    pub third_party_appid: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsAudit {
    /// 商品ID
    pub goods_id: i64,
    /// 审核单ID
    pub audit_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuditId {
    /// 审核单ID
    pub audit_id: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsWarehouseList {
    pub total: i32,
    pub goods: Vec<GoodsStatusInfo>,
}

// 微信文档的 JSON 字段风格是 snake_case, `third_party_appid` 没有在 JSON 字段里
// #[serde(rename_all = "camelCase")]
#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsStatusInfo {
    /// 商品id
    pub goods_id: i64,
    /// 商品封面图链接
    pub cover_img: String,
    /// 商品小程序路径
    pub url: String,
    /// 商品名称
    pub name: String,
    /// 商品价格（分）
    pub price: f64,
    /// 商品价格，使用方式看price_type
    pub price2: f64,
    /// 价格类型，
    /// 1：一口价（只需要传入price，price2不传）
    /// 2：价格区间（price字段为左边界，price2字段为右边界，price和price2必传）
    /// 3：显示折扣价（price字段为原价，price2字段为现价， price和price2必传）
    pub price_type: i32,
    /// 0：未审核，1：审核中，2:审核通过，3审核失败
    pub audit_status: i32,
    /// 1、2：表示是为 API 添加商品，否则是直播控制台添加的商品
    pub third_party_tag: i32,
    /// 第三方商品appid ,当前小程序商品则为空
    pub third_party_appid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryGoodsApproved {
    /// 分页条数起点
    pub offset: i32,
    /// 分页大小，默认30，不超过100
    #[serde(default)]
    pub limit: Option<i32>,
    /// 商品状态，0：未审核。1：审核中，2：审核通过，3：审核驳回
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoodsApprovedList {
    pub total: i64,
    pub goods: Vec<GoodsApprovedInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodsApprovedInfo {
    /// 商品id
    pub goods_id: i64,
    /// 商品封面图链接
    pub cover_img: String,
    /// 商品小程序路径
    pub url: String,
    /// 商品名称
    pub name: String,
    /// 商品价格（分）
    pub price: f64,
    /// 商品价格，使用方式看price_type
    pub price2: f64,
    /// 价格类型，
    /// 1：一口价（只需要传入price，price2不传）
    /// 2：价格区间（price字段为左边界，price2字段为右边界，price和price2必传）
    /// 3：显示折扣价（price字段为原价，price2字段为现价， price和price2必传）
    pub price_type: i32,
    /// 1、2：表示是为 API 添加商品，否则是直播控制台添加的商品
    pub third_party_tag: i32,
    /// 第三方商品appid ,当前小程序商品则为空
    pub third_party_appid: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRoleList {
    /// 查询的用户角色，取值 [-1-所有成员， 0-超级管理员，1-管理员，2-主播，3-运营者]，默认-1
    #[serde(default)]
    pub role: Option<i32>,
    /// 起始偏移量, 默认0
    #[serde(default)]
    pub offset: Option<i32>,
    /// 查询个数，最大30，默认10
    #[serde(default)]
    pub limit: Option<i32>,
    /// 搜索的微信号或昵称，不传则返回全部
    #[serde(default)]
    pub keyword: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleList {
    pub total: i64,
    pub list: Vec<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Role {
    /// 微信用户头像url
    pub headingimg: String,
    /// 微信用户昵称
    pub nickname: String,
    /// openid
    pub openid: String,
    /// 具有的身份，[0-超级管理员，1-管理员，2-主播，3-运营者]
    pub role_list: Vec<i64>,
    /// 更新时间
    pub update_timestamp: String,
    /// 微信号
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PushMessage {
    /// 直播开始事件的房间ID
    pub room_id: i32,
    /// 接收该群发开播事件的订阅用户OpenId列表
    pub user_openid: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageId {
    /// 直播开始事件的房间ID
    pub room_id: i32,
    /// 此次群发消息的标识ID，用于对应【长期订阅群发结果回调】的message_id
    pub message_id: String,
}

pub struct LiveBroadcastModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> LiveBroadcastModule<'a, T> {
    /// 创建直播间
    pub async fn create_room(&self, data: &CreateRoom) -> SdkResult<CreateRoomRes> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/create";
        post_send(self.0, url, data).await
    }

    /// 获取直播间列表及直播间信息
    pub async fn get_live_info(&self, data: &QueryLiveInfo) -> SdkResult<RoomInfos> {
        let url = "https://api.weixin.qq.com/wxa/business/getliveinfo";
        post_send(self.0, url, data).await
    }

    /// 获取已结束直播间的回放源视频（一般在直播结束后10分钟内生成，源视频无评论等内容）
    pub async fn get_live_replay(&self, data: &QueryLiveInfo) -> SdkResult<LiveReplayRes> {
        let url = "https://api.weixin.qq.com/wxa/business/getliveinfo";
        post_send(self.0, url, data).await
    }

    /// 调用接口往指定直播间导入已入库的商品
    pub async fn add_room_goods(&self, data: &AddRoomGoods) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/addgoods";
        post_send(self.0, url, data).await
    }

    /// 删除直播间
    pub async fn delete_room(&self, room_id: i64) -> SdkResult<()> {
        let data = &serde_json::json!({ "id": room_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/deleteroom";
        post_send(self.0, url, data).await
    }

    /// 编辑直播间
    pub async fn edit_room(&self, data: &EditRoom) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/editroom";
        post_send(self.0, url, data).await
    }

    /// 获取直播间推流地址
    pub async fn get_push_url(&self, room_id: i64) -> SdkResult<PushAddr> {
        let data = &serde_json::json!({ "roomId": room_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/getpushurl";
        get_send(self.0, url, data).await
    }

    /// 获取直播间分享二维码
    pub async fn get_shared_code(&self, data: &QuerySharedCode) -> SdkResult<SharedCodeRes> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/getsharedcode";
        get_send(self.0, url, data).await
    }

    /// 添加管理直播间小助手
    pub async fn add_assistant(&self, data: &AddAssistant) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/addassistant";
        post_send(self.0, url, data).await
    }

    /// 修改管理直播间小助手
    pub async fn modify_assistant(&self, data: &ModifyAssistant) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/modifyassistant";
        post_send(self.0, url, data).await
    }

    /// 删除管理直播间小助手
    pub async fn remove_assistant(&self, data: &RemoveAssistant) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/removeassistant";
        post_send(self.0, url, data).await
    }

    /// 查询管理直播间小助手
    pub async fn get_assistant_list(&self, room_id: i64) -> SdkResult<AssistantList> {
        let data = &serde_json::json!({ "roomId": room_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/getassistantlist";
        get_send(self.0, url, data).await
    }

    /// 添加主播副号
    pub async fn add_subanchor(&self, data: &Subanchor) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/addsubanchor";
        post_send(self.0, url, data).await
    }

    /// 修改主播副号
    pub async fn modify_subanchor(&self, data: &Subanchor) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/modifysubanchor";
        post_send(self.0, url, data).await
    }

    /// 删除主播副号
    pub async fn delete_subanchor(&self, room_id: i64) -> SdkResult<()> {
        let data = &serde_json::json!({ "roomId": room_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/deletesubanchor";
        post_send(self.0, url, data).await
    }

    /// 获取主播副号
    pub async fn get_subanchor(&self, room_id: i64) -> SdkResult<Username> {
        let data = &serde_json::json!({ "roomId": room_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/getsubanchor";
        get_send(self.0, url, data).await
    }

    /// 开启/关闭直播间官方收录
    pub async fn update_feed_public(&self, data: &RoomFnFeedPublic) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/updatefeedpublic";
        post_send(self.0, url, data).await
    }

    /// 开启/关闭回放功能
    pub async fn update_replay(&self, data: &RoomFnReplay) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/updatereplay";
        post_send(self.0, url, data).await
    }

    /// 开启/关闭客服
    pub async fn update_kf(&self, data: &RoomFnKf) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/updatekf";
        post_send(self.0, url, data).await
    }

    /// 开启/关闭直播间全局禁言
    pub async fn update_comment(&self, data: &RoomFnComment) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/room/updatecomment";
        post_send(self.0, url, data).await
    }

    /// 上下架商品
    pub async fn update_goods_onsale(&self, data: &RoomFnGoodsOnsale) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/onsale";
        post_send(self.0, url, data).await
    }

    /// 删除直播间商品
    pub async fn delete_room_goods(&self, data: &RoomGoods) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/deleteInRoom";
        post_send(self.0, url, data).await
    }

    /// 推送商品
    pub async fn push_room_goods(&self, data: &RoomGoods) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/push";
        post_send(self.0, url, data).await
    }

    /// 直播间商品排序
    pub async fn sort_room_goods(&self, data: &RoomGoodsList) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/sort";
        post_send(self.0, url, data).await
    }

    /// 下载商品讲解视频
    pub async fn get_goods_video(&self, data: &RoomGoods) -> SdkResult<UrlRes> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/getVideo";
        post_send(self.0, url, data).await
    }

    /// 上传并提审需要直播的商品信息，审核通过后商品录入【小程序直播】商品库
    /// 注意：开发者必须保存【商品ID】与【审核单ID】，如果丢失，则无法调用其他相关接口
    pub async fn add_goods(&self, data: &AddGoods) -> SdkResult<GoodsAudit> {
        let data = &serde_json::json!({ "goodsInfo": data });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/add";
        post_send(self.0, url, data).await
    }

    /// 可撤回直播商品的提审申请，消耗的提审次数不返还
    pub async fn reset_audit(&self, data: &AddGoods) -> SdkResult<GoodsAudit> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/resetaudit";
        post_send(self.0, url, data).await
    }

    /// 可以对已撤回提审的商品再次发起提审申请
    pub async fn audit_goods(&self, goods_id: i64) -> SdkResult<AuditId> {
        let data = &serde_json::json!({ "goodsId": goods_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/audit";
        post_send(self.0, url, data).await
    }

    /// 可删除【小程序直播】商品库中的商品，删除后直播间上架的该商品也将被同步删除，不可恢复
    pub async fn delete_goods(&self, goods_id: i64) -> SdkResult<()> {
        let data = &serde_json::json!({ "goodsId": goods_id });
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/delete";
        post_send(self.0, url, data).await
    }

    /// 更新商品信息，审核通过的商品仅允许更新价格类型与价格，审核中的商品不允许更新，未审核的商品允许更新所有字段，只传入需要更新的字段
    pub async fn update_goods(&self, data: &GoodsInfo) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/update";
        post_send(self.0, url, data).await
    }

    /// 获取商品的信息与审核状态
    pub async fn get_goods_warehouse(&self, goods_ids: &[i64]) -> SdkResult<GoodsWarehouseList> {
        let data = &serde_json::json!({ "goods_ids": goods_ids });
        let url = "https://api.weixin.qq.com/wxa/business/getgoodswarehouse";
        post_send(self.0, url, data).await
    }

    /// 获取商品列表
    pub async fn get_goods_approved(
        &self,
        data: &QueryGoodsApproved,
    ) -> SdkResult<GoodsApprovedList> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/goods/getapproved";
        post_send(self.0, url, data).await
    }

    /// 设置小程序直播成员的管理员、运营者和主播角色
    pub async fn add_role(&self, data: &UserRole) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/role/addrole";
        post_send(self.0, url, data).await
    }

    /// 移除小程序直播成员的管理员、运营者和主播角色
    pub async fn delete_role(&self, data: &UserRole) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/role/deleterole";
        post_send(self.0, url, data).await
    }

    /// 移除小程序直播成员的管理员、运营者和主播角色
    pub async fn get_role_list(&self, data: &QueryRoleList) -> SdkResult<RoleList> {
        let url = "https://api.weixin.qq.com/wxaapi/broadcast/role/getrolelist";
        post_send(self.0, url, data).await
    }

    /// 向长期订阅用户群发直播间开始事件
    pub async fn push_message(&self, data: &PushMessage) -> SdkResult<MessageId> {
        let url = "https://api.weixin.qq.com/wxa/business/push_message";
        post_send(self.0, url, data).await
    }
}
