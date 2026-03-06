# Specification: Search/Scrape Caching Layer

## Goal
Improve performance and reduce external load by implementing a local caching layer for search results and scraped recipes.

## Functional Requirements
- Implement a cache for JSON responses from search providers.
- Implement a cache for scraped `Recipe` objects.
- Support TTL (Time To Live) for cache entries.
- Allow clearing the cache via a CLI argument or environment variable.

## Technical Requirements
- Use a library like `mini-moka` for in-memory caching or SQLite for persistent caching.
- Integrate the cache into the `search_recipes` and `scrape_recipe` functions.
