use roxmltree::{Document, Node};

use crate::{error::SdkError, SdkResult};

use msg_image::ImageMessage;
use msg_text::TextMessage;

use self::{
    msg_event::EventMessage, msg_link::LinkMessage, msg_location::LocationMessage,
    msg_video::VideoMessage, msg_voice::VoiceMessage,
};

pub mod base64;
pub mod crypto;
pub mod msg_event;
pub mod msg_image;
pub mod msg_link;
pub mod msg_location;
pub mod msg_text;
pub mod msg_video;
pub mod msg_voice;
pub mod signature;
pub mod xmlutil;

const MSG_TEXT: &'static str = "text";
const MSG_IMAGE: &'static str = "image";
const MSG_VOICE: &'static str = "voice";
const MSG_VIDEO: &'static str = "video";
const MSG_SHORTVIDEO: &'static str = "shortvideo";
const MSG_LOCATION: &'static str = "location"; // 地理位置消息
const MSG_LINK: &'static str = "link";
const MSG_EVENT: &'static str = "event";

pub trait ReceivedMessageParser {
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
    UnhandledMessage(String),
    Text(TextMessage),
    Image(ImageMessage),
    Voice(VoiceMessage),
    Video(VideoMessage),
    ShortVideo(VideoMessage),
    Location(LocationMessage),
    Link(LinkMessage),
    Event(EventMessage),
}

impl ReceivedEvent {
    pub fn parse(input: &str) -> SdkResult<Self> {
        let xml = Document::parse(input)?;
        let root = xml.root();
        let msg_type = root
            .descendants()
            .find(|n| n.has_tag_name("MsgType"))
            .map(|n| n.text())
            .flatten()
            .ok_or_else(|| SdkError::InvalidParams("parse xml need `MsgType` params".to_owned()))?;
        let from = root
            .descendants()
            .find(|n| n.has_tag_name("FromUserName"))
            .map(|n| n.text())
            .flatten()
            .ok_or_else(|| {
                SdkError::InvalidParams("parse xml need `FromUserName` params".to_owned())
            })?;
        let to = root
            .descendants()
            .find(|n| n.has_tag_name("ToUserName"))
            .map(|n| n.text())
            .flatten()
            .ok_or_else(|| {
                SdkError::InvalidParams("parse xml need `ToUserName` params".to_owned())
            })?;
        let create_time = root
            .descendants()
            .find(|n| n.has_tag_name("CreateTime"))
            .map(|n| n.text())
            .flatten()
            .ok_or_else(|| {
                SdkError::InvalidParams("parse xml need CreateTime params".to_owned())
            })?;
        let create_time = create_time.parse::<u64>().map_err(|_e| {
            SdkError::InvalidParams(
                "Parse XML msg from wechat error: tag `CreateTime` should be number".to_string(),
            )
        })?;

        let body = match msg_type {
            MSG_TEXT => ReceivedMessage::Text(TextMessage::from_xml(&root)?),
            MSG_IMAGE => ReceivedMessage::Image(ImageMessage::from_xml(&root)?),
            MSG_VOICE => ReceivedMessage::Voice(VoiceMessage::from_xml(&root)?),
            MSG_VIDEO | MSG_SHORTVIDEO => {
                if msg_type == MSG_VIDEO {
                    ReceivedMessage::Video(VideoMessage::from_xml(&root)?)
                } else {
                    ReceivedMessage::ShortVideo(VideoMessage::from_xml(&root)?)
                }
            }
            MSG_LOCATION => ReceivedMessage::Location(LocationMessage::from_xml(&root)?),
            MSG_LINK => ReceivedMessage::Link(LinkMessage::from_xml(&root)?),
            MSG_EVENT => ReceivedMessage::Event(EventMessage::from_xml(&root)?),
            _ => ReceivedMessage::UnhandledMessage(format!(
                "Havent' handle for this message type `{}`",
                msg_type
            )),
        };
        Ok(ReceivedEvent {
            from: from.to_owned(),
            to: to.to_owned(),
            msg_type: msg_type.to_owned(),
            create_time,
            body,
        })
    }
}
