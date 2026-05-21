#!/usr/bin/env bash
# check-no-skipped-tests.sh
#
# Fails if any source file under the scan paths contains a skipped test marker.
# Skipped tests are a common dumping ground for flakiness and should never be
# committed to main. If a test must be quarantined, mark it `todo` with a
# linked issue and remove it from the suite via configuration, not via `.skip`.
#
# Patterns flagged:
#   JS/TS:   .skip(   xit(   test.skip   it.skip   describe.skip
#   Java:    @Disabled   @Ignore
#   Python:  @pytest.mark.skip   unittest.skip   @skip
#
# Compatible with bash 3.2+ (macOS default).
#
# Exit codes:
#   0 — clean
#   1 — skipped tests found
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-no-skipped-tests: error: not a directory: $ROOT" >&2
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
        # Strip glob suffixes like '/**' for directory enumeration.
        base = re.sub(r'/\*\*?$', '', p)
        print(base)
except Exception:
    pass
" "$CONFIG")
fi

if [[ ${#SCAN_PATHS[@]} -eq 0 ]]; then
  SCAN_PATHS=(src packages services apps tests test __tests__)
fi

EXISTING=()
for p in "${SCAN_PATHS[@]}"; do
  [[ -d "$ROOT/$p" ]] && EXISTING+=("$ROOT/$p")
done

if [[ ${#EXISTING[@]} -eq 0 ]]; then
  echo "check-no-skipped-tests: no scan paths exist under $ROOT; skipping"
  exit 0
fi

PATTERN='\.skip\(|xit\(|test\.skip|it\.skip|describe\.skip|@Disabled\b|@Ignore\b|@pytest\.mark\.skip|unittest\.skip|@skip\b'

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

# grep exit codes: 0=match, 1=no match, 2=error.
if [[ $GREP_RC -gt 1 ]]; then
  echo "check-no-skipped-tests: error running grep" >&2
  exit 2
fi

if [[ -n "$MATCHES" ]]; then
  echo "check-no-skipped-tests: skipped tests found" >&2
  echo "$MATCHES" >&2
  exit 1
fi

echo "check-no-skipped-tests: clean (${#EXISTING[@]} scan paths)"
exit 0
