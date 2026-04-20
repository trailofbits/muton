# MuTON

> In genetics, a muton is the smallest unit of DNA that can produce a mutation.

`muton` runs mutation testing campaigns for TON smart contracts.

Supported languages are auto-detected by file extension:
- **FunC** (`.fc`, `.func`)
- **Tact** (`.tact`)
- **Tolk** (`.tolk`)

## Installation

### npm (recommended)

```bash
npm install -g @trailofbits/muton
```

### Prebuilt binaries

```bash
curl --proto '=https' --tlsv1.2 -LsSf \
  https://github.com/trailofbits/muton/releases/latest/download/muton-installer.sh | sh
```

### Build from source (native toolchain)

Requirements:
- Rust toolchain (via `rustup`)
- C toolchain (`gcc`/`clang`) and `make`
- `pkg-config`
- SQLite development headers (`libsqlite3-dev`/`sqlite`)

Build and run:

```bash
cargo build --release
./target/release/muton --help
```

### Build from source (Nix)

With Nix flakes enabled:

```bash
git clone https://github.com/trailofbits/muton.git
cd muton
nix develop --command bash -c 'just build'
./target/debug/muton --version
```

## Quick start

Initialize a workspace (creates `muton.toml` and `muton.sqlite`):

```bash
muton init
```

Run a campaign against a target file or directory:

```bash
muton run path/to/contract.tact --test.cmd "npx blueprint test"
```

Use globs when needed:

```bash
muton run "contracts/**/*.tact" --test.cmd "npx blueprint test"
```

Inspect campaign progress and outcomes:

```bash
muton status
muton results --all
muton results --status uncaught --severity high,medium
```

Inspect generated mutants:

```bash
muton print mutations --language tact
muton print mutants --target path/to/contract.tact
muton print mutant --id 42
```

Test all mutants even if more severe mutants on the same line were uncaught:

```bash
muton run path/to/contract.tact --comprehensive --test.cmd "npx blueprint test"
```

## How muton works

Mutation campaigns can be slow. `muton` is designed to make them resumable and predictable:

- Targets and mutants are stored in a single SQLite database (`muton.sqlite` by default).
- Interrupted campaigns can resume where they left off.
- By default, less-severe mutants on a line may be skipped if a more-severe mutant on that same line was already uncaught.

> [!TIP]
> Use `--comprehensive` to disable skip optimization and force testing of all generated mutants.

Because mutation testing is usually slow, it is commonly run periodically (for example, overnight) rather than on every commit.

## Configuration

Configuration precedence (highest to lowest):
1. CLI flags
2. `muton.toml`
3. Built-in defaults

`muton.toml` is discovered by walking up from the current working directory. You can also pin a config file explicitly with `--config path/to/muton.toml`.

Print your effective configuration:

```bash
muton print config
```

Example config:

```toml
db = "muton.sqlite"

[log]
level = "info"
# color = true

[targets]
# include = ["contracts/**/*.tact", "contracts/**/*.func"]
# ignore = ["build", "node_modules"]

[run]
# mutations = ["ER", "CR"]
# comprehensive = false

[test]
# cmd = "npx blueprint test"
# timeout = 120
```

## Example contracts in this repo

- FunC: `tests/func/examples/hello-world.fc`
- Tact: `tests/tact/examples/hello-world.tact`
- Tolk: `tests/tolk/examples/hello-world.tolk`

## Notes

- Mixed-language projects are supported.
- Directory targets recurse automatically; only supported file extensions are considered.
