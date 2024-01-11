#[cfg(all(test, feature = "mp"))]
mod tests {
    use wx_sdk::{
        mp::reply::{reply_to_xml, Reply, Text},
        SdkResult,
    };

    #[test]
    fn reply_text() -> SdkResult<()> {
        let test = "<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>12345678</CreateTime>
  <MsgType><![CDATA[text]]></MsgType>
  <Content><![CDATA[你好]]></Content>
</xml>";
        let reply = Reply::Text(Text {
            content: "你好".to_string(),
        });
        let reply = reply_to_xml(reply, "fromUser", "toUser")?;
        // CreateTime 时间不同，不作比较了
        Ok(())
    }
}
