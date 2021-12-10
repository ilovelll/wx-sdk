use crate::{
    mp::{
        event::{
            xmlutil::{get_node_by_tag, get_number_from_root, get_text_from_root},
            ReceivedMessageParser,
        },
        freepublish::{ArticleDetail, ArticleDetailItem},
    },
    SdkResult,
};
use roxmltree::Node;
use serde::{Deserialize, Serialize};

pub enum PublishJobFinishEvent {
    Success {
        publish_id: String,
        publish_status: i8,
        article_id: String,
        article_detail: ArticleDetail,
    },
    Fail {
        publish_id: String,
        publish_status: i8,
        fail_id: Vec<i8>,
    },
}

impl ReceivedMessageParser for PublishJobFinishEvent {
    type ReceivedMessage = PublishJobFinishEvent;

    fn from_xml(node: &roxmltree::Node) -> crate::SdkResult<Self::ReceivedMessage> {
        let publish_event_info = get_node_by_tag(node, "PublishEventInfo")?;
        let publish_id = get_text_from_root(&publish_event_info, "publish_id")?.to_string();
        let publish_status = get_number_from_root::<i8>(&publish_event_info, "publish_status")?;
        if publish_status == 0 {
            let article_id = get_text_from_root(&publish_event_info, "article_id")?.to_string();
            let article_detail = get_node_by_tag(&publish_event_info, "article_detail")?;
            let count = get_number_from_root::<i8>(&article_detail, "count")?;
            let article_list: Vec<Node> = article_detail
                .descendants()
                .filter(|n| n.has_tag_name("item"))
                .collect();
            let article_list: Vec<ArticleDetailItem> = article_list
                .iter()
                .map(|n| {
                    let idx = n
                        .first_child()
                        .unwrap()
                        .text()
                        .unwrap()
                        .parse::<i8>()
                        .unwrap();
                    let article_url = n
                        .descendants()
                        .find(|n| n.has_tag_name("article_url"))
                        .unwrap()
                        .text()
                        .unwrap()
                        .to_string();
                    ArticleDetailItem { idx, article_url }
                })
                .collect();
            let article_detail = ArticleDetail {
                count,
                item: article_list,
            };
            Ok(PublishJobFinishEvent::Success {
                publish_id,
                publish_status,
                article_id,
                article_detail,
            })
        } else {
            let fail_id: Vec<i8> = publish_event_info
                .descendants()
                .filter(|n| n.has_tag_name("fail_idx"))
                .map(|n| n.text().unwrap().parse::<i8>().unwrap())
                .collect();
            Ok(PublishJobFinishEvent::Fail {
                publish_id,
                publish_status,
                fail_id,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SdkResult;
    #[test]
    pub fn parse() -> SdkResult<()> {
        use crate::mp::event::EventMessage;
        use roxmltree::Document;
        let s = "<xml> 
        <ToUserName><![CDATA[gh_4d00ed8d6399]]></ToUserName>  
        <FromUserName><![CDATA[oV5CrjpxgaGXNHIQigzNlgLTnwic]]></FromUserName>  
        <CreateTime>1481013459</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[PUBLISHJOBFINISH]]></Event>
        <PublishEventInfo>
          <publish_id>2247503051</publish_id>
          <publish_status>2</publish_status>
          <fail_idx>1</fail_idx>
          <fail_idx>2</fail_idx>
        </PublishEventInfo>
      </xml>
      ";
        let node = Document::parse(&s)?;
        let msg = PublishJobFinishEvent::from_xml(&node.root())?;

        matches!(msg, PublishJobFinishEvent::Fail { .. });
        let s = "<xml> 
  <ToUserName><![CDATA[gh_4d00ed8d6399]]></ToUserName>  
  <FromUserName><![CDATA[oV5CrjpxgaGXNHIQigzNlgLTnwic]]></FromUserName>  
  <CreateTime>1481013459</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[PUBLISHJOBFINISH]]></Event>
  <PublishEventInfo>
    <publish_id>2247503051</publish_id>
    <publish_status>0</publish_status>
    <article_id><![CDATA[b5O2OUs25HBxRceL7hfReg-U9QGeq9zQjiDvy
WP4Hq4]]></article_id>
    <article_detail>
      <count>1</count>
      <item>
        <idx>1</idx>
        <article_url><![CDATA[ARTICLE_URL]]></article_url>
      </item>
    </article_detail>
  </PublishEventInfo>
</xml>
";
        let msg = Document::parse(s)?;
        let msg = PublishJobFinishEvent::from_xml(&node.root())?;
        matches!(msg, PublishJobFinishEvent::Success { .. });
        Ok(())
    }
}
