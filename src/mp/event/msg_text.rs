use roxmltree::Node;

use crate::{error::SdkError, SdkResult};

use super::ReceivedMessageParser;

pub struct TextMessage {
    pub msg_id: u64,
    pub content: String,
}

impl ReceivedMessageParser for TextMessage {
    type ReceivedMessage = TextMessage;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
        let content = node
            .descendants()
            .find(|n| n.has_tag_name("Content"))
            .map(|n| n.text())
            .flatten()
            .ok_or_else(|| SdkError::InvalidParams("parse xml need Content params".to_owned()))?;
        let msg_id = node
            .descendants()
            .find(|n| n.has_tag_name("MsgId"))
            .map(|n| n.text())
            .flatten()
            .ok_or_else(|| SdkError::InvalidParams("parse xml need MsgId params".to_owned()))?;
        let msg_id = msg_id.parse::<u64>().map_err(|_e| {
            SdkError::InvalidParams(
                "Parse XML msg from wechat error: tag `MsgId` should be number".to_string(),
            )
        })?;
        Ok(TextMessage {
            content: content.to_owned(),
            msg_id,
        })
    }
}

#[test]
pub fn parse() -> SdkResult<()> {
    use roxmltree::Document;

    let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = TextMessage::from_xml(&node.root())?;
    assert_eq!(msg.content, "this is a test");
    assert_eq!(msg.msg_id, 1234567890123456);
    Ok(())
}
