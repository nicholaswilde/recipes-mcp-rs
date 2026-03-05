# Implementation Plan: Dietary Metadata Support

## Phase 1: Data Model and Logic [checkpoint: dc44f85]
- [x] Task: Update `Recipe` Data Model
    - [ ] Add `nutrition` and `diets` fields to `Recipe` in `src/scraper.rs`.
    - [ ] Update `From` implementations to handle these fields.
    - [ ] **TDD:** Write unit tests to verify dietary metadata extraction.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Data Model and Logic' (Protocol in workflow.md)

## Phase 2: System Validation [checkpoint: 67c5f40]
- [x] Task: System Integration and Quality Check
    - [ ] Verify scraping with recipes containing nutrition data.
    - [ ] Run `task test:ci` to ensure everything is correct.
- [x] Task: Conductor - User Manual Verification 'Phase 2: System Validation' (Protocol in workflow.md)
