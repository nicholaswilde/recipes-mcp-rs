# Specification: HTTP and SSE Transport Mode Support

## Goals
- Add support for HTTP and SSE (Server-Sent Events) transport modes to the Recipes MCP Server.
- Maintain existing stdio transport support.
- Use `axum` for the HTTP/SSE implementation.

## Requirements
- The server should switch between stdio and HTTP/SSE modes based on a configuration flag or CLI argument.
- Implement the MCP HTTP transport specification:
    - POST `/message`: Handle individual JSON-RPC messages.
    - GET `/sse`: Initialize a Server-Sent Events connection.
- Ensure all existing tools (`manage_recipes`) are available through both transport modes.
- Log incoming requests and outgoing responses in HTTP mode for debugging.

## Proposed Tech Stack
- `axum`: For the web server and SSE handling.
- `tower-http`: For CORS and other middleware if needed.
- `tokio`: Existing async runtime.
