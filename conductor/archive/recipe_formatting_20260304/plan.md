# Implementation Plan: Recipe Formatting Support

## Phase 1: Formatting Logic and Tool Update [checkpoint: 82aa5b2]
- [x] Task: Create Markdown Template Utility
    - [x] Create a utility to format `Recipe` structs into Markdown.
    - [x] **TDD:** Write unit tests for the Markdown output.
- [x] Task: Update `manage_recipes` Tool
    - [x] Add the `"format"` action to `src/main.rs`.
    - [x] Update the `ManageRecipesArgs` struct and the tool's JSON schema.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Formatting Logic and Tool Update' (Protocol in workflow.md)

## Phase 2: Validation
- [x] Task: System Integration and Quality Check
    - [x] Verify multiple formats (Markdown, Plain-Text).
    - [x] Run `task test:ci` to ensure everything is correct.
- [x] Task: Conductor - User Manual Verification 'Phase 2: Validation' (Protocol in workflow.md)
