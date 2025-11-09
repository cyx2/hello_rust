# Welcome to hello_rust

This application is a Rust-based API gateway that exposes MongoDB CRUD operations over HTTP. It is also a testbed for AI-driven engineering workflows, but end users can treat it as a regular service for forwarding REST calls to a single MongoDB cluster.

## Overview
- REST endpoints live under `/api/v1` and mirror MongoDB driver semantics (insert/find/update/replace/delete).
- Clients send JSON bodies containing `database`, `collection`, filters, updates, and optional driver-style options.
- Responses return driver-shaped payloads (`inserted_id`, `UpdateResult`, `DeleteResult`, etc.) plus clear HTTP status codes.

See `AGENTS.md` for the complete product specification and deeper engineering notes.

## Prerequisites
- Rust toolchain (`rustup`, `cargo`, Rust 1.75+ recommended).
- A reachable MongoDB cluster (local or remote) and credentials/URI.
- (Optional) `mongodb` CLI for verifying connectivity.

## Configuration
All settings are environment-driven. Copy the sample file and customize it for your cluster:
```bash
cp .env.example .env
```
Key variables:
- `MONGODB_URI`: Full connection string including credentials and options.
- `MONGODB_DEFAULT_DATABASE`, `MONGODB_DEFAULT_COLLECTION`: Defaults applied when requests omit them (if supported).
- `MONGODB_POOL_MIN_SIZE`, `MONGODB_POOL_MAX_SIZE`: Driver connection pooling.
- `MONGODB_CONNECT_TIMEOUT_MS`, `MONGODB_SERVER_SELECTION_TIMEOUT_MS`: Driver timeout knobs.
- `LOG_LEVEL`: `trace|debug|info|warn|error`.
- `APP_BIND_ADDRESS`: Address/port the HTTP server listens on (defaults to `127.0.0.1:3000`).

Optional knobs such as retry behavior or read preference can also be expressed via env vars (see `AGENTS.md`).

## Running the Gateway
1. Install dependencies and configure `.env` as above.
2. Start the service:
   ```bash
   cargo run
   ```
3. The server binds to `APP_BIND_ADDRESS`. Verify readiness via `curl http://127.0.0.1:3000/health` (or your configured port) once a health endpoint is implemented.

## API Quick Reference
- `POST /api/v1/documents/insert-one|insert-many`
- `POST /api/v1/documents/find-one|find-many`
- `POST /api/v1/documents/update-one|update-many`
- `POST /api/v1/documents/replace-one`
- `POST /api/v1/documents/delete-one|delete-many`
- `GET /api/v1/collections?database=...`

Each request body must include `database` and `collection`; optional `options` objects follow the respective MongoDB driver structs. Refer to `AGENTS.md` for payload examples and error contracts.

## Troubleshooting
- Ensure MongoDB is reachable from the host running the gateway; driver errors surface as `502` responses with sanitized messages.
- Missing or invalid env vars cause a startup failure with actionable log output.
- If you encounter inconsistencies between this README and `AGENTS.md`, defer to `AGENTS.md` and open a PR to reconcile both.
