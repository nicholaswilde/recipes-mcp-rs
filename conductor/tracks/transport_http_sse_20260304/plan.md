# Implementation Plan: HTTP and SSE Transport Mode Support

## Phase 1: Preparation
- [ ] Add `axum`, `tower-http`, and `futures` to `Cargo.toml`.
- [ ] Update `Args` in `src/config.rs` to include a `--transport` flag (default: `stdio`) and optional `--port` (default: `3000`).

## Phase 2: Refactoring
- [ ] Abstract the request handling logic in `src/main.rs` into a standalone function or module if needed to be reused between stdio and HTTP.
- [ ] Move tool implementation to a more modular structure if necessary for clarity.

## Phase 3: Implementation
- [ ] Create an `http_transport` module (e.g., in `src/transport/http.rs`).
- [ ] Implement the Axum routes:
    - POST `/message`: For receiving MCP messages.
    - GET `/sse`: For handling Server-Sent Events.
- [ ] Integrate with the existing `handle_request` logic.

## Phase 4: Integration
- [ ] Update `main.rs` to start either the stdio loop or the Axum server based on the CLI arguments.
- [ ] Ensure proper shutdown handling for the HTTP server.

## Phase 5: Testing
- [ ] Add unit tests for `Args` parsing (new `--port` flag).
- [ ] Add unit tests for `handle_request` in a standalone module.
- [ ] Create integration tests for Axum routes using `axum::test_helpers` or `tower::ServiceExt`.
- [ ] Mock MCP client to verify SSE message delivery and endpoint URL initialization.

## Phase 6: Verification
- [ ] Verify `stdio` mode still works correctly with `task test`.
- [ ] Run new integration tests for HTTP/SSE.
- [ ] Manual verification of HTTP mode using `curl` or an MCP client that supports HTTP/SSE.
