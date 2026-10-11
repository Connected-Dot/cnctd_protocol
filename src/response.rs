//! The JSON success envelope and the macros that build it.

use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum SuccessCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
}

impl SuccessCode {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.as_u16()).expect("success codes are valid")
    }

    /// 0.6 name for [`Self::status_code`]; kept so consumers compile.
    #[deprecated(since = "1.0.0", note = "use status_code()")]
    pub fn to_warp_status_code(&self) -> StatusCode {
        self.status_code()
    }
}

/// The JSON envelope every successful API call returns:
/// `{"success": true, "status": "OK", "msg": ..., "data": ...}`.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SuccessResponse {
    pub success: bool,
    pub status: SuccessCode,
    pub msg: Option<String>,
    pub data: Option<Value>,
}

impl SuccessResponse {
    pub fn new(status: Option<SuccessCode>, msg: Option<String>, data: Option<Value>) -> Self {
        Self {
            success: true,
            status: status.unwrap_or(SuccessCode::OK),
            msg,
            data,
        }
    }
}

/// Reply shape for request/response over the WebSocket, keyed by the
/// caller's `response_channel`.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SocketResponse {
    pub success: bool,
    pub msg: Option<String>,
    pub data: Option<Value>,
    pub response_channel: String,
}

impl SocketResponse {
    pub fn success(msg: Option<String>, data: Option<Value>, response_channel: String) -> Self {
        Self {
            success: true,
            msg,
            data,
            response_channel,
        }
    }

    pub fn failure(msg: Option<String>, data: Option<Value>, response_channel: String) -> Self {
        Self {
            success: false,
            msg,
            data,
            response_channel,
        }
    }
}

// The macros expand to bare `SuccessResponse` / `SuccessCode` names, so the
// caller needs those in scope. That has been the contract since 0.1 and
// every call site in cnctd.world relies on it.

#[macro_export]
macro_rules! success_response {
    () => {
        SuccessResponse::new(None, None, None)
    };
    ($status:expr) => {
        SuccessResponse::new(Some($status), None, None)
    };
    ($status:expr, $msg:expr) => {
        SuccessResponse::new(Some($status), Some($msg.to_string()), None)
    };
    ($status:expr, $msg:expr, $data:expr) => {
        SuccessResponse::new(Some($status), Some($msg.to_string()), Some($data))
    };
}

#[macro_export]
macro_rules! success_data {
    ($data:expr) => {
        SuccessResponse::new(None, None, Some($data))
    };
}

#[macro_export]
macro_rules! success_msg {
    ($msg:expr) => {
        SuccessResponse::new(None, Some($msg.to_string()), None)
    };
}

#[macro_export]
macro_rules! created {
    ($msg:expr, $data:expr) => {
        SuccessResponse::new(Some(SuccessCode::Created), Some($msg.to_string()), Some($data))
    };
}
