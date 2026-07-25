#!/usr/bin/env bash
# check-sprint-disjointness.sh
#
# Two sprints may run concurrently only if they are disjoint. Until this probe
# existed the rule was prose discipline in using-praxis and the capability
# guardrails; this makes a violation a build signal instead of something noticed
# afterwards.
#
# Reads the `Sprint Footprint` fenced json block from each active sprint file
# (schema: scripts/data/sprint-footprint.schema.json) and compares every pair.
#
# The four implementation axes come from the executable-seams wave's
# four-condition rule:
#   1. capabilities + fileGlobs   2. persistentResources
#   3. configKeysWritten          4. dependsOnContracts (frozen <name>@vN only)
#
# Two axes were added after the first real concurrent dispatch, because the four
# alone reported two sprints as fully disjoint that demonstrably collided:
#   - closeArtifacts: files written at CLOSE, not during implementation. The
#     four conditions model implementation footprint only. Overlap here does not
#     forbid concurrency — it requires close to be reconciled centrally rather
#     than raced. Reported as ADVISORY, never as a violation.
#   - baseRevision: two worktrees cut from different bases build against
#     different trees while every contract hash still matches.
#
# Posture: warn-first. Reports and exits 0 unless .sprint-coordination.json sets
# "mode": "enforce". A repo that never dispatches concurrently never needs the
# footprint block at all — absence is not a failure.
#
# Usage:
#   check-sprint-disjointness.sh [sprint-dir]
#
# Exit codes:
#   0 — disjoint, or warn mode, or fewer than two active sprints
#   1 — overlap on a blocking axis and mode=enforce

set -uo pipefail

ROOT="${1:-.}"
SPRINT_DIR="$ROOT/docs/product/sprints"
CONFIG="$ROOT/.sprint-coordination.json"

MODE="warn"
if [[ -f "$CONFIG" ]] && command -v jq >/dev/null 2>&1; then
  MODE=$(jq -r '.mode // "warn"' "$CONFIG" 2>/dev/null || echo warn)
fi

if [[ ! -d "$SPRINT_DIR" ]]; then
  echo "check-sprint-disjointness: no $SPRINT_DIR; skipping"
  exit 0
fi

python3 - "$SPRINT_DIR" "$MODE" <<'PY'
import json, os, re, sys

sprint_dir, mode = sys.argv[1], sys.argv[2]

BLOCK_RE = re.compile(r'##\s*Sprint Footprint[^\n]*\n(.*?)```json\n(.*?)\n```', re.S)

footprints = []
for fn in sorted(os.listdir(sprint_dir)):
    if not (fn.startswith('SPRINT.') and fn.endswith('.md')) or fn.endswith('.ledger.md'):
        continue
    text = open(os.path.join(sprint_dir, fn), errors='replace').read()
    m = BLOCK_RE.search(text)
    if not m:
        # A sprint without a footprint is not an error: the artifact is only
        # required once concurrent dispatch actually happens.
        continue
    try:
        footprints.append((fn, json.loads(m.group(2))))
    except Exception as e:
        print('check-sprint-disjointness: %s has an unparseable footprint: %s' % (fn, e),
              file=sys.stderr)
        sys.exit(1)

if len(footprints) < 2:
    print('check-sprint-disjointness: %d sprint(s) declare a footprint; nothing to compare'
          % len(footprints))
    sys.exit(0)

BLOCKING = [
    ('capabilities',        'capability'),
    ('fileGlobs',           'file glob'),
    ('persistentResources', 'persistent resource'),
    ('configKeysWritten',   'config key'),
]

violations, advisories = [], []

for i in range(len(footprints)):
    for j in range(i + 1, len(footprints)):
        an, a = footprints[i]
        bn, b = footprints[j]
        for key, label in BLOCKING:
            shared = sorted(set(a.get(key, [])) & set(b.get(key, [])))
            for s in shared:
                violations.append('%s <-> %s: shared %s `%s`' % (an, bn, label, s))

        # Condition 4: depending on the other's in-flight internals rather than a
        # frozen contract. A dependency naming a capability the sibling is
        # actively writing is the unsafe shape.
        for dep in a.get('dependsOnContracts', []):
            if dep.split('@')[0] in set(b.get('capabilities', [])):
                violations.append('%s depends on `%s` while %s is writing that capability '
                                  '-- that is in-flight internals, not a frozen contract'
                                  % (an, dep, bn))
        for dep in b.get('dependsOnContracts', []):
            if dep.split('@')[0] in set(a.get('capabilities', [])):
                violations.append('%s depends on `%s` while %s is writing that capability '
                                  '-- that is in-flight internals, not a frozen contract'
                                  % (bn, dep, an))

        shared_close = sorted(set(a.get('closeArtifacts', [])) & set(b.get('closeArtifacts', [])))
        for s in shared_close:
            advisories.append('%s <-> %s: both write `%s` at close -- reconcile centrally, do not race'
                              % (an, bn, s))

        ra, rb = a.get('baseRevision', ''), b.get('baseRevision', '')
        if ra and rb and ra != rb:
            advisories.append('%s froze at `%s` but %s froze at `%s` -- concurrent sprints on '
                              'different bases build against different trees' % (an, ra, bn, rb))

for v in violations:
    print('  VIOLATION  ' + v)
for a_ in advisories:
    print('  advisory   ' + a_)

n = len(footprints)
if violations:
    print('check-sprint-disjointness: %d overlap(s) across %d sprint(s), mode=%s'
          % (len(violations), n, mode))
    sys.exit(1 if mode == 'enforce' else 0)

print('check-sprint-disjointness: %d sprint(s) disjoint on all blocking axes, '
      '%d advisory note(s), mode=%s' % (n, len(advisories), mode))
sys.exit(0)
PY
