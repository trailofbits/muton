#!/usr/bin/env bash
set -euo pipefail

# Update tree-sitter grammar for a specific language
# Usage: update-grammar.sh <language> [dry_run]
# Examples:
#   update-grammar.sh func true   # Preview what would be updated
#   update-grammar.sh func false  # Actually perform the update

language="${1:-}"
dry_run="${2:-false}"

# Language configuration mappings
# To add a new language, just add entries to these associative arrays
declare -A REPO_URLS=(
  ["func"]="https://github.com/ton-blockchain/ton-language-server"
  ["tact"]="https://github.com/tact-lang/tact-language-server"
  ["tolk"]="https://github.com/ton-blockchain/ton-language-server"
)

declare -A GRAMMAR_PATHS=(
  ["func"]="server/src/languages/func/tree-sitter-func"
  ["tact"]="server/src/languages/tact/tree-sitter-tact"
  ["tolk"]="server/src/languages/tolk/tree-sitter-tolk"
)

# Validate language argument
if [ -z "$language" ]; then
  echo "Error: Language argument is required"
  echo "Usage: $0 <language> [dry_run]"
  echo "Supported languages: ${!REPO_URLS[*]}"
  exit 1
fi

# Check if language is supported
if [[ ! -v REPO_URLS["$language"] ]]; then
  echo "Error: Language '$language' is not supported"
  echo "Supported languages: ${!REPO_URLS[*]}"
  echo ""
  echo "To add support for a new language, add entries to REPO_URLS and GRAMMAR_PATHS in this script"
  exit 1
fi

# Get configuration for the specified language
repo_url="${REPO_URLS[$language]}"
grammar_path="${GRAMMAR_PATHS[$language]}"

if [ "$dry_run" = "true" ]; then
  echo "DRY RUN: Would update $language grammar (no changes will be made)"
  echo "Repository: $repo_url"
  echo "Grammar path: $grammar_path"
else
  echo "Updating $language grammar..."
  echo "Repository: $repo_url"
  echo "Grammar path: $grammar_path"
fi

# Step 1: Backup current grammar (temporary, outside repo)
echo "Backing up current grammar..."
BACKUP_DIR="/tmp/${language}-src.backup.$(date +%Y%m%d_%H%M%S)"
if [ -d "grammar/$language/src" ]; then
  if [ "$dry_run" = "false" ]; then
    rm -rf "$BACKUP_DIR"
    mkdir -p "$BACKUP_DIR"
    cp -r "grammar/$language/src" "$BACKUP_DIR/"
    echo "Backup created (temporary): $BACKUP_DIR"
  else
    echo "Would create temporary backup: $BACKUP_DIR"
  fi
fi

# Step 2: Clone upstream grammar repository  
echo "Cloning upstream grammar repository..."
TEMP_DIR="/tmp/$language-grammar-update"
if [ "$dry_run" = "false" ]; then
  rm -rf "$TEMP_DIR"
  git clone "$repo_url" "$TEMP_DIR"
  # Capture the vendored commit (latest of default branch)
  vendored_commit="$(git -C "$TEMP_DIR" rev-parse HEAD)"
else
  echo "Would clone: $repo_url → $TEMP_DIR"
  echo "Would verify files: parser.c and tree_sitter/ headers"
  echo "Would copy new grammar files to grammar/$language/src/"
  echo "Would copy grammar.js to grammar/$language/"
  echo "Would test compilation with: cargo check"
  echo "Would run parser tests with: cargo test parser"
  echo "Dry run completed - no changes made"
  echo "Run 'just update-grammar $language' to perform the actual update"
  exit 0
fi

# Step 3: Verify generated files exist
echo "Verifying generated files..."
if [ ! -f "$TEMP_DIR/$grammar_path/src/parser.c" ]; then
  echo "Error: parser.c not found in upstream repository"
  echo "Expected: $TEMP_DIR/$grammar_path/src/parser.c"
  rm -rf "$TEMP_DIR"
  exit 1
fi

if [ ! -d "$TEMP_DIR/$grammar_path/src/tree_sitter" ]; then
  echo "Error: tree_sitter headers not found in upstream repository"
  echo "Expected: $TEMP_DIR/$grammar_path/src/tree_sitter/"
  rm -rf "$TEMP_DIR"
  exit 1
fi

# Step 4: Copy new files
echo "Copying new grammar files..."
rm -rf "grammar/$language/src"
mkdir -p "grammar/$language/src"
cp -r "$TEMP_DIR/$grammar_path/src/"* "grammar/$language/src/"
cp "$TEMP_DIR/$grammar_path/grammar.js" "grammar/$language/"

# Record vendored metadata for traceability
{
  echo "$vendored_commit" > "grammar/$language/VENDORED_COMMIT"
} >/dev/null 2>&1 || true
{
  echo "repo=$repo_url"
  echo "grammar_path=$grammar_path"
  echo "commit=$vendored_commit"
  echo "date=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > "grammar/$language/VENDORED_METADATA" || true

# Step 5: Clean up temporary directory
echo "Cleaning up..."
rm -rf "$TEMP_DIR"

# Step 6: Test compilation
if command -v cargo >/dev/null 2>&1; then
  echo "Testing compilation..."
  if ! cargo check; then
    echo "Error: Compilation failed after grammar update"
    echo "You may need to update node type mappings in src/parser/$language.rs"
    exit 1
  fi
else
  echo "cargo not found, skipping compilation test"
fi

# Step 7: Run parser tests
if command -v cargo >/dev/null 2>&1; then
  echo "Running parser tests..."
  if ! cargo test parser; then
    echo "Warning: Parser tests failed - you may need to update node type mappings"
    echo "Check src/parser/$language.rs NodeType::from() mappings"
    exit 1
  fi
else
  echo "cargo not found, skipping parser tests"
fi

echo "Grammar update completed successfully!"
echo "Consider running 'just check && just build' to verify everything works"
echo "Ready to commit the changes"
