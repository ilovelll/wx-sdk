use futures::stream::Scan;
// use quick_xml::{events::Event, Reader};
use roxmltree::{Document, Node};
use serde::{Deserialize, Serialize};

use crate::{error::SdkError, SdkResult};

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

pub mod signature {
    use serde_derive::Deserialize;
    use sha1::{Digest, Sha1};

    use crate::{access_token::AccessTokenProvider, wechat::WxSdk, AccessToken};

    #[derive(Debug, Deserialize)]
    pub struct Signature {
        pub signature: String,
        pub timestamp: String,
        pub nonce: String,
        pub echostr: Option<String>,
    }

    impl Signature {
        pub fn check_signature(&self, token: impl AsRef<str>) -> bool {
            let mut arr: [String; 3] = [token.as_ref().to_owned(), self.timestamp.clone(), self.nonce.clone()];
            arr.sort();
            let str = arr.join("");

            let mut hasher = Sha1::new();
            hasher.update(str);
            let result = hasher.finalize();
            format!("{:x}", result) == self.signature
        }
    }
}

#[derive(Debug)]
pub struct ReceivedEvent {
    from: String,

    to: String,

    create_time: i64,

    msg_type: String,

    pub body: ReceivedMessage,
}

impl ReceivedEvent {
    pub fn new(
        from: &str,
        to: &str,
        create_time: i64,
        msg_type: &str,
        body: ReceivedMessage,
    ) -> Self {
        ReceivedEvent {
            from: from.to_owned(),
            to: to.to_owned(),
            create_time,
            msg_type: msg_type.to_owned(),
            body,
        }
    }

