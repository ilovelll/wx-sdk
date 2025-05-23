use crate::{error::SdkError, SdkResult};
use wx_message; // Using the local wx-message crate

// Keep existing imports for the message structs themselves, they will be adapted
use self::msg_event::EventMessage; // Already adapted to be a struct
use self::msg_image::ImageMessage;
use self::msg_link::LinkMessage;
use self::msg_location::LocationMessage;
use self::msg_text::TextMessage;
use self::msg_video::VideoMessage; // Used for both Video and ShortVideo
use self::msg_voice::VoiceMessage;

pub mod crypto;
pub mod msg_event;
pub mod msg_image;
pub mod msg_link;
pub mod msg_location;
pub mod msg_text;
pub mod msg_video;
pub mod msg_voice;
pub mod signature;

// Constants for message types are not strictly needed for parsing anymore,
// but might be used by other logic. For now, keep them.
const MSG_TEXT: &'static str = "text";
const MSG_IMAGE: &'static str = "image";
const MSG_VOICE: &'static str = "voice";
const MSG_VIDEO: &'static str = "video";
const MSG_SHORTVIDEO: &'static str = "shortvideo";
const MSG_LOCATION: &'static str = "location";
const MSG_LINK: &'static str = "link";
const MSG_EVENT: &'static str = "event";

#[derive(Debug, Clone)]
pub struct ReceivedEvent {
    pub from: String,
    pub to: String,
    pub create_time: u64,
    pub msg_type: String, // The original MsgType string e.g. "text", "image", "event"
    pub msg_id: Option<i64>,
    pub body: ReceivedMessage,
}

#[derive(Debug, Clone)]
pub enum ReceivedMessage {
    UnhandledMessage(String), // Stores the raw_content from wx_message::Unhandled or error string
    Text(TextMessage),
    Image(ImageMessage),
    Voice(VoiceMessage),
    Video(VideoMessage), // Represents general video type
    ShortVideo(VideoMessage), // Represents short video, uses the same VideoMessage struct
    Location(LocationMessage),
    Link(LinkMessage),
    Event(EventMessage), // Uses the adapted msg_event::EventMessage struct
}

impl ReceivedEvent {
    pub fn parse(input: &str) -> SdkResult<Self> {
        let wx_msg = wx_message::parse(input)
            .map_err(|e| SdkError::InvalidParams(format!("XML parsing error by wx-message: {}", e)))?;

        let mapped_body = match wx_msg.message {
            wx_message::MessageBody::Text { content } => {
                ReceivedMessage::Text(TextMessage { content })
            }
            wx_message::MessageBody::Image { pic_url, media_id } => {
                ReceivedMessage::Image(ImageMessage { pic_url, media_id })
            }
            wx_message::MessageBody::Voice { media_id, format, recognition } => {
                ReceivedMessage::Voice(VoiceMessage { media_id, format, recognition })
            }
            wx_message::MessageBody::Video { media_id, thumb_media_id } => {
                // wx_msg.msg_type will determine if it's Video or ShortVideo
                let app_video_msg = VideoMessage { media_id, thumb_media_id };
                if wx_msg.msg_type == MSG_SHORTVIDEO {
                    ReceivedMessage::ShortVideo(app_video_msg)
                } else {
                    ReceivedMessage::Video(app_video_msg)
                }
            }
            wx_message::MessageBody::ShortVideo { media_id, thumb_media_id } => {
                 // This case should ideally be handled by the above Video block if wx-message unifies them.
                 // However, if wx_message::MessageBody explicitly has ShortVideo, we map it.
                 // Current wx-message has distinct Video and ShortVideo, so this mapping is correct.
                let app_video_msg = VideoMessage { media_id, thumb_media_id };
                ReceivedMessage::ShortVideo(app_video_msg)
            }
            wx_message::MessageBody::Location { location_x, location_y, scale, label } => {
                ReceivedMessage::Location(LocationMessage { location_x, location_y, scale, label })
            }
            wx_message::MessageBody::Link { title, description, url } => {
                ReceivedMessage::Link(LinkMessage { title, description, url })
            }
            wx_message::MessageBody::Event {
                event,
                event_key,
                ticket,
                latitude,
                longitude,
                precision,
                menu_id,
                .. // Ignore all other fields from wx_message::MessageBody::Event
            } => {
                let app_event_msg = EventMessage {
                    event,
                    event_key,
                    ticket,
                    latitude,
                    longitude,
                    precision,
                    menu_id,
                    // All other fields removed, matching the reverted EventMessage struct
                };
                ReceivedMessage::Event(app_event_msg)
            }
            wx_message::MessageBody::Unhandled { raw_content } => {
                // Pass through the raw content that couldn't be parsed by wx-message
                ReceivedMessage::UnhandledMessage(raw_content)
            }
        };

        Ok(ReceivedEvent {
            from: wx_msg.from_user_name,
            to: wx_msg.to_user_name,
            create_time: wx_msg.create_time,
            msg_type: wx_msg.msg_type,
            msg_id: wx_msg.msg_id,
            body: mapped_body,
        })
    }
}
