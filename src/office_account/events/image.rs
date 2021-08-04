use roxmltree::Node;

use crate::{SdkResult, error::SdkError};

use super::{EventMessage, xmlutil::{get_number_from_root, get_text_from_root}};


pub struct ImageMessage {
    pub msg_id: u64,
    pub pic_url: String,
    pub media_id: String,
}

impl EventMessage for ImageMessage {
    type ReceivedMessage = Self;

    fn from_xml(node: &Node) -> SdkResult<Self::ReceivedMessage> {
	    let msg_id = get_number_from_root::<u64>(&node, "MsgId")?;
        let pic_url = get_text_from_root(&node, "PicUrl")?;
        let media_id = get_text_from_root(&node, "MediaId")?;
        
        Ok(ImageMessage {
            msg_id,
            pic_url: pic_url.to_owned(),
            media_id: media_id.to_owned(),
        })
    }
}