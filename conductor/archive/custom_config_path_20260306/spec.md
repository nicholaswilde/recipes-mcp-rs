# Specification: Custom Configuration Path

## Goal
Allow users to specify a custom configuration file (TOML, YAML, or JSON) via the command line.

## Requirements
- Add a `--config` / `-c` CLI argument to the `Args` struct in `src/config.rs`.
- Update `AppConfig::load` to include the specified file as a configuration source if the flag is provided.
- Ensure the custom file has the highest priority among file sources (but lower than environment variables and other explicit CLI flags).
- Maintain backward compatibility with the existing `config/` directory logic.
