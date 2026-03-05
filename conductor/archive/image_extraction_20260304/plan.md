# Implementation Plan: Image Extraction Support

## Phase 1: Data Model and Logic [checkpoint: f7767b5]
- [x] Task: Update `Recipe` Data Model
    - [x] Add `image_url` field to `Recipe` in `src/scraper.rs`.
    - [x] Update `From` implementations to handle image extraction.
    - [x] **TDD:** Write unit tests to verify image URL extraction for both scrapers.
- [x] Task: Conductor - User Manual Verification 'Phase 1: Data Model and Logic' (Protocol in workflow.md)

## Phase 2: System Validation [checkpoint: b9be324]
- [x] Task: System Integration and Quality Check
    - [x] Verify scraping with image-rich recipes (e.g., AllRecipes).
    - [x] Run `task test:ci` to ensure everything is correct.
- [x] Task: Conductor - User Manual Verification 'Phase 2: System Validation' (Protocol in workflow.md)
