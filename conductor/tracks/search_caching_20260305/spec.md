# Specification: Search/Scrape Caching Layer

## Goal
Implement a caching layer to store results from recipe searches and individual recipe scrapes. This will improve responsiveness for repeated queries and reduce the load on external recipe providers, mitigating rate-limiting risks.

## Problem Description
Currently, every search or scrape request triggers an external network call. This is inefficient for repeated requests and increases the likelihood of being blocked by providers (as seen in the "Fix Search Provider Tests" track). There is no mechanism to reuse previously fetched data.

## Proposed Changes
- **Unified Cache Trait**: Define a trait for caching that can be implemented for different backends (in-memory, file-based).
- **File-Based Backend**: Implement a simple JSON file-based cache stored in a local directory (e.g., `.cache/`).
- **Search Caching**: Cache the `Vec<SearchResult>` returned by `search_recipes` based on query, limit, provider, and filters.
- **Scrape Caching**: Cache the `Recipe` struct returned by `scrape_recipe` based on the URL.
- **TTL (Time-To-Live)**: Implement basic expiration for cached items (e.g., 24 hours for searches, 7 days for recipes).
- **Configuration**: Allow enabling/disabling the cache and setting the cache directory via configuration.
- **Cache Bypass**: Allow bypassing the cache via a flag in the MCP tool parameters.

## Acceptance Criteria
- Search results are retrieved from the cache if a matching unexpired entry exists.
- Recipe data is retrieved from the cache if a matching unexpired entry exists.
- Cache misses trigger network requests and populate the cache.
- The cache can be disabled via configuration.
- No regressions in search or scraping functionality.
- Automated tests verify cache hits, misses, and expiration.

## Out of Scope
- Implementing a distributed cache (e.g., Redis).
- Advanced cache eviction policies (e.g., LRU by size).
- Caching of partial results or image data.
