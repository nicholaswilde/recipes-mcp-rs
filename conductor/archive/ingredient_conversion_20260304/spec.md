# Specification: Standalone Ingredient Conversion Tool

## Goals
- Expose the existing weight conversion engine as a standalone MCP tool.
- Support conversion of single ingredients or lists of ingredients.

## Requirements
- New MCP tool: `convert_ingredients`.
- Input: `ingredients` (list of strings).
- Output: List of objects containing original string and converted weight in grams.
- Handle cases where ingredients are not found in the `WeightChart`.

## Proposed Tech Stack
- Existing `conversion::engine`.
- `mcp-sdk-rs`.
