use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ClientMessage {
    #[serde(rename = "messageType")]
    pub message_type: String,
    pub data: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ServerMessage {
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(default)]
    pub data: String,
    #[serde(rename = "dataArray", default)]
    pub data_array: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct ChatMessage {
    pub from: String,
    pub message: String,
    pub time: u64,
}

impl ChatMessage {
    /// Format timestamp (milliseconds) ke "HH:MM"
    pub fn formatted_time(&self) -> String {
        let secs = self.time / 1000;
        let minutes = (secs % 3600) / 60;
        let hours = (secs % 86400) / 3600;
        // Offset UTC+7 (WIB)
        let hours_wib = (hours + 7) % 24;
        format!("{:02}:{:02}", hours_wib, minutes)
    }
}