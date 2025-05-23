// Old enum variants and specific event structs (like ClickEvent, ScanEvent, etc.) are removed.
// Also, the old ReceivedMessageParser impl is removed.

#[derive(Debug, Clone, Default)] // Added Default
pub struct EventMessage {
    pub event: String, // e.g., "subscribe", "CLICK", "SCAN", "LOCATION", "VIEW"
    pub event_key: Option<String>,
    pub ticket: Option<String>,
    // Fields for LOCATION event
    pub latitude: Option<String>,
    pub longitude: Option<String>,
    pub precision: Option<String>,
    // Fields for menu events (e.g. view_miniprogram)
    pub menu_id: Option<String>,
    // All other detailed event fields (scan_type, pic_list, status, etc.) are removed.
}

// Constants for event types can be kept if they are used elsewhere,
// or removed if EventMessage.event (String) is used directly.
// For now, keeping them for potential reference, though they are not directly used by this struct.
pub const EVENT_SUBSCRIBE: &'static str = "subscribe";
pub const EVENT_UNSUBSCRIBE: &'static str = "unsubscribe";
pub const EVENT_SCAN: &'static str = "SCAN";
pub const EVENT_LOCATION: &'static str = "LOCATION";
pub const EVENT_CLICK: &'static str = "CLICK";
pub const EVENT_VIEW: &'static str = "VIEW";
pub const EVENT_SCANCODE_PUSH: &'static str = "scancode_push";
pub const EVENT_SCANCODE_WAITMSG: &'static str = "scancode_waitmsg";
pub const EVENT_PIC_SYSPHOTO: &'static str = "pic_sysphoto";
pub const EVENT_PIC_PHOTO_OR_ALBUM: &'static str = "pic_photo_or_album";
pub const EVENT_PIC_WEIXIN: &'static str = "pic_weixin";
pub const EVENT_LOCATION_SELECT: &'static str = "location_select";
pub const EVENT_VIEW_MINIPROGRAM: &'static str = "view_miniprogram";
pub const EVENT_TEMPLATESENDJOBFINISH: &'static str = "TEMPLATESENDJOBFINISH";
pub const EVENT_MASSSENDJOBFINISH: &'static str = "MASSSENDJOBFINISH";
pub const EVENT_GUIDE_INVITE_RESULT: &'static str = "guide_invite_result_event";
pub const EVENT_GUIDE_QRCODE_SCAN: &'static str = "guide_qrcode_scan_event";
pub const EVENT_PUBLISHJOBFINISH: &'static str = "PUBLISHJOBFINISH";
// Remove individual event struct modules if they are no longer used.
// pub mod event_click;
// pub mod event_guide;
// pub mod event_location;
// pub mod event_publishjob;
// pub mod event_scan;
// pub mod event_send;
// pub mod event_sendjob;
// pub mod event_subscribe;
// pub mod event_view;

// The EventMessage::get_event_type() method is no longer applicable in this struct form directly,
// unless we want to return self.event.as_str(). For now, it's removed.
// The old parsing logic (from_xml) is also removed as parsing is now done by wx-message.
