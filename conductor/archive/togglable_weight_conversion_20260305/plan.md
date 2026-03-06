# Implementation Plan: Prioritized Weight Conversion Matching

## Phase 1: Implementation
- [x] Add "Powdered Sugar" to `standard_entries` in `src/conversion/data.rs`. (COMPLETED)
- [x] Modify `find_best_match` to sort keys by length descending. (COMPLETED)

## Phase 2: Testing & Verification
- [x] Add `test_find_best_match_prioritize_longer` to `src/conversion/data.rs`. (COMPLETED)
- [x] Add `test_find_best_match_prioritize_longer_partial` to `src/conversion/data.rs`. (COMPLETED)
- [x] Add `test_format_powdered_sugar` to `src/conversion/engine.rs` for integration verification. (COMPLETED)
- [x] Run unit tests and verify all pass. (COMPLETED)
