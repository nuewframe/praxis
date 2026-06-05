#!/usr/bin/env bash
# validate-plugin.sh
#
# Repository self-test for the praxis plugin. Validates:
#   1. Every SKILL.md has a parseable YAML frontmatter block with required keys.
#   2. Every JSON file in the repo parses cleanly.
#   3. Every YAML file in the repo parses cleanly.
#   4. Every cross-reference (`<plugin-root>/...`, `skills/<name>/...`,
#      `agents/<name>...`, `instructions/<name>...`) resolves to a real file.
#
# Compatible with bash 3.2+ (macOS default). Requires python3.
#
# Exit codes:
#   0 — clean
#   1 — validation failures
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "validate-plugin: error: not a directory: $ROOT" >&2
  exit 2
fi

if ! command -v python3 >/dev/null 2>&1; then
  echo "validate-plugin: error: python3 is required" >&2
  exit 2
fi

cd "$ROOT"

FAILED=0

# 1. SKILL.md frontmatter.
echo "validate-plugin: checking SKILL.md frontmatter..."
SKILL_REPORT=$(python3 <<'PY'
import os, sys, re

required = {'name', 'description'}
problems = []
seen_names = {}

for path in sorted(p for p in os.popen('find skills -type f -name SKILL.md').read().splitlines() if p):
    try:
        text = open(path).read()
    except Exception as e:
        problems.append(f'{path}: cannot read ({e})')
        continue
    if not text.startswith('---'):
        problems.append(f'{path}: missing YAML frontmatter')
        continue
    end = text.find('\n---', 3)
    if end < 0:
        problems.append(f'{path}: unterminated frontmatter')
        continue
    fm = text[3:end].lstrip('\n')
    try:
        import yaml
        data = yaml.safe_load(fm) or {}
    except ImportError:
        # Minimal manual parse: only verify `key:` presence.
        keys = set()
        for line in fm.splitlines():
            m = re.match(r'^([A-Za-z_][\w-]*)\s*:', line)
            if m: keys.add(m.group(1))
        missing = required - keys
        if missing:
            problems.append(f'{path}: missing keys: {sorted(missing)}')
        continue
    except Exception as e:
        problems.append(f'{path}: yaml error: {e}')
        continue

    missing = required - set(data.keys())
    if missing:
        problems.append(f'{path}: missing keys: {sorted(missing)}')
    name = data.get('name')
    if name:
        if name in seen_names:
            problems.append(f'{path}: duplicate name "{name}" (also in {seen_names[name]})')
        else:
            seen_names[name] = path
    mode = data.get('mode')
    if mode and mode not in ('architect', 'implementer', 'reviewer'):
        problems.append(f'{path}: invalid mode "{mode}" (expected architect|implementer|reviewer)')

for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
SKILL_RC=$?
if [[ $SKILL_RC -ne 0 ]]; then
  echo "$SKILL_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 2. JSON files.
echo "validate-plugin: checking JSON files..."
JSON_REPORT=$(python3 <<'PY'
import json, os, sys
problems = []
for path in sorted(os.popen("find . -type f -name '*.json' -not -path './node_modules/*' -not -path './.git/*'").read().splitlines()):
    try:
        json.load(open(path))
    except Exception as e:
        problems.append(f'{path}: {e}')
for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
JSON_RC=$?
if [[ $JSON_RC -ne 0 ]]; then
  echo "$JSON_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 3. YAML files.
echo "validate-plugin: checking YAML files..."
YAML_REPORT=$(python3 <<'PY'
import os, sys
try:
    import yaml
except ImportError:
    print('skipped (PyYAML not installed)')
    sys.exit(0)
problems = []
for path in sorted(os.popen("find . -type f \\( -name '*.yaml' -o -name '*.yml' \\) -not -path './node_modules/*' -not -path './.git/*'").read().splitlines()):
    try:
        with open(path) as f:
            list(yaml.safe_load_all(f))
    except Exception as e:
        problems.append(f'{path}: {e}')
for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
YAML_RC=$?
if [[ $YAML_RC -ne 0 ]]; then
  echo "$YAML_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 4. Cross-references in markdown.
echo "validate-plugin: checking cross-references..."
XREF_REPORT=$(python3 <<'PY'
import os, re, sys

# Match relative refs to skills/<name>, agents/<name>, instructions/<name>,
# scripts/<name>. Only check existence of the directory or file root, not deep
# anchors. Backtick-wrapped paths are fine; angle-bracket links too.
# docs/plans/* is excluded: planning docs are forward-looking and legitimately
# reference not-yet-built artifacts.
patterns = [
    re.compile(r'`(skills/[A-Za-z0-9_./-]+)`'),
    re.compile(r'`(agents/[A-Za-z0-9_./-]+)`'),
    re.compile(r'`(instructions/[A-Za-z0-9_./-]+)`'),
    re.compile(r'`(scripts/[A-Za-z0-9_./-]+)`'),
]

# Refs to project-bootstrapped files (created by bootstrap-project in target
# repos, not shipped by the plugin itself).
allowed_missing = {
    'scripts/verify.sh',
}

problems = []
md_files = [p for p in os.popen("find . -type f -name '*.md' -not -path './node_modules/*' -not -path './.git/*' -not -path './docs/plans/*'").read().splitlines() if p]

for path in sorted(md_files):
    try:
        text = open(path).read()
    except Exception:
        continue
    for pat in patterns:
        for m in pat.finditer(text):
            ref = m.group(1).rstrip('/.')
            # Strip line/anchor suffixes.
            ref = ref.split('#', 1)[0]
            # Allow templates folder references that include placeholders.
            if '<' in ref or '{' in ref:
                continue
            if ref in allowed_missing:
                continue
            # Bare directory refs like skills/foo (no trailing file) → check dir.
            if not os.path.exists(ref):
                # Try as directory with SKILL.md.
                if os.path.isdir(ref):
                    continue
                problems.append(f'{path}: broken ref `{m.group(1)}`')

for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
XREF_RC=$?
if [[ $XREF_RC -ne 0 ]]; then
  echo "$XREF_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 5. Manifest version parity.
echo "validate-plugin: checking manifest version parity..."
if [[ -x "scripts/bump-version.sh" && -f ".version-bump.json" ]] && command -v jq >/dev/null 2>&1; then
  if ! BUMP_REPORT=$(scripts/bump-version.sh --check 2>&1); then
    echo "$BUMP_REPORT" >&2
    FAILED=$((FAILED + 1))
  else
    echo "  ok"
  fi
else
  echo "  skipped (jq or bump-version.sh missing)"
fi

# 6. Enforcement script syntax + executability (meta-loop: the quality
#    instruments are themselves under validation — see executable-seams-first.md D5).
echo "validate-plugin: checking enforcement scripts..."
ENFORCE_REPORT=""
ENFORCE_FAIL=0
for s in scripts/check-*.sh; do
  [[ -e "$s" ]] || continue
  if ! bash -n "$s" 2>/dev/null; then
    ENFORCE_REPORT="${ENFORCE_REPORT}  $s: syntax error"$'\n'
    ENFORCE_FAIL=1
  fi
  if [[ ! -x "$s" ]]; then
    ENFORCE_REPORT="${ENFORCE_REPORT}  $s: not executable (chmod +x)"$'\n'
    ENFORCE_FAIL=1
  fi
done
if [[ $ENFORCE_FAIL -ne 0 ]]; then
  printf '%s' "$ENFORCE_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

if [[ $FAILED -gt 0 ]]; then
  echo "validate-plugin: $FAILED check(s) failed" >&2
  exit 1
fi

echo "validate-plugin: all checks passed"
exit 0
