use wx_message::{parse, MessageBody, WxMessage};

#[test]
fn test_parse_text_message_simple() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831860,
        msg_type: "text".to_string(),
        msg_id: Some(1234567890123456),
        message: MessageBody::Text {
            content: "this is a test".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for simple text message: {}", e),
    }
}

#[test]
fn test_parse_text_message_with_special_chars() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831861</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[Hello & <world> "你好" '©']]></Content>
    <MsgId>1234567890123457</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831861,
        msg_type: "text".to_string(),
        msg_id: Some(1234567890123457),
        message: MessageBody::Text {
            content: "Hello & <world> \"你好\" '©'".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for text message with special chars: {}", e),
    }
}

#[test]
fn test_parse_image_message() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831862</CreateTime>
    <MsgType><![CDATA[image]]></MsgType>
    <PicUrl><![CDATA[this is a url]]></PicUrl>
    <MediaId><![CDATA[media_id]]></MediaId>
    <MsgId>1234567890123458</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831862,
        msg_type: "image".to_string(),
        msg_id: Some(1234567890123458),
        message: MessageBody::Image {
            pic_url: "this is a url".to_string(),
            media_id: "media_id".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for image message: {}", e),
    }
}

#[test]
fn test_parse_voice_message() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1357290913</CreateTime>
    <MsgType><![CDATA[voice]]></MsgType>
    <MediaId><![CDATA[media_id]]></MediaId>
    <Format><![CDATA[Format]]></Format>
    <MsgId>1234567890123459</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1357290913,
        msg_type: "voice".to_string(),
        msg_id: Some(1234567890123459),
        message: MessageBody::Voice {
            media_id: "media_id".to_string(),
            format: "Format".to_string(),
            recognition: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for voice message: {}", e),
    }
}

#[test]
fn test_parse_voice_message_with_recognition() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1357290914</CreateTime>
    <MsgType><![CDATA[voice]]></MsgType>
    <MediaId><![CDATA[media_id_rec]]></MediaId>
    <Format><![CDATA[amr]]></Format>
    <Recognition><![CDATA[腾讯微信团队]]></Recognition>
    <MsgId>1234567890123460</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1357290914,
        msg_type: "voice".to_string(),
        msg_id: Some(1234567890123460),
        message: MessageBody::Voice {
            media_id: "media_id_rec".to_string(),
            format: "amr".to_string(),
            recognition: Some("腾讯微信团队".to_string()),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for voice message with recognition: {}", e),
    }
}

#[test]
fn test_parse_video_message() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1357290915</CreateTime>
    <MsgType><![CDATA[video]]></MsgType>
    <MediaId><![CDATA[media_id_video]]></MediaId>
    <ThumbMediaId><![CDATA[thumb_media_id_video]]></ThumbMediaId>
    <MsgId>1234567890123461</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1357290915,
        msg_type: "video".to_string(),
        msg_id: Some(1234567890123461),
        message: MessageBody::Video {
            media_id: "media_id_video".to_string(),
            thumb_media_id: "thumb_media_id_video".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for video message: {}", e),
    }
}

#[test]
fn test_parse_shortvideo_message() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1357290916</CreateTime>
    <MsgType><![CDATA[shortvideo]]></MsgType>
    <MediaId><![CDATA[media_id_shortvideo]]></MediaId>
    <ThumbMediaId><![CDATA[thumb_media_id_shortvideo]]></ThumbMediaId>
    <MsgId>1234567890123462</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1357290916,
        msg_type: "shortvideo".to_string(),
        msg_id: Some(1234567890123462),
        message: MessageBody::ShortVideo {
            media_id: "media_id_shortvideo".to_string(),
            thumb_media_id: "thumb_media_id_shortvideo".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for shortvideo message: {}", e),
    }
}

