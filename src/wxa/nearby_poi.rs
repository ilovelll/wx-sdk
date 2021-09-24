use super::{get_send, post_send};
use crate::{wechat::WxApiRequestBuilder, SdkResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    /// 必填,写死为"1"
    pub is_comm_nearby: String,
    /// 门店图片，最多9张，最少1张，上传门店图片如门店外景、环境设施、商品服务等，图片将展示在微信客户端的门店页。<br/>
    /// 图片链接通过文档 https://mp.weixin.qq.com/wiki?t=resource/res_main&id=mp1444738729
    /// 中的《上传图文消息内的图片获取URL》接口获取。<br/>
    /// 必填，文件格式为bmp、png、jpeg、jpg或gif，大小不超过5M pic_list是字符串，内容是一个json
    pub pic_list: String,
    /// 必服务标签列表，必填，需要填写 <br/>
    /// 1、服务标签ID <br/>
    /// 2、服务类型tpye
    /// 3、服务名称name详细字段格式见下方《服务标签id编号、类型与服务名称表》 <br/>
    /// 4、APPID <br/>
    /// 5、对应服务落地页的path路径：path路径页面要与对应的服务标签一致，例如选取外卖服务，
    ///    path路径应该是小程序的外卖对应的那个页面，path路径获取咨询开发或者到小程序管理后台-工具-生成小程序码页面获取 <br/>
    /// 6、新增服务描述desc：描述服务内容，例如满减、折扣等优惠信息或新品、爆品等商品信息，仅标准服务都可添加，10个字符以内。 <br/>
    /// service_infos是字符串，内容是一个json
    pub service_infos: String,
    /// 客服信息 选填，可自定义服务头像与昵称，具体填写字段见下方示例kf_info pic_list是字符串，内容是一个json
    pub kf_info: Option<String>,
    /// 门店名字 必填，门店名称需按照所选地理位置自动拉取腾讯地图门店名称，不可修改，如需修改请重现选择地图地点或重新创建地点。
    pub store_name: String,
    /// 营业时间，格式11:11-12:12 必填
    pub hour: String,
    /// 地址 必填
    pub address: String,
    /// 如果创建新的门店，poi_id字段为空 如果更新门店，poi_id参数则填对应门店的poi_id 选填
    pub poi_id: Option<String>,
    /// 主体名字 必填
    pub company_name: String,
    /// 门店电话 必填
    pub contract_phone: String,
    /// 资质号 必填, 15位营业执照注册号或9位组织机构代码
    pub credential: String,
    /// 证明材料 必填 如果company_name和该小程序主体不一致，需要填qualification_list，
    /// 详细规则见附近的小程序使用指南-如何证明门店的经营主体跟公众号或小程序帐号主体相关
    /// http://kf.qq.com/faq/170401MbUnim17040122m2qY.html
    pub qualification_list: String,
    /// 对应《在腾讯地图中搜索门店》中的sosomap_poi_uid字段，腾讯地图那边有些数据不一致， <br/>
    /// 如果不填map_poi_id的话，小概率会提交失败！ <br/>
    /// 注：poi_id与map_poi_id关系：map_poi_id是腾讯地图对于poi的唯一标识poi_id是门店进驻附近后的门店唯一标识
    pub map_poi_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddResult {
    /// 返回数据
    pub data: AddData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddData {
    /// 审核单 ID
    pub audit_id: String,
    /// 附近地点 ID
    pub poi_id: String,
    /// 经营资质证件号
    pub related_credential: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryList {
    /// 起始页id（从1开始计数）
    pub page: i32,
    /// 每页展示个数（最多1000个）
    pub page_rows: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoiListResult {
    /// 返回数据
    pub data: PoiListData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoiListData {
    /// 剩余可添加地点个数
    pub left_apply_num: i32,
    /// 最大可添加地点个数
    pub max_apply_num: i32,
    /// 地址列表的 JSON 格式字符串。 [PoiList]
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoiList {
    /// 地址列表
    pub poi_list: Vec<PoiItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PoiItem {
    /// 附近地点 ID
    pub poi_id: String,
    /// 资质证件地址
    pub qualification_address: String,
    /// 资质证件证件号
    pub qualification_num: String,
    /// 地点审核状态
    /// - `3`, 审核中
    /// - `4`, 审核失败
    /// - `5`, 审核通过
    pub audit_status: i32,
    /// 地点展示在附近状态
    /// - `0`, 未展示
    /// - `1`, 展示中
    pub display_status: i32,
    /// 审核失败原因，audit_status=4 时返回
    pub refuse_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetShowStatus {
    /// 附近地点 ID
    pub poi_id: String,
    /// 是否展示
    /// - `0`, 不展示
    /// - `1`, 展示
    pub status: i32,
}

pub struct NearbyPoiModule<'a, T: WxApiRequestBuilder>(pub(crate) &'a T);

impl<'a, T: WxApiRequestBuilder> NearbyPoiModule<'a, T> {
    /// 添加地点
    pub async fn add(&self, data: &Address) -> SdkResult<AddResult> {
        let url = "https://api.weixin.qq.com/wxa/addnearbypoi";
        post_send(self.0, url, data).await
    }

    /// 删除地点
    pub async fn delete(&self, poi_id: &str) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/delnearbypoi";
        let data = &serde_json::json!({ "poi_id": poi_id });
        post_send(self.0, url, data).await
    }

    /// 查看地点列表
    pub async fn get_list(&self, query: &QueryList) -> SdkResult<PoiListResult> {
        let url = "https://api.weixin.qq.com/wxa/getnearbypoilist";
        get_send(self.0, url, query).await
    }

    /// 展示/取消展示附近小程序
    pub async fn set_show_status(&self, data: &SetShowStatus) -> SdkResult<()> {
        let url = "https://api.weixin.qq.com/wxa/setnearbypoishowstatus";
        post_send(self.0, url, data).await
    }
}
