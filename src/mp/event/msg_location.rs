// Removed roxmltree, SdkResult, xmlutil, ReceivedMessageParser imports

#[derive(Debug, Clone)] // Added derive
pub struct LocationMessage {
    // pub msg_id: u64, // Removed: MsgId is now part of ReceivedEvent
    pub location_x: String, // Changed from f32 to String to align with wx_message
    pub location_y: String, // Changed from f32 to String
    pub scale: String,      // Changed from f32 to String
    pub label: String,
}

// Removed ReceivedMessageParser impl block.
// Removed from_xml method.
// Removed cfg(test) mod tests block.