#[test]
fn test_parse_location_message() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1351776360</CreateTime>
    <MsgType><![CDATA[location]]></MsgType>
    <Location_X>23.134521</Location_X>
    <Location_Y>113.358803</Location_Y>
    <Scale>20</Scale>
    <Label><![CDATA[位置信息]]></Label>
    <MsgId>1234567890123463</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1351776360,
        msg_type: "location".to_string(),
        msg_id: Some(1234567890123463),
        message: MessageBody::Location {
            location_x: "23.134521".to_string(),
            location_y: "113.358803".to_string(),
            scale: "20".to_string(),
            label: "位置信息".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for location message: {}", e),
    }
}

#[test]
fn test_parse_link_message() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1351776361</CreateTime>
    <MsgType><![CDATA[link]]></MsgType>
    <Title><![CDATA[公众平台官网链接]]></Title>
    <Description><![CDATA[公众平台官网链接]]></Description>
    <Url><![CDATA[https://mp.weixin.qq.com]]></Url>
    <MsgId>1234567890123464</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1351776361,
        msg_type: "link".to_string(),
        msg_id: Some(1234567890123464),
        message: MessageBody::Link {
            title: "公众平台官网链接".to_string(),
            description: "公众平台官网链接".to_string(),
            url: "https://mp.weixin.qq.com".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for link message: {}", e),
    }
}

// Event Message Tests

#[test]
fn test_parse_event_subscribe() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[subscribe]]></Event>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None, 
        message: MessageBody::Event {
            event: "subscribe".to_string(), event_key: None, ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: None, scan_result: None, count: None, pic_list: None, location_x_select: None, location_y_select: None,
            scale_select: None, label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None,
            filter_count: None, sent_count: None, error_count: None, copyright_check_result: None, publish_id: None,
            publish_status: None, article_id: None, article_idx: None, fail_idx_list: None, result_status: None,
            guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for subscribe event: {}", e),
    }
}

#[test]
fn test_parse_event_subscribe_with_event_key_ticket() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[subscribe]]></Event>
    <EventKey><![CDATA[qrscene_123123]]></EventKey>
    <Ticket><![CDATA[TICKET]]></Ticket>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "subscribe".to_string(), event_key: Some("qrscene_123123".to_string()), ticket: Some("TICKET".to_string()),
            latitude: None, longitude: None, precision: None, menu_id: None, scan_type: None, scan_result: None,
            count: None, pic_list: None, location_x_select: None, location_y_select: None, scale_select: None,
            label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None, filter_count: None,
            sent_count: None, error_count: None, copyright_check_result: None, publish_id: None, publish_status: None,
            article_id: None, article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for subscribe event with EventKey & Ticket: {}", e),
    }
}

#[test]
fn test_parse_event_unsubscribe() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[unsubscribe]]></Event>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "unsubscribe".to_string(), event_key: None, ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: None, scan_result: None, count: None, pic_list: None, location_x_select: None, location_y_select: None,
            scale_select: None, label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None,
            filter_count: None, sent_count: None, error_count: None, copyright_check_result: None, publish_id: None,
            publish_status: None, article_id: None, article_idx: None, fail_idx_list: None, result_status: None,
            guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for unsubscribe event: {}", e),
    }
}

#[test]
fn test_parse_event_scan() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[SCAN]]></Event>
    <EventKey><![CDATA[SCENE_VALUE]]></EventKey>
    <Ticket><![CDATA[TICKET_SCAN]]></Ticket>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "SCAN".to_string(), event_key: Some("SCENE_VALUE".to_string()), ticket: Some("TICKET_SCAN".to_string()),
            latitude: None, longitude: None, precision: None, menu_id: None, scan_type: None, scan_result: None,
            count: None, pic_list: None, location_x_select: None, location_y_select: None, scale_select: None,
            label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None, filter_count: None,
            sent_count: None, error_count: None, copyright_check_result: None, publish_id: None, publish_status: None,
            article_id: None, article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for SCAN event: {}", e),
    }
}

#[test]
fn test_parse_event_location() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[LOCATION]]></Event>
    <Latitude>23.137466</Latitude>
    <Longitude>113.352425</Longitude>
    <Precision>119.385040</Precision>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "LOCATION".to_string(), event_key: None, ticket: None, latitude: Some("23.137466".to_string()),
            longitude: Some("113.352425".to_string()), precision: Some("119.385040".to_string()), menu_id: None,
            scan_type: None, scan_result: None, count: None, pic_list: None, location_x_select: None,
            location_y_select: None, scale_select: None, label_select: None, poiname: None, status: None,
            new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for LOCATION event: {}", e),
    }
}

