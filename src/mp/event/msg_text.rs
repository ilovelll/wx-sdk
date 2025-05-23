// Removed roxmltree, SdkError, SdkResult, ReceivedMessageParser imports as they are no longer used here.
// The SdkError and SdkResult might be needed if this module had other functions returning results,
// but for a simple struct definition, they are not.

#[derive(Debug, Clone)] // Added derive for consistency and usability
pub struct TextMessage {
    // pub msg_id: u64, // Removed: MsgId is now part of ReceivedEvent
    pub content: String,
}

// Removed ReceivedMessageParser impl block.
// Removed from_xml method.

// Removed old test function `parse()` that relied on from_xml.
// Testing for text message parsing is handled by wx-message crate
// and integration is tested via ReceivedEvent::parse.
