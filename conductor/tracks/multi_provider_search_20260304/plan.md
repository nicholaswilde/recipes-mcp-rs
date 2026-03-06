# Implementation Plan: Multi-Provider Recipe Search

## Phase 1: Preparation [checkpoint: 92839d9333f34f392d54fbd3f3bd0ae3a0cab3dc]
- [x] Refactor `src/search.rs` to support multiple provider backends. (COMPLETED)
- [x] Define a `RecipeProvider` trait for consistency. (COMPLETED)

## Phase 2: Implementation
- [ ] Implement `FoodNetworkProvider`.
- [ ] Implement `SeriousEatsProvider`.
- [ ] Update `search_recipes` to accept a `provider` enum.

## Phase 3: Integration
- [ ] Update the `manage_recipes` tool definition to include `provider`.
- [ ] Update the `search` action handler in `src/main.rs`.

## Phase 4: Testing
- [ ] Add integration tests for searching each provider.
- [ ] Test the default provider behavior (AllRecipes).

## Phase 5: Verification
- [ ] Verify search results are returned from multiple providers.