#[test]
fn test_parse_event_click() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[CLICK]]></Event>
    <EventKey><![CDATA[EVENT_KEY_CLICK]]></EventKey>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "CLICK".to_string(), event_key: Some("EVENT_KEY_CLICK".to_string()), ticket: None, latitude: None,
            longitude: None, precision: None, menu_id: None, scan_type: None, scan_result: None, count: None,
            pic_list: None, location_x_select: None, location_y_select: None, scale_select: None, label_select: None,
            poiname: None, status: None, new_msg_id: None, total_count: None, filter_count: None, sent_count: None,
            error_count: None, copyright_check_result: None, publish_id: None, publish_status: None,
            article_id: None, article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for CLICK event: {}", e),
    }
}

#[test]
fn test_parse_event_view() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>123456789</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[VIEW]]></Event>
    <EventKey><![CDATA[www.qq.com]]></EventKey>
    <MenuId>MENU_ID_VIEW</MenuId> 
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 123456789,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "VIEW".to_string(), event_key: Some("www.qq.com".to_string()), ticket: None, latitude: None,
            longitude: None, precision: None, menu_id: Some("MENU_ID_VIEW".to_string()), scan_type: None,
            scan_result: None, count: None, pic_list: None, location_x_select: None, location_y_select: None,
            scale_select: None, label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None,
            filter_count: None, sent_count: None, error_count: None, copyright_check_result: None, publish_id: None,
            publish_status: None, article_id: None, article_idx: None, fail_idx_list: None, result_status: None,
            guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for VIEW event: {}", e),
    }
}

#[test]
fn test_parse_event_view_miniprogram() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1600000000</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[view_miniprogram]]></Event>
    <EventKey><![CDATA[mpages/index/index?foo=bar]]></EventKey>
    <MenuId>201923432</MenuId>
</xml>"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1600000000,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "view_miniprogram".to_string(), event_key: Some("mpages/index/index?foo=bar".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: Some("201923432".to_string()),
            scan_type: None, scan_result: None, count: None, pic_list: None, location_x_select: None,
            location_y_select: None, scale_select: None, label_select: None, poiname: None, status: None,
            new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for view_miniprogram event: {}", e),
    }
}

// Edge Case Tests

#[test]
fn test_parse_missing_required_tousername() {
    let xml_input = r#"
<xml>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let result = parse(xml_input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Essential fields (ToUserName, FromUserName, MsgType) are missing.");
}

#[test]
fn test_parse_missing_required_fromusername() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let result = parse(xml_input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Essential fields (ToUserName, FromUserName, MsgType) are missing.");
}

#[test]
fn test_parse_missing_required_msgtype() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let result = parse(xml_input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "MsgType is empty. XML might be malformed or not a valid WeChat message.");
}


#[test]
fn test_parse_missing_optional_msgid() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test without MsgId]]></Content>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831860,
        msg_type: "text".to_string(),
        msg_id: None, 
        message: MessageBody::Text {
            content: "this is a test without MsgId".to_string(),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for text message missing optional MsgId: {}", e),
    }
}


#[test]
fn test_parse_malformed_xml_unclosed_tag() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]> 
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#; 
    let result = parse(xml_input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), msg if msg.contains("XML parsing error")));
}

#[test]
fn test_parse_malformed_xml_unexpected_eof() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]>
"#; 
    let result = parse(xml_input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "XML ended prematurely or was malformed (EOF before </xml>).");
}


#[test]
fn test_parse_unknown_msgtype() {
    let original_xml = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[custom_unknown_type]]></MsgType>
    <CustomField><![CDATA[some custom data]]></CustomField>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831860,
        msg_type: "custom_unknown_type".to_string(),
        msg_id: Some(1234567890123456),
        message: MessageBody::Unhandled {
            raw_content: original_xml.to_string(),
        },
    };
    match parse(original_xml) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for unknown MsgType: {}", e),
    }
}

