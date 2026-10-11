//! WebSocket message shapes.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// What the server pushes to subscribers of a channel:
/// `{"channel": ..., "instruction": ..., "data": ...}`.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
    pub channel: String,
    pub instruction: String,
    pub data: Option<Value>,
}

impl Message {
    pub fn new(channel: &str, instruction: &str, data: Option<Value>) -> Self {
        Self {
            channel: channel.into(),
            instruction: instruction.into(),
            data,
        }
    }
}

#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserMessage {
    pub data_type: Option<String>,
    pub data: Option<Value>,
    pub user_id: String,
}

#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSocketMessage {
    pub receiver_id: String,
    pub message: Message,
}

/// What clients send: a channel, an optional instruction and data, and an
/// optional `response_channel` for request/response over the socket.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocketMessage {
    pub channel: String,
    pub instruction: Option<String>,
    pub data: Option<Value>,
    pub response_channel: Option<String>,
}

impl SocketMessage {
    pub fn new(channel: &str, instruction: Option<&str>, data: Option<Value>) -> Self {
        Self {
            channel: channel.into(),
            instruction: instruction.map(str::to_string),
            data,
            response_channel: None,
        }
    }

    pub fn with_response_channel(mut self, response_channel: &str) -> Self {
        self.response_channel = Some(response_channel.into());
        self
    }
}
