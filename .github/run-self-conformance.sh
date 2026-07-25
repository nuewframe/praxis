#!/usr/bin/env bash
# run-self-conformance.sh
#
# Runs the shipped enforcement gates against Praxis itself, per the declaration
# in .self-conformance.json. This is the executable half of "Praxis runs the
# gates it ships" — before it existed, CI ran exactly one of the eleven probes
# and only `bash -n`-parsed the other ten, so "we ship this check" and "this
# check passes" were two different claims.
#
# For each declared gate:
#   runs: true   -> execute it; a non-zero exit fails this script
#   runs: false  -> print an explicit `n/a` line carrying the declared reason
#
# The loop deliberately does NOT short-circuit on the first failure: a
# half-reported build hides every gate behind the first red one. It accumulates
# all verdicts, prints them, then exits non-zero if any declared-run gate failed.
#
# This lives under .github/ rather than scripts/ on purpose: it is Praxis's own
# CI tooling, not generic host-facing enforcement that ships to adopters.
#
# Usage: bash .github/run-self-conformance.sh [repo-root]
# Exit:  0 = every declared-run gate passed; 1 = at least one failed or the
#        manifest is unusable.

set -uo pipefail

ROOT="${1:-.}"
cd "$ROOT" || { echo "run-self-conformance: cannot cd to $ROOT" >&2; exit 1; }

MANIFEST=".self-conformance.json"

if [[ ! -f "$MANIFEST" ]]; then
  echo "run-self-conformance: $MANIFEST not found" >&2
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "run-self-conformance: jq is required but not installed" >&2
  exit 1
fi

if ! jq empty "$MANIFEST" >/dev/null 2>&1; then
  echo "run-self-conformance: $MANIFEST is not valid JSON" >&2
  exit 1
fi

# Unit-separator delimited, NOT tab: tab is an IFS *whitespace* character, so
# `IFS=$'\t' read` collapses consecutive tabs and an empty `reason` would shift
# `note` into its place. \x1f is not IFS whitespace, so empty fields survive.
GATES=$(jq -r '.gates[] | [.script, (.runs|tostring), (.reason // ""), (.note // "")] | join("")' "$MANIFEST")

if [[ -z "$GATES" ]]; then
  echo "run-self-conformance: $MANIFEST declares no gates" >&2
  exit 1
fi

RAN=0
SKIPPED=0
FAILED=0
FAILED_NAMES=""

echo "run-self-conformance: executing shipped gates against Praxis"
echo ""

while IFS=$'\x1f' read -r script runs reason note; do
  [[ -n "$script" ]] || continue

  if [[ ! -f "scripts/$script" ]]; then
    echo "FAIL scripts/$script -> declared in $MANIFEST but not on disk"
    FAILED=$((FAILED + 1))
    FAILED_NAMES="${FAILED_NAMES}  scripts/$script (missing)"$'\n'
    continue
  fi

  if [[ "$runs" == "true" ]]; then
    output=$(bash "scripts/$script" 2>&1)
    rc=$?
    RAN=$((RAN + 1))
    if [[ $rc -eq 0 ]]; then
      echo "RUN  $script -> pass"
    else
      echo "RUN  $script -> FAIL (exit $rc)"
      FAILED=$((FAILED + 1))
      FAILED_NAMES="${FAILED_NAMES}  $script (exit $rc)"$'\n'
    fi
    # Indent the gate's own output so a reader can see what it actually said.
    printf '%s\n' "$output" | sed 's/^/       /'
    [[ -n "$note" ]] && echo "       note: $note"
  else
    SKIPPED=$((SKIPPED + 1))
    echo "n/a  $script -> $reason"
  fi
  echo ""
done <<EOF
$GATES
EOF

echo "run-self-conformance: $RAN gate(s) executed, $SKIPPED declared n/a, $FAILED failure(s)"

if [[ $FAILED -ne 0 ]]; then
  echo "" >&2
  echo "run-self-conformance: failing gates:" >&2
  printf '%s' "$FAILED_NAMES" >&2
  exit 1
fi

exit 0
