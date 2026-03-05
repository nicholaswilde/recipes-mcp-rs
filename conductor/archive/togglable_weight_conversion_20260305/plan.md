# Implementation Plan: Togglable Weight Conversion

## Phase 1: Configuration Update
- [ ] Add `weight_conversion` field to `Args` struct in `src/config.rs`.
- [ ] Add `weight_conversion` field to `AppConfig` struct in `src/config.rs`.
- [ ] Update `AppConfig::load` to correctly prioritize CLI, then env var, then config file.
- [ ] Set the default value to `true`.

## Phase 2: Core Logic and Tool Handlers
- [ ] Update `scrape_recipe` and `scrape_recipes` in `src/scraper.rs` to only call `convert_ingredients` if enabled.
- [ ] Update the `manage_recipes` tool's `scale` and `format` actions in `src/main.rs` to respect the toggle.
- [ ] Ensure `convert_ingredients` tool remains functional (it is an explicit call).

## Phase 3: Testing and Verification
- [ ] Add tests in `src/config.rs` to verify the setting is loaded correctly from different sources.
- [ ] Add unit tests for the toggled behavior in `src/scraper.rs`.
- [ ] Verify manually via CLI arguments.
