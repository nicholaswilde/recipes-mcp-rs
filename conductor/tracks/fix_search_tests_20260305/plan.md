# Implementation Plan: Fix Search Provider Tests

## Phase 1: Research & Debugging
- [ ] Task: Add temporary debug logging to `src/search.rs` to capture HTML content from each provider.
- [ ] Task: Execute search tests and analyze the output to identify new selectors.
- [ ] Task: Conductor - User Manual Verification 'Research & Debugging' (Protocol in workflow.md)

## Phase 2: AllRecipes Fix
- [ ] Task: Update `AllRecipesProvider` with new CSS selectors.
- [ ] Task: Implement TDD for AllRecipes:
    - [ ] Sub-task: Verify `test_search_recipes_not_empty` fails before fix (already confirmed).
    - [ ] Sub-task: Apply selector changes.
    - [ ] Sub-task: Verify tests pass.
- [ ] Task: Conductor - User Manual Verification 'AllRecipes Fix' (Protocol in workflow.md)

## Phase 3: Serious Eats Fix
- [ ] Task: Update `SeriousEatsProvider` with new CSS selectors.
- [ ] Task: Implement TDD for Serious Eats:
    - [ ] Sub-task: Verify `test_search_serious_eats` fails before fix (already confirmed).
    - [ ] Sub-task: Apply selector changes.
    - [ ] Sub-task: Verify tests pass.
- [ ] Task: Conductor - User Manual Verification 'Serious Eats Fix' (Protocol in workflow.md)

## Phase 4: Food Network Fix
- [ ] Task: Update `FoodNetworkProvider` with new CSS selectors and re-enable `test_search_food_network`.
- [ ] Task: Implement TDD for Food Network:
    - [ ] Sub-task: Remove `#[ignore]` from the test.
    - [ ] Sub-task: Apply selector changes.
    - [ ] Sub-task: Verify tests pass.
- [ ] Task: Conductor - User Manual Verification 'Food Network Fix' (Protocol in workflow.md)

## Phase 5: Robustness & Logging
- [ ] Task: Clean up debug logging and add structured `tracing` events.
- [ ] Task: Improve error reporting for blocked requests vs. empty results.
- [ ] Task: Conductor - User Manual Verification 'Robustness & Logging' (Protocol in workflow.md)
