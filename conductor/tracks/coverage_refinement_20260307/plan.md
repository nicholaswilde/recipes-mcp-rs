# Implementation Plan: Coverage Refinement (90%)

This plan outlines the strategy and tasks to increase the code coverage of the Recipes MCP Server to at least 90%, following the project's TDD and quality guidelines.

## Phase 1: Baseline Analysis & Prioritization
In this phase, we will establish the current coverage baseline and identify the specific files and modules that require the most attention.

- [x] Task: Baseline Analysis
    - [x] Run `task coverage` to get the current overall and per-file coverage.
    - [x] Identify top 5 files with lowest coverage in the `src/` directory.
    - [x] Document the baseline in a new `coverage_report.md` (temporary).
- [x] Task: Testability Review
    - [x] Analyze identified files for testability (e.g., hardcoded dependencies).
    - [x] Create missing test files where necessary.
- [ ] Task: Conductor - User Manual Verification 'Baseline Analysis & Prioritization' (Protocol in workflow.md)

## Phase 2: Core Logic Enhancement
This phase focuses on the core business logic: scraping, parsing, scaling, and conversion.

- [ ] Task: Scraper & Parser Coverage
    - [ ] Write failing unit tests for edge cases in `src/scraper.rs` and `src/lib.rs` (parsing).
    - [ ] Implement code to pass tests and ensure branch coverage.
- [ ] Task: Scaling & Conversion Coverage
    - [ ] Write failing unit tests for complex unit conversions in `src/scaling.rs` and `src/conversion/`.
    - [ ] Implement code to pass tests.
- [ ] Task: Integration Tests for Core Tools
    - [ ] Add integration tests in `tests/` to verify `manage_recipes` tool with various configurations.
- [ ] Task: Conductor - User Manual Verification 'Core Logic Enhancement' (Protocol in workflow.md)

## Phase 3: Broad Coverage Refinement
This phase targets remaining modules and ensures all edge cases and error paths are covered.

- [ ] Task: Config & Transport Coverage
    - [ ] Ensure `src/config.rs` and `src/transport/` have comprehensive tests for all supported formats and transport modes.
- [ ] Task: Error Handling & Resilience
    - [ ] Specifically target error paths (e.g., network failures, malformed input) to ensure they are exercised by tests.
- [ ] Task: Final Coverage Sweep
    - [ ] Run `task coverage:report` and address any remaining "red" lines or uncovered branches.
- [ ] Task: Conductor - User Manual Verification 'Broad Coverage Refinement' (Protocol in workflow.md)

## Phase 4: Finalization & Upload
The final phase verifies the 90% target and syncs with Coveralls.

- [ ] Task: Final Verification
    - [ ] Confirm `cargo llvm-cov` reports >= 90% total workspace coverage.
- [ ] Task: Coveralls Sync
    - [ ] Run `task coverage:upload` to push the final results.
- [ ] Task: Cleanup
    - [ ] Remove any temporary analysis files.
- [ ] Task: Conductor - User Manual Verification 'Finalization & Upload' (Protocol in workflow.md)
