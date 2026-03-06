# Implementation Plan: Cooklang Export Support

This plan outlines the steps for adding Cooklang export support to the `recipes-mcp-rs` project.

## Phase 1: Preparation [x]
- [x] Research Cooklang's handling of multi-word ingredients and cookware.
- [x] Create a set of representative `Recipe` objects for testing (simple, complex, with/without metadata).

## Phase 2: Implementation [~]

### Task 2.1: Implement Metadata Header Export [x]
- [x] Implement a helper function to format `Recipe` metadata into `>> key: value` headers.
- [x] Include title, description, servings, and times.

### Task 2.2: Implement Ingredient Parsing and Formatting [x]
- [x] Implement logic to parse ingredients (currently `Vec<String>`) to extract name, amount, and unit.
- [x] Implement `to_cooklang_ingredient` helper that formats ingredients using `@name{amount%unit}` syntax.

### Task 2.3: Implement Instruction Formatting [x]
- [x] Create a `to_cooklang` function in `src/formatter.rs`.
- [x] Join instructions into paragraphs.
- [x] (Advanced) Scan instructions for ingredient matches and replace them with Cooklang syntax.
- [x] Handle cookware and timers if possible.

## Phase 3: Testing and Validation [x]
- [x] Write unit tests for `to_cooklang` with various recipe types.
- [x] Verify formatting output manually with a Cooklang viewer or parser if possible.

## Phase 4: Integration [x]
- [x] Expose Cooklang export functionality if relevant for MCP tools (e.g., as an option in `format_recipe`).
