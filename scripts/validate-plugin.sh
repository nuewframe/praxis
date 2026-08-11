#!/usr/bin/env bash
# validate-plugin.sh
#
# Repository self-test for the praxis plugin. Validates:
#   1. Every SKILL.md has a parseable YAML frontmatter block with required keys,
#      and any `tools:` key is a single-line flow sequence (multi-line/block
#      form silently breaks Claude Code skill registration).
#   2. Every JSON file in the repo parses cleanly.
#   3. Every YAML file in the repo parses cleanly.
#   4. Every repository FILE path named in markdown prose — backticked or bare,
#      under docs/, skills/, agents/, instructions/, or scripts/ — resolves.
#      Fence/blockquote/praxis:allow-path aware; markdown links excluded (#14
#      owns those and resolves them file-relative).
#   5. Manifest versions are in parity across all declared manifests.
#   6. Every enforcement script parses and is executable.
#   7. Inventory parity: every skill, script, and instruction on disk is
#      referenced in the canonical self-describing docs (README.md,
#      docs/product.md, and — for instructions — using-praxis), so the
#      docs cannot silently drift behind the file tree.
#   8. Every agent (`agents/*.agent.md`) has parseable frontmatter with the
#      required keys.
#   9. Fenced-code balance: every markdown/template file closes its fences, so a
#      template broken by a nested bare fence cannot ship again.
#  10. Terminology drift: forbidden legacy terms (.praxis-canon.json) must not
#      reappear in the doctrine surfaces.
#  11. Template placeholder parity: every {{key}} in an overlay template resolves
#      against a key in praxis.config.yaml.tmpl.
#  12. Required-phrase presence: .praxis-canon.json's requiredPhrases appear in
#      the files that must carry them.
#  13. Version single-source: package.json is the only home of the version
#      (delegates to bump-version.sh --audit).
#  14. Link resolution: every relative markdown link points at a real file.
#      Fence/code-span/praxis:allow-path aware via citation_scan (blockquote
#      excepted — a link in a blockquote renders live and must still resolve).
#  15. CHANGELOG structure: version headings parse, are unique, and descend.
#  16. Self-conformance declaration parity: every shipped check-*.sh is declared
#      in .self-conformance.json as run-against-Praxis or a reasoned n/a.
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
for path in sorted(os.popen("find . -type f -name '*.json' -not -path './node_modules/*' -not -path './.git/*' -not -path './.claude/*'").read().splitlines()):
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
for path in sorted(os.popen("find . -type f \\( -name '*.yaml' -o -name '*.yml' \\) -not -path './node_modules/*' -not -path './.git/*' -not -path './.claude/*'").read().splitlines()):
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

