# Implementation Plan: Cooklang Export Support

## Phase 1: Implementation
- [ ] Task: Create a `cooklang` module in `src/formatter/` (if it exists) or update `src/formatter.rs`.
- [ ] Task: Implement logic to map `Recipe` fields to Cooklang syntax (e.g., `@ingredient{quantity%unit}`).

## Phase 2: Integration
- [ ] Task: Update `ManageRecipesArgs` and tool definition to include `cooklang`.

## Phase 3: Verification
- [ ] Task: Verify exported strings against Cooklang parsers.
