use roxmltree::{Document, Node};

use crate::{SdkResult, error::SdkError};

use msg_text::TextMessage;
use msg_image::ImageMessage;

use self::{msg_location::LocationMessage, msg_video::VideoMessage, msg_voice::VoiceMessage};

pub mod signature;
pub mod xmlutil;
pub mod crypto;
pub mod msg_text;
pub mod msg_image;
pub mod msg_voice;
pub mod msg_video;
pub mod msg_location;

const MSG_TEXT: &'static str = "text";
const MSG_IMAGE: &'static str = "image";
const MSG_VOICE: &'static str = "voice";
const MSG_VIDEO: &'static str = "video";
const MSG_SHORTVIDEO: &'static str = "shortvideo";
const MSG_LOCATION: &'static str = "location"; // 地理位置消息
const MSG_LINK: &'static str = "link";
const MSG_EVENT: &'static str = "event";

const EVENT_SUBSCRIBE: &'static str = "subscribe";
const EVENT_SCAN: &'static str = "SCAN";
const EVENT_LOCATION: &'static str = "LOCATION"; // 地理位置事件
const EVENT_CLICK: &'static str = "CLICK";
const EVENT_VIEW: &'static str = "VIEW";
const EVENT_SCANCODE_PUSH: &'static str = "scancode_push";
const EVENT_SCANCODE_WAITMSG: &'static str = "scancode_waitmsg";
const EVENT_PIC_SYSPHOTO: &'static str = "pic_sysphoto";
const EVENT_PIC_PHOTO_OR_ALBUM: &'static str = "pic_photo_or_album";
const EVENT_PIC_WEIXIN: &'static str = "pic_weixin";
const EVENT_LOCATION_SELECT: &'static str = "location_select";
const EVENT_VIEW_MINIPROGRAM: &'static str = "view_miniprogram";
const EVENT_TEMPLATESENDJOBFINISH: &'static str = "TEMPLATESENDJOBFINISH";
const EVENT_MASSSENDJOBFINISH: &'static str = "MASSSENDJOBFINISH";
const EVENT_GUIDE_INVITE_RESULT: &'static str = "guide_invite_result_event";
const EVENT_GUIDE_QRCODE_SCAN: &'static str = "guide_qrcode_scan_event";

pub trait EventMessage {
    type ReceivedMessage;
    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage>;
}

pub struct ReceivedEvent {
    pub from: String,

    pub to: String,

    pub create_time: u64,

    pub msg_type: String,

    pub body: ReceivedMessage,
}

pub enum ReceivedMessage {
    UnhandledEvent(String),
    Text(TextMessage),
    Image(ImageMessage),
    Voice(VoiceMessage),
    Video(VideoMessage),
    ShortVideo(VideoMessage),
    Location(LocationMessage)
}

impl ReceivedEvent {
    pub fn parse(input: &str) -> SdkResult<Self> {
        let xml = Document::parse(input)?;
        let root = xml.root();
        let msg_type = root.descendants().find(|n| n.has_tag_name("MsgType")).map(|n| n.text()).flatten().ok_or_else(|| SdkError::InvalidParams("parse xml need `MsgType` params".to_owned()))?;
        let from = root.descendants().find(|n| n.has_tag_name("FromUserName")).map(|n| n.text()).flatten().ok_or_else(|| SdkError::InvalidParams("parse xml need `FromUserName` params".to_owned()))?;
        let to = root.descendants().find(|n| n.has_tag_name("ToUserName")).map(|n| n.text()).flatten().ok_or_else(|| SdkError::InvalidParams("parse xml need `ToUserName` params".to_owned()))?;
        let create_time = root.descendants().find(|n| n.has_tag_name("CreateTime")).map(|n| n.text()).flatten().ok_or_else(|| SdkError::InvalidParams("parse xml need CreateTime params".to_owned()))?;
        let create_time = create_time.parse::<u64>().map_err(|_e| SdkError::InvalidParams(
                                "Parse XML msg from wechat error: tag `CreateTime` should be number"
                                    .to_string()))?;
        
        let body = match msg_type {
            MSG_TEXT => {
                ReceivedMessage::Text(TextMessage::from_xml(&root)?)
            },
            MSG_IMAGE => {
                ReceivedMessage::Image(ImageMessage::from_xml(&root)?)
            }
            MSG_VOICE => {
                ReceivedMessage::Voice(VoiceMessage::from_xml(&root)?)
            }
            MSG_VIDEO | MSG_SHORTVIDEO => {
                let msg = if msg_type == MSG_VIDEO {
                    ReceivedMessage::Video(VideoMessage::from_xml(&root)?)
                } else {
                    ReceivedMessage::ShortVideo(VideoMessage::from_xml(&root)?)
                };
                msg
            }
            MSG_LOCATION => {
                ReceivedMessage::Location(LocationMessage::from_xml(&root)?)
            }
            _ => {
                ReceivedMessage::UnhandledEvent(
                        format!("Havent' handle for this event type `{}`", msg_type),
                )
            }
        };
        Ok(ReceivedEvent {
                    from: from.to_owned(),
                    to: to.to_owned(),
                    msg_type: msg_type.to_owned(),
                    create_time,
                    body: body
        })
    }
}