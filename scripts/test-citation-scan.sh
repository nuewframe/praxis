#!/usr/bin/env bash
# test-citation-scan.sh
#
# Self-test for scripts/citation_scan.py — the shared citation-vs-assertion
# implementation both literal scanners consume (ADR.260725).
#
# It is a self-test rather than an inline assertion because ADR.260725's risk
# table names the exact regression to guard: a second fence parser repeating
# check #14's marker-length false negative. The nesting case below is that bug.
#
# Exit: 0 = all cases pass; 1 = at least one failed.

set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

python3 - "$HERE" <<'PY'
import sys, os, tempfile
sys.path.insert(0, sys.argv[1])
import citation_scan as cs

SEMVER = r'[0-9]+\.[0-9]+\.[0-9]+'
VMARK = 'praxis:allow-version-literal'
fails = []

def mk(text):
    fd, p = tempfile.mkstemp(suffix='.md')
    os.write(fd, text.encode()); os.close(fd)
    return p

def check(name, cond):
    print(("  PASS " if cond else "  FAIL ") + name)
    if not cond:
        fails.append(name)

print("test-citation-scan: Layer 1 (structural citation detection)")

r = cs.analyze(mk("prose 1.2.3 here\n```\nfenced 4.5.6\n```\n"), pattern=SEMVER)
check("literal inside a fenced block is a citation", 3 in r.exempt)
check("literal in running prose is not", 1 not in r.exempt)

r = cs.analyze(mk("````\ninner ```\nstill 1.2.3 inside\n````\nprose 9.9.9\n"), pattern=SEMVER)
check("long fence is not closed early by a shorter inner fence", 3 in r.exempt)
check("prose after the long fence closes is not exempt", 5 not in r.exempt)

r = cs.analyze(mk("> quoted 1.2.3\nprose 1.2.3\n"), pattern=SEMVER)
check("literal inside a blockquote is a citation", 1 in r.exempt)
check("prose following a blockquote is not", 2 not in r.exempt)

r = cs.analyze(mk("see `version 1.2.3` for detail\nwe are on 1.2.3 today\n"), pattern=SEMVER)
check("literal inside an inline code span is a citation", 1 in r.exempt)
check("bare prose literal is not", 2 not in r.exempt)

r = cs.analyze(mk("`1.2.3` was old but we are on 9.9.9 now\n"), pattern=SEMVER)
check("a code span does not excuse a prose literal on the same line", 1 not in r.exempt)

print("test-citation-scan: Layer 2 (inline declared markers)")

r = cs.analyze(mk('<!-- %s reason="release context" -->\nshipped in 0.4.0\n' % VMARK),
               marker=VMARK, pattern=SEMVER)
check("marker with a non-empty reason exempts the following line", 2 in r.exempt)
check("a well-formed marker reports no defect", r.bad_markers == [])

r = cs.analyze(mk('<!-- %s -->\nshipped in 0.4.0\n' % VMARK), marker=VMARK, pattern=SEMVER)
check("marker with no reason= is a failure", len(r.bad_markers) == 1)

r = cs.analyze(mk('<!-- %s reason="   " -->\nshipped in 0.4.0\n' % VMARK), marker=VMARK, pattern=SEMVER)
check("marker with an empty reason= is a failure", len(r.bad_markers) == 1)

r = cs.analyze(mk('<!-- praxis:allow-term reason="x" -->\nwe are on 9.9.9\n'),
               marker=VMARK, pattern=SEMVER)
check("a marker of a different kind does not exempt", 2 not in r.exempt)

if fails:
    print("\ntest-citation-scan: %d case(s) failed" % len(fails))
    sys.exit(1)
print("\ntest-citation-scan: all cases passed")
PY
