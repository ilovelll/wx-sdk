// Removed roxmltree, SdkResult, xmlutil, ReceivedMessageParser imports

#[derive(Debug, Clone)] // Added derive
pub struct ImageMessage {
    // pub msg_id: u64, // Removed: MsgId is now part of ReceivedEvent
    pub pic_url: String,
    pub media_id: String,
}

// Removed ReceivedMessageParser impl block.
// Removed from_xml method.

// Removed cfg(test) mod tests block.
