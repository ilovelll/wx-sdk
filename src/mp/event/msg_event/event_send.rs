use crate::{
    error::SdkError,
    mp::event::{
        xmlutil::{get_number_from_root, get_text_from_root},
        ReceivedMessageParser,
    },
    SdkResult,
};

pub struct SendPicsEvent {
    pub event_key: String,
    pub count: u16,
    pub pic_md5_sum_list: Vec<String>,
}

impl ReceivedMessageParser for SendPicsEvent {
    type ReceivedMessage = SendPicsEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let event_key = get_text_from_root(&node, "EventKey")?;
        let send_pics_info = node
            .descendants()
            .find(|n| n.has_tag_name("SendPicsInfo"))
            .ok_or_else(|| {
                SdkError::InvalidParams(format!(
                    "Parse XML msg from wechat error: tag `{}` is none",
                    "SendPicsInfo"
                ))
            })?;
        let count = get_number_from_root::<u16>(&send_pics_info, "Count")?;

        let pic_list = send_pics_info
            .children()
            .find(|n| n.has_tag_name("PicList"))
            .ok_or_else(|| {
                SdkError::InvalidParams(format!(
                    "Parse XML msg from wechat error: tag `{}` is none",
                    "PicList"
                ))
            })?;
        let md5_vec: Vec<String> = pic_list
            .descendants()
            .filter(|n| n.has_tag_name("PicMd5Sum"))
            .filter_map(|n| n.text())
            .map(|s| s.to_owned())
            .collect();
        let event = SendPicsEvent {
            event_key: event_key.to_owned(),
            count,
            pic_md5_sum_list: md5_vec,
        };
        Ok(event)
    }
}

pub struct SendLocationEvent {
    pub event_key: String,
    pub location_x: f32,
    pub location_y: f32,
    pub scale: f32,
    pub label: String,
    pub poiname: Option<String>,
}

impl ReceivedMessageParser for SendLocationEvent {
    type ReceivedMessage = SendLocationEvent;

    fn from_xml(node: &roxmltree::Node) -> SdkResult<Self::ReceivedMessage> {
        let event_key = get_text_from_root(&node, "EventKey")?;
        let send_location_info = node
            .descendants()
            .find(|n| n.has_tag_name("SendLocationInfo"))
            .ok_or_else(|| {
                SdkError::InvalidParams(format!(
                    "Parse XML msg from wechat error: tag `{}` is none",
                    "SendLocationInfo"
                ))
            })?;

        let location_x = get_number_from_root::<f32>(&send_location_info, "Location_X")?;
        let location_y = get_number_from_root::<f32>(&send_location_info, "Location_Y")?;
        let scale = get_number_from_root::<f32>(&send_location_info, "Scale")?;
        let label = get_text_from_root(&send_location_info, "Label")?;
        let poiname = get_text_from_root(&send_location_info, "Poiname")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.to_string());
        Ok(SendLocationEvent {
            event_key: event_key.to_string(),
            location_x,
            location_y,
            scale,
            label: label.to_string(),
            poiname,
        })
    }
}

#[test]
pub fn parse_send_pics() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml>
    <ToUserName><![CDATA[gh_e136c6e50636]]></ToUserName>
    <FromUserName><![CDATA[oMgHVjngRipVsoxg6TuX3vz6glDg]]></FromUserName>
    <CreateTime>1408090651</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[pic_sysphoto]]></Event>
    <EventKey><![CDATA[6]]></EventKey>
    <SendPicsInfo><Count>1</Count>
    <PicList><item><PicMd5Sum><![CDATA[1b5f7c23b5bf75682a53e7b6d163e185]]></PicMd5Sum>
    </item>
    </PicList>
    </SendPicsInfo>
  </xml>";
    let node = Document::parse(&s)?;
    let msg = SendPicsEvent::from_xml(&node.root())?;

    assert_eq!(msg.count, 1);
    assert_eq!(
        msg.pic_md5_sum_list,
        vec!["1b5f7c23b5bf75682a53e7b6d163e185".to_string()]
    );
    Ok(())
}

#[test]
pub fn parse_send_locaiton() -> SdkResult<()> {
    use roxmltree::Document;
    let s = "<xml><ToUserName><![CDATA[gh_e136c6e50636]]></ToUserName>
<FromUserName><![CDATA[oMgHVjngRipVsoxg6TuX3vz6glDg]]></FromUserName>
<CreateTime>1408091189</CreateTime>
<MsgType><![CDATA[event]]></MsgType>
<Event><![CDATA[location_select]]></Event>
<EventKey><![CDATA[6]]></EventKey>
<SendLocationInfo><Location_X><![CDATA[23]]></Location_X>
<Location_Y><![CDATA[113]]></Location_Y>
<Scale><![CDATA[15]]></Scale>
<Label><![CDATA[ 广州市海珠区客村艺苑路 106号]]></Label>
<Poiname><![CDATA[]]></Poiname>
</SendLocationInfo>
</xml>";
    let node = Document::parse(&s)?;
    let msg = SendLocationEvent::from_xml(&node.root())?;

    assert_eq!(msg.event_key, "6");
    assert_eq!(msg.location_x, 23.0);
    assert_eq!(msg.location_y, 113.0);
    assert_eq!(msg.poiname, None);
    Ok(())
}