    pub fn parse(input: &str) -> SdkResult<Self> {
        let xml = Document::parse(input)?;
        let root = xml.root();
        let mt = get_node_by_tag(&root, "MsgType")?;
        let from = get_node_by_tag(&root, "FromUserName")?;
        let to = get_node_by_tag(&root, "ToUserName")?;

        let from = get_text_from_node(&from)?;
        let to = get_text_from_node(&to)?;

        let create_time = get_node_by_tag(&root, "CreateTime")?;
        let create_time = get_text_from_node(&create_time)?;
        let create_time = create_time.parse::<i64>().map_err(|_x| {
            SdkError::ParmasInvalid(
                "Parse XML msg from wechat error: tag `CreateTime` should be number".to_string(),
            )
        })?;
        {
            if let Some(t) = mt.text() {
                match t {
                    MSG_TEXT => {
                        let content = get_node_by_tag(&root, "Content")?;
                        let content = get_text_from_node(&content)?;

                        let msgid = get_node_by_tag(&root, "MsgId")?;
                        let msgid = get_text_from_node(&msgid)?;
                        let msgid = msgid.parse::<u64>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string(),
                            )
                        })?;
                        let r = ReceivedEvent::new(
                            from,
                            to,
                            create_time,
                            t,
                            ReceivedMessage::Text(TextMessage {
                                id: msgid,
                                content: content.to_string(),
                            }),
                        );
                        return Ok(r);
                    }
                    MSG_IMAGE => {
                        let msgid = get_node_by_tag(&root, "MsgId")?;
                        let msgid = get_text_from_node(&msgid)?;
                        let msgid = msgid.parse::<u64>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string(),
                            )
                        })?;

                        let pic_url = get_node_by_tag(&root, "PicUrl")?;
                        let pic_url = get_text_from_node(&pic_url)?;

                        let media_id = get_node_by_tag(&root, "MediaId")?;
                        let media_id = get_text_from_node(&media_id)?;

                        return Ok(ReceivedEvent::new(
                            from,
                            to,
                            create_time,
                            t,
                            ReceivedMessage::Image(ImageMessage {
                                id: msgid,
                                pic_url: pic_url.to_string(),
                                media_id: media_id.to_string(),
                            }),
                        ));
                    }
                    MSG_VOICE => {
                        let msgid = get_node_by_tag(&root, "MsgId")?;
                        let msgid = get_text_from_node(&msgid)?;
                        let msgid = msgid.parse::<u64>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string(),
                            )
                        })?;

                        let media_id = get_node_by_tag(&root, "MediaId")?;
                        let media_id = get_text_from_node(&media_id)?;

                        let format = get_node_by_tag(&root, "Format")?;
                        let format = get_text_from_node(&format)?;

                        let recognition =
                            root.descendants().find(|n| n.has_tag_name("Recognition"));
                        let recognition = recognition
                            .map(|n| n.text())
                            .map(|t| t.map(|s| s.to_string()))
                            .unwrap_or_else(|| None);
                        return Ok(ReceivedEvent::new(
                            from,
                            to,
                            create_time,
                            t,
                            ReceivedMessage::Voice(VoiceMessage {
                                id: msgid,
                                format: format.to_string(),
                                media_id: media_id.to_string(),
                                recognition: recognition,
                            }),
                        ));
                    }
                    MSG_VIDEO | MSG_SHORTVIDEO => {
                        let msgid = get_node_by_tag(&root, "MsgId")?;
                        let msgid = get_text_from_node(&msgid)?;
                        let msgid = msgid.parse::<u64>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string(),
                            )
                        })?;

                        let media_id = get_node_by_tag(&root, "MediaId")?;
                        let media_id = get_text_from_node(&media_id)?;
                        let thumb = get_node_by_tag(&root, "ThumbMediaId")?;
                        let thumb = get_text_from_node(&thumb)?;
                        let vm = ViedoMessage {
                            id: msgid,
                            thumb_media_id: thumb.to_string(),
                            media_id: media_id.to_string(),
                        };
                        let body = if t == MSG_VIDEO {
                            ReceivedMessage::Video(vm)
                        } else {
                            ReceivedMessage::ShortVideo(vm)
                        };
                        return Ok(ReceivedEvent::new(from, to, create_time, t, body));
                    }
                    MSG_LOCATION => {
                        let msgid = get_node_by_tag(&root, "MsgId")?;
                        let msgid = get_text_from_node(&msgid)?;
                        let msgid = msgid.parse::<u64>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string(),
                            )
                        })?;

                        let location_x = get_node_by_tag(&root, "Location_X")?;
                        let location_x = get_text_from_node(&location_x)?;
                        let location_x = location_x.parse::<f32>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `location_x` should be number"
                                    .to_string(),
                            )
                        })?;
                        let location_y = get_node_by_tag(&root, "Location_Y")?;
                        let location_y = get_text_from_node(&location_y)?;
                        let location_y = location_y.parse::<f32>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `location_y` should be number"
                                    .to_string(),
                            )
                        })?; 

                        let scale = get_node_by_tag(&root, "Scale")?;
                        let scale = get_text_from_node(&scale)?;
                        let scale = scale.parse::<i32>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `scale` should be number"
                                    .to_string(),
                            )
                        })?;
                        let label = get_node_by_tag(&root, "Label")?;
                        let label = get_text_from_node(&label)?; 
                        return Ok(ReceivedEvent::new(from, to, create_time, t, ReceivedMessage::Location(LocationMessage {
                            id: msgid,
                            location_x,
                            location_y,
                            scale,
                            label: label.to_string(),
                        })));
                    }
                    MSG_LINK => {
                        let msgid = get_node_by_tag(&root, "MsgId")?;
                        let msgid = get_text_from_node(&msgid)?;
                        let msgid = msgid.parse::<u64>().map_err(|_x| {
                            SdkError::ParmasInvalid(
                                "Parse XML msg from wechat error: tag `MsgId` should be number"
                                    .to_string(),
                            )
                        })?;

                        let title = get_node_by_tag(&root, "Title")?;
                        let title = get_text_from_node(&title)?; 
                        let description = get_node_by_tag(&root, "Description")?;
                        let description = get_text_from_node(&description)?; 
                        let url = get_node_by_tag(&root, "Url")?;
                        let url = get_text_from_node(&url)?; 
                        return Ok(ReceivedEvent::new(from, to, create_time, t, ReceivedMessage::Link(LinkMessage {
                            id: msgid,
                            title: title.to_string(),
                            description: description.to_string(),
                            url: url.to_string(),
                        })));
                    }
                    MSG_EVENT => {
                        let et = get_node_by_tag(&root, "Event")?;
                        if let Some(t) = et.text() {
                            match t {
                                EVENT_SUBSCRIBE => {
                                    let ekn =
                                        root.descendants().find(|n| n.has_tag_name("EventKey"));
                                    match ekn {
                                        Some(n) => {
                                            let ek = get_text_from_node(&n)?;
                                            // let ek = ek.trim_start_matches("qrscene_");
                                            let ticket = get_node_by_tag(&root, "Ticket")?;
                                            let ticket = get_text_from_node(&ticket)?;
                                            let r = ReceivedEvent::new(
                                                from,
                                                to,
                                                create_time,
                                                t,
                                                ReceivedMessage::EventPush(
                                                    EventPush::SubscribeScan(ScanEvent {
                                                        event_key: ek.to_string(),
                                                        ticket: ticket.to_string(),
                                                    }),
                                                ),
                                            );

                                            return Ok(r);
                                        }
                                        None => {
                                            let r = ReceivedEvent::new(
                                                from,
                                                to,
                                                create_time,
                                                t,
                                                ReceivedMessage::EventPush(EventPush::Subscribe),
                                            );
                                            return Ok(r);
                                        }
                                    }
                                }
                                EVENT_SCAN => {
                                    let ek = get_node_by_tag(&root, "EventKey")?;
                                    let ek = get_text_from_node(&ek)?;
                                    // let ek = ek.trim_start_matches("qrscene_");
                                    let ticket = get_node_by_tag(&root, "Ticket")?;
                                    let ticket = get_text_from_node(&ticket)?;
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(EventPush::Scan(ScanEvent {
                                            event_key: ek.to_string(),
                                            ticket: ticket.to_string(),
                                        })),
                                    );
                                    return Ok(r);
                                }
                                EVENT_LOCATION => {
                                    let latitude = get_node_by_tag(&root, "Latitude")?;
                                    let latitude = get_text_from_node(&latitude)?;
                                    let latitude = latitude.parse::<f32>().map_err(|_x| {
                                        SdkError::ParmasInvalid(
                                            "Parse XML msg from wechat error: tag `latitude` should be number"
                                                .to_string(),
                                        )
                                    })?;
                                    let longitude = get_node_by_tag(&root, "Longitude")?;
                                    let longitude = get_text_from_node(&longitude)?;
                                    let longitude = longitude.parse::<f32>().map_err(|_x| {
                                        SdkError::ParmasInvalid(
                                            "Parse XML msg from wechat error: tag `longitude` should be number"
                                                .to_string(),
                                        )
                                    })?;
                                    let precision = get_node_by_tag(&root, "Precision")?;
                                    let precision = get_text_from_node(&precision)?;
                                    let precision = precision.parse::<f32>().map_err(|_x| {
                                        SdkError::ParmasInvalid(
                                            "Parse XML msg from wechat error: tag `longitude` should be number"
                                                .to_string(),
                                        )
                                    })?;
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(EventPush::Location(LocationEvent {
                                            latitude, longitude, precision
                                        })),
                                    );
                                    return Ok(r);
                                },
                                EVENT_CLICK => {
                                    let ek = get_node_by_tag(&root, "EventKey")?;
                                    let ek = get_text_from_node(&ek)?;
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(EventPush::Click(ClickEvent {
                                            event_key: ek.to_string(),
                                        })),
                                    );
                                    return Ok(r);
                                },
                                EVENT_VIEW => {
                                    let ek = get_node_by_tag(&root, "EventKey")?;
                                    let ek = get_text_from_node(&ek)?;
                                    let menuid = get_node_by_tag(&root, "MenuID");
                                    
                                    let mut event = ViewEvent {
                                            event_key: ek.to_string(),
                                            menu_id: None
                                    };
                                    if let Ok(id) = menuid {
                                        let sid = get_text_from_node(&id)?;
                                        event.menu_id = Some(sid.to_owned());
                                    }
                                    
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(EventPush::View(event)),
                                    );
                                    return Ok(r);                                    
                                }
                                EVENT_SCANCODE_PUSH | EVENT_SCANCODE_WAITMSG => {
                                    let ek = get_node_by_tag(&root, "EventKey")?;
                                    let ek = get_text_from_node(&ek)?;
                                    let scan_code_info = get_node_by_tag(&root, "ScanCodeInfo")?;
                                    let scan_type = get_node_by_tag(&scan_code_info, "ScanType")?;
                                    let scan_type = get_text_from_node(&scan_type)?;
                                    let scan_result = get_node_by_tag(&scan_code_info, "ScanResult")?;
                                    let scan_result = get_text_from_node(&scan_result)?;
                                    let event = ScanCodeEvent{
                                            event_key: ek.to_owned(),
                                            scan_type: scan_type.to_owned(),
                                            scan_result: scan_result.to_owned(),
                                    };
                                    let event = if t == EVENT_SCANCODE_PUSH {
                                        EventPush::ScanCodePush(event)
                                    } else {
                                        EventPush::ScanCodeWaitMsg(event)
                                    };
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(event),
                                    );
                                    return Ok(r);
                                }
                                EVENT_PIC_SYSPHOTO |EVENT_PIC_PHOTO_OR_ALBUM |EVENT_PIC_WEIXIN => {
                                    let ek = get_node_by_tag(&root, "EventKey")?;
                                    let ek = get_text_from_node(&ek)?;
                                    let send_pics_info = get_node_by_tag(&root, "SendPicsInfo")?;
                                    let count = get_node_by_tag(&send_pics_info, "Count")?;
                                    let count = get_text_from_node(&count)?;
                                    let count = count.parse::<u16>().map_err(|_x| {
                                        SdkError::ParmasInvalid(
                                            "Parse XML msg from wechat error: tag `count` should be number"
                                                .to_string(),
                                        )
                                    })?;
                                    let pic_list = get_node_by_tag(&send_pics_info, "PicList")?;
                                    let md5_vec: Vec<PicMd5Sum> = pic_list.descendants()
                                        .filter(|n| n.has_tag_name("PicMd5Sum"))
                                        .filter_map(|n| get_text_from_node(&n).ok())
                                        .map(|s| PicMd5Sum(s.to_owned()))
                                        .collect();
                                    let event = SendPicsInfo {
                                        event_key: ek.to_owned(),
                                        count,
                                        pic_list: md5_vec,
                                    };
                                    let event = match t {
                                        EVENT_PIC_SYSPHOTO => EventPush::PicSysPhoto(event),
                                        EVENT_PIC_PHOTO_OR_ALBUM => EventPush::PicPhotoOrAlbum(event),
                                        EVENT_PIC_WEIXIN => EventPush::PicWeixin(event),
                                        _ => unreachable!()
                                    };
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(event),
                                    );
                                    return Ok(r);
                                }
                                EVENT_LOCATION_SELECT => {
                                    todo!()
                                }
                                EVENT_VIEW_MINIPROGRAM => {
                                    todo!()
                                }
                                _ => {
                                    let r = ReceivedEvent::new(
                                        from,
                                        to,
                                        create_time,
                                        t,
                                        ReceivedMessage::EventPush(EventPush::UnhandledEvent(
                                            format!("Havent' handle for this event type `{}`", t),
                                        )),
                                    );
                                    return Ok(r);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        todo!()
    }
}

