//! A connection as both sides see it, and the WebSocket upgrade query.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Wire format a connection expects for push messages.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionFormat {
    #[default]
    Json,
    Binary,
}

impl ConnectionFormat {
    pub fn from_str_opt(s: Option<&str>) -> Self {
        match s {
            Some("binary") => Self::Binary,
            _ => Self::Json,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Binary => "binary",
        }
    }
}

/// A connection as the app sees it (and as it is pushed on `server-info`
/// and in the optional `socket/connected` first frame).
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionInfo {
    pub connection_id: String,
    pub user_id: String,
    pub ip_address: Option<String>,
    pub authenticated: bool,
    pub subscriptions: Vec<String>,
    pub format: ConnectionFormat,
    pub data: Value,
    pub connected: bool,
    pub server_id: String,
    pub server_session_id: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

/// Query string of the WebSocket upgrade. One of `connection_id` (from a
/// prior registration) or `subscriptions` (inline registration) is required.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeQuery {
    /// A connection id from a prior REST registration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    /// Comma-separated channels: registers inline, for clients with no HTTP
    /// step (ESP32 and friends).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<String>,
    /// `json` (default) or `binary`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// A JWT (HS256, `sub` claim); when the server has a socket secret, a
    /// valid token sets `user_id` and `authenticated` at upgrade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl UpgradeQuery {
    pub fn for_connection(connection_id: &str) -> Self {
        Self {
            connection_id: Some(connection_id.into()),
            ..Default::default()
        }
    }

    pub fn inline(subscriptions: &[&str], format: ConnectionFormat) -> Self {
        Self {
            subscriptions: Some(subscriptions.join(",")),
            format: Some(format.as_str().into()),
            ..Default::default()
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.into());
        self
    }
}
