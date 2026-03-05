# Specification: Volumetric to Weight Conversion

**Track ID:** `weight_conversion_20260304`

## Overview
Enhance the scraping process to automatically convert volumetric measurements (cups, tablespoons, etc.) to weight (grams) using the King Arthur ingredient weight chart. This improves accuracy for bakers and provides more professional data extraction.

## Goals
1. **Automated Conversion:** Integrate weight conversion into the primary scraping lifecycle.
2. **Multi-tiered Data Source:** 
    - **Tier 1:** Local look-up (hardcoded or static file) based on the King Arthur chart.
    - **Tier 2:** Dynamic look-up on King Arthur's website if local match fails.
    - **Tier 3:** Google search fallback for obscure ingredients.
3. **Smart Matching:** Implement "best guess" logic for ambiguous ingredients (e.g., matching "flour" to "All-Purpose Flour").

## Functional Requirements
- **Ingredient Parsing:** Extract volume and ingredient names from strings (e.g., "1 cup flour").
- **Conversion Engine:** Apply the weight chart ratio to the volume.
- **Data Augmentation:** Update the `ingredients` list in the `Recipe` struct to include both volume and weight.
- **Formatting:** Append the weight in parentheses (e.g., "1 cup (120g) all-purpose flour").

## Non-Functional Requirements
- **Latency:** Tier 1 look-ups should be near-instant. Tier 2/3 should be cached to minimize network calls.
- **Resilience:** If conversion fails completely, return the original volumetric string.

## Acceptance Criteria
- Recipes containing volumetric measurements for standard baking ingredients (flour, sugar, etc.) return strings with gram weights appended.
- The conversion logic handles common units: cups, tablespoons, teaspoons.
- Ingredients not found in the local chart are successfully resolved via online fallbacks or remain unchanged.

## Out of Scope
- Support for non-culinary units.
- Mass-to-volume conversion (reverse).
- Nutritional calculation based on weight (covered in a separate track).
