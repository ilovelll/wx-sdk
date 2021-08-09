use crate::{
    mp::event::{xmlutil::get_number_from_root, ReceivedMessageParser},
    SdkResult,
};

pub struct LocationEvent {
    pub latitude: f32,
    pub longitude: f32,
    pub precision: f32,
}

impl ReceivedMessageParser for LocationEvent {
    type ReceivedMessage = LocationEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let latitude = get_number_from_root::<f32>(node, "Latitude")?;
        let longitude = get_number_from_root::<f32>(node, "Longitude")?;
        let precision = get_number_from_root::<f32>(node, "Precision")?;
        Ok(LocationEvent {
            latitude,
            longitude,
            precision,
        })
    }
}

#[test]
pub fn parse() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[LOCATION]]></Event>
    <Latitude>23.137466</Latitude>
    <Longitude>113.352425</Longitude>
    <Precision>119.385040</Precision>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = LocationEvent::from_xml(&node.root())?;

    assert_eq!(msg.latitude, 23.137466);
    assert_eq!(msg.longitude, 113.352425);
    assert_eq!(msg.precision, 119.385040);
    Ok(())
}
