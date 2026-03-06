# Specification: Recipe Admonition Extraction (Tips & Notes)

## Goal
Enable the extraction and display of recipe-specific admonitions such as tips, notes, and variations from web pages.

## Functional Requirements
- **Extraction Scope**:
    - General Recipe Tips.
    - Ingredient or Step-specific Notes.
    - Recipe Variations and Substitutions.
- **Tool Integration**:
    - Update `manage_recipes` tool to accept an optional `admonition_types` argument (array of strings: `tips`, `notes`, `variations`).
    - Defaults to extracting all available types if the feature is implicitly active or requested.
- **Scraper Implementation**:
    - Update the scraping engine to identify common HTML patterns for tips and notes (e.g., `div.recipe-notes`, `div.tips`, `section.variations`).
    - Map extracted content to a new `admonitions` field in the `Recipe` struct.
- **Output Presentation (Hybrid Approach)**:
    - **Separate Section**: A "Tips & Notes" section in the Markdown output.
    - **Inlined Metadata**: If an admonition is explicitly tied to a step or ingredient, it should be associated with that item in the JSON output.

## Technical Requirements
- Update `src/scraper.rs` to include an `Admonition` struct and update the `Recipe` struct.
- Enhance the HTML parsing logic in `src/scraper.rs` to look for admonition containers.
- Update `src/formatter.rs` to include an "Admonitions" section in Markdown generation.
- Update `src/handler.rs` to support the new `admonition_types` argument.

## Acceptance Criteria
- Recipes scraped from supported sites (like AllRecipes or Food Network) include detected tips and notes.
- The `manage_recipes` tool correctly filters extraction based on `admonition_types`.
- Markdown output clearly displays a "Tips & Notes" section when data is present.
- Unit tests verify the parsing of various admonition formats.

## Out of Scope
- Extracting full "headnotes" or lengthy author stories that are purely narrative and not instructional.
- Sentiment analysis of the notes.
