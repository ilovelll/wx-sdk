use roxmltree::Node;

use crate::SdkResult;

use self::{
    event_click::ClickEvent,
    event_guide::{GuideInviteResultEvent, GuideQrcodeScanEvent},
    event_location::LocationEvent,
    event_publishjob::PublishJobFinishEvent,
    event_scan::{MenuScanEvent, ScanEvent},
    event_send::{SendLocationEvent, SendPicsEvent},
    event_sendjob::{MassSendJobFinishEvent, TemplateSendJobFinishEvent},
    event_subscribe::SubScribeEvent,
    event_view::ViewEvent,
};

use super::{xmlutil::get_text_from_root, ReceivedMessageParser};

pub mod event_click;
pub mod event_guide;
pub mod event_location;
pub mod event_publishjob;
pub mod event_scan;
pub mod event_send;
pub mod event_sendjob;
pub mod event_subscribe;
pub mod event_view;

const EVENT_SUBSCRIBE: &'static str = "subscribe";
const EVENT_UNSUBSCRIBE: &'static str = "unsubscribe";
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
const EVENT_PUBLISHJOBFINISH: &'static str = "PUBLISHJOBFINISH";

pub enum EventMessage {
    Subscribe,
    UnSubscribe,
    SubscribeScan(ScanEvent),
    Scan(ScanEvent),
    Location(LocationEvent),
    Click(ClickEvent),
    View(ViewEvent),
    ViewMiniProgram(ViewEvent),
    ScanCodePush(MenuScanEvent),
    ScanCodeWaitMsg(MenuScanEvent),
    PicSysPhoto(SendPicsEvent),
    PicPhotoOrAlbum(SendPicsEvent),
    PicWeixin(SendPicsEvent),
    LocationSelect(SendLocationEvent),
    TemplateSendJobFinish(TemplateSendJobFinishEvent),
    MassSendJobFinish(MassSendJobFinishEvent),
    GuideInviteResult(GuideInviteResultEvent),
    GuideQrcodeScan(GuideQrcodeScanEvent),
    PublishJobFinish(PublishJobFinishEvent),
    UnhandledEvent(String),
}

impl ReceivedMessageParser for EventMessage {
    type ReceivedMessage = Self;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
        let event_type = get_text_from_root(node, "Event")?;
        let event = match event_type {
            EVENT_SUBSCRIBE => SubScribeEvent::from_xml(node)?,
            EVENT_UNSUBSCRIBE => EventMessage::UnSubscribe,
            EVENT_SCAN => EventMessage::Scan(ScanEvent::from_xml(node)?),
            EVENT_LOCATION => EventMessage::Location(LocationEvent::from_xml(node)?),
            EVENT_CLICK => EventMessage::Click(ClickEvent::from_xml(node)?),
            EVENT_VIEW => EventMessage::View(ViewEvent::from_xml(node)?),
            EVENT_VIEW_MINIPROGRAM => EventMessage::ViewMiniProgram(ViewEvent::from_xml(node)?),
            EVENT_SCANCODE_PUSH => EventMessage::ScanCodePush(MenuScanEvent::from_xml(node)?),
            EVENT_SCANCODE_WAITMSG => EventMessage::ScanCodeWaitMsg(MenuScanEvent::from_xml(node)?),
            EVENT_PIC_SYSPHOTO => EventMessage::PicSysPhoto(SendPicsEvent::from_xml(node)?),
            EVENT_PIC_PHOTO_OR_ALBUM => {
                EventMessage::PicPhotoOrAlbum(SendPicsEvent::from_xml(node)?)
            }
            EVENT_PIC_WEIXIN => EventMessage::PicWeixin(SendPicsEvent::from_xml(node)?),
            EVENT_LOCATION_SELECT => {
                EventMessage::LocationSelect(SendLocationEvent::from_xml(node)?)
            }
            EVENT_MASSSENDJOBFINISH => {
                EventMessage::MassSendJobFinish(MassSendJobFinishEvent::from_xml(node)?)
            }
            EVENT_TEMPLATESENDJOBFINISH => {
                EventMessage::TemplateSendJobFinish(TemplateSendJobFinishEvent::from_xml(node)?)
            }
            EVENT_GUIDE_INVITE_RESULT => {
                EventMessage::GuideInviteResult(GuideInviteResultEvent::from_xml(node)?)
            }
            EVENT_GUIDE_QRCODE_SCAN => {
                EventMessage::GuideQrcodeScan(GuideQrcodeScanEvent::from_xml(node)?)
            }
            EVENT_PUBLISHJOBFINISH => {
                EventMessage::PublishJobFinish(PublishJobFinishEvent::from_xml(node)?)
            }
            _ => EventMessage::UnhandledEvent(format!("unhandle this event type: {}", event_type)),
        };
        Ok(event)
    }
}

impl EventMessage {
    pub fn get_event_type(&self) -> &'static str {
        match self {
            EventMessage::Subscribe => EVENT_SUBSCRIBE,
            EventMessage::UnSubscribe => EVENT_UNSUBSCRIBE,
            EventMessage::SubscribeScan(_) => EVENT_SUBSCRIBE,
            EventMessage::Scan(_) => EVENT_SCAN,
            EventMessage::Location(_) => EVENT_LOCATION,
            EventMessage::Click(_) => EVENT_CLICK,
            EventMessage::View(_) => EVENT_VIEW,
            EventMessage::ViewMiniProgram(_) => EVENT_VIEW_MINIPROGRAM,
            EventMessage::ScanCodePush(_) => EVENT_SCANCODE_PUSH,
            EventMessage::ScanCodeWaitMsg(_) => EVENT_SCANCODE_WAITMSG,
            EventMessage::PicSysPhoto(_) => EVENT_PIC_SYSPHOTO,
            EventMessage::PicPhotoOrAlbum(_) => EVENT_PIC_PHOTO_OR_ALBUM,
            EventMessage::PicWeixin(_) => EVENT_PIC_WEIXIN,
            EventMessage::LocationSelect(_) => EVENT_LOCATION_SELECT,
            EventMessage::TemplateSendJobFinish(_) => EVENT_TEMPLATESENDJOBFINISH,
            EventMessage::MassSendJobFinish(_) => EVENT_MASSSENDJOBFINISH,
            EventMessage::GuideInviteResult(_) => EVENT_GUIDE_INVITE_RESULT,
            EventMessage::GuideQrcodeScan(_) => EVENT_GUIDE_QRCODE_SCAN,
            EventMessage::PublishJobFinish(_) => EVENT_PUBLISHJOBFINISH,
            EventMessage::UnhandledEvent(_) => "UnhandledEvent",
        }
    }
}
