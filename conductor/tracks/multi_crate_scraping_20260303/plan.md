# Implementation Plan: Enhanced Multi-Crate Scraping

## Phase 1: Logic and Conversion Refactoring
- [ ] Task: Implement conversion for `recipe-scraper` models
    - [ ] Create conversion logic from `recipe_scraper::SchemaOrgRecipe` to our internal `Recipe` struct.
    - [ ] **TDD:** Write unit tests for the conversion.
- [ ] Task: Implement Tiered Scraping Logic
    - [ ] Refactor `scrape_recipe` in `src/scraper.rs` to include fallback logic.
    - [ ] Add explicit HTML fetching for the fallback scraper.
    - [ ] **TDD:** Write unit tests for the fallback mechanism (using mocks if possible).
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Logic and Conversion Refactoring' (Protocol in workflow.md)

## Phase 2: Integration and Validation
- [ ] Task: Final System Verification
    - [ ] Run `task test:ci` to ensure all tests pass.
    - [ ] Manually verify scraping with a known "hard" recipe URL.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Integration and Validation' (Protocol in workflow.md)
