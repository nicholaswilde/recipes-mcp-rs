# Product Guidelines: Recipes MCP Server

## General Style
- **Voice and Tone:** Clear, concise, and professional.
- **Terminology:** Use consistent naming for internal tools, resources, and configuration keys.

## Data Handling
- **Privacy:** Do not log or store personal user data or private recipe URLs without explicit consent.
- **Accuracy:** Prioritize high-fidelity parsing of ingredient measurements and preparation steps.
- **Validation:** Sanitize all inputs from external web sources before processing.

## Output Formatting
- **Standardization:** All exported recipe data should follow a consistent schema (e.g., JSON-LD or a custom internal format).
- **Readability:** Ensure that printed steps and ingredient lists are well-formatted for human consumption.

## Configuration & Integration
- **Convention Over Configuration:** Provide sensible defaults for all configuration options.
- **Descriptive Errors:** Provide clear and actionable error messages for common issues (e.g., malformed URL, parsing failure).
- **Compatibility:** Maintain compatibility with common MCP clients and host environments.

## Documentation
- **Visual Engagement:** Use emoji shortcodes (e.g., `:stew:`, `:sparkles:`) for all top-level and second-level headings in the `README.md` to enhance readability and visual appeal.

## MCP Tool Design
- **Unified and Action-Oriented Tools:** Related operations should be consolidated into unified tools using an `action` pattern (e.g., `manage_recipes`). This reduces token consumption and simplifies the toolset while maintaining full functionality and clarity through well-defined sub-actions.
