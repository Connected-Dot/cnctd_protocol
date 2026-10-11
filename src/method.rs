use std::fmt;

use serde::{Deserialize, Serialize};

/// The methods the API prefix answers. Anything else is a 405.
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
}

impl HttpMethod {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GET => "GET",
            Self::POST => "POST",
            Self::PUT => "PUT",
            Self::DELETE => "DELETE",
        }
    }

    pub fn from_http(method: &http::Method) -> Option<Self> {
        match *method {
            http::Method::GET => Some(Self::GET),
            http::Method::POST => Some(Self::POST),
            http::Method::PUT => Some(Self::PUT),
            http::Method::DELETE => Some(Self::DELETE),
            _ => None,
        }
    }

    pub fn to_http(&self) -> http::Method {
        match self {
            Self::GET => http::Method::GET,
            Self::POST => http::Method::POST,
            Self::PUT => http::Method::PUT,
            Self::DELETE => http::Method::DELETE,
        }
    }

    /// GET and DELETE carry their data in the query string; POST and PUT in
    /// the body.
    pub fn has_body(&self) -> bool {
        matches!(self, Self::POST | Self::PUT)
    }
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for HttpMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            other => Err(format!("not an API method: {}", other)),
        }
    }
}
