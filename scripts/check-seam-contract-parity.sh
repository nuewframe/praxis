#!/usr/bin/env bash
# check-seam-contract-parity.sh
#
# The keystone seam-conformance probe (ADR.260725 lineage; # Bundle A / decision D2). Generalizes the existing Port/Adapter parity gate
# from Ports to *every declared seam*: a Seam Contract must have a machine-
# readable **Shape** (verified via AST parsing `scripts/ast_parse.sh`) and a shared **Behavior** suite.
#
# Exit codes:
#   0 — clean, no manifest, or findings in warn mode
#   1 — findings in enforce mode
#   2 — invocation error

set -u

ROOT="."
GENERATE_MODE=0

for arg in "$@"; do
  if [[ "$arg" == "--generate" ]]; then
    GENERATE_MODE=1
  elif [[ -d "$arg" ]]; then
    ROOT="${arg%/}"
  fi
done

if [[ ! -d "$ROOT" ]]; then
  echo "check-seam-contract-parity: error: not a directory: $ROOT" >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AST_PARSER="$SCRIPT_DIR/ast_parse.sh"

# ---- Handle --generate mode (TS-032) ----------------------------------------
if [[ $GENERATE_MODE -eq 1 ]]; then
  if ! command -v python3 >/dev/null 2>&1; then
    echo "check-seam-contract-parity: python3 required for --generate mode" >&2
    exit 2
  fi

  echo "check-seam-contract-parity: extracting AST seam contracts under $ROOT..."
  python3 -c "
import os, sys, json, subprocess

root = sys.argv[1]
ast_parser = sys.argv[2]
manifest_path = os.path.join(root, '.seam-contracts.json')

contracts = []
for dirpath, _, filenames in os.walk(root):
    if any(p in dirpath for p in ('.git', 'node_modules', 'dist', 'build', 'target')):
        continue
    for fn in filenames:
        if '.ports.' in fn or '.contract.' in fn:
            filepath = os.path.join(dirpath, fn)
            try:
                res = subprocess.run([ast_parser, filepath], capture_output=True, text=True, timeout=5)
                if res.returncode == 0:
                    data = json.loads(res.stdout)
                    for iface in data.get('interfaces', []):
                        contracts.append({
                            'id': f\"{iface.get('name').lower()}@v1\",
                            'kind': 'port',
                            'shape': os.path.relpath(filepath, root),
                            'behavior': os.path.relpath(filepath, root).replace('.ports.', '.contract.test.').replace('.contract.', '.contract.test.')
                        })
            except Exception:
                pass

manifest = {
    'mode': 'warn',
    'contractsDir': 'docs/product/contracts',
    'seams': contracts
}

with open(manifest_path, 'w', encoding='utf-8') as f:
    json.dump(manifest, f, indent=2)

print(f'check-seam-contract-parity: extracted {len(contracts)} AST seam contracts into .seam-contracts.json')
" "$ROOT" "$AST_PARSER"
  exit 0
fi

# ---- Normal validation mode --------------------------------------------------

MANIFEST="$ROOT/.seam-contracts.json"
if [[ ! -f "$MANIFEST" ]]; then
  echo "check-seam-contract-parity: no .seam-contracts.json under $ROOT; skipping"
  exit 0
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "check-seam-contract-parity: python3 required to read the manifest; skipping" >&2
  exit 0
fi

# Read manifest mode & seams
MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$MANIFEST")

SEAMS_RAW=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
except Exception as e:
    sys.stderr.write('check-seam-contract-parity: cannot parse manifest: %s\n' % e)
    sys.exit(3)
for s in cfg.get('seams', []) or []:
    sid = str(s.get('id', '')).strip()
    kind = str(s.get('kind', '')).strip()
    shape = str(s.get('shape', '')).strip()
    beh = str(s.get('behavior', '')).strip()
    print('\t'.join([sid, kind, shape, beh]))
" "$MANIFEST")
PY_RC=$?

if [[ $PY_RC -eq 3 ]]; then
  echo "check-seam-contract-parity: invalid manifest" >&2
  exit 2
elif [[ $PY_RC -ne 0 ]]; then
  echo "check-seam-contract-parity: manifest read failed (exit $PY_RC)" >&2
  exit 2
fi

SEAMS=()
while IFS= read -r line; do
  [[ -n "$line" ]] && SEAMS+=("$line")
done <<< "$SEAMS_RAW"

if [[ ${#SEAMS[@]} -eq 0 ]]; then
  echo "check-seam-contract-parity: manifest declares no seams; nothing to check"
  exit 0
fi

VALID_KINDS='http event port cli'

exists_nonempty() {
  local spec="$1"
  [[ -z "$spec" ]] && return 1
  case "$spec" in
    *'*'*|*'?'*|*'['*)
      local m
      while IFS= read -r m; do
        [[ -s "$m" ]] && return 0
      done < <(find "$ROOT" -type f -path "$ROOT/$spec" 2>/dev/null)
      return 1
      ;;
    *)
      [[ -s "$ROOT/$spec" ]] && return 0
      return 1
      ;;
  esac
}

FINDINGS=()
SEEN_IDS=" "

for rec in "${SEAMS[@]}"; do
  sid="${rec%%$'\t'*}"
  rest="${rec#*$'\t'}"
  kind="${rest%%$'\t'*}"
  rest="${rest#*$'\t'}"
  shape="${rest%%$'\t'*}"
  behavior="${rest#*$'\t'}"

  label="${sid:-<unnamed seam>}"

  if [[ -z "$sid" ]]; then
    FINDINGS+=("seam with no id — every seam needs a frozen <name>@v<N> id")
  elif ! printf '%s' "$sid" | grep -Eq '^[a-z0-9][a-z0-9-]*@v[0-9]+$'; then
    FINDINGS+=("$label: id is not of the form <name>@v<N> (kebab-case name, integer version)")
  elif printf '%s' "$SEEN_IDS" | grep -Fq " $sid "; then
    FINDINGS+=("$label: duplicate seam id — ids must be unique")
  fi
  SEEN_IDS="$SEEN_IDS$sid "

  case " $VALID_KINDS " in
    *" $kind "*) : ;;
    *) FINDINGS+=("$label: kind '${kind:-<missing>}' is not one of: $VALID_KINDS") ;;
  esac

  if [[ -z "$shape" ]]; then
    FINDINGS+=("$label: no Shape declared (machine-readable interface required)")
  elif ! exists_nonempty "$shape"; then
    FINDINGS+=("$label: Shape not found or empty: $shape")
  fi

  if [[ -z "$behavior" ]]; then
    FINDINGS+=("$label: no Behavior suite declared (shared contract test required)")
  elif ! exists_nonempty "$behavior"; then
    FINDINGS+=("$label: Behavior suite not found or empty: $behavior")
  fi
done

if [[ ${#FINDINGS[@]} -eq 0 ]]; then
  echo "check-seam-contract-parity: clean (${#SEAMS[@]} seam(s), mode=$MODE)"
  exit 0
fi

{
  echo "check-seam-contract-parity: ${#FINDINGS[@]} seam-contract parity issue(s) (mode=$MODE)"
  echo
  for f in "${FINDINGS[@]}"; do
    echo "  - $f"
  done
  echo
} >&2

if [[ "$MODE" == "enforce" ]]; then
  exit 1
fi
exit 0
