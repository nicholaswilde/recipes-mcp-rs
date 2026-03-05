# Specification: Togglable Weight Conversion

## Goal
Make the automatic volumetric-to-weight conversion in ingredient lists togglable via configuration, environment variables, and command-line arguments.

## Scope
- Update `AppConfig` and `Args` to include a `weight_conversion` boolean setting.
- Ensure the setting can be set via `--weight-conversion`, `RECIPES_MCP_WEIGHT_CONVERSION` env var, or `weight_conversion` in TOML/YAML/JSON config files.
- Default the setting to `true`.
- Update the MCP tool handlers to respect this setting when formatting and scaling recipes.

## Requirements
- The setting must be globally accessible or passed to relevant conversion functions.
- If disabled, ingredients should retain their original volumetric strings without weight parentheticals.
- The standalone `convert_ingredients` tool should still work if explicitly called, but automatic conversion during `scrape`, `scale`, and `format` should be controlled by this toggle.

## Tech Stack References
- Language: Rust
- Libraries: `clap`, `config`, `serde`.
