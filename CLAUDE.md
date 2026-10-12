# CLAUDE.md - cnctd_protocol

> Wire types shared by `cnctd_server` (the server framework) and every client
> of it. Deliberately no HTTP library: a client for an ESP32 or the Car
> Thing must not compile axum.

## Contents

| Module | Types |
|---|---|
| `response` | `SuccessCode`, `SuccessResponse`, `SocketResponse`; macros `success_response!`, `success_data!`, `success_msg!`, `created!` |
| `error` | `ErrorCode` (+ `TryFrom<u16>`, `FromStr` on the word, `as_word`, `status_code`), `ErrorResponse`; macros `not_found!`, `bad_request!`, `unauthorized!`, `forbidden!`, `conflict!`, `internal_server_error!`, ... |
| `message` | `Message` (server push), `SocketMessage` (client send), `UserMessage`, `UserSocketMessage` |
| `connection` | `ConnectionFormat`, `ConnectionInfo`, `UpgradeQuery` |
| `server_info` | `ServerInfo`, `ServerStatus` |
| `method` | `HttpMethod` (+ `from_http`/`to_http` over the `http` crate, `FromStr`) |

Everything is re-exported at the crate root too.

## Rules

- **A change here is a wire change.** It lands in `cnctd_server`, the
  TypeScript package and every Rust client. Add fields as `Option` or with
  defaults; never rename what is on the wire.
- **Macros expand to bare names** (`SuccessResponse`, `ErrorResponse`,
  `SuccessCode`, `ErrorCode`), so callers need those in scope. Thousands of
  call sites in cnctd.world rely on this.
- **Every wire type carries** `#[cfg_attr(feature = "ts", derive(ts_rs::TS),
  ts(export))]`. `scripts/export-ts.sh` emits the TypeScript; the
  `cnctd-server` npm package's `scripts/sync-types.sh` consumes it.
- Dependencies stay at serde, serde_json, chrono, http (and ts-rs behind
  `ts`).

## Development

```bash
cargo test
cargo test --features ts   # also runs the ts-rs export tests
scripts/export-ts.sh       # ./bindings/*.ts
```

---

*Part of the cnctd monorepo. See `../../../CLAUDE.md` for ecosystem context.*
