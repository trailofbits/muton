project := "muton"
export SQLITE_FILE := project + ".sqlite"

########################################
# Common dev commands

lint:
  cargo clippy --lib -p {{project}} --tests

lint-fix:
  cargo clippy --lib -p {{project}} --tests --fix

check:
  cargo check

fmt:
  cargo fmt

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
  cargo build --bin muton

build-nix:
  nix build .#muton

build-x86_64-linux:
  nix build .#muton-x86_64-linux

build-aarch64-linux:
  nix build .#muton-aarch64-linux

build-aarch64-darwin:
  nix build .#muton-aarch64-darwin

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

install-nix: build-nix
  nix profile add ./result

uninstall-nix:
  nix profile remove muton

reinstall-nix: uninstall-nix
  just install-nix