#[test]
fn test_parse_empty_input() {
    let xml_input = "";
    let result = parse(xml_input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "XML parsing error: Premature end of document");
}

#[test]
fn test_parse_invalid_createtime() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>not_a_number</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let result = parse(xml_input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), msg if msg.contains("invalid digit found in string")));
}

#[test]
fn test_parse_invalid_msgid() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>not_a_number_either</MsgId>
</xml>
"#;
    let result = parse(xml_input);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), msg if msg.contains("invalid digit found in string")));
}

#[test]
fn test_no_xml_tag_at_root() {
    let xml_input = r#"
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[this is a test]]></Content>
    <MsgId>1234567890123456</MsgId>
"#;
    let result = parse(xml_input);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "XML parsing error: Missing <xml> root tag or invalid structure.");
}

#[test]
fn test_text_message_with_empty_content_tag() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831860,
        msg_type: "text".to_string(),
        msg_id: Some(1234567890123456),
        message: MessageBody::Text {
            content: "".to_string(), 
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for text message with empty content tag: {}", e),
    }
}

#[test]
fn test_text_message_with_empty_cdata_content() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1348831860</CreateTime>
    <MsgType><![CDATA[text]]></MsgType>
    <Content><![CDATA[]]></Content>
    <MsgId>1234567890123456</MsgId>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1348831860,
        msg_type: "text".to_string(),
        msg_id: Some(1234567890123456),
        message: MessageBody::Text {
            content: "".to_string(), 
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for text message with empty CDATA content: {}", e),
    }
}

// --- New Complex Event Tests ---

#[test]
fn test_parse_event_scancode_push() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090506</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[scancode_push]]></Event>
    <EventKey><![CDATA[EVENT_KEY_SCANCODE_PUSH]]></EventKey>
    <ScanCodeInfo>
        <ScanType><![CDATA[qrcode]]></ScanType>
        <ScanResult><![CDATA[QR_CODE_RESULT_123]]></ScanResult>
    </ScanCodeInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090506,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "scancode_push".to_string(), event_key: Some("EVENT_KEY_SCANCODE_PUSH".to_string()), ticket: None,
            latitude: None, longitude: None, precision: None, menu_id: None, scan_type: Some("qrcode".to_string()),
            scan_result: Some("QR_CODE_RESULT_123".to_string()), count: None, pic_list: None, location_x_select: None,
            location_y_select: None, scale_select: None, label_select: None, poiname: None, status: None,
            new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for scancode_push event: {}", e),
    }
}

#[test]
fn test_parse_event_scancode_waitmsg() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090507</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[scancode_waitmsg]]></Event>
    <EventKey><![CDATA[EVENT_KEY_SCANCODE_WAITMSG]]></EventKey>
    <ScanCodeInfo>
        <ScanType><![CDATA[barcode]]></ScanType>
        <ScanResult><![CDATA[BARCODE_RESULT_456]]></ScanResult>
    </ScanCodeInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090507,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "scancode_waitmsg".to_string(), event_key: Some("EVENT_KEY_SCANCODE_WAITMSG".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: Some("barcode".to_string()), scan_result: Some("BARCODE_RESULT_456".to_string()),
            count: None, pic_list: None, location_x_select: None, location_y_select: None, scale_select: None,
            label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None,
            filter_count: None, sent_count: None, error_count: None, copyright_check_result: None,
            publish_id: None, publish_status: None, article_id: None, article_idx: None, fail_idx_list: None,
            result_status: None, guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for scancode_waitmsg event: {}", e),
    }
}

