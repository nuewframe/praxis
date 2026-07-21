#!/usr/bin/env bash
# check-escape-hatch-usage.sh
#
# Scans a diff for Praxis's four escape-hatch opt-out markers and reports
# each occurrence by file:line. This is informational, NOT a gate — escape
# hatches are sometimes legitimate; the point is that using one is never
# silent to a PR reviewer.
#
# Unlike the anchor probes (check-config-externalized.sh,
# check-observability-at-seams.sh, check-stateless-request-path.sh,
# check-resilient-boundary.sh), this is not a heuristic scanning source for
# risky shapes — it is exact substring matching against four known literal
# marker strings in diff text. There is no judgment call in the matching
# itself; the judgment call belongs to the human reviewer who sees the hit.
#
# The four markers (repo-wide grep confirmed; do not add a fifth without
# updating this list and the header comments of the check-*.sh probes that
# define them):
#   praxis:allow-config-literal
#   praxis:allow-local-state
#   praxis:allow-unguarded-boundary
#   praxis:allow-unobserved-boundary
#
# Usage:
#   scripts/check-escape-hatch-usage.sh [--base <ref>]
#
#   --base <ref>   Diff base to compare HEAD against. If omitted, auto-
#                  detects in order: origin/main, then main, then falls back
#                  to HEAD~1 with a stderr warning (HEAD~1 is not a full PR
#                  diff — it only sees the last commit).
#
# Exit codes:
#   0 — always. This probe never fails the build, regardless of how many
#       markers it finds, whether it's run outside a git repo, or whether
#       the diff is empty. Unlike check-anti-dumping.sh and
#       check-sprint-id-collision.sh (which do fail / increment a FAILED
#       counter when wired into verify.sh), this script's job is to make
#       escape-hatch usage visible, not to block on it.
#
# Dependencies: bash 3.2+ (macOS default), git.

set -euo pipefail

MARKERS=(
  "praxis:allow-config-literal"
  "praxis:allow-local-state"
  "praxis:allow-unguarded-boundary"
  "praxis:allow-unobserved-boundary"
)

# ---- Argument parsing --------------------------------------------------------

BASE=''
while [[ $# -gt 0 ]]; do
  case "$1" in
    --base)
      BASE="${2:-}"
      shift
      [[ $# -gt 0 ]] && shift
      ;;
    *)
      echo "check-escape-hatch-usage: warning: ignoring unrecognized argument '$1'" >&2
      shift
      ;;
  esac
done

# ---- Base auto-detection -----------------------------------------------------

if [[ -z "$BASE" ]]; then
  if git rev-parse --verify origin/main >/dev/null 2>&1; then
    BASE='origin/main'
  elif git rev-parse --verify main >/dev/null 2>&1; then
    BASE='main'
  else
    BASE='HEAD~1'
    echo "check-escape-hatch-usage: warning: no origin/main or main found; falling back to HEAD~1 (not a full PR diff)" >&2
  fi
fi

MERGE_BASE=$(git merge-base "$BASE" HEAD 2>/dev/null || echo "$BASE")

# ---- Diff collection ----------------------------------------------------------
#
# Tolerate any failure here (not a git repo, no commits yet, bad ref) and
# just proceed with an empty diff rather than crashing — this probe reports
# zero hits in that case instead of failing.

DIFF_OUTPUT=$(git diff --unified=0 "$MERGE_BASE" HEAD 2>/dev/null || true)

# ---- Parse the diff, tracking file + line number precisely ------------------
#
# --unified=0 means every changed line is a +/- line with no unchanged
# context, so we never need to handle context lines incrementing the
# counter.

CURRENT_FILE=''
LINE_NUM=0
HITS=''

while IFS= read -r line; do
  if [[ "$line" =~ ^\+\+\+\ b/(.*)$ ]]; then
    CURRENT_FILE="${BASH_REMATCH[1]}"
    LINE_NUM=0
    continue
  fi
  if [[ "$line" =~ ^@@\ -[0-9]+(,[0-9]+)?\ \+([0-9]+)(,[0-9]+)?\ @@ ]]; then
    LINE_NUM="${BASH_REMATCH[2]}"
    continue
  fi
  if [[ "$line" == +* ]]; then
    for marker in "${MARKERS[@]}"; do
      if [[ "$line" == *"$marker"* ]]; then
        HITS="${HITS}${CURRENT_FILE}	${LINE_NUM}	${marker}"$'\n'
      fi
    done
    LINE_NUM=$((LINE_NUM + 1))
    continue
  fi
  # Removed ('-') lines and other diff metadata (diff --git, index, ---)
  # do not belong to the new file and never increment the counter.
done <<< "$DIFF_OUTPUT"

# ---- Report -------------------------------------------------------------------

if [[ -z "$HITS" ]]; then
  echo "check-escape-hatch-usage: clean (0 escape hatch markers in diff against $BASE)"
  exit 0
fi

N=$(printf '%s' "$HITS" | grep -c $'\t' || true)
M=$(printf '%s' "$HITS" | awk -F'\t' 'NF{print $1}' | sort -u | grep -c . || true)

while IFS=$'\t' read -r f l m; do
  [[ -z "$f" ]] && continue
  echo "${f}:${l}: ${m}"
done <<< "$HITS"

echo "check-escape-hatch-usage: $N escape hatch marker(s) across $M file(s) in diff against $BASE (informational, not blocking)"

exit 0
