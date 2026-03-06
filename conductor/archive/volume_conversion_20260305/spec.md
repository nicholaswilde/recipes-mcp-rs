# Specification: Metric/Imperial Volume Conversion

**Track ID:** `volume_conversion_20260305`

## Overview
Enhance the recipe formatting and MCP tool capabilities to support conversion between Metric and Imperial volume units (e.g., ml to cups, fl oz to liters). This allows users to view recipes in their preferred measurement system, regardless of the source format.

## Goals
1. **Accurate Volume Conversion:** Implement robust conversion logic for standard culinary volume units.
2. **Preference-Based Formatting:** Allow users to specify a target unit system (Metric or Imperial) for recipe display.
3. **Stand-alone Conversion Tool:** Provide an MCP tool to convert volume-based ingredient lists.

## Functional Requirements
- **Volume Parsing:** Extract quantities and units from volumetric strings (e.g., "250ml milk", "1.5 cups water").
- **Conversion Engine:** Implement conversion ratios between ml, liters, fl oz, cups, tablespoons, and teaspoons.
- **Recipe Augmentation:** Update the `manage_recipes` output to optionally show converted volume units.
- **Formatting:** Support displaying both units (e.g., "1 cup (237ml) milk") or swapping the primary unit.

## Unit Support
- **Imperial:** `tsp`, `tbsp`, `fl oz`, `cup`, `pint`, `quart`, `gallon`.
- **Metric:** `ml`, `l`.

## Non-Functional Requirements
- **Precision:** Use standard culinary conversion factors (e.g., 1 cup = 236.588 ml, though 240ml or 250ml are common in some regions; we should use a consistent standard, likely the US legal cup or similar).
- **Rounding:** Implement sensible rounding for converted values (e.g., "236.588ml" -> "237ml" or "240ml").

## Acceptance Criteria
- Ingredients containing volumetric measurements can be converted between Metric and Imperial.
- The `manage_recipes` tool supports a `target_system` parameter.
- Unit tests cover all standard conversion paths.

## Out of Scope
- Weight-to-Volume conversion (covered in weight conversion track).
- Non-liquid volume (e.g., cubic inches).
