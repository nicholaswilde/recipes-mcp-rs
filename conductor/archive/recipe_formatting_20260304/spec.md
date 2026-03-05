# Specification: Recipe Formatting Support

**Track ID:** `recipe_formatting_20260304`

## Overview
Implement a "format" action for the `manage_recipes` tool to allow users to export recipes in different text formats (e.g., Markdown).

## Goals
1. **Human-Readable Export:** Convert a structured recipe to a clean, readable text format.
2. **Markdown Support:** Primary support for Markdown (ideal for Obsidian/Notion users).
3. **Plain-Text Support:** Basic readable format.

## Functional Requirements
- **Action Update:** Add a `"format"` action to the `manage_recipes` tool.
- **Input Parameters:** Add `format_type: String` to the tool's input schema.
- **Formatting Logic:** Create templates or logic for Markdown and plain-text conversion.
- **Output:** Return formatted strings instead of (or in addition to) the JSON objects.

## Acceptance Criteria
- A user can request a recipe in Markdown and receive a properly formatted document.
- All recipe components (name, ingredients, steps, etc.) are included in the export.
