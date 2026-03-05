# Specification: Advanced Dietary Filtering

## Goals
- Filter search results and scraped recipes based on dietary preferences.
- Use metadata like `is_vegan`, `is_gluten_free`, etc.

## Requirements
- Update `manage_recipes` to accept a `dietary_filters` list.
- Implement filtering in `search` and `scrape` actions.
- Surface dietary information in formatted output (e.g., Markdown).

## Proposed Tech Stack
- Existing `scraper` metadata (Phase 2 completion).