fn get_node_by_tag<'a, 'b>(node: &'a Node, tag_name: &'b str) -> SdkResult<Node<'a, 'a>> {
    node.descendants()
        .find(|n| n.has_tag_name(tag_name))
        .ok_or_else(|| {
            SdkError::ParmasInvalid(format!(
                "Parse XML msg from wechat error: tag `{}` invalid",
                tag_name
            ))
        })
}

fn get_text_from_node<'a>(node: &Node<'a, 'a>) -> SdkResult<&'a str> {
    node.text().ok_or_else(|| {
        SdkError::ParmasInvalid(format!(
            "Parse XML msg from wechat error: tag `{}` text content is none",
            node.tag_name().name()
        ))
    })
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ReceivedMessage {
    Text(TextMessage),
    Image(ImageMessage),
    Voice(VoiceMessage),
    Video(ViedoMessage),
    ShortVideo(ViedoMessage),
    Location(LocationMessage),
    Link(LinkMessage),
    EventPush(EventPush),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum EventPush {
    Subscribe,
    UnSubscribe,
    SubscribeScan(ScanEvent),
    Scan(ScanEvent),
    Location(LocationEvent),
    Click(ClickEvent),
    View(ViewEvent),
    ScanCodePush(ScanCodeEvent),
    ScanCodeWaitMsg(ScanCodeEvent),
    PicSysPhoto(SendPicsInfo),
    PicPhotoOrAlbum(SendPicsInfo),
    PicWeixin(SendPicsInfo),
    TemplateSendJobFinish(TemplateSendJobFinishEvent),
    MassSendJobFinish(MassSendJobFinishEvent),
    GuideInviteResult(GuideInviteResultEvent),
    GuideQrcodeScan(GuideQrcodeScanEvent),
    UnhandledEvent(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TextMessage {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "Content")]
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageMessage {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "PicUrl")]
    pic_url: String,
    #[serde(alias = "MediaId")]
    media_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VoiceMessage {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "Format")]
    format: String,
    #[serde(alias = "MediaId")]
    media_id: String,
    #[serde(alias = "Recognition")]
    recognition: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViedoMessage {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "ThumbMediaId")]
    thumb_media_id: String,
    #[serde(alias = "MediaId")]
    media_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationMessage {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "Location_X")]
    location_x: f32,
    #[serde(alias = "Location_Y")]
    location_y: f32,
    #[serde(alias = "Scale")]
    scale: i32,
    #[serde(alias = "Label")]
    label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinkMessage {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgId")]
    id: u64,
    #[serde(alias = "Title")]
    title: String,
    #[serde(alias = "Description")]
    description: String,
    #[serde(alias = "Url")]
    url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ScanEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "EventKey")]
    event_key: String,
    #[serde(alias = "Ticket")]
    ticket: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "Latitude")]
    latitude: f32,
    #[serde(alias = "Longitude")]
    longitude: f32,
    #[serde(alias = "Precision")]
    precision: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ClickEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "EventKey")]
    event_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ViewEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "EventKey")]
    event_key: String,
    #[serde(alias = "MenuId")]
    menu_id: Option<String>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct ScanCodeEvent {
    event_key: String,
    scan_type: String,
    scan_result: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct PicMd5Sum(String);

#[derive(Serialize, Deserialize, Debug)]
pub struct SendPicsInfo {
    event_key: String,
    count: u16,
    pic_list: Vec<PicMd5Sum>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MassSendJobFinishEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgID")]
    id: u64,
    #[serde(alias = "Status")]
    status: String,
    #[serde(alias = "TotalCount")]
    total_count: u64,
    #[serde(alias = "FilterCount")]
    filter_count: u64,
    #[serde(alias = "SentCount")]
    sent_count: u64,
    #[serde(alias = "ErrorCount")]
    error_count: u64,
    #[serde(alias = "CopyrightCheckResult")]
    copyright_check_result: CopyrightCheckResult,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyrightCheckResult {
    #[serde(alias = "Count")]
    count: u16,
    #[serde(alias = "CheckState")]
    check_state: u8,
    #[serde(alias = "ResultList")]
    result_list: Vec<CopyrightCheckResultItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CopyrightCheckResultItem {
    pub article_idx: i8,              //群发文章的序号，从1开始
    pub user_declare_state: i8,       //用户声明文章的状态
    pub audit_state: i8,              //系统校验的状态,
    pub original_article_url: String, //相似原创文的url
    pub original_article_type: i8,    //相似原创文的类型
    pub can_reprint: i8,              //是否能转载
    pub need_replace_content: i8,     //是否需要替换成原创文内容
    pub need_show_reprint_source: i8, //是否需要注明转载来
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TemplateSendJobFinishEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    #[serde(alias = "MsgID")]
    id: u64,
    #[serde(alias = "Status")]
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuideInviteResultEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    guide_account: String,
    guide_openid: String,
    invite_result: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GuideQrcodeScanEvent {
    // #[serde(alias = "FromUserName")]
    // from: String,
    // #[serde(alias = "ToUserName")]
    // to: String,
    // #[serde(alias = "CreateTime")]
    // timestamp: i64,
    qrcode_guide_account: String,
    qrcode_guide_openid: String,
    openid: String,
    action: u8,
    qrcode_info: String,
}

#[cfg(test)]
mod tests {

    use std::convert::Infallible;

    use super::*;
    // use quick_xml::de::{from_str, DeError};
    use roxmltree::Document;

    #[test]
    fn feature() {
        let data = "
         <xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[fromUser]]></FromUserName>
            <CreateTime>1348831860</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[this is a test]]></Content>
            <MsgId>1234567890123456</MsgId>
        </xml>";
        let rect = ReceivedEvent::parse(data).unwrap();
        // assert_eq!(ReceivedEvent {
        //    to:  "toUser".to_owned(),
        //    from: "fromUser".to_string(),
        //    msg_type: "text".to_string(),
        //    create_time: 1348831860,
        //    body: ReceivedMessage::Text(TextMessage{content: "".to_owned(), id: 212312})
        // }, rect);
        println!("aa  {:?}", rect);
    }
    #[test]
    fn test_text() -> Result<(), Box<dyn std::error::Error>> {
        let text = "
        <xml>
            <ToUserName><![CDATA[toUser]]></ToUserName>
            <FromUserName><![CDATA[fromUser]]></FromUserName>
            <CreateTime>1348831860</CreateTime>
            <MsgType><![CDATA[text]]></MsgType>
            <Content><![CDATA[this is a test]]></Content>
            <MsgId>1234567890123456</MsgId>
        </xml>";
        // let text = "
        // <xml>
        //     <ToUserName>toUser</ToUserName>
        //     <FromUserName>fromUser</FromUserName>
        //     <CreateTime>1348831860</CreateTime>
        //     <MsgType>text</MsgType>
        //     <Content>this is a test</Content>
        //     <MsgId>1234567890123456</MsgId>
        // </xml>";
        // parse_xml(text);
        let text = "<xml> 
  <ToUserName><![CDATA[gh_4d00ed8d6399]]></ToUserName>  
  <FromUserName><![CDATA[oV5CrjpxgaGXNHIQigzNlgLTnwic]]></FromUserName>  
  <CreateTime>1481013459</CreateTime>  
  <MsgType><![CDATA[event]]></MsgType>  
  <Event><![CDATA[MASSSENDJOBFINISH]]></Event>  
  <MsgID>1000001625</MsgID>  
  <Status><![CDATA[err(30003)]]></Status>  
  <TotalCount>0</TotalCount>  
  <FilterCount>0</FilterCount>  
  <SentCount>0</SentCount>  
  <ErrorCount>0</ErrorCount>  
  <CopyrightCheckResult> 
    <Count>2</Count>  
    <ResultList> 
      <item> 
        <ArticleIdx>1</ArticleIdx>  
        <UserDeclareState>0</UserDeclareState>  
        <AuditState>2</AuditState>  
        <OriginalArticleUrl><![CDATA[Url_1]]></OriginalArticleUrl>  
        <OriginalArticleType>1</OriginalArticleType>  
        <CanReprint>1</CanReprint>  
        <NeedReplaceContent>1</NeedReplaceContent>  
        <NeedShowReprintSource>1</NeedShowReprintSource> 
      </item>  
      <item> 
        <ArticleIdx>2</ArticleIdx>  
        <UserDeclareState>0</UserDeclareState>  
        <AuditState>2</AuditState>  
        <OriginalArticleUrl><![CDATA[Url_2]]></OriginalArticleUrl>  
        <OriginalArticleType>1</OriginalArticleType>  
        <CanReprint>1</CanReprint>  
        <NeedReplaceContent>1</NeedReplaceContent>  
        <NeedShowReprintSource>1</NeedShowReprintSource> 
      </item> 
    </ResultList>  
    <CheckState>2</CheckState> 
  </CopyrightCheckResult> 
  <ArticleUrlResult>
     <Count>1</Count>
     <ResultList>
       <item>
         <ArticleIdx>1</ArticleIdx>
         <ArticleUrl><![CDATA[Url]]></ArticleUrl>
       </item>
     </ResultList>
  </ArticleUrlResult>
</xml>
";
        // parse_xml(text);
        // let r_text: ReceivedEvent = from_str(text)?;
        // assert_eq!("text", r_text.get_type());

        // let image = "
        // <xml>
        //     <ToUserName><![CDATA[toUser]]></ToUserName>
        //     <FromUserName><![CDATA[fromUser]]></FromUserName>
        //     <CreateTime>1348831860</CreateTime>
        //     <MsgType><![CDATA[image]]></MsgType>
        //     <PicUrl><![CDATA[this is a url]]></PicUrl>
        //     <MediaId><![CDATA[media_id]]></MediaId>
        //     <MsgId>1234567890123456</MsgId>
        // </xml>";
        // let r_image: ReceivedEvent = from_str(image)?;
        // assert_eq!("image", r_image.get_type());

        // let voice = "<xml>
        //     <ToUserName><![CDATA[toUser]]></ToUserName>
        //     <FromUserName><![CDATA[fromUser]]></FromUserName>
        //     <CreateTime>1357290913</CreateTime>
        //     <MsgType><![CDATA[voice]]></MsgType>
        //     <MediaId><![CDATA[media_id]]></MediaId>
        //     <Format><![CDATA[Format]]></Format>
        //     <MsgId>1234567890123456</MsgId>
        // </xml>";
        // let r_voice: ReceivedEvent = from_str(voice)?;
        // assert_eq!("voice", r_voice.get_type());
        Ok(())
    }
}
