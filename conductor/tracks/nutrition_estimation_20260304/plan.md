# Implementation Plan: Recipe Nutrition Estimation

## Phase 1: Data Modeling
- [x] Define the `NutritionalInfo` struct for macro/micronutrients.
- [x] Create an initial internal dataset of nutritional values for common ingredients (matching `WeightChart`). (COMPLETED)

## Phase 2: Engine Implementation
- [ ] Implement a `calculate_nutrition` function that uses ingredient weights to calculate totals.
- [ ] Handle missing nutritional data gracefully.

## Phase 3: Integration
- [ ] Add the `nutrition` action to the `manage_recipes` tool.
- [ ] Update the action handler in `src/main.rs`.
- [ ] Update the `formatter` to include a nutrition table.

## Phase 4: Testing
- [ ] Unit test the calculation logic for a simple recipe.
- [ ] Verify handling of multiple servings.

## Phase 5: Verification
- [ ] Verify the estimated nutritional values are consistent with the ingredient weights.
