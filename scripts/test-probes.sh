#!/usr/bin/env bash
# test-probes.sh — self-test for the language-coverage of the guardrail probes.
#
# Runs check-no-skipped-tests.sh and check-no-sleep-waits.sh against the
# multi-language fixtures under __fixtures__/ and asserts the expected verdict:
#   - the "dirty" fixtures (a skip + a sleep in each of Go/Rust/Ruby/C#/PHP/TS)
#     must be flagged (exit 1)
#   - the "clean" fixtures must pass (exit 0)
#
# This is the regression guard for the probe language-coverage gap. Wire it into
# CI. bash 3.2+ compatible.

set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
FIX="$HERE/__fixtures__"
fail=0

assert_rc() {
  local desc="$1" script="$2" root="$3" want="$4"
  bash "$HERE/$script" "$root" >/dev/null 2>&1
  local rc=$?
  if [[ "$rc" -ne "$want" ]]; then
    echo "  FAIL: $desc (rc=$rc, expected $want)"
    fail=1
  else
    echo "  ok:   $desc"
  fi
}

echo "test-probes: language coverage"
assert_rc "no-skipped-tests flags dirty fixtures" check-no-skipped-tests.sh "$FIX/dirty" 1
assert_rc "no-skipped-tests passes clean fixtures" check-no-skipped-tests.sh "$FIX/clean" 0
assert_rc "no-sleep-waits flags dirty fixtures"   check-no-sleep-waits.sh   "$FIX/dirty" 1
assert_rc "no-sleep-waits passes clean fixtures"   check-no-sleep-waits.sh   "$FIX/clean" 0

if [[ "$fail" -ne 0 ]]; then
  echo "test-probes: FAILURES"
  exit 1
fi
echo "test-probes: all passed"
exit 0
