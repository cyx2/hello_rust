# Welcome to hello_rust

This is an application being built from scratch, that is being used to test AI software enginering tooling and processes. Nearly all code written in this repo is / will be written by agentic coding tools. Current stack is OpenAI Codex (web, cli), and Github Copilot (ide).

Specification for a Rust-based API gateway that fronts MongoDB CRUD operations is as follows.

## Overview

The gateway exposes REST endpoints that match the MongoDB Rust driver’s CRUD surface. Clients send JSON payloads describing the target database, collection, filters, and options; the service forwards those inputs directly to the driver while returning structured responses and errors. Connectivity targets a single MongoDB cluster at a time and is configured entirely by environment variables.

## Product Requirements

### Core Behavior

- Use the official MongoDB Rust driver to perform CRUD operations (create, read, update, delete) against one cluster at a time.
- Support every primary CRUD capability the driver offers (single/multi inserts, finds, updates, replacements, deletes); defer advanced features (aggregations, transactions, bulk writes, change streams) until later phases.
- Keep endpoint semantics, payloads, option names, and responses 1:1 with the driver so exposing new driver features requires minimal API redesign.
- Require callers to provide `database` and `collection` identifiers on each request to control the namespace.
- Assume no authentication for MVP: requests are accepted without API keys, OAuth, or session state.
- Favor ergonomic JSON payloads familiar to MongoDB users (BSON-like documents, filter syntax, option maps).
- Produce meaningful HTTP status codes plus machine-readable error payloads so clients understand validation versus driver failures.

### CRUD Endpoints

All endpoints live under `/api/v1`, expect `Content-Type: application/json`, and must include `database` and `collection` fields. Each payload also accepts an `options` object that mirrors the driver’s method-specific options.

#### Insert One — `POST /api/v1/documents/insert-one`
```json
{ "database": "app", "collection": "users", "document": { ... }, "options": { ...InsertOneOptions } }
```
Returns `{ "inserted_id": "..." }`.

#### Insert Many — `POST /api/v1/documents/insert-many`
```json
{ "database": "app", "collection": "users", "documents": [ { ... } ], "options": { ...InsertManyOptions } }
```
Returns `{ "inserted_ids": ["...", "..."] }`.

#### Find One — `POST /api/v1/documents/find-one`
```json
{ "database": "app", "collection": "users", "filter": { ... }, "options": { ...FindOneOptions } }
```
Returns `{ "document": { ... } }` or `404` when no match exists.

#### Find Many — `POST /api/v1/documents/find-many`
```json
{
  "database": "app",
  "collection": "users",
  "filter": { ... },
  "options": { "projection": { ... }, "sort": { ... }, "limit": 10, "skip": 0 }
}
```
Returns `{ "documents": [ { ... }, { ... } ] }`.

#### Update One — `POST /api/v1/documents/update-one`
```json
{ "database": "app", "collection": "users", "filter": { ... }, "update": { ... }, "options": { "upsert": false, ...UpdateOptions } }
```
Returns driver-style `UpdateResult` fields (`matched_count`, `modified_count`, `upserted_id`).

#### Update Many — `POST /api/v1/documents/update-many`
Same shape as Update One; performs multi updates and returns driver `UpdateResult`.

#### Replace One — `POST /api/v1/documents/replace-one`
```json
{ "database": "app", "collection": "users", "filter": { ... }, "replacement": { ... }, "options": { ...ReplaceOptions } }
```
Returns driver `UpdateResult` semantics for replacements.

#### Delete One — `POST /api/v1/documents/delete-one`
```json
{ "database": "app", "collection": "users", "filter": { ... }, "options": { ...DeleteOptions } }
```
Returns `{ "deleted_count": 1 }`.

#### Delete Many — `POST /api/v1/documents/delete-many`
Same shape as Delete One; returns driver `DeleteResult`.

#### Collection Metadata Helper — `GET /api/v1/collections?database=app`
Lists collections within the specified database so clients can discover available namespaces.

### Error Contract

- `400 Bad Request` for validation failures (missing database/collection, malformed filters, invalid options) with payload `{ "error": "validation_error", "details": "..." }`.
- `404 Not Found` when operations expecting a document (find-one, update with `upsert=false`, delete-one) match nothing.
- `502 Bad Gateway` for MongoDB driver or network failures; include sanitized driver messages and correlation IDs.
- `500 Internal Server Error` for unexpected failures; always emit a correlation ID to aid debugging.

## Technical Requirements

### Configuration & Environment

- All connection details (`MONGODB_URI`, default database/collection, pool sizes, timeouts) must come from environment variables—no inline or hard-coded defaults.
- Load the `.env` file automatically on startup (e.g., `dotenvy::dotenv().ok();`) so running the binary without extra flags always picks up local configuration.
- Provide a `.env.example` documenting required keys:
  - `MONGODB_URI`, `MONGODB_DEFAULT_DATABASE`, `MONGODB_DEFAULT_COLLECTION`
  - Pool sizing knobs (`MONGODB_POOL_MIN_SIZE`, `MONGODB_POOL_MAX_SIZE`)
  - Timeout settings (`MONGODB_CONNECT_TIMEOUT_MS`, `MONGODB_SERVER_SELECTION_TIMEOUT_MS`)
  - Logging verbosity (`LOG_LEVEL`)
- Optional tuning knobs (retry behavior, read preference) should also be expressed via env vars to avoid recompilation for config changes.

### Startup Expectations

- Initialize logging and configuration only after environment variables are loaded to guarantee parity between development and production.
- Validate mandatory env vars on startup and fail fast with actionable error messages if any are missing.
- The service assumes one MongoDB cluster per process. To change clusters, update `.env` values and restart the gateway.

### Developer Setup

- Provide a `launch.json` under `.vscode/` configured to run `cargo run` with `.env` loading so the service is debuggable in VS Code without manual configuration.
- Document any required tooling (`rustup`, `cargo`, `mongodb` CLI) and commands (`cargo fmt`, `cargo clippy -- -D warnings`, `cargo test`) that developers should run before submitting changes.
- Every PR must include appropriate unit/integration tests and demonstrate passing coverage via `cargo test`; new functionality without tests is not acceptable.
- Ensure the README or a CONTRIBUTING file references the `.env.example` and describes how to copy it to `.env` for local development.

## Local Development

1. Install the Rust toolchain (Rustup/Cargo) and ensure MongoDB is available for integration testing.
2. Copy the sample environment: `cp .env.example .env` and update the values to match your MongoDB cluster and desired pool/timeouts.
3. Run the server with `cargo run`. The gateway binds to the address specified in `APP_BIND_ADDRESS` (defaults to `127.0.0.1:3000`).

### Required Environment Variables

The application reads all configuration from the environment. The `.env.example` file documents every supported knob:

- `MONGODB_URI`, `MONGODB_DEFAULT_DATABASE`, `MONGODB_DEFAULT_COLLECTION`
- Pool sizing: `MONGODB_POOL_MIN_SIZE`, `MONGODB_POOL_MAX_SIZE`
- Timeouts: `MONGODB_CONNECT_TIMEOUT_MS`, `MONGODB_SERVER_SELECTION_TIMEOUT_MS`
- Logging: `LOG_LEVEL`
- HTTP binding: `APP_BIND_ADDRESS`

### Development Commands

Run the following before sending a change for review:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

## Debugging in VS Code

The repo includes `.vscode/launch.json` configured to start the gateway with environment variables sourced from the active shell. Use the “Debug Mongo Gateway” configuration to run with `cargo run` semantics directly from the editor.
