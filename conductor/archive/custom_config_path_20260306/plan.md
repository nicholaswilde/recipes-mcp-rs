# Implementation Plan: Custom Configuration Path

## User Review Required
> [!IMPORTANT]
> This change modifies the CLI interface. The `-c` shorthand will be reserved for `--config`.

## Proposed Changes

### Configuration Logic
#### [src/config.rs]
- Update `Args` struct:
  - Add `config_path: Option<String>` with `short = 'c'` and `long = "config"`.
- Update `AppConfig::load`:
  - If `args.config_path` is `Some`, add it as a source using `builder.add_source(File::with_name(&path))`.

## Verification Plan

### Automated Tests
- Create a temporary config file in a test.
- Pass the path to `AppConfig::load` via a mock `Args` struct.
- Verify that values from the custom file are correctly loaded.
- Verify that standard `config/` files are still loaded if the custom one doesn't override them.

### Manual Verification
- Create a `test_config.toml` in the project root.
- Run `cargo run -- --config test_config.toml` and verify settings (e.g., changing the port).
