# Specification: Recipe Scaling Support

**Track ID:** `recipe_scaling_20260304`

## Overview
Implement a "scale" action for the `manage_recipes` tool to allow users to adjust ingredient quantities for a different number of servings.

## Goals
1. **Parse Quantities:** Accurately identify numeric quantities in ingredient strings.
2. **Calculate Ratios:** Compute the scaling factor based on original and target servings.
3. **Format Output:** Return the updated ingredient list with adjusted quantities.

## Functional Requirements
- **Action Update:** Add a `"scale"` action to the `manage_recipes` tool.
- **Input Parameters:** Add `target_servings: u32` to the tool's input schema.
- **Scaling Logic:** Implement a utility to parse and scale numbers within strings (e.g., "1.5 cups" scaled by 2 becomes "3 cups").
- **Fallback:** If a quantity cannot be parsed, return the original string to avoid data loss.

## Acceptance Criteria
- The MCP server correctly scales a known recipe (e.g., doubling ingredients for 2 servings to 4).
- The output clearly indicates the scaled quantities.
- The system handles complex quantities (fractions, decimals).
