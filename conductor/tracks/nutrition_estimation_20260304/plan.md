# Implementation Plan: Recipe Nutrition Estimation

## Phase 1: Data Modeling [checkpoint: 7c5ade3b59c8aaca6db938a75528f502d7753aee]
- [x] Define the `NutritionalInfo` struct for macro/micronutrients.
- [x] Create an initial internal dataset of nutritional values for common ingredients (matching `WeightChart`). (COMPLETED)

## Phase 2: Engine Implementation [checkpoint: c6f73b23dd418afb39a2cc5b2e15018567eec8d2]
- [x] Implement a `calculate_nutrition` function that uses ingredient weights to calculate totals. (COMPLETED)
- [x] Handle missing nutritional data gracefully. (COMPLETED)

## Phase 3: Integration
- [ ] Add the `nutrition` action to the `manage_recipes` tool.
- [ ] Update the action handler in `src/main.rs`.
- [ ] Update the `formatter` to include a nutrition table.

## Phase 4: Testing
- [ ] Unit test the calculation logic for a simple recipe.
- [ ] Verify handling of multiple servings.

## Phase 5: Verification
- [ ] Verify the estimated nutritional values are consistent with the ingredient weights.
