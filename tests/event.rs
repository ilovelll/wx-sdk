#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use wx_func::{ wechat::{EncodingMode, ServerConfig, WxSdk}};

  #[test]
  fn test_parse_event() {
    let mut url = HashMap::new();
    url.insert("msg_signature".to_owned(), "477715d11cdb4164915debcba66cb864d751f3e6".to_owned());
    url.insert("timestamp".to_owned(), "1409659813".to_owned());
    url.insert("nonce".to_owned(), "1372623149".to_owned());
    let server_config = ServerConfig::new("QDG6eK", EncodingMode::Security("jWmYm7qr5nMoAUwZRjGtBxmz3KA1tkAj3ykkR6q2B2C".to_owned()));
    let wsdk = WxSdk::new_with_default_token_client("wx5823bf96d3bd56c7", "app_secret", server_config);
    let msg = "<xml><ToUserName><![CDATA[wx5823bf96d3bd56c7]]></ToUserName><Encrypt><![CDATA[RypEvHKD8QQKFhvQ6QleEB4J58tiPdvo+rtK1I9qca6aM/wvqnLSV5zEPeusUiX5L5X/0lWfrf0QADHHhGd3QczcdCUpj911L3vg3W/sYYvuJTs3TUUkSUXxaccAS0qhxchrRYt66wiSpGLYL42aM6A8dTT+6k4aSknmPj48kzJs8qLjvd4Xgpue06DOdnLxAUHzM6+kDZ+HMZfJYuR+LtwGc2hgf5gsijff0ekUNXZiqATP7PF5mZxZ3Izoun1s4zG4LUMnvw2r+KqCKIw+3IQH03v+BCA9nMELNqbSf6tiWSrXJB3LAVGUcallcrw8V2t9EL4EhzJWrQUax5wLVMNS0+rUPA3k22Ncx4XXZS9o0MBH27Bo6BpNelZpS+/uh9KsNlY6bHCmJU9p8g7m3fVKn28H3KDYA5Pl/T8Z1ptDAVe0lXdQ2YoyyH2uyPIGHBZZIs2pDBS8R07+qN+E7Q==]]></Encrypt></xml>";
    let event = wsdk.parse_received_decrypt_msg(msg, url).unwrap();
    assert_eq!(event.msg_type, "text".to_owned());
  }
}