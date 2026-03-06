# Implementation Plan: HTTP and SSE Transport Mode Support

## Phase 1: Preparation [checkpoint: 9e7f8be8dd2f6529ab3c1c026e3fde332fabfc3c]
- [x] Add `axum`, `tower-http`, and `futures` to `Cargo.toml`.
- [x] Update `Args` in `src/config.rs` to include a `--transport` flag (default: `stdio`) and optional `--port` (default: `3000`).

## Phase 2: Refactoring [checkpoint: 18b1f29938c8cf9844b7a2436c0547ccc4908425]
- [x] Abstract the request handling logic in `src/main.rs` into a standalone function or module if needed to be reused between stdio and HTTP.
- [x] Move tool implementation to a more modular structure if necessary for clarity.

## Phase 3: Implementation
- [x] Create an `http_transport` module (e.g., in `src/transport/http.rs`).
- [x] Implement the Axum routes:
    - POST `/message`: For receiving MCP messages.
    - GET `/sse`: For handling Server-Sent Events.
- [x] Integrate with the existing `handle_request` logic.

## Phase 4: Integration
- [x] Update `main.rs` to start either the stdio loop or the Axum server based on the CLI arguments.
- [x] Ensure proper shutdown handling for the HTTP server.

## Phase 5: Testing
- [x] Add unit tests for `Args` parsing: Verify `--port` and `--transport` flags are correctly parsed in `src/config.rs`.
- [x] Add unit tests for `handle_request`: Verify the refactored request handler independently of any transport mode.
- [x] Add unit tests for transport switch: Verify the application correctly chooses the transport mode based on configuration.
- [x] Create integration tests for HTTP Server Lifecycle: Verify the Axum server starts and stops gracefully.
- [x] Create integration tests for `POST /message`:
    - [x] Test sending a valid MCP `tools/list` request and getting a valid response.
    - [x] Test error handling for malformed JSON or invalid MCP messages.
- [x] Create integration tests for `GET /sse`:
    - [x] Verify the SSE stream starts correctly with `Content-Type: text/event-stream`.
    - [x] Verify the first message in the SSE stream contains the server's endpoint URL.
- [x] Create integration test for Tool Execution: Call `manage_recipes` tool through HTTP transport and verify results.

## Phase 6: Verification
- [x] Verify `stdio` mode still works correctly with `task test`.
- [x] Run new integration tests for HTTP/SSE.
- [x] Manual verification of HTTP mode using `curl` or an MCP client that supports HTTP/SSE.
