# Mutation Test Suites — Baseline Inventory

_Last updated: 2026-04-15_

## Goal
Document the current layout and coverage of Mutons language-targeted tests to guide the multi-phase reorganization toward the `mewt.improve-tests/tests/rust` pattern.

## Reference Layout (Target Pattern)
- Root `mod.rs` exports:
  - `comment_ignorance_tests`
  - `integration_tests`
  - `mutations` (per-slug module tree)
- `integration_tests.rs` provides reusable helpers (e.g., `create_test_target`) and guardrails (ensuring every registered slug has a matching test file).
- `mutations/` directory contains one file per slug with deterministic assertions for new text / behavior.
- Shared fixtures under `examples/`.

## Muton Status by Language

### FunC (`tests/func`)
| File | Purpose | Local helpers | Slugs observed / validated |
|------|---------|---------------|-----------------------------|
| `mod.rs` | Wires golden, comment, integration, mutation, and parser suites. |  | - |
| `mutation_tests.rs` | Smoke tests generator output. | `func_target_from_source` | Explicit: `ER`, `CR`, `AS`; implicit coverage of loop/conditional mutants without slug checks. |
| `golden_mutations.rs` | Golden-output verification via `ASTMutationEngine`. | `create_func_target`, `apply_first_mutant_with_slug` | `ER`, `CR`, `IT`, `IF`, `WF`, `RZ`, `AS`. |
| `comment_ignorance_tests.rs` | Ensures comment regions stay untouched. | `func_target_from_source`, `block_spans` | Monitors comment-sensitive slugs (`CR`). |
| `integration_tests.rs` | End-to-end generation, overlap analysis. | `create_test_target` | Confirms presence of core slugs (`ER`, `CR`, `AS`) via counts. |
| `parser_tests.rs` | Syntax sanity checks. | `parse_func` | N/A. |
| `examples/hello-world.fc` | Fixture for parser tests. |  | N/A. |

Key notes:
- Helpers (`*_target_from_source`, `create_test_target`, `apply_first_mutant_with_slug`) are file-local duplicates; none are shared.
- No `mutations/` subtree; slug coverage largely bundled in `golden_mutations`.

### Tact (`tests/tact`)
| File | Purpose | Local helpers | Slugs observed / validated |
|------|---------|---------------|-----------------------------|
| `mod.rs` | Declares parser, mutation, integration modules **and `slug_uniqueness` (missing file)**. |  | - |
| `mutation_tests.rs` | Golden-output checks similar to FunC. | `tact_target_from_source`, `apply_first_mutant_with_slug` | `ER`, `CR`, `IT`, `IF`, `WF`, `AS`, `BL`, `TT`, `TF`, `UF`, `COS`. |
| `comment_ignorance_tests.rs` | Comment handling assertions **not wired into `mod.rs`**. | `tact_target_from_source`, `block_spans` | Validates comment behavior (`CR`). |
| `integration_tests.rs` | Registry-based end-to-end test. | Inline temp-target builder | Ensures any mutant; no per-slug assertions. |
| `parser_tests.rs` | Syntax coverage, common node mapping. | `parse_tact` | N/A. |
| `examples/*.tact` | Sample programs (hello world, complex contract, type features). |  | N/A. |

Key notes:
- `slug_uniqueness` module missing, causing build failure if tests collect.
- Comment ignorance suite exists but never executed due to missing module registration.
- No shared helper module; repeated temp-target constructors.

### Tolk (`tests/tolk`)
| File | Purpose | Local helpers | Slugs observed / validated |
|------|---------|---------------|-----------------------------|
| `tests/tolk_tests.rs` | Wraps `integration_tests` in a namespace. |  | - |
| `integration_tests.rs` | Handles all assertions (mutation diversity, slug presence, engine metadata). | `create_test_target` | `IF`, `IT`, `ER`, `CR`, `COS`, `AOS`, `BL`, `WF`; mix of presence checks only. |
| `examples/hello-world.tolk` | Example program. |  | N/A. |

Key notes:
- Entire suite resides in one integration file; no parser or comment coverage.
- No per-slug tests or guardrails verifying slug-to-file mapping.

## Cross-Cutting Gaps vs Target Pattern
1. **Per-slug coverage**: Absent for all languages (FunC/Tact partially approximate via golden tests; Tolk missing entirely).
2. **Shared utilities**: Each module redefines temp directory + target construction helpers; nothing centralized akin to `create_test_target` in the reference suite.
3. **Module wiring issues**:
   - `tests/tact/mod.rs` references `slug_uniqueness` (missing file) and omits `comment_ignorance_tests`.
4. **Guardrails**: No automated check ensuring each registered mutation slug has a dedicated test module (Rust suite enforces this).
5. **Tolk suite maturity**: Lacks parser tests, comment ignorance coverage, and granular slug assertions.

## Recommended Actions (mirrors PLAN.md Phases)
- **Phase 2**: Design `tests/common` (or equivalent) to host shared temp-target builders / helpers. Decide on API alignment with the mewt reference.
- **Phase 3**: Create `tests/<lang>/mutations/` directories, migrating existing slug assertions into per-file tests; add missing suites for Tolk (parser, comments, examples).
- **Phase 4**: Introduce guard test(s) mirroring Rusts `compound_assignment_slug_tests_are_present` and document conventions in repo docs.

## Maintenance
- Keep this document updated at the end of each project phase.
- Cross-reference the entry in `PLAN.md` (Phase 1 now complete) when adjusting milestones.
