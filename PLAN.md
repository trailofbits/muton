# Testing Reorganization Plan

## Goal
Align Muton’s language test suites with the structure and rigor of `../mewt.improve-tests/tests/rust`, yielding focused per-mutation coverage, shared utilities, and automated guardrails across FunC, Tact, and Tolk.

## Guiding Assumptions
- Mutation slugs exported by each language engine remain stable enough to map one-to-one onto dedicated test files.
- Shared fixtures and helpers can be centralized without breaking existing tests (they will be re-exported as needed).
- Parity with the sibling project is the long-term target, but incremental delivery is acceptable.

## Execution Order
We progress in four phases; each task links to a TODO entry for status tracking.

### Phase 1 — Baseline Audit
- [x] **Inventory current coverage** (`map-muton-tests-to-mewt-pattern`)
  - Document existing modules, helpers, and slug coverage to highlight divergences.

### Phase 2 — Shared Foundations
- [x] **Design shared test utilities** (`introduce-shared-test-utilities`)
  - Agree on the API/structure for `tests/common` (or equivalent) before refactoring suites.

### Phase 3 — Language Suites Realignment
- [x] **FunC per-slug refactor** (`refactor-func-tests-to-per-slug-structure`)
  - Introduce `mutations/` submodule, migrate assertions, prune redundant tests.
- [x] **Tact suite expansion** (`expand-tact-tests-to-match-pattern`)
  - Mirror the sibling hierarchy, add comment-ignore and per-slug coverage.
- [ ] **Tolk suite rebuild** (`build-tolk-test-suite-like-mewt`)
  - Replace the top-level test file with a module structure and per-slug tests.

### Phase 4 — Guardrails & Documentation
- [ ] **Automate slug-to-test enforcement** (`enforce-mutation-slug-test-coverage`)
  - Add guard tests/scripts that ensure each slug has a dedicated test module.
- [ ] **Document the new conventions** (`add-test-plan-documentation`)
  - Capture layout, helper usage, and contribution guidelines in docs/README.

## Progress Log
Use this section to note major milestones, decisions, or blockers encountered while executing the plan.

| Date | Update | Owner |
|------|--------|-------|
| 2026-04-15 | Completed baseline audit of FunC/Tact/Tolk tests vs mewt; documented gaps and per-slug coverage needs. | ChatGPT |
| 2026-04-15 | Introduced shared `tests/common` fixtures and slug helpers; FunC/Tact/Tolk suites now reuse the centralized targets. | ChatGPT |
| 2026-04-15 | Refactored FunC tests into per-slug modules with shared helpers and slug-specific assertions; removed the legacy monolithic suite. | ChatGPT |
| 2026-04-16 | Rebuilt Tact suite with per-slug mutation modules, comment-ignore checks, and shared helpers; removed legacy monolithic mutation tests. | ChatGPT |

## Maintenance Notes
- Keep the checkbox list in sync with the `.todo` entries; mark both as tasks complete.
- Update the progress log whenever a phase completes or when new issues are discovered.
- Revisit the assumptions if future language engines introduce incompatible slug patterns or helper requirements.
