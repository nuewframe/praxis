#!/usr/bin/env bash
# check-stateless-request-path.sh
#
# Executable probe for the "Horizontally scalable" production-readiness anchor
# (plan: executable-seams-first.md, Bundle B3). Converts the anchor from an
# asserted checklist line into a build-time gate: it fails (or warns) when the
# request path introduces node-local mutable state — a module-level mutable
# singleton used as a cache/session/registry, or a static mutable collection.
# Node-local request state breaks horizontal scaling: a second replica does not
# share it, so behavior depends on which node served the request.
#
# This is a HEURISTIC, not a proof. It is conservative and precision-biased: it
# flags the high-signal deterministic shapes (a *module-level* mutable container
# bound to a state-ish name, or a *static* mutable collection) and offers an
# explicit reviewed opt-out for legitimate process-local caches (e.g. a memoized
# pure lookup, a connection pool) so exceptions are a visible artifact.
#
# Opt-out (per-line, reviewed):
#   Append a comment containing  praxis:allow-local-state  to the offending line
#   (with the reason). Silence fails; an un-reasoned singleton does not.
#
# Mode (decision D3 — warn-first, mechanical promotion to fail-closed):
#   Read from .statelessness.json `mode`:
#     "warn"    (default) — report findings, exit 0. Use until a wave closes clean.
#     "enforce"           — report findings, exit 1. Promote here once clean.
#
# Config (.statelessness.json at repo root, all keys optional):
#   {
#     "mode":      "warn",          // "warn" | "enforce"
#     "scanPaths": ["src/**"],      // falls back to .anti-dumping.json, then defaults
#     "allow":     ["regex", ...]   // extra allowlist regexes (matched against the line)
#   }
#
# Compatible with bash 3.2+ (macOS default).
#
# Exit codes:
#   0 — clean, or findings in warn mode
#   1 — findings in enforce mode
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-stateless-request-path: error: not a directory: $ROOT" >&2
  exit 2
fi

OPT_OUT_MARKER='praxis:allow-local-state'

# ---- Config -----------------------------------------------------------------

MODE='warn'
ALLOW_PATTERNS=()
SCAN_PATHS=()

