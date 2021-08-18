use roxmltree::Node;

use crate::{
    mp::event::{
        xmlutil::{get_node_by_tag, get_number_from_root, get_text_from_root},
        ReceivedMessageParser,
    },
    SdkResult,
};

pub struct MassSendJobFinishEvent {
    pub msg_id: u64,
    pub status: String,
    // #[serde(alias = "TotalCount")]
    pub total_count: u64,
    // #[serde(alias = "FilterCount")]
    pub filter_count: u64,
    // #[serde(alias = "SentCount")]
    pub sent_count: u64,
    // #[serde(alias = "ErrorCount")]
    pub error_count: u64,
    // #[serde(alias = "CopyrightCheckResult")]
    pub copyright_check_result: CopyrightCheckResult,
}

pub struct CopyrightCheckResult {
    // #[serde(alias = "Count")]
    pub count: u16,
    // #[serde(alias = "CheckState")]
    pub check_state: u8,
    // #[serde(alias = "ResultList")]
    pub result_list: Vec<CopyrightCheckResultItem>,
}

pub struct CopyrightCheckResultItem {
    pub article_idx: i8,              //群发文章的序号，从1开始
    pub user_declare_state: i8,       //用户声明文章的状态
    pub audit_state: i8,              //系统校验的状态,
    pub original_article_url: String, //相似原创文的url
    pub original_article_type: i8,    //相似原创文的类型
    pub can_reprint: i8,              //是否能转载
    pub need_replace_content: i8,     //是否需要替换成原创文内容
    pub need_show_reprint_source: i8, //是否需要注明转载来
}
impl ReceivedMessageParser for MassSendJobFinishEvent {
    type ReceivedMessage = MassSendJobFinishEvent;

    fn from_xml(node: &roxmltree::Node) -> crate::SdkResult<Self::ReceivedMessage> {
        let msg_id = get_number_from_root::<u64>(node, "MsgId")?;
        let status = get_text_from_root(node, "Status")?;
        let total_count = get_number_from_root::<u64>(node, "TotalCount")?;
        let filter_count = get_number_from_root::<u64>(node, "FilterCount")?;
        let sent_count = get_number_from_root::<u64>(node, "SentCount")?;
        let error_count = get_number_from_root::<u64>(node, "ErrorCount")?;
        let copyright_check = get_node_by_tag(node, "CopyrightCheckResult")?;
        let copyright_count = get_number_from_root::<u16>(&copyright_check, "Count")?;
        let copyright_check_state = get_number_from_root::<u8>(&copyright_check, "CheckState")?;
        let copyright_result_list = get_node_by_tag(&copyright_check, "ResultList")?;
        let copyright_result_list: Vec<Node> = copyright_result_list
            .descendants()
            .filter(|n| n.has_tag_name("item"))
            .collect();
        let copyright_result_list: SdkResult<Vec<CopyrightCheckResultItem>> = copyright_result_list
            .iter()
            .map(|n| {
                let article_idx = get_number_from_root::<i8>(n, "ArticleIdx")?;
                let user_declare_state = get_number_from_root::<i8>(n, "UserDeclareState")?;
                let audit_state = get_number_from_root::<i8>(n, "AuditState")?;
                let original_article_url = get_text_from_root(n, "OriginalArticleUrl")?;
                let original_article_type = get_number_from_root::<i8>(n, "OriginalArticleType")?;
                let can_reprint = get_number_from_root(n, "CanReprint")?;
                let need_replace_content = get_number_from_root(n, "NeedReplaceContent")?;
                let need_show_reprint_source = get_number_from_root(n, "NeedShowReprintSource")?;
                Ok(CopyrightCheckResultItem {
                    article_idx,
                    user_declare_state,
                    audit_state,
                    original_article_url: original_article_url.to_owned(),
                    original_article_type,
                    can_reprint,
                    need_replace_content,
                    need_show_reprint_source,
                })
            })
            .collect();
        let copyright_result_list = copyright_result_list?;
        let copyright_check_result = CopyrightCheckResult {
            count: copyright_count,
            check_state: copyright_check_state,
            result_list: copyright_result_list,
        };
        Ok(MassSendJobFinishEvent {
            msg_id,
            status: status.to_string(),
            total_count,
            filter_count,
            sent_count,
            error_count,
            copyright_check_result,
        })
    }
}
pub struct TemplateSendJobFinishEvent {
    pub msg_id: u64,
    pub status: String,
}

impl ReceivedMessageParser for TemplateSendJobFinishEvent {
    type ReceivedMessage = TemplateSendJobFinishEvent;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
        let msg_id = get_number_from_root::<u64>(node, "MsgId")?;
        let status = get_text_from_root(node, "Status")?;
        Ok(TemplateSendJobFinishEvent {
            msg_id,
            status: status.to_string(),
        })
    }
}
