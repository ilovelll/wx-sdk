#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use reqwest::blocking::get;
    use wx_sdk::{
        access_token::AccessTokenProvider,
        mp::reply::{Reply, Text},
        mp::{EncodingMode, ServerConfig},
        wechat::WxSdk,
        SdkResult, TokenClient,
    };

    fn get_sdk() -> WxSdk<TokenClient> {
        let wsdk = WxSdk::new_with_default_token_client("wx5823bf96d3bd56c7", "app_secret");
        wsdk
    }

    fn get_server_config() -> ServerConfig {
        let server_config = ServerConfig::new(
            "QDG6eK",
            EncodingMode::Security("jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C".to_owned()),
        );
        server_config
    }

    #[test]
    fn test_parse_event() {
        let mut url = HashMap::new();
        url.insert(
            "msg_signature".to_owned(),
            "477715d11cdb4164915debcba66cb864d751f3e6".to_owned(),
        );
        url.insert("timestamp".to_owned(), "1409659813".to_owned());
        url.insert("nonce".to_owned(), "1372623149".to_owned());
        let wsdk = get_sdk();
        let server_config = get_server_config();
        let mpsdk = wsdk.mp(server_config);
        let msg = "<xml><ToUserName><![CDATA[wx5823bf96d3bd56c7]]></ToUserName><Encrypt><![CDATA[RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==]]></Encrypt></xml>";
        let event = mpsdk.parse_received_msg(msg, Some(url)).unwrap();

        assert_eq!(event.msg_type, "text".to_owned());
    }

    #[test]
    fn test_encrypt_msg() -> SdkResult<()> {
        let sdk = get_sdk();
        let server_config = get_server_config();
        let sdk = sdk.mp(server_config);
        let mut url = HashMap::new();
        // url.insert(
        //     "msg_signature".to_owned(),
        //     "477715d11cdb4164915debcba66cb864d751f3e6".to_owned(),
        // );
        url.insert("timestamp".to_owned(), "1409659813".to_owned());
        url.insert("nonce".to_owned(), "1372623149".to_owned());
        let reply = Reply::Text(Text {
            content: "你好".to_owned(),
        });
        let reply_xml = sdk.reply_to_xml(
            reply,
            "wx5823bf96d3bd56c7",
            "oia2TjjewbmiOUlr6X-1crbLOvLw",
            Some(url.clone()),
        )?;
        let doc = exile::parse(reply_xml.clone()).unwrap();
        let elem = doc.root().child("MsgSignature").unwrap();
        // url.insert("msg_signature".to_owned(), elem.text().unwrap());
        // let event = sdk.parse_received_msg(reply_xml, Some(url)).unwrap();

        Ok(())
    }

    // #[test]
    // fn test_base_64() -> Result<(), Box<dyn std::error::Error>> {
    //     use base64ct::{Base64, Encoding};
    //     let txt = "jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C=";
    //     let result = Base64::decode_vec(txt);
    //     let input = "B2C=";
    //     let mut buf = [0u8; 1024];
    //     assert!(Base64::decode(input, &mut buf).is_err());
    //     assert_eq!(wx_sdk::mp::event::base64::decode(input).unwrap(), vec![1u8]);
    //     assert!(result.is_err());
    //     Ok(())
    // }
}