ST_CONFIG="$ROOT/.statelessness.json"
if [[ -f "$ST_CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
  MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$ST_CONFIG")
  while IFS= read -r line; do
    [[ -n "$line" ]] && ALLOW_PATTERNS+=("$line")
  done < <(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    for p in cfg.get('allow', []) or []:
        print(p)
except Exception:
    pass
" "$ST_CONFIG")
  while IFS= read -r line; do
    [[ -n "$line" ]] && SCAN_PATHS+=("$line")
  done < <(python3 -c "
import json, sys, re
try:
    cfg = json.load(open(sys.argv[1]))
    for p in cfg.get('scanPaths', []) or []:
        print(re.sub(r'/\*\*?\$', '', p))
except Exception:
    pass
" "$ST_CONFIG")
fi

# Fall back to anti-dumping scanPaths, then to conventional defaults.
if [[ ${#SCAN_PATHS[@]} -eq 0 ]]; then
  AD_CONFIG="$ROOT/.anti-dumping.json"
  if [[ -f "$AD_CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
    while IFS= read -r line; do
      [[ -n "$line" ]] && SCAN_PATHS+=("$line")
    done < <(python3 -c "
import json, sys, re
try:
    cfg = json.load(open(sys.argv[1]))
    for p in cfg.get('scanPaths', []) or []:
        print(re.sub(r'/\*\*?\$', '', p))
except Exception:
    pass
" "$AD_CONFIG")
  fi
fi

if [[ ${#SCAN_PATHS[@]} -eq 0 ]]; then
  SCAN_PATHS=(src packages services apps)
fi

EXISTING=()
for p in "${SCAN_PATHS[@]}"; do
  [[ -d "$ROOT/$p" ]] && EXISTING+=("$ROOT/$p")
done

if [[ ${#EXISTING[@]} -eq 0 ]]; then
  echo "check-stateless-request-path: no scan paths exist under $ROOT; skipping"
  exit 0
fi

# ---- Detection patterns -----------------------------------------------------
#
# Each pattern targets a deterministic, high-signal node-local-state shape. The
# name gate (cache|session|store|registry|state|counter|...) keeps precision
# high: a module-level container with a state-ish name is the smell, not every
# top-level array.
STATE_NAME='([Cc]ache|[Ss]ession|[Ss]tore|[Rr]egistr|[Cc]ounter|[Bb]uffer|[Pp]ool|[Ss]tate|[Mm]emo|[Vv]isits|[Hh]its|[Tt]ally|[Ss]eenS)'

# 1. JS/TS module-level (no indentation ⇒ top-level) mutable container.
JS_STATE='^(export[[:space:]]+)?(const|let|var)[[:space:]]+[A-Za-z_$][A-Za-z0-9_$]*'"$STATE_NAME"'[A-Za-z0-9_$]*[[:space:]]*=[[:space:]]*(new[[:space:]]+(Map|Set|WeakMap|WeakSet|Array)\b|\{\}|\[\])'

# 2. Python module-level (no indentation) mutable container.
PY_STATE='^[A-Za-z_][A-Za-z0-9_]*'"$STATE_NAME"'[A-Za-z0-9_]*[[:space:]]*=[[:space:]]*(\{\}|\[\]|dict\(\)|set\(\)|list\(\)|defaultdict)'

# 3. Java/C#/Kotlin static mutable collection bound to a state-ish name.
JVM_STATE='static[[:space:]][^=;]*\b(Map|List|Set|Dictionary|HashMap|ConcurrentHashMap|ArrayList|HashSet|LinkedList|MutableMap|MutableList)\b[^=;]*'"$STATE_NAME"'[^=;]*='

PATTERN="$JS_STATE|$PY_STATE|$JVM_STATE"

# Built-in allowlist: shapes that are not request-scoped mutable state.
#   - frozen / immutable constants
#   - declarations explicitly inside a function would be indented (JS/PY anchors
#     already exclude those); these guard the remaining static cases.
BUILTIN_ALLOW='Object\.freeze|as const|readonly[[:space:]]|final[[:space:]]+static[[:space:]]+[A-Z0-9_]+[[:space:]]*=|=[[:space:]]*Collections\.(unmodifiable|empty)|ImmutableMap|ImmutableList|ImmutableSet|frozenset'

# ---- Scan -------------------------------------------------------------------

set +e
RAW=$(grep -RIEn \
  --include='*.ts' --include='*.tsx' --include='*.js' --include='*.jsx' \
  --include='*.mjs' --include='*.cjs' --include='*.java' --include='*.kt' \
  --include='*.py' --include='*.go' --include='*.rb' --include='*.cs' \
  --include='*.php' --include='*.rs' \
  --exclude-dir=node_modules --exclude-dir=.git --exclude-dir=dist \
  --exclude-dir=build --exclude-dir=out --exclude-dir=target \
  --exclude-dir=vendor --exclude-dir=__pycache__ --exclude-dir=.venv \
  --exclude-dir=venv --exclude-dir=.next --exclude-dir=coverage \
  "$PATTERN" "${EXISTING[@]}" 2>/dev/null)
GREP_RC=$?
set -e 2>/dev/null || true

if [[ $GREP_RC -gt 1 ]]; then
  echo "check-stateless-request-path: error running grep" >&2
  exit 2
fi

FINDINGS=()
if [[ -n "$RAW" ]]; then
  while IFS= read -r hit; do
    [[ -z "$hit" ]] && continue
    file="${hit%%:*}"
    # Skip test / fixture / mock / example / generated sources.
    case "$file" in
      *test*|*Test*|*spec*|*Spec*|*__tests__*|*__mocks__*|*/e2e/*|*/fixtures/*|*/mocks/*|*/examples/*|*.test.*|*.spec.*|*.stories.*|*.min.*|*generated*|*.pb.*|*.g.*) continue ;;
    esac
    # Per-line reviewed opt-out.
    case "$hit" in
      *"$OPT_OUT_MARKER"*) continue ;;
    esac
    # Built-in allowlist.
    if echo "$hit" | grep -Eq "$BUILTIN_ALLOW"; then
      continue
    fi
    # Project allowlist regexes.
    allowed=0
    for ap in "${ALLOW_PATTERNS[@]+"${ALLOW_PATTERNS[@]}"}"; do
      [[ -z "$ap" ]] && continue
      if echo "$hit" | grep -Eq "$ap"; then allowed=1; break; fi
    done
    [[ $allowed -eq 1 ]] && continue
    FINDINGS+=("$hit")
  done <<< "$RAW"
fi

# ---- Report -----------------------------------------------------------------

if [[ ${#FINDINGS[@]} -eq 0 ]]; then
  echo "check-stateless-request-path: clean (${#EXISTING[@]} scan paths, mode=$MODE)"
  exit 0
fi

{
  echo "check-stateless-request-path: ${#FINDINGS[@]} node-local state singleton(s) found (mode=$MODE)"
  echo
  for f in "${FINDINGS[@]}"; do
    echo "  - $f"
  done
  echo
  cat <<EOF
These look like node-local mutable state on the request path (a module-level or
static cache/session/registry). A second replica will not share it, so behavior
becomes node-dependent and horizontal scaling breaks. Move the state out of the
process:

  - Put request/session state in a shared store (cache service, database).
  - Keep the functional core pure; pass state in at the boundary.
  - For a genuinely safe process-local cache (memoized pure lookup, connection
    pool), append a reviewed opt-out comment containing
    '$OPT_OUT_MARKER' (with the reason) to the line.

Mode is '$MODE'. Set "mode": "enforce" in .statelessness.json once a wave
closes with zero un-opted-out findings (plan decision D3).
EOF
} >&2

if [[ "$MODE" == "enforce" ]]; then
  exit 1
fi
exit 0
