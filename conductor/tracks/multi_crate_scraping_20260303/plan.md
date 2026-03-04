# Implementation Plan: Enhanced Multi-Crate Scraping

## Phase 1: Logic and Conversion Refactoring [checkpoint: f345767]
- [~] Task: Implement conversion for `recipe-scraper` models
    - [x] Create conversion logic from `recipe_scraper::SchemaOrgRecipe` to our internal `Recipe` struct.
    - [x] **TDD:** Write unit tests for the conversion.
- [x] Task: Implement Tiered Scraping Logic
    - [x] Refactor `scrape_recipe` in `src/scraper.rs` to include fallback logic.
    - [x] Add explicit HTML fetching for the fallback scraper.
    - [x] **TDD:** Write unit tests for the fallback mechanism (using mocks if possible).
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Logic and Conversion Refactoring' (Protocol in workflow.md)

## Phase 2: Integration and Validation
- [x] Task: Final System Verification
    - [x] Run `task test:ci` to ensure all tests pass.
    - [x] Manually verify scraping with a known "hard" recipe URL.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration and Validation' (Protocol in workflow.md)
