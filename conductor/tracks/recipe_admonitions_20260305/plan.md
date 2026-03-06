# Implementation Plan: Recipe Admonition Extraction

## Phase 1: Data Modeling [checkpoint: f7d74012584b61504038dcac833244021b1b5168]
- [x] Task: Define `AdmonitionType` enum and `Admonition` struct in `src/scraper.rs`. (COMPLETED)
- [x] Task: Update the `Recipe` struct to include an optional `Vec<Admonition>`. (COMPLETED)
- [x] Task: Conductor - User Manual Verification 'Data Modeling' (Protocol in workflow.md) (COMPLETED)

## Phase 2: Scraper Enhancement [checkpoint: fb82358d4ed046bfd4470efe17a5401e4ad04d93]
- [x] Task: Update `RecipeSearchProvider` logic or tiered scraping to detect admonition sections in HTML. (COMPLETED)
- [x] Task: Implement TDD for admonition parsing: (COMPLETED)
    - [x] Sub-task: Write failing unit tests with mock HTML containing tips and notes. (COMPLETED)
    - [x] Sub-task: Implement the parsing logic to extract text from detected containers. (COMPLETED)
    - [x] Sub-task: Verify tests pass. (COMPLETED)
- [x] Task: Conductor - User Manual Verification 'Scraper Enhancement' (Protocol in workflow.md) (COMPLETED)

## Phase 3: Tool & Handler Integration
- [x] Task: Update `ManageRecipesArgs` in `src/handler.rs` to include `admonition_types`. (COMPLETED)
- [x] Task: Update the `manage_recipes` tool definition in `src/handler.rs`. (COMPLETED)
- [x] Task: Integrate the filtering logic in the handler to only include requested admonition types. (COMPLETED)
- [x] Task: Conductor - User Manual Verification 'Tool & Handler Integration' (Protocol in workflow.md) (COMPLETED)

## Phase 4: Output Formatting
- [ ] Task: Update `src/formatter.rs` to include a "Tips & Notes" section in the Markdown output.
- [ ] Task: Implement TDD for formatting:
    - [ ] Sub-task: Write tests for `to_markdown` with recipes containing different admonitions.
    - [ ] Sub-task: Update `to_markdown` to render the new section.
- [ ] Task: Conductor - User Manual Verification 'Output Formatting' (Protocol in workflow.md)

## Phase 5: Verification
- [ ] Task: Run integration tests scraping a real or complex mock recipe to verify end-to-end flow.
- [ ] Task: Verify that `admonition_types` CLI argument correctly filters output.
- [ ] Task: Conductor - User Manual Verification 'Verification' (Protocol in workflow.md)
