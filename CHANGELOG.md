# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## 2.0.1 - 2026-02-10

### Changed
- **BREAKING**: Major architectural overhaul - muton now imports and uses [mewt](https://github.com/trailofbits/mewt) as its core library
  - All core mutation testing logic moved to mewt
  - Muton now focuses solely on supporting TON-specific languages and grammar integration
- **BREAKING**: Configuration system overhauled with unified CLI/file symmetry (from mewt)
  - Configuration now uses dotted notation for CLI flags (e.g., `--log.level`, `--test.cmd`, `--test.timeout`)
  - Config file structure reorganized with nested sections (`[log]`, `[targets]`, `[run]`, `[test]`)
  - Added support for per-target test rules via `[[test.per_target]]` array in config file
  - CLI overrides now replace (not merge) config file values
- **BREAKING**: Removed environment variable configuration support
  - Previously supported variables (`MUTON_LOG_LEVEL`, `MUTON_DB`, `MUTON_TEST_CMD`, etc.) are no longer recognized
- **BREAKING**: Removed `muton print results` command
  - Use `muton results` instead

### Added
- `muton status` command for campaign overview with per-file breakdown and aggregates
  - `--format` option: "table" (default) or "json"
- `muton results` command with enhanced filtering
  - Filtering options: `--status`, `--language`, `--mutation_type`, `--line`, `--file`
  - SARIF output format support (`--format sarif`)
  - JSON and "ids" output formats
- `muton test --ids-file` option to read mutant IDs from file or stdin (use `-` for stdin)
- `muton print config` command to display the effective configuration
- JSON output format support for multiple commands:
  - `muton print mutations --format json`
  - `muton print targets --format json`
  - `muton print mutants --format json`
- Enhanced filtering for `print results`, `print mutants`, and `results` commands:
  - `--status`: Filter by outcome status (Uncaught, TestFail, Skipped, Timeout)
  - `--language`: Filter by programming language
  - `--mutation_type`: Filter by mutation slug (e.g., ER, CR, BR)
  - `--line`: Filter by line number
  - `--file`: Filter by file path (substring match)
- `muton print mutants` filtering options:
  - `--tested`: Show only mutants with test outcomes
  - `--untested`: Show only mutants without test outcomes
  - `--format ids`: Output just mutant IDs, one per line

### Fixed
- Percentage complete display in `status` command campaign summary
- Status filtering is now case-insensitive for `--status` flag

### Removed
- `BuildFail` outcome status (simplified outcome types)

## 1.0.0 - 2024-12-20

Initial release of muton as a standalone mutation testing tool.
