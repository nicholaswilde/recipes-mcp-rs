# Implementation Plan: Search/Scrape Caching Layer

## Phase 1: Architecture & Data Structures
- [x] Task: Create `src/cache.rs` and define `CacheEntry<T>` struct with TTL support.
- [x] Task: Define `RecipeCache` trait with methods for getting and setting search and scrape results.
- [x] Task: Implement `FileRecipeCache` using `serde_json` for persistence.
- [~] Task: Conductor - User Manual Verification 'Architecture & Data Structures'

## Phase 2: Search Caching Integration
- [ ] Task: Update `search_recipes` in `src/search.rs` to check the cache before making requests.
- [ ] Task: Update `search_recipes` to store successful results in the cache.
- [ ] Task: Add unit tests for search caching.
- [ ] Task: Conductor - User Manual Verification 'Search Caching Integration'

## Phase 3: Scrape Caching Integration
- [ ] Task: Update `scrape_recipe` in `src/scraper.rs` to check the cache before making requests.
- [ ] Task: Update `scrape_recipe` to store successful results in the cache.
- [ ] Task: Add unit tests for scrape caching.
- [ ] Task: Conductor - User Manual Verification 'Scrape Caching Integration'

## Phase 4: Persistence & TTL Management
- [ ] Task: Implement logic to handle expired entries (TTL).
- [ ] Task: Implement cache directory management (creation and initialization).
- [ ] Task: Conductor - User Manual Verification 'Persistence & TTL Management'

## Phase 5: Configuration & Tool Integration
- [ ] Task: Add caching options to `src/config.rs`.
- [ ] Task: Add `bypass_cache` flag to MCP tool parameters in `src/handler.rs`.
- [ ] Task: Final integration testing and cleanup.
- [ ] Task: Conductor - User Manual Verification 'Configuration & Tool Integration'
