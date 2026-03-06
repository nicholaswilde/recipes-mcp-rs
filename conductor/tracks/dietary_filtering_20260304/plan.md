# Implementation Plan: Advanced Dietary Filtering

## Phase 1: Preparation [checkpoint: 924097d213c6e3a5c04cbdc3d01613f551480e19]
- [x] Define the `DietaryFilters` struct and enums. (COMPLETED)
- [x] Update the tool schema for `manage_recipes`. (COMPLETED)

## Phase 2: Implementation
- [ ] Update `search_recipes` to handle filtering after fetching results.
- [ ] Update `scrape_recipes` to optionally filter out non-compliant recipes.

## Phase 3: Integration
- [ ] Surface dietary metadata in the `formatter`.
- [ ] Update the `search` and `scrape` tool handlers in `src/main.rs`.

## Phase 4: Testing
- [ ] Test filtering behavior for `vegan` and `gluten-free`.
- [ ] Test formatting when dietary info is present.

## Phase 5: Verification
- [ ] Verify the tool only returns recipes that meet the specified filters.
