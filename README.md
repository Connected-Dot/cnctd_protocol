# cnctd_protocol

The wire contract between a [`cnctd_server`](https://crates.io/crates/cnctd_server)
and its clients, as one small crate with no HTTP library: the JSON envelopes
(`SuccessResponse`, `ErrorResponse`) and the macros that build them
(`success_data!`, `bad_request!`, ...), the WebSocket shapes
(`SocketMessage`, `Message`, `SocketResponse`), `ConnectionInfo`,
`ConnectionFormat`, `UpgradeQuery`, `ServerInfo`, `HttpMethod`.

`cnctd_server` re-exports all of it at its usual paths. Clients
(`cnctd_client`, the Car Thing, ESP32 firmware) depend on this crate alone.

With feature `ts`, `scripts/export-ts.sh [out-dir]` writes a TypeScript
definition per type (ts-rs); the `cnctd-server` npm package is generated
from them.

MIT
