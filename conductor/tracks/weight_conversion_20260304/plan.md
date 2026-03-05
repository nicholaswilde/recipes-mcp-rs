# Implementation Plan: Volumetric to Weight Conversion

## Phase 1: Local Conversion Core [checkpoint: 1d02430]
- [x] Task: Implement Weight Chart Data Structure (Local Look-up)
    - [x] Create `src/conversion/data.rs` with hardcoded King Arthur chart entries.
    - [x] Add support for loading an external `config/weights.json` file.
    - [x] **TDD:** Write unit tests for data retrieval.
- [x] Task: Implement Volumetric Parsing Utility
    - [x] Create `src/conversion/parser.rs` to extract amounts and units from ingredient strings.
    - [x] **TDD:** Write unit tests for various string formats ("1 cup", "2 tbsp", "3/4 tsp").
- [x] Task: Implement Conversion Logic (Tier 1)
    - [x] Create `src/conversion/engine.rs` to apply ratios to parsed volumes.
    - [x] **TDD:** Write unit tests for local look-up conversion.
- [~] Task: Conductor - User Manual Verification 'Phase 1: Local Conversion Core' (Protocol in workflow.md)

## Phase 2: Online Fallbacks [checkpoint: ]
- [ ] Task: Implement King Arthur Web Look-up (Tier 2)
    - [ ] Implement scraper for `https://www.kingarthurbaking.com/learn/ingredient-weight-chart`.
    - [ ] **TDD:** Mock web responses and test extraction.
- [ ] Task: Implement Google Search Fallback (Tier 3)
    - [ ] Integrate a search mechanism to find weights for missing ingredients.
    - [ ] **TDD:** Verify search query generation and result parsing.
- [ ] Task: Implement "Best Guess" and Result Parsing
    - [ ] Logic to select the most likely match from search results.
    - [ ] **TDD:** Test ambiguity resolution (e.g., "flour" defaults to AP).
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Online Fallbacks' (Protocol in workflow.md)

## Phase 3: Integration and Normalization [checkpoint: ]
- [ ] Task: Integrate Conversion into `scrape_recipe` flow
    - [ ] Update `src/scraper.rs` to call the conversion engine for each ingredient.
    - [ ] **TDD:** Integration tests for full scraping-to-conversion flow.
- [ ] Task: Implement Output Formatting
    - [ ] Logic to append weight in parentheses to original string.
    - [ ] **TDD:** Verify final string formatting.
- [ ] Task: Final System Verification
    - [ ] Run `task test:ci` and perform manual verification with complex recipes.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Integration and Normalization' (Protocol in workflow.md)
