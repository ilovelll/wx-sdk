use quick_xml::events::Event;
use quick_xml::Reader;
use std::str;

#[derive(Debug, PartialEq)]
pub struct WxMessage {
    pub to_user_name: String,
    pub from_user_name: String,
    pub create_time: u64, 
    pub msg_type: String, 
    pub msg_id: Option<i64>, 
    pub message: MessageBody,
}

#[derive(Debug, PartialEq)]
pub enum MessageBody {
    Text { content: String },
    Image { pic_url: String, media_id: String },
    Voice { media_id: String, format: String, recognition: Option<String> },
    Video { media_id: String, thumb_media_id: String },
    ShortVideo { media_id: String, thumb_media_id: String },
    Location { location_x: String, location_y: String, scale: String, label: String },
    Link { title: String, description: String, url: String },
    Event {
        event: String, 
        event_key: Option<String>,
        ticket: Option<String>,
        latitude: Option<String>, // For LOCATION event
        longitude: Option<String>,// For LOCATION event
        precision: Option<String>,// For LOCATION event
        menu_id: Option<String>,  // For some menu events

        // Fields for scancode_push / scancode_waitmsg
        scan_type: Option<String>,
        scan_result: Option<String>,

        // Fields for pic_sysphoto / pic_photo_or_album / pic_weixin
        count: Option<String>, // from SendPicsInfo
        pic_list: Option<String>, // Placeholder for PicList content (e.g., concatenated MD5s or raw XML)

        // Fields for location_select
        location_x_select: Option<String>, // From SendLocationInfo
        location_y_select: Option<String>,
        scale_select: Option<String>,
        label_select: Option<String>,
        poiname: Option<String>,

        // Fields for view_miniprogram (app_id and page_path are usually in EventKey for older versions, but can be separate)
        // EventKey often holds path?query for miniprogram, or URL for VIEW.
        // Let's assume app_id and page_path might appear as direct tags for some hypothetical future or specific cases.
        // For now, EventKey is the primary source for miniprogram path/url.
        // No direct app_id, page_path tags are standardly documented at the same level as Event, EventKey.
        // These are usually part of a more complex structure if not in EventKey.
        // Keep menu_id for view_miniprogram as it's a common field.

        // Fields for TEMPLATESENDJOBFINISH / MASSSENDJOBFINISH
        status: Option<String>, // Job status
        new_msg_id: Option<String>, // Job's message ID (MsgID or NewMsgId)

        // Fields specific to MASSSENDJOBFINISH
        total_count: Option<String>,
        filter_count: Option<String>,
        sent_count: Option<String>,
        error_count: Option<String>,
        copyright_check_result: Option<String>, // Simplified from CopyrightCheckResult struct

        // Fields for PUBLISHJOBFINISH
        publish_id: Option<String>, // from PublishEventInfo
        publish_status: Option<String>, // from PublishEventInfo (maps to check_state)
        article_id: Option<String>, // from PublishEventInfo
        article_idx: Option<String>, // from PublishEventInfo
        fail_idx_list: Option<String>, // from PublishEventInfo (concatenated list)

        // Fields for guide_invite_result_event
        result_status: Option<String>, // from guide_invite_result_event/status

        // Fields for guide_qrcode_scan_event
        guide_account: Option<String>, // from guide_qrcode_scan_event/guide_account
        guide_qrcode_scene: Option<String>, // from guide_qrcode_scan_event/scene
    },
    Unhandled { raw_content: String }, 
}

