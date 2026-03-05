# Implementation Plan: Standalone Ingredient Conversion Tool

## Phase 1: Tool Definition
- [ ] Define the `convert_ingredients` tool in `src/main.rs`.
- [ ] Add the tool to the `tools/list` response.

## Phase 2: Implementation
- [ ] Implement the `tools/call` handler for `convert_ingredients`.
- [ ] Map the request to the existing `conversion::engine::convert_to_weight` function.

## Phase 3: Testing
- [ ] Add unit tests for the new tool handler.
- [ ] Verify handling of missing ingredients and invalid units.

## Phase 4: Verification
- [ ] Verify the tool is discoverable and callable via MCP.
