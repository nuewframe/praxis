#!/usr/bin/env bash
# check-port-adapter-parity.sh
#
# For every `*.ports.*` file (a capability's port/interface module), ensure at
# least one matching adapter exists in the same capability folder. This guards
# against the dumping-ground pattern of declaring an interface and never
# implementing it, or implementing it only as production code without an
# in-memory test double (which signals untestable design).
#
# Parity rule: for each `<base>.ports.<ext>` we expect EITHER:
#   - `<base>.adapter-memory.<ext>` (the test double), AND
#   - at least one of `<base>.repository.<ext>` or `<base>.adapter-*.<ext>`
#
# When only the production adapter is present the script warns rather than
# fails (the in-memory double is the stricter ideal but not universal).
#
# Compatible with bash 3.2+ (macOS default).
#
# Exit codes:
#   0 — clean (or warnings only)
#   1 — port without any adapter
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-port-adapter-parity: error: not a directory: $ROOT" >&2
  exit 2
fi

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
  # Strip ".ports.<ext>" → keep <base>; capture <ext>.
  base="${fname%%.ports.*}"
  ext="${fname##*.ports.}"

  has_memory=0
  has_prod=0

  for f in "$dir"/"$base".adapter-memory."$ext" "$dir"/"$base".adapter-memory.*; do
    [[ -f "$f" ]] && has_memory=1
  done

  for f in "$dir"/"$base".repository."$ext" "$dir"/"$base".repository.* \
           "$dir"/"$base".adapter-*."$ext" "$dir"/"$base".adapter-*.*; do
    [[ -f "$f" && "$f" != *adapter-memory* ]] && has_prod=1
  done

  if [[ $has_memory -eq 0 && $has_prod -eq 0 ]]; then
    echo "check-port-adapter-parity: no adapter for $port" >&2
    VIOLATIONS=$((VIOLATIONS + 1))
  elif [[ $has_memory -eq 0 ]]; then
    echo "check-port-adapter-parity: warn: no in-memory adapter for $port" >&2
    WARNINGS=$((WARNINGS + 1))
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
  echo "check-port-adapter-parity: $VIOLATIONS port file(s) without any adapter" >&2
  exit 1
fi

echo "check-port-adapter-parity: clean ($PORT_COUNT port files, $WARNINGS warning(s))"
exit 0
