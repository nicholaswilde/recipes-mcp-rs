# Specification: Image Extraction Support

**Track ID:** `image_extraction_20260304`

## Overview
Expand the scraping capabilities to extract image URLs from recipe pages, providing a more visual and complete data structure for recipe importers.

## Goals
1.  **Extract Primary Image:** Identify the main recipe image from both scrapers.
2.  **Schema.org Support:** Map the `image` property from `recipe-scraper::SchemaOrgRecipe`.
3.  **Rust-Recipe Support:** If available, map the image URL from `rust_recipe::RecipeInformationProvider`.
4.  **Graceful Handling:** Ensure image extraction is optional and doesn't cause overall scraping to fail.

## Functional Requirements
-   **Data Model Update:** Add an `image_url: Option<String>` field to the `Recipe` struct.
-   **Conversion Logic:** Update `From<Box<dyn RecipeInformationProvider>>` and `From<SchemaOrgRecipe>` for the `Recipe` struct.
-   **Error Resilience:** If no image is found, the field should simply be `None`.

## Acceptance Criteria
-   The MCP server returns a valid `image_url` for websites that provide it.
-   Recipes without images still scrape correctly.
-   The output JSON includes the `image_url` field.
