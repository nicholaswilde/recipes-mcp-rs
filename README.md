# :stew: Recipes MCP Server

An MCP server written in Rust for importing, parsing, scaling, formatting, and searching recipes from the internet.

## :sparkles: Features

- **Intelligent Web Scraping:** Extracts recipes from a wide variety of online sources using a tiered strategy with `rust-recipe` and `recipe-scraper`.
- **Recipe Scaling:** Adjust servings for any recipe, automatically scaling ingredient quantities.
- **Weight Conversion:** Automatically converts volumetric measurements (cups, tbsp, tsp) to gram weights using the King Arthur ingredient weight chart.
- **Multi-Format Export:** Supports exporting recipes to Markdown and JSON.
- **Recipe Discovery:** Integrated search functionality to find recipe URLs from supported providers (e.g., AllRecipes).
- **Flexible Configuration:** Supports TOML, YAML, and JSON configuration files, as well as environment variables and command-line arguments.

## :package: Installation

### :clipboard: Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- [Task](https://taskfile.dev/installation/) (optional, for easier development)

### :hammer: Build

```bash
cargo build --release
```

Or using Task:

```bash
task build
```

## :rocket: Usage

### :gear: Configuration

The server can be configured via environment variables, command-line arguments, or configuration files.

| Option | Environment Variable | CLI Argument | Default |
|--------|----------------------|--------------|---------|
| Log Level | `RECIPES_LOG_LEVEL` | `--log-level` | `info` |
| MCP Transport | `RECIPES_MCP_TRANSPORT` | `--mcp-transport` | `stdio` |

### :wrench: MCP Tools

The server provides a single tool `manage_recipes` with multiple actions:

#### `manage_recipes`

**Arguments:**
- `action` (required): `scrape`, `scale`, `format`, or `search`.
- `urls`: List of recipe URLs to scrape (required for `scrape`, optional for `scale`/`format`).
- `recipes`: List of recipe objects to scale or format (required if `urls` not provided).
- `target_servings`: Desired number of servings (required for `scale`).
- `format_type`: Output format (`markdown` or `json`, defaults to `markdown`).
- `query`: Search query (required for `search`).
- `limit`: Maximum search results (optional for `search`, default 5).

### :memo: Example MCP Config

```json
{
  "mcpServers": {
    "recipes": {
      "command": "/path/to/recipes-mcp-rs",
      "args": [],
      "env": {
        "RECIPES_LOG_LEVEL": "info"
      }
    }
  }
}
```

## :hammer_and_wrench: Development

### :test_tube: Running Tests

```bash
task test
```

### :white_check_mark: CI Checks (Lint, Format, Test)

```bash
task test:ci
```

## :page_facing_up: License

This project is licensed under the Apache License 2.0. See the `LICENSE` file for details.
