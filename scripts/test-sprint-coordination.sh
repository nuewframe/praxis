#!/usr/bin/env bash
# test-sprint-coordination.sh
#
# Self-test for the two concurrency probes, check-sprint-disjointness.sh and
# check-contract-freshness.sh.
#
# The fixtures are not invented. `sprints-disjoint/` reproduces the two sprints
# that were actually dispatched concurrently the first time this method was run
# in parallel; `sprints-overlap/` is the unsafe shape they would have had if the
# footprints collided. The disjoint pair is the important case: it is disjoint on
# all four conditions of the original rule and still collides, which is why the
# footprint carries closeArtifacts and baseRevision.
#
# Exit: 0 = all cases pass; 1 = at least one failed.

set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIX="$HERE/__fixtures__"
FAILS=0

setup() { # <fixture-name> -> echoes a temp root
  local d; d=$(mktemp -d)
  mkdir -p "$d/docs/product"
  cp -r "$FIX/$1" "$d/docs/product/sprints"
  echo "$d"
}

expect() { # <label> <actual> <wanted>
  if [[ "$2" == "$3" ]]; then
    echo "  PASS $1"
  else
    echo "  FAIL $1 (got $2, wanted $3)"
    FAILS=$((FAILS + 1))
  fi
}

contains() { # <label> <haystack> <needle>
  case "$2" in
    *"$3"*) echo "  PASS $1" ;;
    *) echo "  FAIL $1 (missing: $3)"; FAILS=$((FAILS + 1)) ;;
  esac
}

echo "test-sprint-coordination: no footprints declared"
EMPTY=$(mktemp -d); mkdir -p "$EMPTY/docs/product/sprints"
bash "$HERE/check-sprint-disjointness.sh" "$EMPTY" >/dev/null 2>&1
expect "disjointness skips cleanly when nothing declares a footprint" "$?" "0"
bash "$HERE/check-contract-freshness.sh" "$EMPTY" >/dev/null 2>&1
expect "freshness skips cleanly when nothing declares a footprint" "$?" "0"

echo "test-sprint-coordination: the real concurrent dispatch"
D=$(setup sprints-disjoint)
OUT=$(bash "$HERE/check-sprint-disjointness.sh" "$D" 2>&1); RC=$?
expect "disjoint pair does not fail" "$RC" "0"
contains "reports no blocking violation" "$OUT" "disjoint on all blocking axes"
contains "flags the shared close artifact the four conditions miss" "$OUT" "at close -- reconcile centrally"
contains "flags divergent base revisions" "$OUT" "build against different trees"

echo "test-sprint-coordination: an unsafe pair"
O=$(setup sprints-overlap)
OUT=$(bash "$HERE/check-sprint-disjointness.sh" "$O" 2>&1)
contains "shared file glob is a violation" "$OUT" "shared file glob"
contains "shared persistent resource is a violation" "$OUT" "shared persistent resource"
contains "shared config key is a violation" "$OUT" "shared config key"
contains "depending on a sibling's in-flight capability is a violation" "$OUT" "in-flight internals"

bash "$HERE/check-sprint-disjointness.sh" "$O" >/dev/null 2>&1
expect "warn mode does not fail the build" "$?" "0"
echo '{"mode":"enforce"}' > "$O/.sprint-coordination.json"
bash "$HERE/check-sprint-disjointness.sh" "$O" >/dev/null 2>&1
expect "enforce mode fails the build" "$?" "1"

echo "test-sprint-coordination: contract freshness"
F=$(mktemp -d); mkdir -p "$F/docs/product/sprints"
cat > "$F/docs/product/sprints/SPRINT.260725.09-x.md" <<'EOF'
# SPRINT.260725.09
## Sprint Footprint
```json
{"sprint":"260725.09","capabilities":["a"],"fileGlobs":["a/**"],"persistentResources":[],
"configKeysWritten":[],"dependsOnContracts":["billing@v2"],"closeArtifacts":[],"baseRevision":"deadbee"}
```
EOF
OUT=$(bash "$HERE/check-contract-freshness.sh" "$F" 2>&1)
contains "a dependency with no manifest is stale" "$OUT" "no seam manifest exists"
echo '{"seams":[{"name":"billing","version":"3"}]}' > "$F/.seam-contracts.json"
OUT=$(bash "$HERE/check-contract-freshness.sh" "$F" 2>&1)
contains "a contract that moved version is stale" "$OUT" "the manifest now declares"
echo '{"seams":[{"name":"billing","version":"2"}]}' > "$F/.seam-contracts.json"
OUT=$(bash "$HERE/check-contract-freshness.sh" "$F" 2>&1)
contains "a contract still at its frozen version is fresh" "$OUT" "fresh"

if [[ $FAILS -ne 0 ]]; then
  echo ""
  echo "test-sprint-coordination: $FAILS case(s) failed"
  exit 1
fi
echo ""
echo "test-sprint-coordination: all cases passed"
