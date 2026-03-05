# Implementation Plan: Image Extraction Support

## Phase 1: Data Model and Logic [checkpoint: ]
- [ ] Task: Update `Recipe` Data Model
    - [ ] Add `image_url` field to `Recipe` in `src/scraper.rs`.
    - [ ] Update `From` implementations to handle image extraction.
    - [ ] **TDD:** Write unit tests to verify image URL extraction for both scrapers.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Data Model and Logic' (Protocol in workflow.md)

## Phase 2: System Validation
- [ ] Task: System Integration and Quality Check
    - [ ] Verify scraping with image-rich recipes (e.g., AllRecipes).
    - [ ] Run `task test:ci` to ensure everything is correct.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: System Validation' (Protocol in workflow.md)