pub fn parse(xml_input: &str) -> Result<WxMessage, String> {
    if xml_input.trim().is_empty() {
        // Consistent error message with one of the tests.
        return Err("XML parsing error: Premature end of document".to_string());
    }

    let mut reader = Reader::from_str(xml_input);
    let mut buf = Vec::new(); 
    
    let mut to_user_name = String::new();
    let mut from_user_name = String::new();
    let mut create_time: u64 = 0;
    let mut msg_type = String::new();
    let mut msg_id: Option<i64> = None;

    let mut content = String::new();
    let mut pic_url = String::new();
    let mut media_id = String::new();
    let mut format = String::new();
    let mut recognition: Option<String> = None;
    let mut thumb_media_id = String::new();
    let mut location_x = String::new();
    let mut location_y = String::new();
    let mut scale = String::new();
    let mut label = String::new();
    let mut title = String::new();
    let mut description = String::new();
    let mut url = String::new();
    
    let mut event_name = String::new();
    let mut event_key: Option<String> = None;
    let mut ticket: Option<String> = None;
    let mut latitude: Option<String> = None;
    let mut longitude: Option<String> = None;
    let mut precision: Option<String> = None;
    let mut menu_id: Option<String> = None;

    // scancode event fields
    let mut scan_type: Option<String> = None;
    let mut scan_result: Option<String> = None;
    // pic event fields
    let mut count: Option<String> = None;
    let mut pic_list_content: Option<String> = None; // Temp for PicList items
    // location_select event fields
    let mut location_x_select: Option<String> = None;
    let mut location_y_select: Option<String> = None;
    let mut scale_select: Option<String> = None;
    let mut label_select: Option<String> = None;
    let mut poiname: Option<String> = None;
    // job finish event fields
    let mut status: Option<String> = None;
    let mut new_msg_id: Option<String> = None; // For TEMPLATESENDJOBFINISH, MASSSENDJOBFINISH (MsgID tag)
    // mass send job finish event fields
    let mut total_count: Option<String> = None;
    let mut filter_count: Option<String> = None;
    let mut sent_count: Option<String> = None;
    let mut error_count: Option<String> = None;
    // CopyrightCheckResult related (simplified)
    let mut check_state_copyright: Option<String> = None; 
    // publish job finish event fields
    let mut publish_id: Option<String> = None;
    let mut publish_status: Option<String> = None;
    let mut article_id: Option<String> = None;
    let mut article_idx: Option<String> = None;
    let mut fail_idx_list_content: Option<String> = None; // Temp for fail_idx items
    // guide event fields
    let mut result_status: Option<String> = None;
    let mut guide_account: Option<String> = None;
    let mut guide_qrcode_scene: Option<String> = None;


    // State flags for parsing nested structures
    let mut in_scancode_info = false;
    let mut in_sendpics_info = false;
    let mut in_pic_list = false;
    let mut in_sendlocation_info = false;
    let mut in_copyright_check_result = false;
    let mut in_publish_event_info = false;
    let mut in_fail_idx_list = false; // For PUBLISHJOBFINISH
    // For guide events, the specific fields are nested under a tag with the event name itself
    let mut in_guide_invite_result_event_tag = false;
    let mut in_guide_qrcode_scan_event_tag = false;


    let mut current_tag_name: Option<String> = None;
    let mut xml_root_processed_correctly = false;
    let mut inside_xml_tag = false;


    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag_name_owned = str::from_utf8(e.name().as_ref())
                                        .map_err(|err| err.to_string())?
                                        .to_string();
                if !inside_xml_tag && tag_name_owned == "xml" {
                    inside_xml_tag = true;
                } else if !inside_xml_tag {
                    return Err("XML parsing error: Missing <xml> root tag or invalid structure.".to_string());
                }
                
                // Set flags for nested structures and handle empty tag initializations
                match tag_name_owned.as_str() {
                    "ScanCodeInfo" => in_scancode_info = true,
                    "SendPicsInfo" => in_sendpics_info = true,
                    "PicList" if in_sendpics_info => in_pic_list = true,
                    "SendLocationInfo" => in_sendlocation_info = true,
                    "CopyrightCheckResult" if event_name == "MASSSENDJOBFINISH" => in_copyright_check_result = true,
                    "PublishEventInfo" if event_name == "PUBLISHJOBFINISH" => in_publish_event_info = true,
                    "fail_idx_list" if in_publish_event_info => in_fail_idx_list = true,
                    "guide_invite_result_event" if event_name == "guide_invite_result_event" => in_guide_invite_result_event_tag = true,
                    "guide_qrcode_scan_event" if event_name == "guide_qrcode_scan_event" => in_guide_qrcode_scan_event_tag = true,
                    // Handle empty tag initialization for specific fields within PublishEventInfo
                    "article_id" if in_publish_event_info => article_id = Some("".to_string()),
                    "article_idx" if in_publish_event_info => article_idx = Some("".to_string()),
                    "fail_idx_list" if in_publish_event_info => fail_idx_list_content = Some("".to_string()),
                    _ => {}
                }
                current_tag_name = Some(tag_name_owned);
            }
            Ok(Event::Empty(e)) => { // Handle self-closing empty tags like <Tag/>
                if !inside_xml_tag { continue; }
                let tag_name_owned = str::from_utf8(e.name().as_ref())
                                        .map_err(|err| err.to_string())?
                                        .to_string();
                if in_publish_event_info {
                    match tag_name_owned.as_str() {
                        "article_id" => article_id = Some("".to_string()),
                        "article_idx" => article_idx = Some("".to_string()),
                        "fail_idx_list" => fail_idx_list_content = Some("".to_string()),
                        _ => {}
                    }
                }
                // After handling, current_tag_name should not be set for an empty tag as it has no content or end tag.
            }
            Ok(Event::Text(e)) => {
                if !inside_xml_tag { continue; } 
                if let Some(tag) = &current_tag_name {
                    let text_val = e.unescape()
                                    .map_err(|err| err.to_string())?
                                    .into_owned()
                                    .trim() 
                                    .to_string();
                    match tag.as_str() {
                        "ToUserName" => to_user_name = text_val,
                        "FromUserName" => from_user_name = text_val,
                        "CreateTime" => create_time = text_val.parse().map_err(|err: std::num::ParseIntError| err.to_string())?,
                        "MsgType" => msg_type = text_val,
                        "MsgId" => msg_id = Some(text_val.parse().map_err(|err: std::num::ParseIntError| err.to_string())?),
                        "Content" => content = text_val,
                        "PicUrl" => pic_url = text_val,
                        "MediaId" => media_id = text_val,
                        "Format" => format = text_val,
                        "Recognition" => recognition = Some(text_val),
                        "ThumbMediaId" => thumb_media_id = text_val,
                        
                        // Adjusted handling for Location fields to avoid unreachability for SendLocationInfo
                        "Location_X" => {
                            if in_sendlocation_info { location_x_select = Some(text_val); }
                            else { location_x = text_val; } // For non-event MessageBody::Location
                        },
                        "Location_Y" => {
                            if in_sendlocation_info { location_y_select = Some(text_val); }
                            else { location_y = text_val; } // For non-event MessageBody::Location
                        },
                        "Scale" => {
                            if in_sendlocation_info { scale_select = Some(text_val); }
                            else { scale = text_val; } // For non-event MessageBody::Location
                        },
                        "Label" => {
                            if in_sendlocation_info { label_select = Some(text_val); }
                            else { label = text_val; } // For non-event MessageBody::Location
                        },
                        "Latitude" => latitude = Some(text_val), // For LOCATION event
                        "Longitude" => longitude = Some(text_val), // For LOCATION event
                        "Precision" => precision = Some(text_val), // For LOCATION event

                        "Title" => title = text_val,
                        "Description" => description = text_val,
                        "Url" => url = text_val,
                        "Event" => event_name = text_val,
                        "EventKey" => event_key = Some(text_val),
                        "Ticket" => ticket = Some(text_val),
                        "MenuID" | "MenuId" => menu_id = Some(text_val), 

                        "ScanType" if in_scancode_info => scan_type = Some(text_val),
                        "ScanResult" if in_scancode_info => scan_result = Some(text_val),
                        "Count" if in_sendpics_info => count = Some(text_val),
                        "PicMd5Sum" if in_pic_list => { 
                            let current_list = pic_list_content.get_or_insert_with(String::new);
                            if !current_list.is_empty() { current_list.push_str(","); }
                            current_list.push_str(&text_val);
                        },
                        "Poiname" if in_sendlocation_info => poiname = Some(text_val),
                        
                        // Note: MsgId (top-level optional field) vs MsgID (event-specific job identifier)
                        // The top-level MsgId is parsed into `msg_id`. For events, if there's a <MsgID> specific to the event payload,
                        // it needs to be captured into a field like `new_msg_id`.
                        "MsgID" if msg_type == "event" && (event_name == "TEMPLATESENDJOBFINISH" || event_name == "MASSSENDJOBFINISH") => new_msg_id = Some(text_val),
                        "Status" if msg_type == "event" && (event_name == "TEMPLATESENDJOBFINISH" || event_name == "MASSSENDJOBFINISH" || in_guide_invite_result_event_tag) => {
                            if in_guide_invite_result_event_tag {
                                result_status = Some(text_val);
                            } else {
                                status = Some(text_val);
                            }
                        },

                        // MASSSENDJOBFINISH specific children
                        "TotalCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => total_count = Some(text_val),
                        "FilterCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => filter_count = Some(text_val),
                        "SentCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => sent_count = Some(text_val),
                        "ErrorCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => error_count = Some(text_val),
                        "CheckState" if in_copyright_check_result => check_state_copyright = Some(text_val),

                        // PUBLISHJOBFINISH children (nested in PublishEventInfo)
                        "publish_id" if in_publish_event_info => publish_id = Some(text_val),
                        "publish_status" if in_publish_event_info => publish_status = Some(text_val),
                        "article_id" if in_publish_event_info => article_id = Some(text_val),
                        "article_idx" if in_publish_event_info => article_idx = Some(text_val),
                        "item" if in_fail_idx_list => { // Assuming fail_idx_list contains <item>X</item><item>Y</item>
                             let current_list = fail_idx_list_content.get_or_insert_with(String::new);
                            if !current_list.is_empty() {
                                current_list.push_str(",");
                            }
                            current_list.push_str(&text_val);
                        },
                        
                        // guide_qrcode_scan_event children
                        "guide_account" if in_guide_qrcode_scan_event_tag => guide_account = Some(text_val),
                        "scene" if in_guide_qrcode_scan_event_tag => guide_qrcode_scene = Some(text_val),


                        "xml" => {}, 
                        _ => {} // Ignore other text content
                    }
                }
            }
            Ok(Event::CData(e)) => { 
                if !inside_xml_tag { continue; } // Ignore CDATA outside <xml>
                if let Some(tag) = &current_tag_name {
                     let text_val = str::from_utf8(&e)
                                        .map_err(|err| err.to_string())?
                                        .trim()
                                        .to_string();
                    match tag.as_str() {
                        "ToUserName" => to_user_name = text_val,
                        "FromUserName" => from_user_name = text_val,
                        "MsgType" => msg_type = text_val,
                        "Content" => content = text_val,
                        "PicUrl" => pic_url = text_val,
                        "MediaId" => media_id = text_val,
                        "Format" => format = text_val,
                        "Recognition" => recognition = Some(text_val),
                        "ThumbMediaId" => thumb_media_id = text_val,

                        "Location_X" => {
                            if in_sendlocation_info { location_x_select = Some(text_val); }
                            else { location_x = text_val; } 
                        },
                        "Location_Y" => {
                            if in_sendlocation_info { location_y_select = Some(text_val); }
                            else { location_y = text_val; }
                        },
                        "Scale" => {
                            if in_sendlocation_info { scale_select = Some(text_val); }
                            else { scale = text_val; }
                        },
                        "Label" => {
                            if in_sendlocation_info { label_select = Some(text_val); }
                            else { label = text_val; }
                        },
                        "Latitude" => latitude = Some(text_val), // For LOCATION event
                        "Longitude" => longitude = Some(text_val), // For LOCATION event
                        "Precision" => precision = Some(text_val), // For LOCATION event

                        "Title" => title = text_val,
                        "Description" => description = text_val,
                        "Url" => url = text_val,
                        "Event" => event_name = text_val,
                        "EventKey" => event_key = Some(text_val),
                        "Ticket" => ticket = Some(text_val),
                        "Latitude" => latitude = Some(text_val),
                        "Longitude" => longitude = Some(text_val),
                        "Precision" => precision = Some(text_val),
                        "MenuID" | "MenuId" => menu_id = Some(text_val), // Common event field
                        // Specific event fields from CDATA (similar to Text handling)
                        "ScanType" if in_scancode_info => scan_type = Some(text_val),
                        "ScanResult" if in_scancode_info => scan_result = Some(text_val),
                        "Count" if in_sendpics_info => count = Some(text_val),
                        "PicMd5Sum" if in_pic_list => {
                            let current_list = pic_list_content.get_or_insert_with(String::new);
                            if !current_list.is_empty() { current_list.push_str(","); }
                            current_list.push_str(&text_val);
                        },
                        "Location_X" if in_sendlocation_info => location_x_select = Some(text_val),
                        "Location_Y" if in_sendlocation_info => location_y_select = Some(text_val),
                        "Scale" if in_sendlocation_info => scale_select = Some(text_val),
                        "Label" if in_sendlocation_info => label_select = Some(text_val),
                        "Poiname" if in_sendlocation_info => poiname = Some(text_val),
                        "MsgID" if msg_type == "event" && (event_name == "TEMPLATESENDJOBFINISH" || event_name == "MASSSENDJOBFINISH") => new_msg_id = Some(text_val),
                        "Status" if msg_type == "event" && (event_name == "TEMPLATESENDJOBFINISH" || event_name == "MASSSENDJOBFINISH" || in_guide_invite_result_event_tag) => {
                            if in_guide_invite_result_event_tag { result_status = Some(text_val); }
                            else { status = Some(text_val); }
                        },
                        "TotalCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => total_count = Some(text_val),
                        "FilterCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => filter_count = Some(text_val),
                        "SentCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => sent_count = Some(text_val),
                        "ErrorCount" if msg_type == "event" && event_name == "MASSSENDJOBFINISH" => error_count = Some(text_val),
                        "CheckState" if in_copyright_check_result => check_state_copyright = Some(text_val),
                        "publish_id" if in_publish_event_info => publish_id = Some(text_val),
                        "publish_status" if in_publish_event_info => publish_status = Some(text_val),
                        "article_id" if in_publish_event_info => article_id = Some(text_val),
                        "article_idx" if in_publish_event_info => article_idx = Some(text_val),
                        "item" if in_fail_idx_list => {
                            let current_list = fail_idx_list_content.get_or_insert_with(String::new);
                            if !current_list.is_empty() { current_list.push_str(","); }
                            current_list.push_str(&text_val);
                        },
                        "guide_account" if in_guide_qrcode_scan_event_tag => guide_account = Some(text_val),
                        "scene" if in_guide_qrcode_scan_event_tag => guide_qrcode_scene = Some(text_val),

                        _ => {}
                    }
                }
            }
            Ok(Event::End(ref e)) => {
                let end_tag_name_owned = str::from_utf8(e.name().as_ref())
                                           .map_err(|err| err.to_string())?
                                           .to_string();
                
                // Reset nesting flags when corresponding outer tags end
                match end_tag_name_owned.as_str() {
                    "ScanCodeInfo" => in_scancode_info = false,
                    "SendPicsInfo" => in_sendpics_info = false,
                    "PicList" if in_sendpics_info => in_pic_list = false, // PicList is child of SendPicsInfo
                    "SendLocationInfo" => in_sendlocation_info = false,
                    "CopyrightCheckResult" => in_copyright_check_result = false,
                    "PublishEventInfo" => in_publish_event_info = false,
                    "fail_idx_list" if in_publish_event_info => in_fail_idx_list = false,
                    "guide_invite_result_event" if event_name == "guide_invite_result_event" => in_guide_invite_result_event_tag = false,
                    "guide_qrcode_scan_event" if event_name == "guide_qrcode_scan_event" => in_guide_qrcode_scan_event_tag = false,
                    _ => {}
                }

                if !inside_xml_tag && end_tag_name_owned == "xml" {
                     return Err("XML parsing error: Unexpected </xml> tag or invalid structure.".to_string());
                }

                if current_tag_name.as_deref() == Some(end_tag_name_owned.as_str()) {
                    current_tag_name = None; 
                }
                if end_tag_name_owned == "xml" { 
                    xml_root_processed_correctly = true; 
                    break; 
                }
            }
            Ok(Event::Eof) => { 
                if !xml_root_processed_correctly && inside_xml_tag { // XML tag was opened but not closed by </xml>
                    return Err("XML ended prematurely or was malformed (EOF before </xml>).".to_string());
                } else if !xml_root_processed_correctly && !inside_xml_tag && !xml_input.trim().is_empty() && !to_user_name.is_empty() {
                    // This case might be complex: EOF, no proper </xml> seen, but also no <xml> might have been seen as first tag.
                    // If to_user_name is populated, it means we parsed something without proper root.
                    return Err("XML parsing error: Missing <xml> root tag or invalid structure.".to_string());
                } else if to_user_name.is_empty() && from_user_name.is_empty() && msg_type.is_empty() {
                     // Handles cases like empty string or just whitespace, or "<>"
                     return Err("XML parsing error: Premature end of document".to_string());
                }
                // If xml_root_processed_correctly is true, this break is fine.
                // If it's false, but all essential fields are empty, previous check handles it.
                break; 
            }
            Err(e) => return Err(format!("XML parsing error: {}", e)),
            _ => (), 
        }
    }
    
    if !xml_root_processed_correctly && !xml_input.trim().is_empty() && xml_input.trim() != "<xml></xml>" {
        // If loop finished (e.g. by EOF) but <xml>...</xml> wasn't properly processed.
        // This is a catch-all for malformed structures not caught by specific event errors.
        return Err("XML parsing error: Invalid document structure or missing </xml>.".to_string());
    }

    if to_user_name.is_empty() || from_user_name.is_empty() || msg_type.is_empty() {
        // Check if input was just "<xml></xml>" or similar minimal but valid forms that result in empty fields
        if xml_input.trim() == "<xml></xml>" || (xml_root_processed_correctly && msg_type.is_empty()) {
             return Err("MsgType is empty. XML might be malformed or not a valid WeChat message.".to_string());
        } else if !xml_input.trim().is_empty() { // General case for missing essential fields
             return Err("Essential fields (ToUserName, FromUserName, MsgType) are missing.".to_string());
        } else { // Should have been caught by the initial empty check, but for safety:
            return Err("XML parsing error: Premature end of document".to_string());
        }
    }


    let message_body = match msg_type.as_str() {
        "text" => MessageBody::Text { content },
        "image" => MessageBody::Image { pic_url, media_id },
        "voice" => MessageBody::Voice { media_id, format, recognition },
        "video" => MessageBody::Video { media_id, thumb_media_id },
        "shortvideo" => MessageBody::ShortVideo { media_id, thumb_media_id },
        "location" => MessageBody::Location { location_x, location_y, scale, label },
        "link" => MessageBody::Link { title, description, url },
        "event" => MessageBody::Event {
            event: event_name, // This is the Event name like "subscribe", "CLICK", "scancode_push" etc.
            event_key,
            ticket,
            latitude, // For LOCATION base event
            longitude, // For LOCATION base event
            precision, // For LOCATION base event
            menu_id,   // For some menu events

            scan_type,
            scan_result,
            count,
            pic_list: pic_list_content,
            location_x_select,
            location_y_select,
            scale_select,
            label_select,
            poiname,
            status,
            new_msg_id, // From <MsgID> in job events
            total_count,
            filter_count,
            sent_count,
            error_count,
            copyright_check_result: check_state_copyright,
            publish_id,
            publish_status,
            article_id,
            article_idx,
            fail_idx_list: fail_idx_list_content,
            result_status,
            guide_account,
            guide_qrcode_scene,
        },
        // if empty means error. If empty msg_type could lead to Unhandled, that's different.
        // Current logic: empty msg_type is an error.
        _ => MessageBody::Unhandled { raw_content: xml_input.to_string() },
    };

    Ok(WxMessage {
        to_user_name,
        from_user_name,
        create_time,
        msg_type,
        msg_id,
        message: message_body,
    })
}
