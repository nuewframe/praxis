#!/usr/bin/env bash
# validate-plugin.sh
#
# Repository self-test for the praxis plugin. Validates:
#   1. Every SKILL.md has a parseable YAML frontmatter block with required keys,
#      and any `tools:` key is a single-line flow sequence (multi-line/block
#      form silently breaks Claude Code skill registration).
#   2. Every JSON file in the repo parses cleanly.
#   3. Every YAML file in the repo parses cleanly.
#   4. Every cross-reference (`<plugin-root>/...`, `skills/<name>/...`,
#      `agents/<name>...`, `instructions/<name>...`) resolves to a real file.
#   5. Manifest versions are in parity across all declared manifests.
#   6. Every enforcement script parses and is executable.
#   7. Inventory parity: every skill, script, and instruction on disk is
#      referenced in the canonical self-describing docs (README.md,
#      project-context.md, and — for instructions — using-praxis), so the
#      docs cannot silently drift behind the file tree.
#   8. Every agent (`agents/*.agent.md`) has parseable frontmatter with the
#      required keys.
#   9. Fenced-code balance: every markdown/template file closes its fences, so a
#      template broken by a nested bare fence cannot ship again.
#  10. Terminology drift: forbidden legacy terms (.praxis-canon.json) must not
#      reappear in the doctrine surfaces.
#  11. Template placeholder parity: every {{key}} in an overlay template resolves
#      against a key in praxis.config.yaml.tmpl.
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

    # Claude Code registration guard: a `tools:` key in SKILL.md must be a
    # single-line flow sequence (tools: [a, b, c]). A multi-line flow sequence
    # or a block sequence is valid YAML but silently prevents the skill from
    # registering in Claude Code — the exact defect that disabled six skills.
    # The known-good control case is a single-line flow sequence.
    for line in fm.splitlines():
        m = re.match(r'^tools:\s*(.*)$', line)
        if m:
            val = m.group(1).strip()
            if not (val.startswith('[') and val.endswith(']')):
                problems.append(f'{path}: `tools:` must be a single-line flow sequence '
                                '(tools: [a, b, c]); a multi-line or block form breaks '
                                'Claude Code skill registration')
            break

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

