//! The wire contract between a `cnctd_server` and its clients, in one small
//! crate with no HTTP library: the JSON envelopes and the macros that build
//! them, the WebSocket message shapes, connection and server info, and the
//! upgrade query.
//!
//! `cnctd_server` re-exports everything here at its 0.6 paths; `cnctd_client`
//! and the Car Thing / ESP32 code depend on this crate alone. With feature
//! `ts`, every type exports a TypeScript definition (`scripts/export-ts.sh`).

pub mod connection;
pub mod error;
pub mod message;
pub mod method;
pub mod response;
pub mod server_info;

pub use connection::{ConnectionFormat, ConnectionInfo, UpgradeQuery};
pub use error::{ErrorCode, ErrorResponse};
pub use message::{Message, SocketMessage, UserMessage, UserSocketMessage};
pub use method::HttpMethod;
pub use response::{SocketResponse, SuccessCode, SuccessResponse};
pub use server_info::{ServerInfo, ServerStatus};
