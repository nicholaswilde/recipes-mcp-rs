# Tech Stack: Recipes MCP Server

## Core Language
- **Rust:** The primary language for high-performance and safe data processing.

## Frameworks and SDKs
- **mcp-sdk-rust:** For implementing the Model Context Protocol (MCP) server interface.
- **rust-recipe:** To leverage existing recipe parsing and data structures.
- **recipe-scraper:** For extracting recipe information from diverse web sources.
- **scraper:** For HTML parsing and CSS selectors (used for King Arthur weight chart scraping).
- **async-trait:** For defining asynchronous traits (e.g., RecipeSearchProvider).

## Configuration & Data Formats
- **Serde:** For seamless serialization and deserialization of all data structures.
- **TOML:** Support for configuration files in `.toml` format.
- **YAML:** Support for configuration files in `.yaml` or `.yml` format.
- **JSON:** Support for configuration files in `.json` format.

## CLI & Environment Variables
- **Clap:** For robust command-line argument parsing and help generation.
- **Dotenv / Envy:** For managing environment-based configuration and secrets.
- **urlencoding:** For safe encoding of search queries in URLs.

## Networking & IO
- **Reqwest:** For fetching recipe web pages (likely a dependency for the scrapers).
- **Axum:** For implementing the HTTP and SSE transport server.
- **Tower-HTTP:** For CORS and other middleware support.
- **Futures:** For asynchronous stream handling (SSE).
- **Tokio:** The asynchronous runtime for non-blocking IO operations.

## Build & Task Runner
- **Task (go-task):** Task runner for automating build, test, and development workflows (`Taskfile.yml`).

## Testing & Quality
- **Cargo Test:** Standard Rust testing framework.
- **Tarpaulin:** For code coverage reporting.
