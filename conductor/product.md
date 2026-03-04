# Initial Concept\n\nThis is an MCP server written in rust. It will be used to import recipes from the internet. It should be able to parse the steps, ingredients, and other items. Use both the rust-recipe and recipe-scraper crates. It should be able to read config files in toml, yaml, and json files. Also, use environmental variables and command arguments

---

# Product Guide: Recipes MCP Server

## Vision
To provide a robust, automated interface for importing and parsing recipes from a wide variety of online sources, enabling users to seamlessly integrate external culinary data into their applications and workflows via the Model Context Protocol (MCP).

## Target Audience
- Developers building cooking or recipe management applications.
- Users who want to automate their personal recipe collections.
- Integration platforms that utilize MCP for data enrichment.

## Core Features
- **Intelligent Web Scraping:** Leverages `recipe-scraper` and `rust-recipe` to extract structured data from diverse recipe websites.
- **Granular Parsing:** Accurately identifies and separates recipe components including:
  - Ingredients (quantities, units, items)
  - Preparation and cooking steps
  - Metadata (prep time, cook time, servings, cuisine, tags)
- **Flexible Configuration:** Supports multi-format configuration (TOML, YAML, JSON) for easy customization.
- **Dynamic Input:** Fully configurable via command-line arguments and environment variables for seamless integration into various environments.

## Non-Functional Requirements
- **Efficiency:** Fast parsing and minimal resource usage.
- **Extensibility:** Easy to add new scrapers or output formats.
- **Reliability:** Graceful handling of malformed or unconventional recipe pages.
