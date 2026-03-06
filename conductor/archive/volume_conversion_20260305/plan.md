# Implementation Plan: Metric/Imperial Volume Conversion

## Phase 1: Enhanced Volumetric Parsing
- [x] Task: Expand `VolumetricAmount` and `parse_ingredient`
    - [x] Update `parse_ingredient` to support more units: `ml`, `l`, `liter`, `fl oz`, `oz`, `pint`, `pt`, `quart`, `qt`, `gallon`, `gal`.
    - [x] **TDD:** Write unit tests for the new units and edge cases (e.g., "500ml", "8 fl oz").

## Phase 2: Core Conversion Engine
- [x] Task: Implement Volume Conversion Logic
    - [x] Create `src/conversion/volume.rs` (or add to `engine.rs`) to handle system conversions.
    - [x] Define conversion constants (e.g., 1 cup = 236.588 ml).
    - [x] Implement `convert_volume(amount, target_unit)` function.
    - [x] **TDD:** Write unit tests for various conversion paths (Metric -> Imperial, Imperial -> Metric).

## Phase 3: Integration and Formatting
- [x] Task: Update Formatting Logic
    - [x] Update `src/formatter.rs` or `src/conversion/engine.rs` to support optional volume conversion in recipe strings.
    - [x] **TDD:** Write unit tests for formatted output (e.g., "250 ml (1.06 cups) milk").

## Phase 4: Standalone Tool Update
- [x] Task: Update `manage_recipes` / `convert_ingredients`
    - [x] Add support for target unit system in the MCP tools.
    - [x] **TDD:** Integration tests for the tool logic.
