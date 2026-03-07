# Coverage Baseline Report (2026-03-07)

## Overall Stats
- **Total Workspace Coverage:** 71.19%
- **Total Lines:** 3409
- **Missed Lines:** 982

## File Breakdown (Targeting Low Coverage)

| Filename | Coverage | Missed Lines | Priority |
| :--- | :--- | :--- | :--- |
| `main.rs` | 0.00% | 121 | High |
| `handler.rs` | 26.46% | 492 | High |
| `nutrition/edamam.rs` | 64.10% | 28 | Medium |
| `scraper.rs` | 79.06% | 160 | Medium |
| `scaling.rs` | 79.41% | 14 | Low |
| `conversion/volume.rs` | 80.72% | 16 | Low |
| `search.rs` | 82.57% | 65 | Low |

## Analysis
- **`main.rs` & `handler.rs`:** These are the largest contributors to missed lines. `main.rs` is currently untested, and `handler.rs` contains the bulk of the tool handling logic which needs integration tests.
- **`scraper.rs`:** A large file (764 lines) with significant missed coverage (160 lines), likely in specific provider edge cases or error handling.
- **`nutrition/edamam.rs`:** Only half of the functions are executed. Needs targeted unit tests for its API client logic.
