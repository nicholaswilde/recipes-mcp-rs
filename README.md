# :stew: Recipes MCP Server :robot:

An MCP server written in Rust for importing, parsing, scaling, formatting, and searching recipes from the internet.

## :sparkles: Features

- **Intelligent Web Scraping:** Extracts recipes from a wide variety of online sources using a tiered strategy with `rust-recipe` and `recipe-scraper`, including advanced image and gallery extraction.
- **Recipe Scaling:** Adjust servings for any recipe, automatically scaling ingredient quantities.
- **Weight Conversion:** Automatically converts volumetric measurements (cups, tbsp, tsp) to gram weights using the [King Arthur Baking ingredient weight chart](https://www.kingarthurbaking.com/learn/ingredient-weight-chart).
- **Unit System Conversion:** Automatically converts volume measurements between Metric and Imperial systems.
- **Nutrition Estimation:** Automatically estimates calories and macronutrients (Fat, Carbs, Protein) for recipes based on ingredient weights.
- **Dietary Filtering:** Filter search results and scraped recipes based on dietary preferences (e.g., vegan, gluten-free).
- **Admonition Extraction:** Captures recipe-specific tips, notes, and variations.
- **Standalone Conversion Tool:** Quickly convert a list of ingredient strings into weighted equivalents or switch unit systems.
- **Multi-Format Export:** Supports exporting recipes to Markdown, JSON, and Cooklang.
- **Recipe Discovery:** Integrated search functionality to find recipe URLs from supported providers (AllRecipes, Food Network, Serious Eats, TheMealDB, Epicurious, NYT Cooking, BBC Good Food).
- **Caching:** Built-in recipe caching to reduce redundant network requests.
- **Flexible Transport:** Supports both `stdio` and `HTTP/SSE` (Server-Sent Events) transport modes.
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

The server can be configured via environment variables, command-line arguments, or configuration files located in a `config/` directory.

#### File-based Configuration
The server uses the following file-based configuration hierarchy (later files override earlier ones):
1. `config/default.toml` (or `.yaml`, `.json`): Base configuration.
2. `config/{RUN_MODE}.toml`: Environment-specific (e.g., `config/development.toml`). `RUN_MODE` defaults to `development`.
3. `config/local.toml`: Local overrides (not tracked by git).

#### Configuration Options
| Option | Environment Variable | CLI Argument | Default |
|--------|----------------------|--------------|---------|
| Config Path | `RECIPES__CONFIG_PATH` | `--config`, `-c` | `None` |
| Log Level | `RECIPES__LOG_LEVEL` | `--log-level` | `info` |
| Transport | `RECIPES__TRANSPORT` | `--transport` | `stdio` |
| Port | `RECIPES__PORT` | `--port` | `3000` |
| Weight Conversion | `RECIPES__WEIGHT_CONVERSION` | `--weight-conversion` | `true` |
| Cache Enabled | `RECIPES__CACHE_ENABLED` | `--cache-enabled` | `true` |
| Cache Directory | `RECIPES__CACHE_DIR` | `--cache-dir` | `.cache` |

### :wrench: MCP Tools

The server provides several tools for recipe management and ingredient processing:

#### `manage_recipes`

Unified tool for complex recipe operations.

**Arguments:**
- `action` (required): `scrape`, `scale`, `format`, `search`, or `nutrition`.
- `urls`: List of recipe URLs to scrape (required for `scrape`, optional for `scale`/`format`/`nutrition`).
- `recipes`: List of recipe objects to scale, format, or analyze (required if `urls` not provided).
- `target_servings`: Desired number of servings (required for `scale`, optional for others).
- `format_type`: Output format (`markdown`, `json`, or `cooklang`, defaults to `markdown`).
- `query`: Search query (required for `search`).
- `limit`: Maximum search results (optional for `search`, default 5).
- `provider`: Recipe provider to search (`allrecipes`, `foodnetwork`, `seriouseats`, `themealdb`, `epicurious`, `nytcooking`, `bbcgoodfood`).
- `dietary_filters`: List of dietary preferences (`vegan`, `vegetarian`, `gluten-free`, `dairy-free`, `keto`, `paleo`).
- `admonition_types`: List of admonition types to extract (`tip`, `note`, `variation`).
- `bypass_cache`: If true, bypass the cache and force a fresh request (optional, default: false).
- `target_system`: Desired unit system for volume conversion (`metric` or `imperial`, optional).

#### `convert_ingredients`

Quickly convert a list of volumetric ingredient strings to gram weights or between unit systems.

**Arguments:**
- `ingredients` (required): Array of ingredient strings (e.g., `["1 cup flour", "2 tbsp sugar"]`).
- `target_system`: Desired unit system for volume conversion (`metric` or `imperial`, optional).

### :memo: Example MCP Config

```json
{
  "mcpServers": {
    "recipes": {
      "command": "/path/to/recipes-mcp-rs",
      "args": ["--weight-conversion", "true", "--cache-enabled", "true"],
      "env": {
        "RECIPES__LOG_LEVEL": "info"
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

## :writing_hand: Author

This project was started in 2026 by [Nicholas Wilde][2].

[2]: <https://github.com/nicholaswilde/>
