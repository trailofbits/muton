
# MuTON

> In genetics, a muton refers to the smallest unit of DNA, potentially a single nucleotide, that can produce a mutation.

`muton` is a tool for running mutation testing campaigns against TON smart contracts written in FunC and Tact. Language is auto-detected by file extension (`.fc`/`.func`, `.tact`).

## Installation

### npm (recommended)

```bash
npm install @trailofbits/muton
```

### Prebuilt binaries

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/trailofbits/muton/releases/download/v1.0.0/muton-installer.sh | sh
```

### Build from source (via Nix)

With Nix flakes enabled:

```bash
git clone https://github.com/trailofbits/muton.git
cd muton
nix develop --command bash -c 'just build' # or 'direnv allow' then 'just build'
muton --version
```

### Build from source (native toolchain)

Requirements:
- Rust toolchain (via rustup)
- C toolchain (gcc/clang) and `make`
- `pkg-config`
- SQLite development headers (`libsqlite3-dev`/`sqlite`)

Install common prerequisites:

- macOS (Homebrew):

```bash
# Command Line Tools (if not already installed)
xcode-select --install || true

brew install rustup-init sqlite pkg-config
rustup-init -y
source "$HOME/.cargo/env"
```

- Ubuntu/Debian:

```bash
sudo apt update
sudo apt install -y build-essential pkg-config libsqlite3-dev curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
```

Build and run:

```bash
cargo build --release
./target/release/muton --help
```

Optional (install into your cargo bin):

```bash
cargo install --path . --locked --force
muton --version
```

## Quick start

- Mutate a single file (auto-detected language):

```bash
muton run path/to/contract.tact
```

- Mutate all supported files in a directory (recursive):

```bash
muton run path/to/project
```

- List available mutation slugs for a language:

```bash
muton print mutations --language tact
```

- Print all mutants for a target path:

```bash
muton print mutants --target path/to/contract.tact
```

- Show mutation test results (optionally filtered by target):

```bash
muton print results --target path/to/contract.tact
```

- Test all mutants even if more severe ones were uncaught (disable skip optimization):

```bash
muton run path/to/contract.tact --comprehensive
```

## Overview

This tool is designed to provide as pleasant a developer experience as possible while conducting mutation campaigns, which are notoriously messy and slow.

Muton operates on one single `muton.sqlite` database, this stores the target files and muton will reliably restore the original after a given mutation is tested, or after the campaign is interrupted with ctrl-c. However, this software is a work in progress so we strongly recommend running mutation campaigns against a clean git repo so that you can use `git reset --hard HEAD` to restore any mutations that escape the cleanup phase.

All target files are stored in the database and linked to a series of mutations. Each mutation is linked to one or zero outcomes. At the beginning of a mutation campaign, all targets are saved and all mutations are generated. This generally happens quickly, within a couple seconds.

Then, the real work begins: muton will work through the list of target files, replacing it with a mutated version. For each mutated version, it will run the test command and save the outcome. If the mutation campaign is interrupted, it will pick up where it left off (unless the target file changed, in which case it will start over).

This may take a very long time. Assuming the tests take 1 minute to run, there are 10 files, and 100 mutants were generated for each, the runtime (*assuming zero muton overhead*) will be 1 * 10 * 100 = 1000 minutes or 16 hours.

For this reason, making `muton` run fast is not enough to conduct fast mutation campaigns. Instead, a few features make this process somewhat less painful:
- resume by default: if a campaign gets interrupted halfway through for whatever reason, we don't need to restart from the very beginning
- customizable targets: you can give muton a directory as its `target` and it will mutate all supported files in this directory, which may take a long time. Or, you can give it one file and it will only mutate that file.
- skipping less severe mutants when more severe ones are uncaught: if replacing an expression with a `throw` statement is not caught by the test suite, this indicates the expression is never run by the test suite. Therefore, it's safe to assume that any other mutation to this line, will also not be caught by the test suite so subsequent mutations are skipped. This can drastically decrease the runtime against poorly tested code. However, this also means the runtime will increase after the test suite is improved and the mutation campaign starts testing parts of the code more deeply than it did before.

Tip: pass `--comprehensive` to `muton run` to disable this optimization and test all mutants even when more severe ones on the same line are uncaught.

Despite these features, mutation campaigns are best conducted infrequently eg after an overhaul to the test suite rather than after adding each individual test. Therefore, mutation testing is not suitable for running in the CI after every push. You may want to run a campaign at the end of the day so that it can run overnight.

## Adding a language

The architecture is language-agnostic. To add a new language, follow these steps. Where possible, prefer using the grammar update script to automate vendor steps.

1) Vendor the grammar (recommended: use the script)

- Add entries for your language to `ops/update-grammar.sh` in both `REPO_URLS` and `GRAMMAR_PATHS`.
- Preview:

```bash
just update-grammar language=<language> dry_run=true
```

- Perform the update (copies `parser.c`, headers, and `grammar.js` into `grammar/<language>/` and records vendored metadata):

```bash
just update-grammar language=<language>
```

You can also vendor manually by placing generated C sources under `grammar/<language>/src/` (must include `parser.c`) and `grammar/<language>/grammar.js`.

2) Build integration

- Extend `build.rs` to compile `grammar/<language>/src/parser.c` into a static library (see existing FunC/Tact blocks for reference).

3) Language enumeration

- Update `src/types/language.rs`:
  - Add a new enum variant
  - Update `Display`/`FromStr` and extension detection

4) Parser utilities

- Extend `src/mutations/parser.rs` to bind the tree-sitter language and route parsing for the new enum variant.

5) Mutation engine

- Create `src/mutations/<language>/` with:
  - `engine.rs` implementing `MutationEngine` (copying and modifying an existing engine is easiest)
  - `kinds.rs` list additional language-specific mutations. Will be merged with language-agnostic mutations in `src/mutations/common/kinds.rs`
  - `syntax.rs` provide grammar node/field names used by patterns, pulled from strings in `grammar/<lang>/src/parser.c`
- Wire dispatch in `src/mutations/mod.rs` to return your engine for the new language

6) Tests and examples

- Add example files under `tests/examples/<language>/`
- Add parser and mutation tests under `tests/language_specific/<language>/`

7) Validate

- `just check`
- `muton print mutations --language <language>` shows your slugs
- `muton print mutants --target tests/examples/<language>/...` generates mutants

## Configuration and precedence

Configuration sources (highest to lowest priority):
1. CLI flags
2. Environment variables
3. Nearest `muton.toml` found by walking up from the current working directory
4. Built-in defaults

Notes:
- CLI defaults are treated as built-in defaults (lowest); only flags explicitly provided override.
- Mutation slug whitelist overrides at the highest non-empty source; not merged.
- Ignore targets are merged additively across sources.

Config file discovery: starting from `cwd`, search for `muton.toml` in that directory, then its parent, and so on, stopping at the first match.

Example config:

```toml
[log]
level = "info"            # one of: trace, debug, info, warn, error
color = true               # optional boolean; omit for auto

