use crate::{SdkResult, error::SdkError, mp::{event::{ReceivedMessageParser, xmlutil::get_text_from_root}}};

use super::{EventMessage, event_scan::ScanEvent};

pub struct SubScribeEvent;

impl ReceivedMessageParser for SubScribeEvent {
    type ReceivedMessage = EventMessage;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
	    let ekn =
            node.descendants().find(|n| n.has_tag_name("EventKey"));
        let event = match ekn {
            Some(n) => {
                let event_key = n.text().ok_or_else(|| {
                    SdkError::InvalidParams(format!(
                        "Parse XML msg from wechat error: tag `{}` text content is none",
                        n.tag_name().name()
                    ))
                })?;
                let ticket = get_text_from_root(&node, "Ticket")?;
                EventMessage::SubscribeScan(ScanEvent {
                    event_key: event_key.to_string(),
                    ticket: ticket.to_string()
                })
            }
            None => {
                EventMessage::Subscribe
            }
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
    <Event><![CDATA[subscribe]]></Event>
    <EventKey><![CDATA[qrscene_123123]]></EventKey>
    <Ticket><![CDATA[TICKET]]></Ticket>
</xml>";
    let node = Document::parse(&s)?;
    let msg = SubScribeEvent::from_xml(&node.root())?;

    matches!(msg, EventMessage::SubscribeScan(_));
    Ok(())
}