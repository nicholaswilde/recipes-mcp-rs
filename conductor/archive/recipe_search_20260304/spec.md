# Specification: Recipe Search Support

**Track ID:** `recipe_search_20260304`

## Overview
Implement a "search" action for the `manage_recipes` tool to discover recipe URLs from user queries.

## Goals
1. **Discover Recipes:** Search for recipe URLs based on key terms.
2. **Simplified Interaction:** Users can find recipes without providing exact URLs.

## Functional Requirements
- **Action Update:** Add a `"search"` action to the `manage_recipes` tool.
- **Input Parameters:** Add `query: String` and `limit: Option<u32>` to the tool's input schema.
- **Search Logic:** Integrate a basic search mechanism (or search within specific domains like AllRecipes).
- **Output:** Return a list of `title` and `url` pairs for the found recipes.

## Acceptance Criteria
- A user can search for a recipe and receive a list of matching URLs and titles.
- The results can then be passed to the `scrape` action.
