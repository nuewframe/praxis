#!/usr/bin/env bash
# check-sprint-id-collision.sh
#
# The coordination-artifact gate for emergent parallelism (plan:
# executable-seams-first.md, Bundle D / D2). When sprints may be created
# concurrently — siblings worked in parallel sessions or dispatched by an
# orchestration runtime — a bare "highest NNN + 1" increment collides: two
# slices both grab the same next id and race the coordination layer before
# they ever race the code. This probe makes that collision a build-time
# failure instead of a silent double-id.
#
# Unlike the anchor probes, this is NOT a heuristic — it is an exact, zero-
# judgment invariant: no two active (non-ledger) sprint files may share the
# same id token. The id token is the `<id>` in `sprint-<id>-<description>.md`,
# where `<id>` is `NNN` or the ADR-style collision-safe `NNN.<seq>` form
# (e.g. `007` or `007.01`). A sprint and its own `*.ledger.md` are not a
# collision (the ledger is excluded).
#
# Mode (decision D3 — warn-first, mechanical promotion to fail-closed):
#   Read from .sprint-coordination.json `mode`:
#     "warn"    (default) — report collisions, exit 0.
#     "enforce"           — report collisions, exit 1. Promote once the team
#                           routinely creates sprints in parallel.
#
# Config (.sprint-coordination.json at repo root, all keys optional):
#   {
#     "mode":      "warn",                   // "warn" | "enforce"
#     "sprintDir": "docs/product/sprints"    // falls back to common defaults
#   }
#
# Compatible with bash 3.2+ (macOS default).
#
# Exit codes:
#   0 — no collisions, or collisions in warn mode
#   1 — collisions in enforce mode
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-sprint-id-collision: error: not a directory: $ROOT" >&2
  exit 2
fi

# ---- Config -----------------------------------------------------------------

MODE='warn'
SPRINT_DIR=''

SC_CONFIG="$ROOT/.sprint-coordination.json"
if [[ -f "$SC_CONFIG" ]] && command -v python3 >/dev/null 2>&1; then
  MODE=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    m = str(cfg.get('mode', 'warn')).strip().lower()
    print(m if m in ('warn', 'enforce') else 'warn')
except Exception:
    print('warn')
" "$SC_CONFIG")
  SPRINT_DIR=$(python3 -c "
import json, sys
try:
    cfg = json.load(open(sys.argv[1]))
    print(str(cfg.get('sprintDir', '') or '').strip())
except Exception:
    print('')
" "$SC_CONFIG")
fi

# Fall back to conventional sprint-directory locations.
if [[ -z "$SPRINT_DIR" ]]; then
  for cand in docs/product/sprints docs/sprints sprints; do
    if [[ -d "$ROOT/$cand" ]]; then
      SPRINT_DIR="$cand"
      break
    fi
  done
fi

if [[ -z "$SPRINT_DIR" || ! -d "$ROOT/$SPRINT_DIR" ]]; then
  echo "check-sprint-id-collision: no sprint directory under $ROOT; skipping"
  exit 0
fi

# ---- Collect id tokens ------------------------------------------------------
#
# Each active sprint is `sprint-<id>-<description>.md`. Exclude `*.ledger.md`
# (a ledger shares its sprint's id by design and is not a collision).

set +e
FILES=$(find "$ROOT/$SPRINT_DIR" -type f -name 'sprint-*.md' ! -name '*.ledger.md' 2>/dev/null)
FIND_RC=$?
set -e 2>/dev/null || true

if [[ $FIND_RC -ne 0 ]]; then
  echo "check-sprint-id-collision: error scanning $SPRINT_DIR" >&2
  exit 2
fi

# Build "id<TAB>relative-file" records for conforming sprint filenames.
RECORDS=''
while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  base="${f##*/}"
  stem="${base%.md}"
  rest="${stem#sprint-}"
  [[ "$rest" == "$stem" ]] && continue          # did not start with sprint-
  id="${rest%%-*}"
  [[ "$id" == "$rest" ]] && continue            # no description segment; skip
  # id must be NNN or NNN.seq (ADR-style collision-safe form).
  if ! printf '%s' "$id" | grep -Eq '^[0-9]+(\.[0-9]+)?$'; then
    continue
  fi
  rel="${f#"$ROOT"/}"
  RECORDS="${RECORDS}${id}	${rel}"$'\n'
done <<< "$FILES"

if [[ -z "$RECORDS" ]]; then
  echo "check-sprint-id-collision: no conforming sprint files in $SPRINT_DIR; skipping"
  exit 0
fi

# Duplicate id tokens.
DUP_IDS=$(printf '%s' "$RECORDS" | awk -F'\t' 'NF{print $1}' | sort | uniq -d)

# ---- Report -----------------------------------------------------------------

if [[ -z "$DUP_IDS" ]]; then
  COUNT=$(printf '%s' "$RECORDS" | grep -c '	')
  echo "check-sprint-id-collision: clean ($COUNT sprint id(s) in $SPRINT_DIR, mode=$MODE)"
  exit 0
fi

{
  N=$(printf '%s\n' "$DUP_IDS" | grep -c .)
  echo "check-sprint-id-collision: $N colliding sprint id(s) found (mode=$MODE)"
  echo
  while IFS= read -r id; do
    [[ -z "$id" ]] && continue
    echo "  id '$id' is claimed by:"
    printf '%s' "$RECORDS" | awk -F'\t' -v want="$id" '$1==want{print "    - " $2}'
  done <<< "$DUP_IDS"
  echo
  cat <<'EOF'
Two or more active sprint files share the same id token. Under parallel sprint
creation a bare "highest NNN + 1" increment double-allocates. Resolve by giving
the newer sprint the shortest unique ADR-style tiebreaker suffix '.<seq>'
(e.g. sprint-007.01-...) — existing ids are immutable; rename the new one only
(plan: create-sprint Step 1, Bundle D / D2).

Mode is 'warn'. Set "mode": "enforce" in .sprint-coordination.json once the
team routinely creates sprints in parallel (plan decision D3).
EOF
} >&2

if [[ "$MODE" == "enforce" ]]; then
  exit 1
fi
exit 0
