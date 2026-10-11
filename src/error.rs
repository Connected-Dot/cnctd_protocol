//! The JSON error envelope and the macros that build it.

use std::fmt;

use http::StatusCode;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    RequestTimeout = 408,
    Conflict = 409,
    PayloadTooLarge = 413,
    UnprocessableEntity = 422,
    TooManyRequests = 429,
    InternalServerError = 500,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
}

impl ErrorCode {
    pub const ALL: [ErrorCode; 14] = [
        Self::BadRequest,
        Self::Unauthorized,
        Self::Forbidden,
        Self::NotFound,
        Self::MethodNotAllowed,
        Self::RequestTimeout,
        Self::Conflict,
        Self::PayloadTooLarge,
        Self::UnprocessableEntity,
        Self::TooManyRequests,
        Self::InternalServerError,
        Self::BadGateway,
        Self::ServiceUnavailable,
        Self::GatewayTimeout,
    ];

    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.as_u16()).expect("error codes are valid")
    }

    /// 0.6 name for [`Self::status_code`]; kept so consumers compile.
    #[deprecated(since = "1.0.0", note = "use status_code()")]
    pub fn to_warp_status_code(&self) -> StatusCode {
        self.status_code()
    }

    pub fn from_status(status: StatusCode) -> Self {
        Self::from_u16(status.as_u16())
    }

    /// Unknown codes become 500. See [`TryFrom<u16>`] to know instead.
    pub fn from_u16(code: u16) -> Self {
        Self::try_from(code).unwrap_or(Self::InternalServerError)
    }

    /// The word the envelope carries (`"BadRequest"`).
    pub fn as_word(&self) -> &'static str {
        match self {
            Self::BadRequest => "BadRequest",
            Self::Unauthorized => "Unauthorized",
            Self::Forbidden => "Forbidden",
            Self::NotFound => "NotFound",
            Self::MethodNotAllowed => "MethodNotAllowed",
            Self::RequestTimeout => "RequestTimeout",
            Self::Conflict => "Conflict",
            Self::PayloadTooLarge => "PayloadTooLarge",
            Self::UnprocessableEntity => "UnprocessableEntity",
            Self::TooManyRequests => "TooManyRequests",
            Self::InternalServerError => "InternalServerError",
            Self::BadGateway => "BadGateway",
            Self::ServiceUnavailable => "ServiceUnavailable",
            Self::GatewayTimeout => "GatewayTimeout",
        }
    }
}

/// Exact mapping; an unlisted code is an error, unlike [`ErrorCode::from_u16`].
impl TryFrom<u16> for ErrorCode {
    type Error = u16;

    fn try_from(code: u16) -> Result<Self, u16> {
        Self::ALL.iter().copied().find(|c| c.as_u16() == code).ok_or(code)
    }
}

impl std::str::FromStr for ErrorCode {
    type Err = String;

    fn from_str(word: &str) -> Result<Self, String> {
        Self::ALL
            .iter()
            .copied()
            .find(|c| c.as_word() == word)
            .ok_or_else(|| format!("not an error code word: {}", word))
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_word())
    }
}

/// `{"success": false, "status": "BadRequest", "msg": ...}` with the HTTP
/// status to match. Also a plain `std::error::Error`, so `?` works in routes.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub success: bool,
    pub status: ErrorCode,
    pub msg: Option<String>,
}

impl ErrorResponse {
    /// No code means 404 — `not_found!` is the bare macro.
    pub fn new(error_code: Option<ErrorCode>, msg: Option<String>) -> Self {
        Self {
            success: false,
            status: error_code.unwrap_or(ErrorCode::NotFound),
            msg,
        }
    }
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.msg.clone().unwrap_or_default(), self.status)
    }
}

impl std::error::Error for ErrorResponse {}

#[macro_export]
macro_rules! error_response {
    () => {
        ErrorResponse::new(None, None)
    };
    ($error_code:expr, $msg:expr) => {
        ErrorResponse::new(Some($error_code), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! not_found {
    ($msg:expr) => {
        ErrorResponse::new(None, Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! unauthorized {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::Unauthorized), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! forbidden {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::Forbidden), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! bad_request {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::BadRequest), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! method_not_allowed {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::MethodNotAllowed), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! request_timeout {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::RequestTimeout), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! conflict {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::Conflict), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! too_many_requests {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::TooManyRequests), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! internal_server_error {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::InternalServerError), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! bad_gateway {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::BadGateway), Some($msg.to_string()))
    };
}

#[macro_export]
macro_rules! service_unavailable {
    ($msg:expr) => {
        ErrorResponse::new(Some(ErrorCode::ServiceUnavailable), Some($msg.to_string()))
    };
}
