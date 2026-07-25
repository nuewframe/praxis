#!/usr/bin/env bash
# gen-doctrine-index.sh
#
# Generates the one doctrine fact that is duplicated VERBATIM across surfaces:
# each always-on guardrail's `applyTo` glob.
#
# The glob is declared in the instruction file's own frontmatter and was also
# written out as prose in skills/using-praxis/SKILL.md, hand-synced. One fact,
# two homes, nothing detecting divergence. The instruction file remains the
# single source; this script renders the second copy from it.
#
# Deliberately NOT generated: README's guardrail Scope column, the persona
# table's "when to be this persona" column, and the rule bullets. Those are
# three summaries written for three different readers — they were never required
# to agree word-for-word, and collapsing them into one generated block would
# trade readability for a rigour that was never missing. A fact stated twice is
# not the same thing as text appearing on two surfaces.
#
# Mirrors gen-coverage-matrix.sh / gen-tier-table.sh: generate / --write /
# --check, writing only between BEGIN/END markers so surrounding prose is never
# touched. --check is wired into CI, so a hand-edit inside a marker fails.
#
# Usage:
#   gen-doctrine-index.sh            Print the rendered block to stdout
#   gen-doctrine-index.sh --write    Write it into the target files
#   gen-doctrine-index.sh --check    Fail if any target is out of date
#
# Exit codes:
#   0 — clean / written
#   1 — drift detected (--check)
#   2 — invocation or source error

set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
cd "$REPO_ROOT" || exit 2

MODE="${1:-print}"

python3 - "$MODE" <<'PY'
import os, re, sys

mode = sys.argv[1]

BEGIN = ('<!-- BEGIN GENERATED: guardrail-scope (source: instructions/*.instructions.md '
         'frontmatter; regenerate with scripts/gen-doctrine-index.sh --write) -->')
END = '<!-- END GENERATED -->'

TARGET = 'skills/using-praxis/SKILL.md'


def read_apply_to(path):
    """The glob list from an instruction file's own frontmatter."""
    text = open(path, errors='replace').read()
    if not text.startswith('---'):
        return None
    end = text.find('\n---', 3)
    if end < 0:
        return None
    fm = text[3:end]
    m = re.search(r'^applyTo:\s*"?([^"\n]+)"?\s*$', fm, re.M)
    if not m:
        return None
    return [g.strip() for g in m.group(1).split(',') if g.strip()]


def title_of(path):
    """The instruction file's H1, so the rendered block names it as the doc does."""
    for line in open(path, errors='replace'):
        if line.startswith('# '):
            return line[2:].strip()
    return os.path.basename(path)


def render():
    rows = []
    for fn in sorted(os.listdir('instructions')):
        if not fn.endswith('.instructions.md'):
            continue
        path = os.path.join('instructions', fn)
        globs = read_apply_to(path)
        if globs is None:
            print('gen-doctrine-index: %s has no applyTo in frontmatter' % path, file=sys.stderr)
            sys.exit(2)
        rendered = ', '.join('`%s`' % g for g in globs)
        rows.append('| [%s](../../%s) | %s |' % (title_of(path), path, rendered))
    lines = [BEGIN,
             '| Guardrail | Applies to |',
             '| --------- | ---------- |']
    lines.extend(rows)
    lines.append(END)
    return '\n'.join(lines)


block = render()

if mode == 'print':
    print(block)
    sys.exit(0)

if not os.path.isfile(TARGET):
    print('gen-doctrine-index: target not found: %s' % TARGET, file=sys.stderr)
    sys.exit(2)

text = open(TARGET, errors='replace').read()
pattern = re.compile(re.escape(BEGIN) + r'.*?' + re.escape(END), re.S)

if not pattern.search(text):
    print('gen-doctrine-index: %s has no generated block; insert the BEGIN/END '
          'markers where the table belongs' % TARGET, file=sys.stderr)
    sys.exit(2)

updated = pattern.sub(lambda _: block, text)

if mode == '--check':
    if updated != text:
        print('gen-doctrine-index: %s is out of date -- regenerate with '
              'scripts/gen-doctrine-index.sh --write' % TARGET, file=sys.stderr)
        sys.exit(1)
    print('gen-doctrine-index: up to date')
    sys.exit(0)

if mode == '--write':
    if updated != text:
        open(TARGET, 'w').write(updated)
        print('gen-doctrine-index: wrote %s' % TARGET)
    else:
        print('gen-doctrine-index: %s already current' % TARGET)
    sys.exit(0)

print('gen-doctrine-index: unknown mode %r (expected --write or --check)' % mode, file=sys.stderr)
sys.exit(2)
PY