#[test]
fn test_parse_event_pic_sysphoto() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090508</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[pic_sysphoto]]></Event>
    <EventKey><![CDATA[EVENT_KEY_PIC_SYSPHOTO]]></EventKey>
    <SendPicsInfo>
        <Count>1</Count>
        <PicList>
            <item>
                <PicMd5Sum><![CDATA[md5_hash_1]]></PicMd5Sum>
            </item>
        </PicList>
    </SendPicsInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090508,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "pic_sysphoto".to_string(), event_key: Some("EVENT_KEY_PIC_SYSPHOTO".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: None, scan_result: None, count: Some("1".to_string()), pic_list: Some("md5_hash_1".to_string()),
            location_x_select: None, location_y_select: None, scale_select: None, label_select: None, poiname: None,
            status: None, new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for pic_sysphoto event: {}", e),
    }
}

#[test]
fn test_parse_event_pic_photo_or_album_multiple() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090509</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[pic_photo_or_album]]></Event>
    <EventKey><![CDATA[EVENT_KEY_PIC_PHOTO_OR_ALBUM]]></EventKey>
    <SendPicsInfo>
        <Count>2</Count>
        <PicList>
            <item>
                <PicMd5Sum><![CDATA[md5_hash_A]]></PicMd5Sum>
            </item>
            <item>
                <PicMd5Sum><![CDATA[md5_hash_B]]></PicMd5Sum>
            </item>
        </PicList>
    </SendPicsInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090509,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "pic_photo_or_album".to_string(), event_key: Some("EVENT_KEY_PIC_PHOTO_OR_ALBUM".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: None, scan_result: None, count: Some("2".to_string()), pic_list: Some("md5_hash_A,md5_hash_B".to_string()),
            location_x_select: None, location_y_select: None, scale_select: None, label_select: None, poiname: None,
            status: None, new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for pic_photo_or_album event: {}", e),
    }
}

#[test]
fn test_parse_event_pic_weixin() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090513</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[pic_weixin]]></Event>
    <EventKey><![CDATA[EVENT_KEY_PIC_WEIXIN]]></EventKey>
    <SendPicsInfo>
        <Count>1</Count>
        <PicList>
            <item>
                <PicMd5Sum><![CDATA[md5_hash_C]]></PicMd5Sum>
            </item>
        </PicList>
    </SendPicsInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090513,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "pic_weixin".to_string(), event_key: Some("EVENT_KEY_PIC_WEIXIN".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: None, scan_result: None, count: Some("1".to_string()), pic_list: Some("md5_hash_C".to_string()),
            location_x_select: None, location_y_select: None, scale_select: None, label_select: None, poiname: None,
            status: None, new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for pic_weixin event: {}", e),
    }
}


#[test]
fn test_parse_event_location_select() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090510</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[location_select]]></Event>
    <EventKey><![CDATA[EVENT_KEY_LOCATION_SELECT]]></EventKey>
    <SendLocationInfo>
        <Location_X><![CDATA[23.104321]]></Location_X>
        <Location_Y><![CDATA[113.320432]]></Location_Y>
        <Scale><![CDATA[15]]></Scale>
        <Label><![CDATA[Selected Location]]></Label>
        <Poiname><![CDATA[POI Name Here]]></Poiname>
    </SendLocationInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090510,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "location_select".to_string(), event_key: Some("EVENT_KEY_LOCATION_SELECT".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: None,
            scan_type: None, scan_result: None, count: None, pic_list: None,
            location_x_select: Some("23.104321".to_string()), location_y_select: Some("113.320432".to_string()),
            scale_select: Some("15".to_string()), label_select: Some("Selected Location".to_string()),
            poiname: Some("POI Name Here".to_string()), status: None, new_msg_id: None, total_count: None,
            filter_count: None, sent_count: None, error_count: None, copyright_check_result: None,
            publish_id: None, publish_status: None, article_id: None, article_idx: None, fail_idx_list: None,
            result_status: None, guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for location_select event: {}", e),
    }
}

