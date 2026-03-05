# Specification: Enhanced Multi-Crate Scraping

**Track ID:** `multi_crate_scraping_20260303`

## Overview
Implement a tiered scraping strategy to improve the reliability and success rate of recipe extraction. This approach uses two different libraries to handle diverse website structures and metadata formats.

## Goals
1.  **Primary/Fallback Logic:** Use `rust-recipe` as the primary scraper and `recipe-scraper` as a fallback.
2.  **Increased Coverage:** Support more websites by leveraging different extraction algorithms.
3.  **Data Normalization:** Ensure consistent output regardless of which scraper succeeds.

## Functional Requirements
-   **Scraping Flow:**
    1.  Attempt scraping with `rust-recipe::scrape_recipe_from_url`.
    2.  If it fails or returns no ingredients/instructions, fetch the raw HTML.
    3.  Attempt scraping with `recipe_scraper::SchemaOrgEntry::scrape_html`.
    4.  Combine/prefer the best available data.
-   **Conversion Logic:** Implement `From` or conversion helpers for `recipe_scraper::SchemaOrgRecipe`.

## Acceptance Criteria
-   The MCP server successfully returns recipes from sites where one crate might fail but the other succeeds.
-   The output schema remains identical for both scrapers.
-   Logs indicate which scraper was used for each request.
