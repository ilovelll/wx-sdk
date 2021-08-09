use crate::{
    error::SdkError,
    mp::event::{
        xmlutil::{get_number_from_root, get_text_from_root},
        ReceivedMessageParser,
    },
    SdkResult,
};

pub struct GuideInviteResultEvent {
    pub guide_account: Option<String>,
    pub guide_openid: Option<String>,
    pub invite_result: i32,
}

impl ReceivedMessageParser for GuideInviteResultEvent {
    type ReceivedMessage = GuideInviteResultEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let guide_invite_event = node
            .descendants()
            .find(|n| n.has_tag_name("GuideInviteEvent"))
            .ok_or_else(|| {
                SdkError::InvalidParams(format!(
                    "Parse XML msg from wechat error: tag `{}` is none",
                    "GuideInviteEvent"
                ))
            })?;
        let guide_account = get_text_from_root(&guide_invite_event, "guide_account")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string());
        let guide_openid = get_text_from_root(&guide_invite_event, "guide_openid")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string());
        if guide_account.is_none() && guide_openid.is_none() {
            return Err(SdkError::InvalidParams(format!("parse GuideInviteEvent error: guide_account and guide_openid should exist at least one of them.")));
        }
        let invite_result = get_number_from_root::<i32>(&guide_invite_event, "invite_result")?;
        Ok(GuideInviteResultEvent {
            guide_account,
            guide_openid,
            invite_result,
        })
    }
}

pub struct GuideQrcodeScanEvent {
    pub qrcode_guide_account: Option<String>,
    pub qrcode_guide_openid: Option<String>,
    pub openid: String,
    pub action: u8,
    pub qrcode_info: String,
}

impl ReceivedMessageParser for GuideQrcodeScanEvent {
    type ReceivedMessage = GuideQrcodeScanEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let guide_scan_event = node
            .descendants()
            .find(|n| n.has_tag_name("GuideScanEvent"))
            .ok_or_else(|| {
                SdkError::InvalidParams(format!(
                    "Parse XML msg from wechat error: tag `{}` is none",
                    "GuideScanEvent"
                ))
            })?;

        let qrcode_guide_account = get_text_from_root(&guide_scan_event, "qrcode_guide_account")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string());
        let qrcode_guide_openid = get_text_from_root(&guide_scan_event, "qrcode_guide_openid")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string());
        if qrcode_guide_account.is_none() && qrcode_guide_openid.is_none() {
            return Err(SdkError::InvalidParams(format!("parse GuideScanEvent error: qrcode_guide_account and qrcode_guide_openid should exist at least one of them.")));
        }

        let action = get_number_from_root::<u8>(&guide_scan_event, "action")?;
        let openid = get_text_from_root(&guide_scan_event, "openid")?;
        let qrcode_info = get_text_from_root(&guide_scan_event, "qrcode_info")?;
        Ok(GuideQrcodeScanEvent {
            qrcode_guide_account,
            qrcode_guide_openid,
            action,
            openid: openid.to_string(),
            qrcode_info: qrcode_info.to_string(),
        })
    }
}
