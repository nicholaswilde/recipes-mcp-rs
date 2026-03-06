# Specification: Cooklang Export Support

## Overview
This track implements support for exporting recipes in the [Cooklang](https://cooklang.org/) format. This allows users to export recipes scraped by the MCP server into a format compatible with Cooklang-enabled applications.

## Requirements

### 1. Format Conversion
- Implement a `to_cooklang` function in `src/formatter.rs` that takes a `Recipe` struct and returns a Cooklang-formatted string.
- Convert `Recipe` metadata (name, source, prep time, cook time, servings) into Cooklang metadata headers (`>> key: value`).

### 2. Ingredient Formatting
- Ingredients within instructions should be formatted using Cooklang syntax: `@name{amount%unit}`.
- If an ingredient name contains spaces, it MUST use the `{}` syntax: `@ingredient name{amount%unit}`.
- If amount or unit is missing, it should be omitted: `@ingredient`, `@ingredient{amount}`, or `@ingredient{%unit}`.

### 3. Cookware Extraction
- Attempt to identify cookware in instructions and format them as `#cookware`.
- If cookware name contains spaces, use the `{}` syntax: `#cookware name{}`.

### 4. Timer Extraction
- Identify timers in instructions and format them as `~name{duration%unit}`.

### 5. Instruction Processing
- Cooklang files are typically written as a series of paragraphs where each paragraph is a step.
- Instructions from the `Recipe` struct (which are currently a list of strings) should be joined by newlines.

## Technical Details
- **Module:** `src/formatter.rs`
- **Dependencies:** May require regex for identifying ingredients/cookware/timers within instruction strings if they aren't already structured.
- **Metadata Keys:**
    - `title`
    - `description`
    - `servings`
    - `prep_time`
    - `cook_time`
    - `total_time`
    - `source` (if URL is available)

## Acceptance Criteria
- [ ] `to_cooklang(&recipe)` produces a valid Cooklang string.
- [ ] Metadata is correctly formatted at the top of the file.
- [ ] Ingredients are correctly identified and formatted in the instructions.
- [ ] Tests verify correct formatting for various input recipes.
