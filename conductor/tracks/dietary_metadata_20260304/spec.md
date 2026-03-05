# Specification: Dietary Metadata Support

**Track ID:** `dietary_metadata_20260304`

## Overview
Expand the scraping capabilities to extract and return dietary and nutritional metadata.

## Goals
1. **Extract Nutrition:** Map the `NutritionInformation` property from `rust-recipe`.
2. **Identify Diets:** Map the `RestrictedDiet` property from `rust-recipe`.
3. **Enhance Recipe Data:** Provide more value for health-conscious users.

## Functional Requirements
- **Data Model Update:** Add `nutrition: Option<Nutrition>` and `diets: Vec<String>` to the `Recipe` struct.
- **Conversion Logic:** Update `From` implementations to handle nutrition and diet data.
- **Error Resilience:** Ensure the scraper still works even if metadata is missing.

## Acceptance Criteria
- The MCP server returns dietary information for recipes that provide it.
- Nutritional details are included in the JSON output.