[general]
db = "muton.sqlite"
ignore_targets = ["build/", "node_modules/"]  # substring matches, not globs

[mutations]
slugs = ["ER", "CR"]      # global whitelist; overrides other sources if set/non-empty

[test]
cmd = "npx blueprint test"
timeout = 120
```

Environment variables:
- `MUTON_LOG_LEVEL`, `MUTON_LOG_COLOR` ("on"/"off")
- `MUTON_DB`
- `MUTON_IGNORE_TARGETS` (CSV)
- `MUTON_SLUGS` (CSV; highest non-empty wins)
- `MUTON_TEST_CMD`, `MUTON_TEST_TIMEOUT`

CLI:
- `--ignore` (CSV): comma-separated substrings; any target path containing any will be ignored.
  - Matching is substring-based, not glob-based. Example: `--ignore lib` excludes any path containing "lib". To be more specific, use `lib/`.

## Examples

This repo includes example contracts you can try:

- FunC: `tests/examples/func/hello-world.fc`
- Tact: `tests/examples/tact/hello-world.tact`, `tests/examples/tact/complex-contract.tact`, `tests/examples/tact/type-features.tact`

## Notes

- Mixed-language projects are supported. When a directory is targeted, only files with supported extensions are considered.
- Default test command is `npx blueprint test`. Override with `--test-cmd` or via `MUTON_TEST_CMD` env var or config file.