# 7. Inventory parity: every skill, script, and instruction on disk must be
#    referenced in the canonical self-describing docs, so the docs cannot
#    silently drift behind the file tree.
#      - skills/<name>/        → README.md AND project-context.md
#      - scripts/check-*.sh    → README.md AND project-context.md
#      - scripts/*.sh (others) → project-context.md (README allowlist below)
#      - instructions/*.md     → README.md, project-context.md, using-praxis
echo "validate-plugin: checking inventory parity..."
INV_REPORT=$(python3 <<'PY'
import os, sys

def read(path):
    try:
        return open(path).read()
    except Exception:
        return ''

readme = read('README.md')
context = read('project-context.md')
bootstrap = read('skills/using-praxis/SKILL.md')

problems = []

# README-optional scripts: release/dev tooling not wired into target projects.
readme_optional_scripts = {'bump-version.sh'}

# Skills — each must appear in README and project-context.
for skill in sorted(d for d in os.listdir('skills') if os.path.isdir(os.path.join('skills', d))):
    if not os.path.isfile(os.path.join('skills', skill, 'SKILL.md')):
        continue
    if skill not in readme:
        problems.append(f'skills/{skill}: not referenced in README.md')
    if skill not in context:
        problems.append(f'skills/{skill}: not referenced in project-context.md')

# Scripts — every script must appear in project-context; check-*.sh + others
# (minus the release allowlist) must also appear in README.
for script in sorted(f for f in os.listdir('scripts') if f.endswith('.sh')):
    if script not in context:
        problems.append(f'scripts/{script}: not referenced in project-context.md')
    if script not in readme_optional_scripts and script not in readme:
        problems.append(f'scripts/{script}: not referenced in README.md')

# Instructions — each always-on guardrail must appear in README, project-context,
# and the bootstrap skill index, so the guardrail count never diverges.
for instr in sorted(f for f in os.listdir('instructions') if f.endswith('.instructions.md')):
    if instr not in readme:
        problems.append(f'instructions/{instr}: not referenced in README.md')
    if instr not in context:
        problems.append(f'instructions/{instr}: not referenced in project-context.md')
    if instr not in bootstrap:
        problems.append(f'instructions/{instr}: not referenced in skills/using-praxis/SKILL.md')

for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
INV_RC=$?
if [[ $INV_RC -ne 0 ]]; then
  echo "$INV_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 8. Agent frontmatter — parseable, with required keys. Agent personas are a
#    fourth surface the earlier checks never covered (where wrong tool names and
#    missing keys can hide).
echo "validate-plugin: checking agent frontmatter..."
AGENT_REPORT=$(python3 <<'PY'
import os, re, sys
required = {'name', 'description'}
problems = []
for path in sorted(p for p in os.popen('find agents -type f -name "*.agent.md" 2>/dev/null').read().splitlines() if p):
    text = open(path).read()
    if not text.startswith('---'):
        problems.append(f'{path}: missing YAML frontmatter'); continue
    end = text.find('\n---', 3)
    if end < 0:
        problems.append(f'{path}: unterminated frontmatter'); continue
    fm = text[3:end].lstrip('\n')
    try:
        import yaml
        keys = set((yaml.safe_load(fm) or {}).keys())
    except ImportError:
        keys = set(m.group(1) for m in (re.match(r'^([A-Za-z_][\w-]*)\s*:', l) for l in fm.splitlines()) if m)
    except Exception as e:
        problems.append(f'{path}: yaml error: {e}'); continue
    missing = required - keys
    if missing:
        problems.append(f'{path}: missing keys: {sorted(missing)}')
for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
AGENT_RC=$?
if [[ $AGENT_RC -ne 0 ]]; then
  echo "$AGENT_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 9. Fenced-code balance — every markdown/template file must close its fences.
#    A CommonMark closing fence is bare (no info string) and has at least the
#    opening tick count, so a ```markdown template broken by a nested ``` fence
#    (the item-4 corruption) leaves a residual open fence this check catches.
echo "validate-plugin: checking fenced-code balance..."
FENCE_REPORT=$(python3 <<'PY'
import os, re, sys
problems = []
files = os.popen(r"find . -type f \( -name '*.md' -o -name '*.md.tmpl' \) "
                 r"-not -path './node_modules/*' -not -path './.git/*'").read().splitlines()
for path in sorted(f for f in files if f):
    stack = []
    for line in open(path, errors='replace'):
        m = re.match(r'^(`{3,})(.*)$', line.rstrip('\n'))
        if not m:
            continue
        ticks, info = len(m.group(1)), m.group(2).strip()
        if stack and info == '' and ticks >= stack[-1]:
            stack.pop()
        elif info != '':
            stack.append(ticks)
        elif not stack:
            stack.append(ticks)
    if stack:
        problems.append(f'{path}: unbalanced code fence(s) (residual {stack}); use a '
                        'four-backtick outer fence when a template embeds nested ``` fences')
for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
FENCE_RC=$?
if [[ $FENCE_RC -ne 0 ]]; then
  echo "$FENCE_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 10. Terminology drift — forbidden legacy terms (from .praxis-canon.json) must
#     not reappear in the doctrine surfaces. Single source of truth for terms.
echo "validate-plugin: checking terminology..."
TERM_REPORT=$(python3 <<'PY'
import json, os, re, sys
if not os.path.isfile('.praxis-canon.json'):
    print('skipped (no .praxis-canon.json)'); sys.exit(0)
canon = json.load(open('.praxis-canon.json'))
terms = canon.get('forbiddenTerms', [])
scan_dirs = canon.get('terminologyScanDirs', [])
allow = tuple(canon.get('terminologyAllowPaths', []))
files = []
for d in scan_dirs:
    files += os.popen("find %s -type f \\( -name '*.md' -o -name '*.md.tmpl' \\)" % d).read().splitlines()
problems = []
for path in sorted(f for f in files if f):
    if any(a in path for a in allow):
        continue
    text = open(path, errors='replace').read()
    for t in terms:
        m = re.search(t['pattern'], text)
        if m:
            problems.append("%s: forbidden term '%s' -- %s" % (path, m.group(0), t['reason']))
for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
TERM_RC=$?
if [[ $TERM_RC -ne 0 ]]; then
  echo "$TERM_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 11. Template placeholder parity — every {{key.path}} in an overlay template
#     must resolve against a key in praxis.config.yaml.tmpl. Permanent guard for
#     the alias hyphen/underscore class of defect.
echo "validate-plugin: checking template placeholder parity..."
PH_REPORT=$(python3 <<'PY'
import json, os, re, sys
if not os.path.isfile('.praxis-canon.json'):
    print('skipped (no .praxis-canon.json)'); sys.exit(0)
canon = json.load(open('.praxis-canon.json'))
cfg_path = canon['placeholderConfigTemplate']
scan_root = canon['placeholderScanGlob']
ph = re.compile(r'\{\{\s*([A-Za-z0-9_.]+)\s*\}\}')
# The config template declares every substitutable key as `key: {{that.key}}`,
# so its own placeholders enumerate exactly the valid dotted paths.
valid = set(ph.findall(open(cfg_path, errors='replace').read()))
problems = set()
for path in sorted(p for p in os.popen("find %s -type f -name '*.tmpl'" % scan_root).read().splitlines() if p):
    if os.path.abspath(path) == os.path.abspath(cfg_path):
        continue
    for m in ph.finditer(open(path, errors='replace').read()):
        key = m.group(1)
        if key not in valid:
            problems.add("%s: placeholder {{%s}} has no matching key in %s" % (path, key, os.path.basename(cfg_path)))
for p in sorted(problems):
    print(p)
sys.exit(1 if problems else 0)
PY
)
PH_RC=$?
if [[ $PH_RC -ne 0 ]]; then
  echo "$PH_REPORT" >&2
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
