#!/usr/bin/env bash
# check-seam-contract-parity.sh
#
# The keystone seam-conformance probe (plan: executable-seams-first.md,
# Bundle A / decision D2). Generalizes the existing Port/Adapter parity gate
# from Ports to *every declared seam*: a Seam Contract must have a machine-
# readable **Shape** and a shared **Behavior** suite. This script proves the
# structural half — for each seam declared in the manifest, the Shape file and
# the Behavior test suite must actually exist (and be non-empty).
#
# It does NOT prove the suite *ran* — that is enforced by the `verify` entry
# point wiring plus the Acceptance <-> Test traceability check (see
# `verify-and-assemble-pr` Step 3). This script is the static parity half:
# a declared seam with no Shape, or no Behavior suite on disk, is a red flag.
#
# Declaration source — `.seam-contracts.json` at the repo root:
#   {
#     "mode": "warn",                       // "warn" (default) | "enforce" (D3)
#     "contractsDir": "docs/product/contracts",
#     "seams": [
#       {
#         "id":       "publish-api@v1",      // <seam-name>@v<N>, unique, frozen
#         "kind":     "http",                // http | event | port | cli
#         "shape":    "docs/product/contracts/publish-api.v1.openapi.yaml",
#         "behavior": "src/publishing/publish-api.contract.test.ts"
#       }
#     ]
#   }
#
# `shape` and `behavior` may each be an exact path or a glob (e.g.
# "src/**/publish-api.contract.test.*"). Globs are resolved with `find -path`,
# so `*` and `**` both match across directory separators (portable on bash 3.2,
# which lacks `globstar`). At least one non-empty match must exist.
#
# Default Shape form per kind (D2; project MAY override the file extension):
#   http  -> OpenAPI       (.openapi.yaml / .openapi.json)
#   event -> JSON-Schema   (.schema.json)
#   port  -> native typed interface (*.ports.<ext>)
#   cli   -> usage/spec doc or schema
#
# Mode (D3 — warn-first, mechanical promotion to fail-closed):
#   "warn"    (default) — report findings, exit 0.
#   "enforce"           — report findings, exit 1. Promote once a wave closes clean.
#
# When `.seam-contracts.json` is absent, the project has declared no seams yet;
# there is nothing to check and the probe skips (exit 0).
#
# Compatible with bash 3.2+ (macOS default).
#
# Exit codes:
#   0 — clean, no manifest, or findings in warn mode
#   1 — findings in enforce mode
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-seam-contract-parity: error: not a directory: $ROOT" >&2
  exit 2
fi

MANIFEST="$ROOT/.seam-contracts.json"
if [[ ! -f "$MANIFEST" ]]; then
  echo "check-seam-contract-parity: no .seam-contracts.json under $ROOT; skipping"
  exit 0
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "check-seam-contract-parity: python3 required to read the manifest; skipping" >&2
  exit 0
fi

# ---- Read manifest ----------------------------------------------------------

MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$MANIFEST")

# Emit one tab-separated record per seam: id \t kind \t shape \t behavior.
# Use command substitution (not process substitution) so the manifest-parse
# exit code is captured: `$?` after `done < <(python3 ...)` reflects the while
# loop, not python, which would let a corrupt manifest pass silently as "no
# seams".
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

# ---- Helpers ----------------------------------------------------------------

VALID_KINDS='http event port cli'

# Does at least one non-empty file match PATH_OR_GLOB (relative to ROOT)?
# Globs are resolved with `find -path` so that `*` and `**` both match across
# directory separators. bash 3.2 (the declared floor) has no `globstar`, so a
# bare shell glob cannot honor the documented `src/**/x.contract.test.*` form;
# find can.
exists_nonempty() {
  local spec="$1"
  [[ -z "$spec" ]] && return 1
  case "$spec" in
    *'*'*|*'?'*|*'['*)
      # Glob: match the full path; find's `*`/`**` cross '/'.
      local m
      while IFS= read -r m; do
        [[ -s "$m" ]] && return 0
      done < <(find "$ROOT" -type f -path "$ROOT/$spec" 2>/dev/null)
      return 1
      ;;
    *)
      # Exact path: fast, unambiguous.
      [[ -s "$ROOT/$spec" ]] && return 0
      return 1
      ;;
  esac
}

# ---- Check ------------------------------------------------------------------

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

  # id present and well-formed: <name>@v<N>
  if [[ -z "$sid" ]]; then
    FINDINGS+=("seam with no id — every seam needs a frozen <name>@v<N> id")
  elif ! printf '%s' "$sid" | grep -Eq '^[a-z0-9][a-z0-9-]*@v[0-9]+$'; then
    FINDINGS+=("$label: id is not of the form <name>@v<N> (kebab-case name, integer version)")
  elif printf '%s' "$SEEN_IDS" | grep -Fq " $sid "; then
    FINDINGS+=("$label: duplicate seam id — ids must be unique")
  fi
  SEEN_IDS="$SEEN_IDS$sid "

  # kind in the allowed set
  case " $VALID_KINDS " in
    *" $kind "*) : ;;
    *) FINDINGS+=("$label: kind '${kind:-<missing>}' is not one of: $VALID_KINDS") ;;
  esac

  # Shape exists
  if [[ -z "$shape" ]]; then
    FINDINGS+=("$label: no Shape declared (machine-readable interface required)")
  elif ! exists_nonempty "$shape"; then
    FINDINGS+=("$label: Shape not found or empty: $shape")
  fi

  # Behavior suite exists
  if [[ -z "$behavior" ]]; then
    FINDINGS+=("$label: no Behavior suite declared (shared contract test required)")
  elif ! exists_nonempty "$behavior"; then
    FINDINGS+=("$label: Behavior suite not found or empty: $behavior")
  fi
done

# ---- Report -----------------------------------------------------------------

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
  cat <<EOF
Every seam declared in .seam-contracts.json must carry both halves of its
contract on disk:

  - Shape    — a machine-readable interface (OpenAPI for http, JSON-Schema for
               event, native typed *.ports.* for port, usage/spec for cli).
  - Behavior — the shared contract test suite both sides of the seam must pass.

Declare or repair seams with the 'define-seam-contract' skill. The contract
suite must also RUN in the verify entry point (see verify-and-assemble-pr
Step 3) — this script only proves the Shape and suite exist.

Mode is '$MODE'. Set "mode": "enforce" in .seam-contracts.json once a wave
closes with every declared seam in parity (plan decision D3).
EOF
} >&2

if [[ "$MODE" == "enforce" ]]; then
  exit 1
fi
exit 0
