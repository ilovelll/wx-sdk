use crate::{
    error::SdkError,
    mp::event::{xmlutil::get_text_from_root, ReceivedMessageParser},
    SdkResult,
};

pub struct ScanEvent {
    pub event_key: String,
    pub ticket: String,
}

impl ReceivedMessageParser for ScanEvent {
    type ReceivedMessage = ScanEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let event_key = get_text_from_root(node, "EventKey")?;
        let ticket = get_text_from_root(node, "Ticket")?;
        Ok(ScanEvent {
            event_key: event_key.to_string(),
            ticket: ticket.to_string(),
        })
    }
}

pub struct MenuScanEvent {
    pub event_key: String,
    pub scan_type: String,
    pub scan_result: String,
}

impl ReceivedMessageParser for MenuScanEvent {
    type ReceivedMessage = MenuScanEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let event_key = get_text_from_root(node, "EventKey")?;
        let scan_code_info = node
            .descendants()
            .find(|n| n.has_tag_name("ScanCodeInfo"))
            .ok_or_else(|| {
                SdkError::InvalidParams(format!(
                    "Parse XML msg from wechat error: tag `{}` is none",
                    "ScanCodeInfo"
                ))
            })?;
        let scan_type = get_text_from_root(&scan_code_info, "ScanType")?;
        let scan_result = get_text_from_root(&scan_code_info, "ScanResult")?;
        let event = MenuScanEvent {
            event_key: event_key.to_owned(),
            scan_type: scan_type.to_owned(),
            scan_result: scan_result.to_owned(),
        };
        Ok(event)
    }
}

#[test]
pub fn parse() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[FromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[SCAN]]></Event>
    <EventKey><![CDATA[SCENE_VALUE]]></EventKey>
    <Ticket><![CDATA[TICKET]]></Ticket>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = ScanEvent::from_xml(&node.root())?;

    assert_eq!(msg.event_key, "SCENE_VALUE");
    Ok(())
}

#[test]
pub fn parse_menuscan() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml>
    <FromUserName><![CDATA[oMgHVjngRipVsoxg6TuX3vz6glDg]]></FromUserName>
    <CreateTime>1408090502</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[scancode_push]]></Event>
    <EventKey><![CDATA[6]]></EventKey>
    <ScanCodeInfo><ScanType><![CDATA[qrcode]]></ScanType>
    <ScanResult><![CDATA[1]]></ScanResult>
    </ScanCodeInfo>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = MenuScanEvent::from_xml(&node.root())?;

    assert_eq!(msg.event_key, "6");
    assert_eq!(msg.scan_type, "qrcode");
    assert_eq!(msg.scan_result, "1");
    Ok(())
}
