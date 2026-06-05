#!/usr/bin/env bash
# check-config-externalized.sh
#
# The first executable seam-conformance probe (plan: executable-seams-first.md,
# decision D1). Converts the "Configurable" production-readiness anchor from an
# asserted checklist line into a build-time gate: it fails (or warns) when a
# source file hardcodes a value that belongs in configuration — a remote URL,
# a host:port endpoint, or a credential/secret literal — instead of reading it
# from config or the environment.
#
# This is a HEURISTIC, not a proof. It is deliberately conservative and
# precision-biased: it flags the high-signal, deterministic cases (well-known
# secret token shapes, credentialed/remote URLs, secret-named assignments) and
# offers an explicit, reviewed opt-out so legitimate exceptions are a visible
# artifact rather than a silent pass.
#
# Opt-out (per-line, reviewed):
#   Append a comment containing  praxis:allow-config-literal  to the offending
#   line. The reason should be stated in the same comment. Silence fails; an
#   un-reasoned literal does not.
#
# Mode (decision D3 — warn-first, mechanical promotion to fail-closed):
#   Read from .config-externalization.json `mode`:
#     "warn"    (default) — report findings, exit 0. Use until a wave closes clean.
#     "enforce"           — report findings, exit 1. Promote here once clean.
#
# Config (.config-externalization.json at repo root, all keys optional):
#   {
#     "mode":      "warn",                  // "warn" | "enforce"
#     "scanPaths": ["src/**"],              // falls back to .anti-dumping.json, then defaults
#     "allow":     ["regex", ...]           // extra allowlist regexes (matched against the line)
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
  echo "check-config-externalized: error: not a directory: $ROOT" >&2
  exit 2
fi

OPT_OUT_MARKER='praxis:allow-config-literal'

# ---- Config -----------------------------------------------------------------

MODE='warn'
ALLOW_PATTERNS=()
SCAN_PATHS=()

CE_CONFIG="$ROOT/.config-externalization.json"
if [[ -f "$CE_CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
  MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$CE_CONFIG")
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
" "$CE_CONFIG")
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
" "$CE_CONFIG")
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
  echo "check-config-externalized: no scan paths exist under $ROOT; skipping"
  exit 0
fi

# ---- Detection patterns -----------------------------------------------------
#
# Each pattern targets a deterministic, high-signal hardcoded-config smell.
# Built-in allowlist (below) suppresses the common legitimate literals so the
# default signal-to-noise is high without per-project tuning.

# 1. Well-known secret token shapes (very high precision).
SECRET_TOKENS='AKIA[0-9A-Z]{16}|ASIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{35}|ghp_[0-9A-Za-z]{36}|gho_[0-9A-Za-z]{36}|github_pat_[0-9A-Za-z_]{82}|xox[baprs]-[0-9A-Za-z-]{10,}|sk-[A-Za-z0-9]{32,}|-----BEGIN [A-Z ]*PRIVATE KEY-----'

# 2. Remote URLs with an embedded credential, OR a remote http(s) host literal.
#    (localhost / loopback / example / schema namespaces are allowlisted below.)
CRED_URL='[a-zA-Z][a-zA-Z0-9+.-]*://[^/@:[:space:]"'"'"']+:[^/@[:space:]"'"'"']+@'
REMOTE_URL='https?://[A-Za-z0-9.-]+\.[A-Za-z]{2,}'

# 3. Secret-named identifier assigned a non-trivial string literal.
#    e.g.  password = "hunter2"   apiKey: 'abcd...'   secret_token => "..."
SECRET_ASSIGN='(pass(wd|word)?|secret|api[_-]?key|access[_-]?key|client[_-]?secret|auth[_-]?token|private[_-]?key)["'"'"']?[[:space:]]*[:=>]+[[:space:]]*["'"'"'][^"'"'"']{6,}'

PATTERN="$SECRET_TOKENS|$CRED_URL|$REMOTE_URL|$SECRET_ASSIGN"

# Built-in allowlist: legitimate literals that are not externalizable config.
BUILTIN_ALLOW='localhost|127\.0\.0\.1|0\.0\.0\.0|::1|example\.(com|org|net)|example\.test|\.example$|schemas?\.|xmlns|www\.w3\.org|/dev/null|placeholder|YOUR_|<[A-Za-z_]+>|process\.env|os\.environ|getenv|System\.getenv|config\.|settings\.|ConfigService|\$\{'

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
  echo "check-config-externalized: error running grep" >&2
  exit 2
fi

# Filter: drop opt-out lines, test/fixture paths, and allowlisted literals.
FINDINGS=()
if [[ -n "$RAW" ]]; then
  while IFS= read -r hit; do
    [[ -z "$hit" ]] && continue
    file="${hit%%:*}"
    # Skip test / fixture / mock / example / generated sources — these legitimately
    # carry literal endpoints and dummy secrets.
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
  echo "check-config-externalized: clean (${#EXISTING[@]} scan paths, mode=$MODE)"
  exit 0
fi

{
  echo "check-config-externalized: ${#FINDINGS[@]} hardcoded-config literal(s) found (mode=$MODE)"
  echo
  for f in "${FINDINGS[@]}"; do
    echo "  - $f"
  done
  echo
  cat <<EOF
These values look like configuration (remote URL, endpoint, or secret) hardcoded
into source. Externalize them so the code is configurable per environment:

  - Read from environment or a config service (process.env / os.environ / getenv).
  - Inject the value at the boundary; keep the functional core pure.
  - For genuine non-config literals, append a reviewed opt-out comment containing
    '$OPT_OUT_MARKER' (with the reason) to the line.

Mode is '$MODE'. Set "mode": "enforce" in .config-externalization.json once a
wave closes with zero un-opted-out findings (plan decision D3).
EOF
} >&2

if [[ "$MODE" == "enforce" ]]; then
  exit 1
fi
exit 0
