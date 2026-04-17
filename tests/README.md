# Mutation Testing Conventions

This directory uses a consistent, language-agnostic structure so mutation tests stay easy to extend and review.

## Directory Layout

- `tests/common/` contains shared fixtures and helpers reused across suites.
- `tests/slug_module_guard.rs` ensures mutation slugs advertised by each language engine match the per-slug test modules in that language suite.
- `tests/<language>/` contains one language suite. Typical contents:
  - `mod.rs` to wire suite modules together.
  - `mutations/` with one Rust module per mutation slug (`<SLUG>.rs`).
  - Additional suite modules (for parser behavior, integration behavior, comment handling, regressions, etc.) as needed.
  - `examples/` for source fixtures used by tests.

## Shared Helpers

Keep helper guidance implementation-flexible:

- Shared helpers may live in a centralized location (for example `tests/common/`) and be reused by many suites.
- Shared helpers may also be organized per language if that becomes a better fit.
- Prefer reuse over duplicating fixture setup, sorting, or lookup logic inside individual slug modules.

## Per-Slug Convention

For each language suite:

- Add one module per mutation slug at `tests/<language>/mutations/<SLUG>.rs`.
- Keep each slug module focused on assertions for that slug.
- Add or update broader suites when a change affects behavior beyond one slug.

## Adding or Updating Tests

1. Inspect generated mutants for the language/slug you are changing.
2. Add or update `tests/<language>/mutations/<SLUG>.rs`.
3. If behavior crosses slug boundaries, update the relevant non-slug suites in `tests/<language>/`.
4. Run `just fmt` and `just test` (or `cargo test`) before submitting.

## Slug Module Guardrails

`tests/slug_module_guard.rs` compares:

- slugs registered by each language engine, vs
- slug module filenames discovered in `tests/<language>/mutations/`.

The guard fails when:

- an engine slug has no matching `<SLUG>.rs` test module, or
- a `<SLUG>.rs` test module exists for a slug not advertised by the engine.

File policy inside `tests/<language>/mutations/`:

- `mod.rs` is allowed.
- `<SLUG>.rs` files are treated as slug modules.
- other non-`.rs` files fail loudly.

## Why This Structure Works

- Per-slug modules make coverage gaps obvious.
- Reusable helpers reduce duplication and keep fixtures consistent.
- The slug module guard keeps implementation slugs and test modules in lockstep.
