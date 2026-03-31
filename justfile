project := "muton"
export SQLITE_FILE := project + ".sqlite"

########################################
# Common dev commands

lint:
  cargo clippy --lib -p {{project}} --tests || { echo "clippy linter checks failed"; exit 1; }

lint-fix:
  cargo clippy --lib -p {{project}} --tests --fix

check:
  cargo check || { echo "cargo check failed"; exit 1; }

fmt-check:
  cargo fmt --all --check || { echo "formatting checks failed, run 'just fmt'"; exit 1; }

fmt:
  cargo fmt --all

typos:
  typos || { echo "typos check failed"; exit 1; }

pre-commit:
  just fmt-check
  just check
  just lint
  just typos

install-pre-commit:
  echo 'just pre-commit' > .git/hooks/pre-commit && chmod +x .git/hooks/pre-commit

uninstall-pre-commit:
  rm .git/hooks/pre-commit

########################################
# Database

reset-db:
  rm -f {{SQLITE_FILE}}

db:
  rlwrap sqlite3 -table {{SQLITE_FILE}} || true

########################################
# Build

build-all: build build-nix build-x86_64-linux build-aarch64-linux build-aarch64-darwin build-docs

build:
  cargo build --bin {{project}}

build-nix:
  nix build .#{{project}}

build-x86_64-linux:
  nix build .#{{project}}-x86_64-linux

build-aarch64-linux:
  nix build .#{{project}}-aarch64-linux

build-aarch64-darwin:
  nix build .#{{project}}-aarch64-darwin

build-docs:
  cargo doc

########################################
# Tests

test:
  cargo test

mutate lang:
  cargo run --bin {{project}} -- mutate tests/{{lang}}/examples

remutate lang: reset-db
  just mutate {{lang}}

run lang:
  cargo run --bin {{project}} -- run tests/{{lang}}/examples --test-cmd "sleep 1; echo test passed"

rerun lang: reset-db
  just run {{lang}}

########################################
# Nix Installation

install-nix:
  nix profile add .#{{project}}

uninstall-nix:
  nix profile remove {{project}}

upgrade-nix:
  nix profile upgrade {{project}}

