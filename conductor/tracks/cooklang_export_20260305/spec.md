# Specification: Cooklang Export Support

## Goal
Allow users to export recipes in the Cooklang (`.cook`) format for better integration with external recipe managers.

## Functional Requirements
- Add `cooklang` as an option for `format_type` in the `manage_recipes` tool.
- Implement a formatter that converts `Recipe` objects to valid Cooklang strings.
