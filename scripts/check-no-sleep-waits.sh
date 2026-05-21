#!/usr/bin/env bash
# check-no-sleep-waits.sh
#
# Fails if source code uses fixed-time sleeps to "wait for things to settle".
# Sleeps are non-deterministic dumping grounds that hide race conditions and
# slow down test suites. Use explicit polling, event hooks, fakes/clocks, or
# framework idle-detection helpers instead.
#
# Patterns flagged:
#   JS/TS (test):   waitForTimeout(   page.waitForTimeout(
#   JS/TS (sleep):  setTimeout(.*, *[0-9]{3,}*) inside src/  (>= 3-digit ms hard wait)
#   Java:           Thread.sleep(
#   Python:         time.sleep(   asyncio.sleep( with numeric literal
#   Generic:        sleep N (shell) outside ./scripts/
#
# This script intentionally avoids flagging shorter `setTimeout` calls because
# they're often legitimate scheduling. The 3+ digit threshold targets the
# "sleep 500ms then assert" anti-pattern.
#
# Compatible with bash 3.2+ (macOS default).
#
# Exit codes:
#   0 — clean
#   1 — sleeps found
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-no-sleep-waits: error: not a directory: $ROOT" >&2
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
  SCAN_PATHS=(src packages services apps tests test __tests__ e2e)
fi

EXISTING=()
for p in "${SCAN_PATHS[@]}"; do
  [[ -d "$ROOT/$p" ]] && EXISTING+=("$ROOT/$p")
done

if [[ ${#EXISTING[@]} -eq 0 ]]; then
  echo "check-no-sleep-waits: no scan paths exist under $ROOT; skipping"
  exit 0
fi

PATTERN='waitForTimeout\(|Thread\.sleep\(|(^|[^a-zA-Z_])time\.sleep\(|asyncio\.sleep\([0-9]'

set +e
MATCHES=$(grep -RIEn \
  --include='*.ts' --include='*.tsx' --include='*.js' --include='*.jsx' \
  --include='*.mjs' --include='*.cjs' --include='*.java' --include='*.kt' \
  --include='*.py' \
  --exclude-dir=node_modules --exclude-dir=.git --exclude-dir=dist \
  --exclude-dir=build --exclude-dir=.next --exclude-dir=target \
  --exclude-dir=__pycache__ --exclude-dir=.venv --exclude-dir=venv \
  "$PATTERN" "${EXISTING[@]}" 2>/dev/null)
GREP_RC=$?
set -e

if [[ $GREP_RC -gt 1 ]]; then
  echo "check-no-sleep-waits: error running grep" >&2
  exit 2
fi

if [[ -n "$MATCHES" ]]; then
  echo "check-no-sleep-waits: hard-wait sleeps found" >&2
  echo "$MATCHES" >&2
  exit 1
fi

echo "check-no-sleep-waits: clean (${#EXISTING[@]} scan paths)"
exit 0
