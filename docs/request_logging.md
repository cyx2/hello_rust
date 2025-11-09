# Request Logging Architecture

This document captures the structure and rationale behind the per-request logging that now wraps every MongoDB gateway endpoint.

## Objectives

- Surface actionable telemetry for each HTTP call so operators can trace payloads, status codes, and failure reasons without enabling verbose driver logs.
- Keep instrumentation orthogonal to business logic by centralizing reusable helpers rather than duplicating logging statements across handlers.
- Preserve the existing error model and response contract while enriching emitted spans and log records with endpoint context.

## Entry Points

All instrumentation is owned by `src/routes.rs` where every handler resides.

- `router` wires all REST paths through the shared `AppState` and is the sole place routes are registered. This guarantees that helpers declared in the same module are available to every endpoint.
- Each handler is annotated with `#[instrument(skip_all)]` from `tracing` to suppress automatic argument recording. This keeps the span lean while allowing manual control over which payload fields are logged.

## Logging Helpers

Three helper functions shape the lifecycle logs for a request:

1. `log_request_received(endpoint, request)` – emits an `info` log on the `http` target as soon as Axum deserializes the payload. The handler passes the strongly typed request body, which derives `Debug`, so JSON fields appear in structured form. [`src/routes.rs`](../src/routes.rs)
2. `log_request_success(endpoint, status, response)` – records successful completions, including the final HTTP status and the serialized response payload. Responses are wrapped in `axum::Json`, so the helper accepts anything implementing `Debug` without leaking internal state. [`src/routes.rs`](../src/routes.rs)
3. `log_request_failure(endpoint, error)` – promotes failure handling to a single branch. It logs a `warn` record tagged with the failing endpoint, the error’s HTTP status (exposed via `ApiError::status`), and the full `ApiError` for diagnostics before returning the same error to Axum. [`src/routes.rs`](../src/routes.rs)

By returning the error from `log_request_failure`, handlers can use the helper inline inside `Result::map_err` without altering the control flow. This keeps MongoDB driver errors and validation failures flowing through the existing `ApiResult` alias while attaching consistent telemetry.

## Handler Flow

Every handler follows the same structure:

1. Call `log_request_received` immediately after destructuring the `Json<T>` payload.
2. Use `collection_from_state` to resolve the MongoDB collection, logging validation failures if the namespace is incomplete.
3. Execute the driver call, mapping driver errors through `log_request_failure` after wrapping them with `map_driver_error` to sanitize the message.
4. Construct the response DTO and pass it to `log_request_success` with the appropriate `StatusCode` before returning it wrapped in `Json`.

This pattern produces a minimal log sequence for the happy path and ensures every early return or `Err` branch emits a failure log.

## Error Surface

`ApiError::status` moved out of the `#[cfg(test)]` block in `src/error.rs` so the logging helpers can emit the HTTP code associated with each error variant. The actual JSON response remains unchanged because `IntoResponse` is still implemented directly on `ApiError`. [`src/error.rs`](../src/error.rs)

## Configuration Integration

`src/main.rs` continues to drive runtime configuration through `Config::from_env`. Log verbosity is controlled via the existing `LOG_LEVEL` environment variable, interpreted by `tracing_subscriber::EnvFilter`. Operators can increase verbosity to `debug` or `trace` without code changes. [`src/main.rs`](../src/main.rs)

## Extending the Pattern

- New endpoints should live in `src/routes.rs`, call the same helper trio, and prefer returning `ApiResult<Json<_>>` so logging stays uniform.
- Cross-cutting metadata (e.g., correlation IDs) can be added by extending the helpers, giving all handlers richer logs with a single change.
- When adding request types, ensure `Debug` is derived so payloads render cleanly in logs. The `models` module already derives `Debug` for every request/response struct. [`src/models.rs`](../src/models.rs)

## Operational Impact

The logging scheme provides a balanced trade-off between observability and noise:

- Operators see exactly three structured log lines per request (receive, success, or failure) on the `info` channel, making it easy to follow individual calls in production logs.
- Validation issues (400), not-found cases (404), and driver failures (502) now emit `warn` logs that include the HTTP code and full error object, enabling faster debugging without inspecting HTTP traces.
- Because the helpers avoid capturing references to the `AppState` or MongoDB client, there is no risk of accidental data cloning or contention in the logging path.

This documentation should be reviewed alongside the route module when introducing new endpoints to ensure future work continues to conform to the established logging contract.
