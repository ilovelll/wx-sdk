use crate::{
    mp::event::{xmlutil::get_text_from_root, ReceivedMessageParser},
    SdkResult,
};

pub struct ClickEvent {
    pub event_key: String,
}

impl ReceivedMessageParser for ClickEvent {
    type ReceivedMessage = ClickEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let event_key = get_text_from_root(node, "EventKey")?;
        Ok(ClickEvent {
            event_key: event_key.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SdkResult;
    #[test]
    pub fn parse() -> SdkResult<()> {
        use roxmltree::Document;
        let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[FromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[CLICK]]></Event>
    <EventKey><![CDATA[EVENTKEY]]></EventKey>
  </xml>";
        let node = Document::parse(&s)?;
        let msg = ClickEvent::from_xml(&node.root())?;

        assert_eq!(msg.event_key, "EVENTKEY");
        Ok(())
    }
}
