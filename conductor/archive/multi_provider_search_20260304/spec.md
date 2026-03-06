# Specification: Multi-Provider Recipe Search

## Goals
- Support multiple recipe providers beyond AllRecipes.
- Allow users to specify which provider to search.

## Requirements
- Update `search_recipes` in `src/search.rs` to handle multiple providers.
- Implement scrapers for:
    - Food Network
    - Serious Eats
- Add a `provider` argument to the `search` action in `manage_recipes` tool.

## Proposed Tech Stack
- `reqwest`, `scraper` (existing).
