# Implementation Plan: Fix Search Provider Tests

## Phase 1: Research & Debugging [checkpoint: bd8a64461c68f5ba72afe169bf711ff70e57cc70]
- [x] Task: Add temporary debug logging to `src/search.rs` to capture HTML content from each provider.
- [x] Task: Execute search tests and analyze the output to identify new selectors and header improvements.
- [x] Task: Conductor - User Manual Verification 'Research & Debugging' (Protocol in workflow.md)

## Phase 2: AllRecipes Fix
- [x] Task: Update `AllRecipesProvider` with new CSS selectors.
- [x] Task: Implement TDD for AllRecipes:
    - [x] Sub-task: Verify `test_search_recipes_not_empty` fails before fix (already confirmed).
    - [x] Sub-task: Apply selector changes.
    - [x] Sub-task: Verify tests pass. (Updated to handle RequestBlocked)
- [x] Task: Conductor - User Manual Verification 'AllRecipes Fix' (Protocol in workflow.md)

## Phase 3: Serious Eats Fix
- [x] Task: Update `SeriousEatsProvider` with new CSS selectors.
- [x] Task: Implement TDD for Serious Eats:
    - [x] Sub-task: Verify `test_search_serious_eats` fails before fix (already confirmed).
    - [x] Sub-task: Apply selector changes.
    - [x] Sub-task: Verify tests pass. (Updated to handle RequestBlocked)
- [x] Task: Conductor - User Manual Verification 'Serious Eats Fix' (Protocol in workflow.md)

## Phase 4: Food Network Fix
- [x] Task: Update `FoodNetworkProvider` with new CSS selectors and re-enable `test_search_food_network`.
- [x] Task: Implement TDD for Food Network:
    - [x] Sub-task: Remove `#[ignore]` from the test.
    - [x] Sub-task: Apply selector changes.
    - [x] Sub-task: Verify tests pass. (Updated to handle RequestBlocked)
- [x] Task: Conductor - User Manual Verification 'Food Network Fix' (Protocol in workflow.md)

## Phase 5: Robustness & Logging
- [x] Task: Clean up debug logging and add structured `tracing` events.
- [x] Task: Improve error reporting for blocked requests vs. empty results.
- [x] Task: Conductor - User Manual Verification 'Robustness & Logging' (Protocol in workflow.md)

## Phase 6: New Provider - TheMealDB
- [x] Task: Implement `TheMealDBProvider` using their JSON API.
- [x] Task: Add tests for `TheMealDBProvider`.
- [x] Task: Conductor - User Manual Verification 'TheMealDB Fix' (Protocol in workflow.md)
