# Implementation Plan: MCP Token Optimization

## Phase 1: Core Logic Refactoring [checkpoint: 04ddfa3]
- [~] Task: Update `src/scraper.rs` for bulk operations
    - [ ] Refactor `scrape_recipe` to be used internally by a new bulk function.
    - [ ] Implement `scrape_recipes` (plural) that takes a `Vec<String>` and returns a map of results.
    - [x] **TDD:** Write unit tests for the bulk scraping logic.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Core Logic Refactoring' (Protocol in workflow.md)

## Phase 2: MCP Tool Implementation [checkpoint: 679b3be]
- [ ] Task: Update `src/main.rs` to use the unified tool
    - [ ] Implement the `manage_recipes` tool definition and handler.
    - [ ] Implement the `action`-based dispatch logic.
    - [ ] Remove the old `scrape_recipe` tool registration.
    - [x] **TDD:** Write integration tests (manual or automated) for the new tool schema.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: MCP Tool Implementation' (Protocol in workflow.md)

## Phase 3: Quality and Cleanup [checkpoint: f1d1dd0]
- [x] Task: Final Quality Audit
    - [ ] Run `task test:ci` to ensure all tests pass and coverage is maintained.
    - [ ] Verify `list_tools` output via manual execution.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Quality and Cleanup' (Protocol in workflow.md)
