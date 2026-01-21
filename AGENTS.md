---
alwaysApply: true
---

# Agent Guidelines

## Development Commands
- `just check` - Fast syntax/type checking (prefer over full build)
- `just build` - Full compilation
- `just fmt` - Format code (run after any batch of changes)
- `just lint` - Run linters
- `just test` - Run tests
- `just run` - Run tool against some simple examples

## Git Operations
- **ONLY use read-only git commands** - Never modify the working tree
- You can read git history, logs, and status, but do not commit, push, or modify files via git

## Engineering Guidelines
- Do not write code before stating assumptions.
- Do not claim correctness you haven't verified.
- Do not handle only the happy path.
- Under what conditions does this work?
