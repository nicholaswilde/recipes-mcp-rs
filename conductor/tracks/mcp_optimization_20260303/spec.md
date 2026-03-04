# Specification: MCP Token Optimization

**Track ID:** `mcp_optimization_20260303`

## Overview
Optimize the Recipes MCP server for token efficiency by consolidating granular tools into unified, action-oriented tools and supporting bulk operations. This pattern reduces the token overhead of the `list_tools` response and allows for more efficient interaction between the AI and the server.

## Goals
1.  **Unified Tool Pattern:** Replace individual tools with a single `manage_recipes` tool using an `action` parameter.
2.  **Bulk Operation Support:** Enable scraping multiple URLs in a single tool call.
3.  **Schema Optimization:** Refine tool descriptions and JSON schemas for conciseness.

## Functional Requirements
-   **New Tool:** `manage_recipes`
    -   **Action:** `scrape`
    -   **Arguments:** `urls` (Array of Strings)
-   **Refactor:** The existing `scrape_recipe` logic must be adapted to handle a list of URLs.
-   **Error Handling:** The tool should return a summary of successes and failures for bulk operations.

## Acceptance Criteria
-   The `list_tools` response contains only the `manage_recipes` tool.
-   The `manage_recipes` tool successfully parses multiple URLs in one call.
-   The server maintains backward compatibility for internal logic while exposing the new interface.
