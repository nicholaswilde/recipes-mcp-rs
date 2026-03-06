# Specification: Fix Search Provider Tests

## Goal
Restore the functionality of the recipe search providers (AllRecipes, Serious Eats, Food Network) by updating their extraction logic and selectors to match the current HTML structure of their websites.

## Problem Description
- `test_search_recipes_not_empty` (AllRecipes) is failing.
- `test_search_serious_eats` is failing.
- `test_search_food_network` is currently ignored and known to be failing.
- Failures are manifesting as empty result vectors, indicating that the CSS selectors are likely outdated.

## Proposed Changes
- **Research & Analysis**:
    - Inspect the current search result HTML for AllRecipes, Food Network, and Serious Eats.
    - Identify valid, stable CSS selectors for recipe titles and URLs.
- **Implementation**:
    - Update `src/search.rs` with the new selectors.
    - Improve error handling to distinguish between "No Results Found" and "Request Blocked/Malformed".
    - Add enhanced debug logging (using the `tracing` crate) to capture HTML snippets or lengths when parsing fails.
- **Verification**:
    - Re-enable the Food Network test.
    - Ensure all three tests pass consistently.

## Acceptance Criteria
- `cargo test search::tests` passes for all three providers.
- Debug logging provides enough context to diagnose future failures without manual browser inspection.
- The extraction logic handles variations in result formatting (e.g., sponsored ads vs. actual results).

## Out of Scope
- Adding new search providers.
- Implementing a headless browser for scraping (sticking to `reqwest` + `scraper`).
