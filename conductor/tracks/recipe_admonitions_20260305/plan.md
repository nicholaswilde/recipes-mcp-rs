# Implementation Plan: Recipe Admonition Extraction

## Phase 1: Data Modeling
- [ ] Task: Define `AdmonitionType` enum and `Admonition` struct in `src/scraper.rs`.
- [ ] Task: Update the `Recipe` struct to include an optional `Vec<Admonition>`.
- [ ] Task: Conductor - User Manual Verification 'Data Modeling' (Protocol in workflow.md)

## Phase 2: Scraper Enhancement
- [ ] Task: Update `RecipeSearchProvider` logic or tiered scraping to detect admonition sections in HTML.
- [ ] Task: Implement TDD for admonition parsing:
    - [ ] Sub-task: Write failing unit tests with mock HTML containing tips and notes.
    - [ ] Sub-task: Implement the parsing logic to extract text from detected containers.
    - [ ] Sub-task: Verify tests pass.
- [ ] Task: Conductor - User Manual Verification 'Scraper Enhancement' (Protocol in workflow.md)

## Phase 3: Tool & Handler Integration
- [ ] Task: Update `ManageRecipesArgs` in `src/handler.rs` to include `admonition_types`.
- [ ] Task: Update the `manage_recipes` tool definition in `src/handler.rs`.
- [ ] Task: Integrate the filtering logic in the handler to only include requested admonition types.
- [ ] Task: Conductor - User Manual Verification 'Tool & Handler Integration' (Protocol in workflow.md)

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
