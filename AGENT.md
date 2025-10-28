---
alwaysApply: true
---

# Muton Agent Guidelines

## Development Commands
- `just check` - Fast syntax/type checking (prefer over full build)
- `just build` - Full compilation
- `just fmt` - Format code (run after any batch of changes)
- `just lint` - Run linters
- `just test` - Run tests
- `just run` - Run muton against some simple examples

## Database Changes
- Do not change the database schemas as defined in `migrations/`
- If you think database migrations are required, halt and ask the user for confirmation
- Always run `just reset-db` after making schema or SQL query changes

## Git Operations
- **ONLY use read-only git commands** - Never modify the working tree
- You can read git history, logs, and status, but do not commit, push, or modify files via git

## Code Style Requirements
- **Imports**: Group as std, external crates, then local with `use crate::`
- **Error handling**: Use `thiserror` for custom errors, `Result<T, Error>` pattern, prefer the `?` operator with proper error mapping
- **Async**: Use `async/await` with tokio runtime
- **Types**: All types are in `src/types/`
- **Naming**: snake_case for functions/variables, PascalCase for types
- **Graphics**: Never use emoji characters.
- **SQL queries**: Use sqlx with raw SQL queries inside r#"..."# strings inside the src/store.rs file
