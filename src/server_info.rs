//! What a server says about itself on the `server-info` heartbeat channel.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub id: String,
    /// Fresh per process start.
    pub session_id: String,
    pub local_ip: String,
    /// What the app configured; empty when it did not.
    pub public_ip: String,
    pub port: u16,
    pub last_heartbeat: DateTime<Utc>,
    pub start_time: DateTime<Utc>,
    /// One-minute load average.
    pub cpu_usage: f32,
    /// Percent of physical memory in use.
    pub memory_usage: f32,
    pub status: ServerStatus,
    pub max_connections: usize,
    pub current_connections: usize,
    pub websocket: bool,
    pub redis_url: Option<String>,
    pub redis_active: bool,
}

#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    Active,
    Idle,
    ShuttingDown,
}
