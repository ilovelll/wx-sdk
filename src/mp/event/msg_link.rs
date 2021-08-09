use roxmltree::Node;

use crate::SdkResult;

use super::{ReceivedMessageParser, xmlutil::{get_number_from_root, get_text_from_root}};

pub struct LinkMessage {
    pub msg_id: u64,
    pub title: String,
    pub description: String,
    pub url: String,
}

impl ReceivedMessageParser for LinkMessage {
    type ReceivedMessage = LinkMessage;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
	    let msg_id = get_number_from_root::<u64>(&node, "MsgId")?;
        let title = get_text_from_root(&node, "Title")?;
        let description = get_text_from_root(&node, "Description")?;
        let url = get_text_from_root(&node, "Url")?;

        Ok(LinkMessage {
            msg_id,
	        title: title.to_owned(),
	        description: description.to_owned(),
	        url: url.to_owned(),
        })
    }
}
#[test]
pub fn parse() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1351776360</CreateTime>
    <MsgType><![CDATA[link]]></MsgType>
    <Title><![CDATA[公众平台官网链接]]></Title>
    <Description><![CDATA[公众平台官网链接]]></Description>
    <Url><![CDATA[url]]></Url>
    <MsgId>1234567890123456</MsgId>
</xml>";
    let node = Document::parse(&s)?;
    let msg = LinkMessage::from_xml(&node.root())?;
    assert_eq!(msg.title, "公众平台官网链接");
    assert_eq!(msg.url, "url");
    assert_eq!(msg.msg_id, 1234567890123456);
    Ok(())
}