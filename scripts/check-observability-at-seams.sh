#!/usr/bin/env bash
# check-observability-at-seams.sh
#
# Executable probe for the "Observable" production-readiness anchor
# (ADR.260725 lineage; Bundle B3). Converts the anchor from an
# asserted checklist line into a build-time gate: it fails (or warns) when a
# source file makes a cross-process / boundary call (an outbound HTTP, RPC,
# queue, or DB-client call) but carries NO observability signal — verified via
# AST parent block analysis (`scripts/ast_parse.sh`).
#
# Exit codes:
#   0 — clean, or findings in warn mode
#   1 — findings in enforce mode
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-observability-at-seams: error: not a directory: $ROOT" >&2
  exit 2
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AST_PARSER="$SCRIPT_DIR/ast_parse.sh"
OPT_OUT_MARKER='praxis:allow-unobserved-boundary'

# ---- Config -----------------------------------------------------------------

MODE='warn'
ALLOW_PATTERNS=()
SCAN_PATHS=()

OBS_CONFIG="$ROOT/.observability.json"
if [[ -f "$OBS_CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
  MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$OBS_CONFIG")
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
" "$OBS_CONFIG")
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
" "$OBS_CONFIG")
fi

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
        print(re.sub(r'/\*\*?$', '', p))
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
  echo "check-observability-at-seams: no scan paths exist under $ROOT; skipping"
  exit 0
fi

# ---- Detection --------------------------------------------------------------

BOUNDARY='fetch\(|axios|got\(|superagent|requests\.(get|post|put|delete|patch|head|request)|httpx\.|aiohttp|urlopen|RestTemplate|WebClient|OkHttp|HttpClient|HttpURLConnection|http\.(Get|Post|NewRequest)|client\.Do\(|Net::HTTP|Faraday|HTTParty|\.GetAsync\(|\.PostAsync\(|GraphQLClient|grpc\.|gRPC|kafka|amqp|producer\.send|\.publish\('
OBS='log(ger|ging)?[._]|console\.(log|info|warn|error|debug)|logrus|zap\.|slog\.|winston|pino|structlog|LoggerFactory|Logger\.|@Slf4j|metric|counter|gauge|histogram|\btrace\b|\bspan\b|Tracer|opentelemetry|otel|correlation[_-]?id|request[_-]?id|trace[_-]?id|MDC\.'

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
  echo "check-observability-at-seams: error running grep" >&2
  exit 2
fi

FINDINGS=()
if [[ -n "$FILES" ]]; then
  while IFS= read -r file; do
    [[ -z "$file" ]] && continue
    case "$file" in
      *test*|*Test*|*spec*|*Spec*|*__tests__*|*__mocks__*|*/e2e/*|*/fixtures/*|*/mocks/*|*/examples/*|*.test.*|*.spec.*|*.stories.*|*.min.*|*generated*|*.pb.*|*.g.*) continue ;;
    esac
    if grep -Iq "$OPT_OUT_MARKER" "$file" 2>/dev/null; then continue; fi
    if grep -IEq "$OBS" "$file" 2>/dev/null; then continue; fi
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
  echo "check-observability-at-seams: clean (mode=$MODE)"
  exit 0
fi

{
  echo "check-observability-at-seams: ${#FINDINGS[@]} unobserved boundary call(s) (mode=$MODE)"
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
