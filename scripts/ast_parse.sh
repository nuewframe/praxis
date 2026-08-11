#!/usr/bin/env bash
set -euo pipefail

# =============================================================================
# Polyglot AST Parser Dispatcher Bridge (ast-parser@v1)
#
# Dispatches target source files to language-native AST runners:
#   - *.ts / *.js  -> node scripts/ast_parse_ts.js
#   - *.py         -> python3 scripts/ast_parse_py.py
#   - *.go         -> go run scripts/ast_parse_go.go
#   - *.rs         -> rustc / runner scripts/ast_parse_rs.rs
#   - *.cs         -> dotnet run scripts/ast_parse_cs.cs
#   - *.kt / *.java -> java / kotlinc scripts/ast_parse_kt.kt
# =============================================================================

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <file-path>" >&2
  exit 1
fi

FILE_PATH="$1"

if [ ! -f "$FILE_PATH" ]; then
  echo "Error: File '$FILE_PATH' not found." >&2
  exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

EXT="${FILE_PATH##*.}"

case "$EXT" in
  ts|js|tsx|jsx)
    if command -v node >/dev/null 2>&1; then
      exec node "$SCRIPT_DIR/ast_parse_ts.cjs" "$FILE_PATH"
    else
      echo "Warning: node not found, falling back to Python AST parser bridge." >&2
      exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    fi
    ;;
  py)
    if command -v python3 >/dev/null 2>&1; then
      exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    else
      echo "Error: python3 is required for Python AST parsing." >&2
      exit 1
    fi
    ;;
  go)
    if [ -x "$SCRIPT_DIR/bin/ast_parse_go" ]; then
      exec "$SCRIPT_DIR/bin/ast_parse_go" "$FILE_PATH"
    elif command -v go >/dev/null 2>&1; then
      (cd "$SCRIPT_DIR" && exec go run ast_parse_go.go "$FILE_PATH")
    else
      echo "Warning: go CLI not found, falling back to Python AST parser bridge." >&2
      exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    fi
    ;;
  rs)
    if command -v python3 >/dev/null 2>&1; then
      exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    else
      echo "Error: python3 is required for Rust AST parser bridge." >&2
      exit 1
    fi
    ;;
  cs)
    if command -v python3 >/dev/null 2>&1; then
      exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    else
      echo "Error: python3 is required for C# AST parser bridge." >&2
      exit 1
    fi
    ;;
  kt|java)
    if command -v python3 >/dev/null 2>&1; then
      exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    else
      echo "Error: python3 is required for JVM AST parser bridge." >&2
      exit 1
    fi
    ;;
  *)
    # Default fallback to Python AST parser bridge
    exec python3 "$SCRIPT_DIR/ast_parse_py.py" "$FILE_PATH"
    ;;
esac
