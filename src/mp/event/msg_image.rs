use roxmltree::Node;

use crate::{SdkResult, error::SdkError};

use super::{ReceivedMessageParser, xmlutil::{get_number_from_root, get_text_from_root}};


pub struct ImageMessage {
    pub msg_id: u64,
    pub pic_url: String,
    pub media_id: String,
}

impl ReceivedMessageParser for ImageMessage {
    type ReceivedMessage = Self;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
	    let msg_id = get_number_from_root::<u64>(&node, "MsgId")?;
        let pic_url = get_text_from_root(&node, "PicUrl")?;
        let media_id = get_text_from_root(&node, "MediaId")?;
        
        Ok(ImageMessage {
            msg_id,
            pic_url: pic_url.to_owned(),
            media_id: media_id.to_owned(),
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
    <MsgType><![CDATA[image]]></MsgType>
    <PicUrl><![CDATA[this is a url]]></PicUrl>
    <MediaId><![CDATA[media_id]]></MediaId>
    <MsgId>1234567890123456</MsgId>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = ImageMessage::from_xml(&node.root())?;
    assert_eq!(msg.pic_url, "this is a url");
    assert_eq!(msg.media_id, "media_id");
    assert_eq!(msg.msg_id, 1234567890123456);
    Ok(())
}