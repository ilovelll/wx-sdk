use roxmltree::Node;

use crate::SdkResult;

use super::{ReceivedMessageParser, xmlutil::{get_number_from_root, get_text_from_root}};

pub struct VoiceMessage {
    pub msg_id: u64,
    pub format: String,
    pub media_id: String,
    pub recognition: Option<String>,
}

impl ReceivedMessageParser for VoiceMessage {
    type ReceivedMessage = VoiceMessage;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
	let msg_id = get_number_from_root::<u64>(&node, "MsgId")?;
        let media_id = get_text_from_root(&node, "MediaId")?;
        let format = get_text_from_root(&node, "Format")?;
        let recognition = node.descendants().find(|n| n.has_tag_name("Recognition"));
        let recognition = recognition
                            .map(|n| n.text())
                            .map(|t| t.map(|s| s.to_string()))
                            .unwrap_or_else(|| None);
        Ok(VoiceMessage {
            msg_id,
	    format: format.to_owned(),
	    media_id: media_id.to_owned(),
	    recognition,
        })
    }
}


#[test]
pub fn parse() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1357290913</CreateTime>
    <MsgType><![CDATA[voice]]></MsgType>
    <MediaId><![CDATA[media_id]]></MediaId>
    <Format><![CDATA[Format]]></Format>
    <MsgId>1234567890123456</MsgId>
  </xml> 
  ";
    let node = Document::parse(&s)?;
    let msg = VoiceMessage::from_xml(&node.root())?;
    assert_eq!(msg.media_id, "media_id");
    assert_eq!(msg.recognition, None);
    assert_eq!(msg.msg_id, 1234567890123456);
    Ok(())
}