project := "muton"
export SQLITE_FILE := project + ".sqlite"
export DATABASE_URL := "sqlite:" + SQLITE_FILE

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

init-db:
  command -v sqlx >/dev/null 2>&1 || cargo install sqlx-cli
  touch {{SQLITE_FILE}}
  cargo sqlx migrate run
  cargo sqlx prepare

reset-db:
  rm -f {{SQLITE_FILE}}
  just init-db

db:
  rlwrap sqlite3 -table {{SQLITE_FILE}} || true

########################################
# Build

build-all: build build-nix build-x86_64-linux build-aarch64-linux build-aarch64-darwin build-docs

build: init-db
  cargo build --bin muton

build-nix: init-db
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

remutate: reset-db
  cargo run --bin muton -- mutate tests/examples/func/hello-world.fc

mutate:
  cargo run --bin muton -- mutate tests/examples/func/hello-world.fc

rerun: reset-db
  cargo run --bin muton -- run tests/examples --test-cmd "sleep 1; echo test passed"

run:
  cargo run --bin muton -- run tests/examples --test-cmd "sleep 1; echo test passed"

########################################
# Nix Installation

install-nix: build-nix
  nix profile install ./result

uninstall-nix:
  nix profile remove muton

reinstall-nix: uninstall-nix
  just install-nix

