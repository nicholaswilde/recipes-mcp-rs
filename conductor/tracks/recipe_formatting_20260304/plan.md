# Implementation Plan: Recipe Formatting Support

## Phase 1: Formatting Logic and Tool Update [checkpoint: ]
- [ ] Task: Create Markdown Template Utility
    - [ ] Create a utility to format `Recipe` structs into Markdown.
    - [ ] **TDD:** Write unit tests for the Markdown output.
- [ ] Task: Update `manage_recipes` Tool
    - [ ] Add the `"format"` action to `src/main.rs`.
    - [ ] Update the `ManageRecipesArgs` struct and the tool's JSON schema.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Formatting Logic and Tool Update' (Protocol in workflow.md)

## Phase 2: Validation
- [ ] Task: System Integration and Quality Check
    - [ ] Verify multiple formats (Markdown, Plain-Text).
    - [ ] Run `task test:ci` to ensure everything is correct.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Validation' (Protocol in workflow.md)
