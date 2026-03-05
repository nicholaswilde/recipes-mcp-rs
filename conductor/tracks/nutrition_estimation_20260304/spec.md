# Specification: Recipe Nutrition Estimation

## Goals
- Estimate calories and macronutrients for a recipe based on ingredients and weights.

## Requirements
- Map ingredient weights to nutritional profiles.
- New action: `nutrition` in `manage_recipes` tool.
- Input: `urls` or `recipes`.
- Output: Estimated nutritional breakdown (Calories, Fat, Carbs, Protein).

## Proposed Tech Stack
- Existing `conversion::engine`.
- Nutritional data source (internal lookup or mock).