# 4. Cross-references in markdown — a repository path named in PROSE must
#    resolve, not only one written as a markdown link. Link resolution (#14)
#    proves a link works; it says nothing about the far more common backticked
#    or bare path in running text. Removing a directory used to strand every
#    such reference silently, findable only by grep.
#
#    Four rules keep the signal meaningful:
#      - FILE paths only (a segment carrying an extension). Bare directory names
#        are where illustrative host-repo structure concentrates, and matching
#        them buried the real defects under examples.
#      - Markdown link constructs are EXCLUDED. A prose path resolves from the
#        repo root; a link target resolves from the linking file's directory.
#        Conflating them reported a README in the skills tree as broken when the
#        link was correct. Check #14 owns links and resolves them correctly.
#      - Fence, blockquote, and `praxis:allow-path` exemption via citation_scan
#        — the SAME implementation checks #10 and #14 use. Code-span exemption
#        is deliberately NOT applied: for a path, backticks are the ordinary
#        notation, so exempting them would excuse nearly every reference.
#      - `<placeholder>` segments stay unmatched.
echo "validate-plugin: checking cross-references..."
XREF_REPORT=$(python3 <<'PY'
import os, re, sys
sys.path.insert(0, 'scripts')
import citation_scan

ROOTS = ('docs/', 'skills/', 'agents/', 'instructions/', 'scripts/')

# A repository file path: rooted at a known top-level dir, ending in a segment
# with an extension. Captured either inside backticks or bare in prose.
PATH_RE = re.compile(
    r'`((?:docs|skills|agents|instructions|scripts)/[A-Za-z0-9_./-]+\.[A-Za-z0-9]+)`'
    r'|(?<![\w./-])((?:docs|skills|agents|instructions|scripts)/[A-Za-z0-9_./-]+\.[A-Za-z0-9]+)')

# Markdown link constructs — both the [text] and the (target) halves. These
# resolve file-relative and belong to check #14.
LINK_RE = re.compile(r'\[[^\]]*\]\([^)]*\)')

# Refs to project-bootstrapped files (created by bootstrap-project in target
# repos, not shipped by the plugin itself). NOT an exemption for a path that
# should exist here — this file genuinely never exists in this repo.
allowed_missing = {
    'scripts/verify.sh',
    'scripts/ast_parse.sh',
    'scripts/ast_parse_ts.js',
    'scripts/ast_parse_py.py',
    'scripts/ast_parse_go.go',
    'scripts/ast_parse_rs.rs',
    'scripts/ast_parse_cs.cs',
    'scripts/ast_parse_java.java',
    'scripts/ast_parse_kt.kt',
    'docs/project-context.md',
    'docs/product/README.md',
    'docs/product/design.md',
    'docs/architecture.md',
}

SKIP_DIRS = {'.git', 'node_modules', '.claude'}
md_files = []
for dirpath, dirnames, filenames in os.walk('.'):
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for fn in filenames:
        if fn.endswith('.md'):
            md_files.append(os.path.join(dirpath, fn).replace('./', '', 1))

problems = []
for path in sorted(md_files):
    result = citation_scan.analyze(path, marker='praxis:allow-path')
    for lineno, msg in result.bad_markers:
        problems.append('%s:%d: %s' % (path, lineno, msg))
    try:
        lines = open(path, errors='replace').read().split('\n')
    except Exception:
        continue
    for lineno, line in enumerate(lines, 1):
        if lineno in result.exempt:
            continue
        # Blank out link constructs so their halves are never matched as prose.
        scrubbed = LINK_RE.sub(lambda m: ' ' * len(m.group(0)), line)
        for m in PATH_RE.finditer(scrubbed):
            ref = (m.group(1) or m.group(2))
            ref = ref.split('#', 1)[0].rstrip('/.,;:)')
            if '<' in ref or '>' in ref or '{' in ref or '*' in ref:
                continue
            if ref in allowed_missing:
                continue
            if not os.path.exists(ref):
                problems.append('%s:%d: broken path reference `%s`' % (path, lineno, ref))

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
#    instruments are themselves under validation — see the executable-seams wave).
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
#      - skills/<name>/        → README.md AND docs/product.md
#      - scripts/check-*.sh    → README.md AND docs/product.md
#      - scripts/*.sh (others) → docs/product.md (README allowlist below)
#      - instructions/*.md     → README.md, docs/product.md, using-praxis
echo "validate-plugin: checking inventory parity..."
INV_REPORT=$(python3 <<'PY'
import os, sys

def read(path):
    try:
        return open(path).read()
    except Exception:
        return ''

readme = read('README.md')
context = read('docs/product.md')
bootstrap = read('skills/using-praxis/SKILL.md')

problems = []

# README-optional scripts: release/dev tooling not wired into target projects.
readme_optional_scripts = {'bump-version.sh'}

# Skills — each must appear in README and docs/product.md.
for skill in sorted(d for d in os.listdir('skills') if os.path.isdir(os.path.join('skills', d))):
    if not os.path.isfile(os.path.join('skills', skill, 'SKILL.md')):
        continue
    if skill not in readme:
        problems.append(f'skills/{skill}: not referenced in README.md')
    if skill not in context:
        problems.append(f'skills/{skill}: not referenced in docs/product.md')

# Scripts — every script must appear in docs/product.md; check-*.sh + others
# (minus the release allowlist) must also appear in README.
for script in sorted(f for f in os.listdir('scripts') if f.endswith('.sh')):
    if script not in context:
        problems.append(f'scripts/{script}: not referenced in docs/product.md')
    if script not in readme_optional_scripts and script not in readme:
        problems.append(f'scripts/{script}: not referenced in README.md')

# Instructions — each always-on guardrail must appear in README, docs/product.md,
# and the bootstrap skill index, so the guardrail count never diverges.
for instr in sorted(f for f in os.listdir('instructions') if f.endswith('.instructions.md')):
    if instr not in readme:
        problems.append(f'instructions/{instr}: not referenced in README.md')
    if instr not in context:
        problems.append(f'instructions/{instr}: not referenced in docs/product.md')
    if instr not in bootstrap:
        problems.append(f'instructions/{instr}: not referenced in skills/using-praxis/SKILL.md')

# Reverse direction: a name claimed by a canonical doc must exist on disk.
# The loops above prove "on disk -> mentioned"; nothing proved "mentioned -> on
# disk", so deleting a skill left stale claims across three surfaces with a
# green build. Check #4 does not cover this: it matches FILE paths, and these
# references are directory-shaped.
import re
sys.path.insert(0, 'scripts')
import citation_scan

CANONICAL = ['README.md', 'docs/product.md', 'skills/using-praxis/SKILL.md']
CLAIM_RE = re.compile(
    r'`?(skills/([a-z0-9][a-z0-9-]*)/)`?'
    r'|`?(instructions/([a-z0-9][a-z0-9-]*\.instructions\.md))`?'
    r'|`?(agents/([a-z0-9][a-z0-9-]*\.agent\.md))`?')

for doc in CANONICAL:
    if not os.path.isfile(doc):
        continue
    result = citation_scan.analyze(doc, marker='praxis:allow-path')
    for lineno, line in enumerate(open(doc, errors='replace').read().split('\n'), 1):
        if lineno in result.exempt:
            continue
        for m in CLAIM_RE.finditer(line):
            ref = m.group(1) or m.group(3) or m.group(5)
            if not ref or '<' in ref or '>' in ref:
                continue
            target = ref.rstrip('/') if ref.startswith('skills/') else ref
            if ref.startswith('skills/'):
                if not os.path.isfile(os.path.join(target, 'SKILL.md')):
                    problems.append('%s:%d: names `%s` but no such skill exists on disk'
                                    % (doc, lineno, ref))
            elif not os.path.isfile(target):
                problems.append('%s:%d: names `%s` but no such file exists on disk'
                                % (doc, lineno, ref))

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
                 r"-not -path './node_modules/*' -not -path './.git/*' -not -path './.claude/*'").read().splitlines()
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
#     Line-scoped and citation-aware per ADR.260725: a term inside a fence,
#     blockquote, or inline code span is a citation, and one in running prose may
#     be declared with `praxis:allow-term` carrying a mandatory reason. Reporting
#     file:line is what makes a line-scoped marker possible at all — the previous
#     whole-file scan had no line to attach one to.
echo "validate-plugin: checking terminology..."
TERM_REPORT=$(python3 <<'PY'
import json, os, re, sys
sys.path.insert(0, 'scripts')
import citation_scan

if not os.path.isfile('.praxis-canon.json'):
    print('skipped (no .praxis-canon.json)'); sys.exit(0)
canon = json.load(open('.praxis-canon.json'))
terms = canon.get('forbiddenTerms', [])
scan_dirs = canon.get('terminologyScanDirs', [])
files = []
for d in scan_dirs:
    files += os.popen("find %s -type f \\( -name '*.md' -o -name '*.md.tmpl' \\)" % d).read().splitlines()

problems = []
for path in sorted(f for f in files if f):
    result = citation_scan.analyze(path, marker='praxis:allow-term')
    for lineno, msg in result.bad_markers:
        problems.append('%s:%d: %s' % (path, lineno, msg))
    try:
        lines = open(path, errors='replace').read().split('\n')
    except Exception:
        continue
    for lineno, line in enumerate(lines, 1):
        if lineno in result.exempt:
            continue
        scrubbed = citation_scan.strip_code_spans(line)
        for t in terms:
            m = re.search(t['pattern'], scrubbed)
            if m:
                problems.append("%s:%d: forbidden term '%s' -- %s"
                                % (path, lineno, m.group(0), t['reason']))
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
valid |= set(canon.get('specialPlaceholders', []))  # runtime tokens (e.g. TODAY), not config keys
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

# 12. Required-phrase presence — phrases that MUST appear in specific files
#     (from .praxis-canon.json's requiredPhrases). Inverse of terminology
#     drift: catches an honesty disclosure silently regressing, not a
#     forbidden term reappearing.
echo "validate-plugin: checking required phrases..."
PHRASE_REPORT=$(python3 <<'PY'
import json, os, re, sys
if not os.path.isfile('.praxis-canon.json'):
    print('skipped (no .praxis-canon.json)'); sys.exit(0)
canon = json.load(open('.praxis-canon.json'))
required = canon.get('requiredPhrases', [])
problems = []
for r in required:
    path = r['file']
    if not os.path.isfile(path):
        problems.append("%s: required phrase check failed -- file not found" % path)
        continue
    text = open(path, errors='replace').read()
    if not re.search(r['pattern'], text):
        problems.append("%s: missing required phrase '%s' -- %s" % (path, r['pattern'], r['reason']))
for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
PHRASE_RC=$?
if [[ $PHRASE_RC -ne 0 ]]; then
  echo "$PHRASE_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 13. Version single-source — package.json is the only place the plugin version
#     is authored. Every harness manifest is synced from it; no document may
#     restate it in prose. Delegates to bump-version.sh --audit, which scans for
#     ANY semver literal (not just the current one, so a *stale* claim is caught)
#     outside the declared manifests and the reasoned audit.allow list.
echo "validate-plugin: checking version single-source..."
if VERSION_REPORT=$(bash "$(dirname "$0")/bump-version.sh" --audit 2>&1); then
  echo "  ok"
else
  echo "$VERSION_REPORT" | sed -n '/UNDECLARED/,$p' >&2
  FAILED=$((FAILED + 1))
fi

# 14. Link resolution — every relative markdown link must point at a file or
#     directory that exists. The inventory-parity check (#7) only proves a name is
#     *mentioned*; it cannot prove a link *resolves*, which is how a docs
#     reorganization can silently strand references.
#
#     Citation positions come from citation_scan — the SAME module checks #4 and
#     #10 use. The fence rule was BORN here as a hand-rolled loop and was the last
#     copy of it; ADR.260725 mandates one implementation, so this check now
#     consumes the module rather than duplicating it. Exclusions, each
#     load-bearing:
#       - fenced code blocks: template content whose paths resolve from the
#         destination document, not from the file quoting it (fence tracking is
#         marker-length aware in the module, so a nested fence cannot mis-toggle);
#       - inline code spans: a link construct in backticks is prose ABOUT markdown,
#         not a link. Passing the link regex as citation_scan's `pattern` makes
#         this column-precise: a line is exempt only when EVERY link on it sits
#         inside a span, and the scrub below re-checks the live links on a mixed
#         line. Documenting the link checker no longer breaks the link checker;
#       - `praxis:allow-path` declared exemption, with its mandatory reason;
#       - paths containing < or >: `<placeholder>` template segments;
#       - external schemes: http(s) and mailto are not this check's business.
#
#     Blockquote is deliberately NOT taken from the module. For a *literal* a
#     blockquote is a citation, which is why #4 and #10 honour it — but a link
#     inside a blockquote renders live and breaks like any other, and 18 real
#     links in this repo live on blockquote lines. Honouring it here would have
#     dropped them from coverage silently. Same module, different citation
#     positions per literal kind — the asymmetry is the point.
#
#     Bad `praxis:allow-path` markers are reported by #4, which walks the same
#     file set; re-reporting them here would only duplicate the diagnostic.
echo "validate-plugin: checking link resolution..."
LINK_REPORT=$(python3 <<'PY'
import os, re, sys
sys.path.insert(0, 'scripts')
import citation_scan

SKIP_DIRS = {'.git', 'node_modules', '.claude'}
# One regex, used both as the citation_scan `pattern` (for code-span precision)
# and as the matcher below, so the two can never disagree about what a link is.
LINK_PATTERN = r'\[[^\]]*\]\(([^)\s]+)'
link_re = re.compile(LINK_PATTERN)

problems = []
for dirpath, dirnames, filenames in os.walk('.'):
    dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
    for fn in filenames:
        if not fn.endswith('.md'):
            continue
        path = os.path.join(dirpath, fn)
        try:
            lines = open(path, errors='replace').read().split('\n')
        except Exception as e:
            problems.append('%s: cannot read (%s)' % (path, e))
            continue
        result = citation_scan.analyze(path, marker='praxis:allow-path',
                                       pattern=LINK_PATTERN)
        fenced = citation_scan.fenced_lines(lines)
        declared, _bad = citation_scan.marker_lines(lines, 'praxis:allow-path')
        exempt = set()
        for lineno in result.exempt:
            line = lines[lineno - 1] if 0 < lineno <= len(lines) else ''
            # Drop a line exempted *only* because it is a blockquote; a fence,
            # marker, or all-links-in-code-spans exemption still stands.
            if (citation_scan.BLOCKQUOTE_RE.match(line)
                    and lineno not in fenced and lineno not in declared):
                continue
            exempt.add(lineno)
        for lineno, line in enumerate(lines, 1):
            if lineno in exempt:
                continue
            # Blank inline code spans so a quoted link construct on a line that
            # also carries a live link is not matched, while the live one is.
            scrubbed = citation_scan.strip_code_spans(line)
            for lm in link_re.finditer(scrubbed):
                target = lm.group(1).split('#')[0]
                if not target:
                    continue
                if target.startswith(('http://', 'https://', 'mailto:')):
                    continue
                if '<' in target or '>' in target:
                    continue
                resolved = os.path.normpath(os.path.join(os.path.dirname(path), target))
                if not os.path.exists(resolved):
                    problems.append('%s:%d: unresolved link -> %s' % (path, lineno, target))

for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
LINK_RC=$?
if [[ $LINK_RC -ne 0 ]]; then
  echo "$LINK_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 15. CHANGELOG structure — a lost `## [0.4.0]` heading once went undetected,
#     silently re-parenting a whole release's entries under the heading above it.
#     Verifies the headings parse, are unique, descend strictly by version, that
#     [Unreleased] (if present) leads, and that the version package.json declares
#     actually has an entry.
echo "validate-plugin: checking CHANGELOG structure..."
CHANGELOG_REPORT=$(python3 <<'PY'
import json, os, re, sys

if not os.path.isfile('CHANGELOG.md'):
    print('skipped (no CHANGELOG.md)'); sys.exit(0)

heading_re = re.compile(r'^##\s+(.*)$')
released_re = re.compile(r'^\[(\d+)\.(\d+)\.(\d+)\]\s+—\s+\d{4}-\d{2}-\d{2}$')

problems = []
order = []          # (lineno, (major, minor, patch), raw)
unreleased_at = None
seen = {}

for lineno, line in enumerate(open('CHANGELOG.md', errors='replace').read().split('\n'), 1):
    m = heading_re.match(line)
    if not m:
        continue
    body = m.group(1).strip()
    if body == '[Unreleased]':
        if unreleased_at is not None:
            problems.append('CHANGELOG.md:%d: duplicate [Unreleased] heading' % lineno)
        unreleased_at = lineno
        order.append((lineno, None, body))
        continue
    rm = released_re.match(body)
    if not rm:
        problems.append(
            'CHANGELOG.md:%d: malformed version heading %r -- expected '
            '"## [X.Y.Z] — YYYY-MM-DD" (em dash) or "## [Unreleased]"' % (lineno, body))
        continue
    ver = tuple(int(g) for g in rm.groups())
    if ver in seen:
        problems.append('CHANGELOG.md:%d: duplicate version heading for %d.%d.%d '
                        '(first seen at line %d)' % ((lineno,) + ver + (seen[ver],)))
    else:
        seen[ver] = lineno
    order.append((lineno, ver, body))

versioned = [(ln, v) for ln, v, _ in order if v is not None]
for (ln_a, v_a), (ln_b, v_b) in zip(versioned, versioned[1:]):
    if v_b >= v_a:
        problems.append(
            'CHANGELOG.md:%d: version %s does not descend from %s at line %d -- '
            'entries must be newest-first' % (ln_b, '.'.join(map(str, v_b)),
                                              '.'.join(map(str, v_a)), ln_a))

if unreleased_at is not None and order and order[0][2] != '[Unreleased]':
    problems.append('CHANGELOG.md:%d: [Unreleased] must be the first "##" heading' % unreleased_at)

try:
    declared = json.load(open('package.json'))['version']
except Exception as e:
    problems.append('package.json: cannot read version (%s)' % e)
    declared = None

if declared:
    want = tuple(int(x) for x in declared.split('.')[:3])
    if want not in seen:
        problems.append(
            'CHANGELOG.md: package.json declares version %s but no "## [%s] — <date>" '
            'entry exists -- a released version with no CHANGELOG section is the '
            'orphaned-heading defect this check exists to catch' % (declared, declared))

for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
CHANGELOG_RC=$?
if [[ $CHANGELOG_RC -ne 0 ]]; then
  echo "$CHANGELOG_REPORT" >&2
  FAILED=$((FAILED + 1))
else
  echo "  ok"
fi

# 16. Self-conformance declaration parity — every shipped `check-*.sh` must be
#     declared in .self-conformance.json as either run-against-Praxis or an
#     explicitly reasoned n/a. Before this check, a probe could ship and simply
#     never run against this repo, with nothing recording whether that was a
#     decision or an oversight. An exemption without a reason is the defect.
echo "validate-plugin: checking self-conformance declaration parity..."
SELFCONF_REPORT=$(python3 <<'PY'
import json, os, sys

MANIFEST = '.self-conformance.json'
problems = []

if not os.path.isfile(MANIFEST):
    print('%s: missing -- every shipped check-*.sh must declare whether Praxis '
          'runs it against itself' % MANIFEST)
    sys.exit(1)

try:
    gates = json.load(open(MANIFEST))['gates']
except Exception as e:
    print('%s: cannot read `gates` (%s)' % (MANIFEST, e))
    sys.exit(1)

on_disk = sorted(f for f in os.listdir('scripts')
                 if f.startswith('check-') and f.endswith('.sh'))

seen = {}
for i, gate in enumerate(gates):
    name = gate.get('script')
    if not name:
        problems.append('%s: gates[%d] has no `script` key' % (MANIFEST, i))
        continue
    if name in seen:
        problems.append('%s: duplicate entry for %s (first at gates[%d]) -- one '
                        'declaration per script, or the split is ambiguous'
                        % (MANIFEST, name, seen[name]))
    else:
        seen[name] = i
    if not os.path.isfile(os.path.join('scripts', name)):
        problems.append('%s: stale entry -- scripts/%s does not exist' % (MANIFEST, name))
    runs = gate.get('runs')
    if not isinstance(runs, bool):
        problems.append('%s: %s has no boolean `runs` key' % (MANIFEST, name))
    elif runs is False and not str(gate.get('reason', '')).strip():
        problems.append('%s: %s is declared `runs: false` with no reason -- an '
                        'unexplained exemption is the defect this check removes'
                        % (MANIFEST, name))

for name in on_disk:
    if name not in seen:
        problems.append('%s: undeclared -- scripts/%s ships but is not declared '
                        'run-against-Praxis or a reasoned n/a' % (MANIFEST, name))

for p in problems:
    print(p)
sys.exit(1 if problems else 0)
PY
)
SELFCONF_RC=$?
if [[ $SELFCONF_RC -ne 0 ]]; then
  echo "$SELFCONF_REPORT" >&2
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
