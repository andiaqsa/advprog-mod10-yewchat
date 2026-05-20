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