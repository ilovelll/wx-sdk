// Removed roxmltree, SdkResult, xmlutil, ReceivedMessageParser imports

#[derive(Debug, Clone)] // Added derive
pub struct LinkMessage {
    // pub msg_id: u64, // Removed: MsgId is now part of ReceivedEvent
    pub title: String,
    pub description: String,
    pub url: String,
}

// Removed ReceivedMessageParser impl block.
// Removed from_xml method.
// Removed cfg(test) mod tests block.
