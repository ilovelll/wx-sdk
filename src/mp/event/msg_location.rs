use roxmltree::Node;

use crate::SdkResult;

use super::{
    xmlutil::{get_number_from_root, get_text_from_root},
    ReceivedMessageParser,
};

pub struct LocationMessage {
    pub msg_id: u64,
    pub location_x: f32,
    pub location_y: f32,
    pub scale: f32,
    pub label: String,
}

impl ReceivedMessageParser for LocationMessage {
    type ReceivedMessage = LocationMessage;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
        let msg_id = get_number_from_root::<u64>(node, "MsgId")?;
        let location_x = get_number_from_root::<f32>(node, "Location_X")?;
        let location_y = get_number_from_root::<f32>(node, "Location_Y")?;
        let scale = get_number_from_root::<f32>(node, "Scale")?;
        let label = get_text_from_root(node, "Label")?;

        Ok(LocationMessage {
            msg_id,
            location_x,
            location_y,
            scale,
            label: label.to_owned(),
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
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1351776360</CreateTime>
  <MsgType><![CDATA[location]]></MsgType>
  <Location_X>23.134521</Location_X>
  <Location_Y>113.358803</Location_Y>
  <Scale>20</Scale>
  <Label><![CDATA[位置信息]]></Label>
  <MsgId>1234567890123456</MsgId>
</xml>";
        let node = Document::parse(&s)?;
        let msg = LocationMessage::from_xml(&node.root())?;
        assert_eq!(msg.location_x, 23.134521);
        assert_eq!(msg.scale, 20.0);
        assert_eq!(msg.msg_id, 1234567890123456);
        assert_eq!(msg.label, "位置信息");
        Ok(())
    }
}
