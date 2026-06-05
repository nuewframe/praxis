#!/usr/bin/env bash
# check-resilient-boundary.sh
#
# Executable probe for the "Resilient" production-readiness anchor
# (plan: executable-seams-first.md, Bundle B3). Converts the anchor from an
# asserted checklist line into a build-time gate: it fails (or warns) when a
# source file makes a cross-process / boundary call (an outbound HTTP, RPC,
# queue, or DB-client call) but declares NO resilience policy anywhere — no
# timeout/deadline, no retry/backoff, no circuit breaker, no fallback. An
# unguarded boundary turns a slow dependency into a cascading outage.
#
# This is a HEURISTIC, not a proof, and it is FILE-scoped and precision-biased:
# a file is flagged only when it makes a boundary call AND matches none of a
# deliberately broad set of resilience signals. Broad signals mean few false
# positives — if the file sets a timeout, retries, or has a fallback anywhere,
# it passes.
#
# Opt-out (per-file, reviewed):
#   Put a comment containing  praxis:allow-unguarded-boundary  anywhere in the
#   file (with the reason). Silence fails; a reasoned exemption is a visible
#   artifact.
#
# Mode (decision D3 — warn-first, mechanical promotion to fail-closed):
#   Read from .resilience.json `mode`:
#     "warn"    (default) — report findings, exit 0. Use until a wave closes clean.
#     "enforce"           — report findings, exit 1. Promote here once clean.
#
# Config (.resilience.json at repo root, all keys optional):
#   {
#     "mode":      "warn",          // "warn" | "enforce"
#     "scanPaths": ["src/**"],      // falls back to .anti-dumping.json, then defaults
#     "allow":     ["regex", ...]   // extra filename allowlist regexes
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
  echo "check-resilient-boundary: error: not a directory: $ROOT" >&2
  exit 2
fi

OPT_OUT_MARKER='praxis:allow-unguarded-boundary'

# ---- Config -----------------------------------------------------------------

MODE='warn'
ALLOW_PATTERNS=()
SCAN_PATHS=()

RES_CONFIG="$ROOT/.resilience.json"
if [[ -f "$RES_CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
  MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$RES_CONFIG")
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
" "$RES_CONFIG")
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
" "$RES_CONFIG")
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
  echo "check-resilient-boundary: no scan paths exist under $ROOT; skipping"
  exit 0
fi

# ---- Detection --------------------------------------------------------------
#
# BOUNDARY: high-signal outbound cross-process call shapes across common stacks.
BOUNDARY='fetch\(|axios|got\(|superagent|requests\.(get|post|put|delete|patch|head|request)|httpx\.|aiohttp|urlopen|RestTemplate|WebClient|OkHttp|HttpClient|HttpURLConnection|http\.(Get|Post|NewRequest)|client\.Do\(|Net::HTTP|Faraday|HTTParty|\.GetAsync\(|\.PostAsync\(|GraphQLClient|grpc\.|gRPC|kafka|amqp|producer\.send|\.publish\('
#
# RES: deliberately broad resilience signals. Any one makes the file pass.
RES='timeout|deadline|AbortController|AbortSignal|\bsignal\b|\bretry\b|\bretries\b|backoff|circuit[_-]?breaker|CircuitBreaker|fallback|Polly|resilience4j|Resilience|@Retry|@CircuitBreaker|@TimeLimiter|withTimeout|WithTimeout|bulkhead|rate[_-]?limit|\bttl\b|maxAttempts|max_retries'

# ---- Scan -------------------------------------------------------------------

set +e
FILES=$(grep -RIlE \
  --include='*.ts' --include='*.tsx' --include='*.js' --include='*.jsx' \
  --include='*.mjs' --include='*.cjs' --include='*.java' --include='*.kt' \
  --include='*.py' --include='*.go' --include='*.rb' --include='*.cs' \
  --include='*.php' --include='*.rs' \
  --exclude-dir=node_modules --exclude-dir=.git --exclude-dir=dist \
  --exclude-dir=build --exclude-dir=out --exclude-dir=target \
  --exclude-dir=vendor --exclude-dir=__pycache__ --exclude-dir=.venv \
  --exclude-dir=venv --exclude-dir=.next --exclude-dir=coverage \
  "$BOUNDARY" "${EXISTING[@]}" 2>/dev/null)
GREP_RC=$?
set -e 2>/dev/null || true

if [[ $GREP_RC -gt 1 ]]; then
  echo "check-resilient-boundary: error running grep" >&2
  exit 2
fi

FINDINGS=()
if [[ -n "$FILES" ]]; then
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    # Skip test / fixture / mock / example / generated sources.
    case "$file" in
      *test*|*Test*|*spec*|*Spec*|*__tests__*|*__mocks__*|*/e2e/*|*/fixtures/*|*/mocks/*|*/examples/*|*.test.*|*.spec.*|*.stories.*|*.min.*|*generated*|*.pb.*|*.g.*) continue ;;
    esac
    # Per-file reviewed opt-out.
    if grep -Iq "$OPT_OUT_MARKER" "$file" 2>/dev/null; then continue; fi
    # File declares a resilience policy somewhere → passes.
    if grep -IEq "$RES" "$file" 2>/dev/null; then continue; fi
    # Project allowlist regexes (matched against the filename).
    allowed=0
    for ap in "${ALLOW_PATTERNS[@]+"${ALLOW_PATTERNS[@]}"}"; do
      [[ -z "$ap" ]] && continue
      if echo "$file" | grep -Eq "$ap"; then allowed=1; break; fi
    done
    [[ $allowed -eq 1 ]] && continue
    ev=$(grep -InE "$BOUNDARY" "$file" 2>/dev/null | head -1)
    FINDINGS+=("$file (e.g. $ev)")
  done <<< "$FILES"
fi

# ---- Report -----------------------------------------------------------------

if [[ ${#FINDINGS[@]} -eq 0 ]]; then
  echo "check-resilient-boundary: clean (${#EXISTING[@]} scan paths, mode=$MODE)"
  exit 0
fi

{
  echo "check-resilient-boundary: ${#FINDINGS[@]} unguarded boundary file(s) (mode=$MODE)"
  echo
  for f in "${FINDINGS[@]}"; do
    echo "  - $f"
  done
  echo
  cat <<EOF
These files cross a process boundary (HTTP/RPC/queue/DB client) but declare no
resilience policy. A slow or failing dependency will propagate unbounded. Add
at least one of:

  - A timeout / deadline on the call.
  - A retry with backoff (bounded attempts) for idempotent operations.
  - A circuit breaker and/or fallback for the failure path.

For a genuine exception, put a reviewed opt-out comment containing
'$OPT_OUT_MARKER' (with the reason) in the file.

Mode is '$MODE'. Set "mode": "enforce" in .resilience.json once a wave closes
with zero un-opted-out findings (plan decision D3).
EOF
} >&2

if [[ "$MODE" == "enforce" ]]; then
  exit 1
fi
exit 0
