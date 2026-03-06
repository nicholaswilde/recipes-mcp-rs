# Specification: Metric/Imperial Volume Conversion

## Goal
Enable international users to toggle between Metric (ml) and Imperial (cups/tbsp) volume units for the entire recipe.

## Functional Requirements
- Support converting all volume measurements in a recipe.
- Add a `--volume-unit` CLI argument (options: `metric`, `imperial`).
- Update the Markdown formatter to respect the selected unit.