#[test]
fn test_parse_event_templatesendjobfinish() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090511</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[TEMPLATESENDJOBFINISH]]></Event>
    <MsgID>200163840</MsgID>
    <Status><![CDATA[success]]></Status>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090511,
        msg_type: "event".to_string(),
        msg_id: None, 
        message: MessageBody::Event {
            event: "TEMPLATESENDJOBFINISH".to_string(), event_key: None, ticket: None, latitude: None,
            longitude: None, precision: None, menu_id: None, scan_type: None, scan_result: None, count: None,
            pic_list: None, location_x_select: None, location_y_select: None, scale_select: None,
            label_select: None, poiname: None, status: Some("success".to_string()),
            new_msg_id: Some("200163840".to_string()), total_count: None, filter_count: None, sent_count: None,
            error_count: None, copyright_check_result: None, publish_id: None, publish_status: None,
            article_id: None, article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for TEMPLATESENDJOBFINISH event: {}", e),
    }
}

#[test]
fn test_parse_event_masssendjobfinish() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[toUser]]></ToUserName>
    <FromUserName><![CDATA[fromUser]]></FromUserName>
    <CreateTime>1408090512</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[MASSSENDJOBFINISH]]></Event>
    <MsgID>1000001625</MsgID>
    <Status><![CDATA[send success]]></Status>
    <TotalCount>100</TotalCount>
    <FilterCount>80</FilterCount>
    <SentCount>75</SentCount>
    <ErrorCount>5</ErrorCount>
    <CopyrightCheckResult>
        <CheckState>2</CheckState>
    </CopyrightCheckResult>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "toUser".to_string(),
        from_user_name: "fromUser".to_string(),
        create_time: 1408090512,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "MASSSENDJOBFINISH".to_string(), event_key: None, ticket: None, latitude: None, longitude: None,
            precision: None, menu_id: None, scan_type: None, scan_result: None, count: None, pic_list: None,
            location_x_select: None, location_y_select: None, scale_select: None, label_select: None, poiname: None,
            status: Some("send success".to_string()), new_msg_id: Some("1000001625".to_string()),
            total_count: Some("100".to_string()), filter_count: Some("80".to_string()),
            sent_count: Some("75".to_string()), error_count: Some("5".to_string()),
            copyright_check_result: Some("2".to_string()), publish_id: None, publish_status: None,
            article_id: None, article_idx: None, fail_idx_list: None, result_status: None, guide_account: None,
            guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for MASSSENDJOBFINISH event: {}", e),
    }
}

#[test]
fn test_parse_event_publishjobfinish_success() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[gh_4d00a7783408]]></ToUserName>
    <FromUserName><![CDATA[oY30E0XqhfOcV3Y755O2jQz7o0to]]></FromUserName>
    <CreateTime>1589813753</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[PUBLISHJOBFINISH]]></Event>
    <PublishEventInfo>
        <publish_id>2247503639</publish_id>
        <publish_status>0</publish_status>
        <article_id><![CDATA[b5VUKfN5v2f4ax7GkXRfIK_Lm0y6aLp8NqJgVStSabA]]></article_id>
        <article_idx>1</article_idx>
        <fail_idx_list/>
    </PublishEventInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "gh_4d00a7783408".to_string(),
        from_user_name: "oY30E0XqhfOcV3Y755O2jQz7o0to".to_string(),
        create_time: 1589813753,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "PUBLISHJOBFINISH".to_string(), event_key: None, ticket: None, latitude: None, longitude: None,
            precision: None, menu_id: None, scan_type: None, scan_result: None, count: None, pic_list: None,
            location_x_select: None, location_y_select: None, scale_select: None, label_select: None, poiname: None,
            status: None, new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: Some("2247503639".to_string()),
            publish_status: Some("0".to_string()), article_id: Some("b5VUKfN5v2f4ax7GkXRfIK_Lm0y6aLp8NqJgVStSabA".to_string()),
            article_idx: Some("1".to_string()), fail_idx_list: Some("".to_string()), result_status: None,
            guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for PUBLISHJOBFINISH success event: {}", e),
    }
}

