#!/usr/bin/env bash
# check-port-adapter-parity.sh
#
# For every `*.ports.*` file (a capability's port/interface module), ensure at
# least one matching adapter exists in the same capability folder AND verify via
# AST parsing (`scripts/ast_parse.sh`) that adapter implementations satisfy the
# method signatures declared in the Port interface.
#
# Exit codes:
#   0 — clean (or warnings only)
#   1 — port without adapter or adapter method AST mismatch
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-port-adapter-parity: error: not a directory: $ROOT" >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AST_PARSER="$SCRIPT_DIR/ast_parse.sh"

SCAN_PATHS=()
CONFIG="$ROOT/.anti-dumping.json"
if [[ -f "$CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
  while IFS= read -r line; do
    [[ -n "$line" ]] && SCAN_PATHS+=("$line")
  done < <(python3 -c "
import json, sys, re
try:
    cfg = json.load(open(sys.argv[1]))
    for p in cfg.get('scanPaths', []) or []:
        print(re.sub(r'/\*\*?$', '', p))
except Exception:
    pass
" "$CONFIG")
fi

if [[ ${#SCAN_PATHS[@]} -eq 0 ]]; then
  SCAN_PATHS=(src packages services apps)
fi

EXISTING=()
for p in "${SCAN_PATHS[@]}"; do
  [[ -d "$ROOT/$p" ]] && EXISTING+=("$ROOT/$p")
done

if [[ ${#EXISTING[@]} -eq 0 ]]; then
  echo "check-port-adapter-parity: no scan paths exist under $ROOT; skipping"
  exit 0
fi

VIOLATIONS=0
WARNINGS=0
PORT_COUNT=0

# Find all *.ports.* files.
while IFS= read -r -d '' port; do
  PORT_COUNT=$((PORT_COUNT + 1))
  dir="$(dirname "$port")"
  fname="$(basename "$port")"
  base="${fname%%.ports.*}"
  ext="${fname##*.ports.}"

  has_memory=0
  has_prod=0
  memory_file=""
  prod_file=""

  for f in "$dir"/"$base".adapter-memory."$ext" "$dir"/"$base".adapter-memory.*; do
    if [[ -f "$f" ]]; then
      has_memory=1
      memory_file="$f"
    fi
  done

  for f in "$dir"/"$base".repository."$ext" "$dir"/"$base".repository.* \
           "$dir"/"$base".adapter-*."$ext" "$dir"/"$base".adapter-*.*; do
    if [[ -f "$f" && "$f" != *adapter-memory* ]]; then
      has_prod=1
      prod_file="$f"
    fi
  done

  if [[ $has_memory -eq 0 && $has_prod -eq 0 ]]; then
    echo "check-port-adapter-parity: no adapter for $port" >&2
    VIOLATIONS=$((VIOLATIONS + 1))
    continue
  elif [[ $has_memory -eq 0 ]]; then
    echo "check-port-adapter-parity: warn: no in-memory adapter for $port" >&2
    WARNINGS=$((WARNINGS + 1))
  fi

  # AST Parity Verification (TS-031)
  if [[ -x "$AST_PARSER" ]] && command -v python3 >/dev/null 2>&1; then
    target_adapter="${memory_file:-$prod_file}"
    if [[ -n "$target_adapter" && -f "$target_adapter" ]]; then
      ast_err=$(python3 -c "
import subprocess, json, sys

port_file = sys.argv[1]
adapter_file = sys.argv[2]
ast_parser = sys.argv[3]

def get_ast(path):
    try:
        res = subprocess.run([ast_parser, path], capture_output=True, text=True, timeout=5)
        if res.returncode == 0:
            return json.loads(res.stdout)
    except Exception:
        pass
    return None

port_ast = get_ast(port_file)
adapter_ast = get_ast(adapter_file)

if port_ast and adapter_ast:
    port_methods = set()
    for iface in port_ast.get('interfaces', []):
        for m in iface.get('methods', []):
            port_methods.add(m.get('name'))

    adapter_methods = set()
    for iface in adapter_ast.get('interfaces', []):
        for m in iface.get('methods', []):
            adapter_methods.add(m.get('name'))

    missing = port_methods - adapter_methods
    if missing:
        print(f'missing AST methods: {sorted(list(missing))}')
" "$port" "$target_adapter" "$AST_PARSER" 2>/dev/null || true)

      if [[ -n "$ast_err" ]]; then
        echo "check-port-adapter-parity: AST mismatch on $target_adapter ($ast_err)" >&2
        VIOLATIONS=$((VIOLATIONS + 1))
      fi
    fi
  fi

done < <(find "${EXISTING[@]}" -type f \
  \( -name '*.ports.ts' -o -name '*.ports.tsx' \
     -o -name '*.ports.js' -o -name '*.ports.mjs' \
     -o -name '*.ports.java' -o -name '*.ports.kt' \
     -o -name '*.ports.py' \) \
  -not -path '*/node_modules/*' -not -path '*/dist/*' -not -path '*/build/*' \
  -not -path '*/target/*' -not -path '*/.git/*' \
  -print0 2>/dev/null)

if [[ $VIOLATIONS -gt 0 ]]; then
  echo "check-port-adapter-parity: $VIOLATIONS port file(s) with adapter violations" >&2
  exit 1
fi

echo "check-port-adapter-parity: clean ($PORT_COUNT port files, $WARNINGS warning(s))"
exit 0
