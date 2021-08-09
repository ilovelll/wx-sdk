use roxmltree::Node;

use crate::SdkResult;

use super::{
    xmlutil::{get_number_from_root, get_text_from_root},
    ReceivedMessageParser,
};

pub struct VideoMessage {
    pub msg_id: u64,
    pub thumb_media_id: String,
    pub media_id: String,
}

impl ReceivedMessageParser for VideoMessage {
    type ReceivedMessage = VideoMessage;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
        let msg_id = get_number_from_root::<u64>(node, "MsgId")?;
        let media_id = get_text_from_root(node, "MediaId")?;
        let thumb_media_id = get_text_from_root(node, "ThumbMediaId")?;

        Ok(VideoMessage {
            msg_id,
            media_id: media_id.to_owned(),
            thumb_media_id: thumb_media_id.to_owned(),
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
    <MsgType><![CDATA[video]]></MsgType>
    <MediaId><![CDATA[media_id]]></MediaId>
    <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>
    <MsgId>1234567890123456</MsgId>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = VideoMessage::from_xml(&node.root())?;
    assert_eq!(msg.media_id, "media_id");
    assert_eq!(msg.thumb_media_id, "thumb_media_id");
    assert_eq!(msg.msg_id, 1234567890123456);
    Ok(())
}
