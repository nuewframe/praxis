#!/usr/bin/env bash
# check-contract-freshness.sh
#
# A sprint is an immutable bridge frozen against a moment. If a seam contract it
# depends on moved after that freeze, the sprint is building against a promise
# that no longer holds — the "snapshot rots" failure that frozen `<name>@vN`
# contracts exist to prevent.
#
# For each active sprint's `Sprint Footprint` block, every `dependsOnContracts`
# entry is checked against .seam-contracts.json: the contract must still exist,
# and its declared version must still be the frozen one.
#
# It also checks `baseRevision`, which is NOT a contract check and is the reason
# contract hashing alone is insufficient. The first real concurrent dispatch cut
# both worktrees from a base two releases behind the frozen tip. Every contract
# hash still matched — there were no contracts — while the tree the agents built
# against was stale. Staleness can live in the base revision, not just a seam.
#
# Posture: warn-first via .sprint-coordination.json, same as its sibling probe.
# A repo with no seam manifest and no footprints skips cleanly.
#
# Usage:
#   check-contract-freshness.sh [repo-root]
#
# Exit codes:
#   0 — fresh, or warn mode, or nothing to check
#   1 — a depended-on contract moved or vanished, and mode=enforce

set -uo pipefail

ROOT="${1:-.}"
SPRINT_DIR="$ROOT/docs/product/sprints"
MANIFEST="$ROOT/.seam-contracts.json"
CONFIG="$ROOT/.sprint-coordination.json"

MODE="warn"
if [[ -f "$CONFIG" ]] && command -v jq >/dev/null 2>&1; then
  MODE=$(jq -r '.mode // "warn"' "$CONFIG" 2>/dev/null || echo warn)
fi

if [[ ! -d "$SPRINT_DIR" ]]; then
  echo "check-contract-freshness: no $SPRINT_DIR; skipping"
  exit 0
fi

python3 - "$SPRINT_DIR" "$MANIFEST" "$MODE" "$ROOT" <<'PY'
import json, os, re, subprocess, sys

sprint_dir, manifest_path, mode, root = sys.argv[1:5]

BLOCK_RE = re.compile(r'##\s*Sprint Footprint[^\n]*\n(.*?)```json\n(.*?)\n```', re.S)

manifest = {}
if os.path.isfile(manifest_path):
    try:
        raw = json.load(open(manifest_path))
        for seam in (raw.get('seams') or raw if isinstance(raw, list) else raw.get('seams', [])):
            if isinstance(seam, dict) and seam.get('name'):
                manifest[seam['name']] = seam.get('version')
    except Exception as e:
        print('check-contract-freshness: cannot read %s: %s' % (manifest_path, e), file=sys.stderr)
        sys.exit(1)

problems, advisories, checked = [], [], 0

for fn in sorted(os.listdir(sprint_dir)):
    if not (fn.startswith('SPRINT.') and fn.endswith('.md')) or fn.endswith('.ledger.md'):
        continue
    m = BLOCK_RE.search(open(os.path.join(sprint_dir, fn), errors='replace').read())
    if not m:
        continue
    try:
        fp = json.loads(m.group(2))
    except Exception:
        continue

    for dep in fp.get('dependsOnContracts', []):
        checked += 1
        if '@' not in dep:
            problems.append('%s: `%s` is not a frozen `<name>@vN` id' % (fn, dep))
            continue
        name, ver = dep.rsplit('@', 1)
        if not manifest:
            problems.append('%s: depends on `%s` but no seam manifest exists' % (fn, dep))
        elif name not in manifest:
            problems.append('%s: depends on `%s` but that contract is no longer declared' % (fn, dep))
        elif manifest[name] and ('v%s' % manifest[name]).lstrip('vv') != ver.lstrip('v'):
            problems.append('%s: froze against `%s` but the manifest now declares `%s@v%s`'
                            % (fn, dep, name, manifest[name]))

    base = fp.get('baseRevision')
    if base:
        try:
            r = subprocess.run(['git', '-C', root, 'merge-base', '--is-ancestor', base, 'HEAD'],
                               capture_output=True)
            if r.returncode != 0:
                advisories.append('%s: baseRevision `%s` is not an ancestor of HEAD -- the bridge '
                                  'was frozen against a tree this branch does not contain' % (fn, base))
        except Exception:
            pass

for p in problems:
    print('  STALE      ' + p)
for a in advisories:
    print('  advisory   ' + a)

if problems:
    print('check-contract-freshness: %d stale dependency(ies), mode=%s' % (len(problems), mode))
    sys.exit(1 if mode == 'enforce' else 0)

print('check-contract-freshness: %d contract dependency(ies) fresh, %d advisory note(s), mode=%s'
      % (checked, len(advisories), mode))
sys.exit(0)
PY
