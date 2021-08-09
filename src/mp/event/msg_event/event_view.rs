use crate::{
    mp::event::{xmlutil::get_text_from_root, ReceivedMessageParser},
    SdkResult,
};

pub struct ViewEvent {
    pub event_key: String,
    pub menu_id: Option<String>,
}

impl ReceivedMessageParser for ViewEvent {
    type ReceivedMessage = ViewEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let event_key = get_text_from_root(&node, "EventKey")?;
        let menu_id = node
            .descendants()
            .find(|n| n.has_tag_name("MenuId"))
            .map(|n| n.text())
            .flatten()
            .map(|s| s.to_string());
        Ok(ViewEvent {
            event_key: event_key.to_string(),
            menu_id,
        })
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
    <Event><![CDATA[VIEW]]></Event>
    <EventKey><![CDATA[www.qq.com]]></EventKey>
    <MenuId>MENUID</MenuId>
    </xml>
    ";
    let node = Document::parse(&s)?;
    let msg = ViewEvent::from_xml(&node.root())?;

    assert_eq!(msg.event_key, "www.qq.com");
    assert_eq!(msg.menu_id, Some("MENUID".to_string()));
    Ok(())
}
