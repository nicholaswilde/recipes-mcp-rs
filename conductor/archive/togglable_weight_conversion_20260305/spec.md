# Specification: Prioritized Weight Conversion Matching

## Goal
Improve the accuracy of ingredient weight conversion by prioritizing longer, more specific ingredient names over shorter ones.

## Requirements
- When matching ingredients against the `WeightChart`, longer matches must be preferred.
- Example: "powdered sugar" should match the entry for "Powdered Sugar" rather than falling back to "Sugar" (Granulated Sugar).
- Partial matches and aliases should both follow this prioritization logic.

## Proposed Changes
- Update `src/conversion/data.rs`:
    - Add "Powdered Sugar" to the standard entries.
    - Modify `find_best_match` to sort keys by length descending before performing partial or alias matching.
- Add unit tests to verify the prioritization.