#[test]
fn test_parse_event_publishjobfinish_fail() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[gh_4d00a7783408]]></ToUserName>
    <FromUserName><![CDATA[oY30E0XqhfOcV3Y755O2jQz7o0to]]></FromUserName>
    <CreateTime>1589813754</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[PUBLISHJOBFINISH]]></Event>
    <PublishEventInfo>
        <publish_id>2247503640</publish_id>
        <publish_status>1</publish_status>
        <article_id/>
        <article_idx/>
        <fail_idx_list>
            <item>1</item>
            <item>3</item>
        </fail_idx_list>
    </PublishEventInfo>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "gh_4d00a7783408".to_string(),
        from_user_name: "oY30E0XqhfOcV3Y755O2jQz7o0to".to_string(),
        create_time: 1589813754,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "PUBLISHJOBFINISH".to_string(), event_key: None, ticket: None, latitude: None, longitude: None,
            precision: None, menu_id: None, scan_type: None, scan_result: None, count: None, pic_list: None,
            location_x_select: None, location_y_select: None, scale_select: None, label_select: None, poiname: None,
            status: None, new_msg_id: None, total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: Some("2247503640".to_string()),
            publish_status: Some("1".to_string()), article_id: Some("".to_string()),
            article_idx: Some("".to_string()), fail_idx_list: Some("1,3".to_string()), result_status: None,
            guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for PUBLISHJOBFINISH fail event: {}", e),
    }
}

#[test]
fn test_parse_event_guide_invite_result() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[to_user_]]></ToUserName>
    <FromUserName><![CDATA[from_user_]]></FromUserName>
    <CreateTime>1672502400</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[guide_invite_result_event]]></Event>
    <guide_invite_result_event>
        <Status>1</Status>
    </guide_invite_result_event>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "to_user_".to_string(),
        from_user_name: "from_user_".to_string(),
        create_time: 1672502400,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "guide_invite_result_event".to_string(), event_key: None, ticket: None, latitude: None,
            longitude: None, precision: None, menu_id: None, scan_type: None, scan_result: None, count: None,
            pic_list: None, location_x_select: None, location_y_select: None, scale_select: None,
            label_select: None, poiname: None, status: None, new_msg_id: None, total_count: None,
            filter_count: None, sent_count: None, error_count: None, copyright_check_result: None,
            publish_id: None, publish_status: None, article_id: None, article_idx: None, fail_idx_list: None,
            result_status: Some("1".to_string()), guide_account: None, guide_qrcode_scene: None,
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for guide_invite_result_event: {}", e),
    }
}

#[test]
fn test_parse_event_guide_qrcode_scan() {
    let xml_input = r#"
<xml>
    <ToUserName><![CDATA[to_user_]]></ToUserName>
    <FromUserName><![CDATA[from_user_]]></FromUserName>
    <CreateTime>1672502500</CreateTime>
    <MsgType><![CDATA[event]]></MsgType>
    <Event><![CDATA[guide_qrcode_scan_event]]></Event>
    <EventKey><![CDATA[qr_event_key]]></EventKey>
    <guide_qrcode_scan_event>
        <guide_account><![CDATA[test_guide_account]]></guide_account>
        <scene><![CDATA[100]]></scene>
    </guide_qrcode_scan_event>
</xml>
"#;
    let expected_message = WxMessage {
        to_user_name: "to_user_".to_string(),
        from_user_name: "from_user_".to_string(),
        create_time: 1672502500,
        msg_type: "event".to_string(),
        msg_id: None,
        message: MessageBody::Event {
            event: "guide_qrcode_scan_event".to_string(), event_key: Some("qr_event_key".to_string()),
            ticket: None, latitude: None, longitude: None, precision: None, menu_id: None, scan_type: None,
            scan_result: None, count: None, pic_list: None, location_x_select: None, location_y_select: None,
            scale_select: None, label_select: None, poiname: None, status: None, new_msg_id: None,
            total_count: None, filter_count: None, sent_count: None, error_count: None,
            copyright_check_result: None, publish_id: None, publish_status: None, article_id: None,
            article_idx: None, fail_idx_list: None, result_status: None,
            guide_account: Some("test_guide_account".to_string()),
            guide_qrcode_scene: Some("100".to_string()),
        },
    };
    match parse(xml_input) {
        Ok(parsed_message) => assert_eq!(parsed_message, expected_message),
        Err(e) => panic!("Parsing failed for guide_qrcode_scan_event: {}", e),
    }
}
