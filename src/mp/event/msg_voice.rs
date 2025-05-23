// Removed roxmltree, SdkResult, xmlutil, ReceivedMessageParser imports

#[derive(Debug, Clone)] // Added derive
pub struct VoiceMessage {
    // pub msg_id: u64, // Removed: MsgId is now part of ReceivedEvent
    pub media_id: String,
    pub format: String,
    pub recognition: Option<String>, // This field was already present and matches wx_message
}

// Removed ReceivedMessageParser impl block.
// Removed from_xml method.
// Removed old test function `parse()`.
