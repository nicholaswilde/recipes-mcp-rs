# Implementation Plan: Recipe Scaling Support

## Phase 1: Scaling Logic and Tool Update [checkpoint: ]
- [x] Task: Implement Quantity Parsing Utility
    - [x] Create a utility to extract and scale numbers in strings.
    - [x] **TDD:** Write unit tests for various quantity formats (1, 1.5, 1/2, 2 1/4).
- [x] Task: Update `manage_recipes` Tool
    - [x] Add the `"scale"` action to `src/main.rs`.
    - [x] Update the `ManageRecipesArgs` struct and the tool's JSON schema.
- [x] Task: Integrate Scaling into Scraper
    - [x] Add a `scale` method to the `Recipe` struct.
- [~] Task: Conductor - User Manual Verification 'Phase 1: Scaling Logic and Tool Update' (Protocol in workflow.md)

## Phase 2: Validation
- [ ] Task: System Integration and Quality Check
    - [ ] Verify scaling with real-world recipes.
    - [ ] Run `task test:ci` to ensure everything is correct.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Validation' (Protocol in workflow.md)
