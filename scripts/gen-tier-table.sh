#!/usr/bin/env bash
# gen-tier-table.sh
#
# Generates (or checks) the Trivial/Standard/Major tier-classification table
# across its three hand-restated surfaces from one JSON source of truth
# (scripts/data/tier-classification.json), so the three renderings of the same
# fact cannot silently drift from each other:
#   - skills/intake-code-contribution/SKILL.md  Step 0 (full 4-column table)
#   - skills/start-thin-slice/SKILL.md          Step 5 (2-column routing table)
#   - agents/principal-engineer.agent.md        (terse bullet list)
#
# Each target file wraps its generated block in:
#   <!-- BEGIN GENERATED: tier-table (source: ...; regenerate with ...) -->
#   ...
#   <!-- END GENERATED -->
# Edit scripts/data/tier-classification.json, not the generated blocks — they
# are overwritten on the next --write and checked in CI via --check.
#
# Usage:
#   gen-tier-table.sh           # print all three rendered blocks to stdout
#   gen-tier-table.sh --write   # replace the generated block in each of the 3 files
#   gen-tier-table.sh --check   # exit 1 if any of the 3 files' generated block is stale
#
# bash 3.2+; requires python3.

set -u
ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT"

MODE="${1:-}"

python3 - "$MODE" <<'PY'
import sys, json, difflib

MODE = sys.argv[1] if len(sys.argv) > 1 else ""

SOURCE = "scripts/data/tier-classification.json"
data = json.load(open(SOURCE))
tiers = data["tiers"]
by_name = {t["name"]: t for t in tiers}
ORDER = ["Trivial", "Standard", "Major"]

BEGIN = ("<!-- BEGIN GENERATED: tier-table (source: scripts/data/tier-classification.json; "
         "regenerate with scripts/gen-tier-table.sh --write) -->")
END = "<!-- END GENERATED -->"


def pad_cell(text, width):
    # width includes the leading + trailing space.
    return " " + text.ljust(width - 2) + " "


def render_intake_table():
    """Surface 1: skills/intake-code-contribution/SKILL.md Step 0.
    Full 4-column table; each column padded to the true max content width
    across header + data rows (standard markdown table alignment)."""
    header_labels = ["Tier", "Use when", "Steps required", "Phased workflow path"]
    rows = []
    for name in ORDER:
        t = by_name[name]
        rows.append([f"**{name}**", t["criteria"], t["steps_required"], t["phased_workflow_path"]])
    ncols = 4
    widths = []
    for c in range(ncols):
        col_values = [header_labels[c]] + [r[c] for r in rows]
        widths.append(max(len(v) for v in col_values) + 2)
    lines = []
    lines.append("|" + "|".join(pad_cell(header_labels[c], widths[c]) for c in range(ncols)) + "|")
    lines.append("|" + "|".join(pad_cell("-" * (widths[c] - 2), widths[c]) for c in range(ncols)) + "|")
    for r in rows:
        lines.append("|" + "|".join(pad_cell(r[c], widths[c]) for c in range(ncols)) + "|")
    return "\n".join(lines)


def render_routing_table():
    """Surface 2: skills/start-thin-slice/SKILL.md Step 5.
    2-column routing table; this file's table was never column-padded, so
    each cell is rendered as single-space-wrapped content only (no width
    alignment), matching its existing convention exactly."""
    header_labels = ["Tier", "Route"]
    lines = []
    lines.append("| " + " | ".join(header_labels) + " |")
    lines.append("| " + " | ".join("-" * len(h) for h in header_labels) + " |")
    for name in ORDER:
        t = by_name[name]
        lines.append(f"| **{name}** | {t['routing_detailed']} |")
    return "\n".join(lines)


def render_agent_bullets():
    """Surface 3: agents/principal-engineer.agent.md terse bullet list."""
    lines = []
    for name in ORDER:
        t = by_name[name]
        lines.append(f"- **{name}:** {t['agent_bullet']}")
    return "\n".join(lines)


TARGETS = [
    ("skills/intake-code-contribution/SKILL.md", render_intake_table),
    ("skills/start-thin-slice/SKILL.md", render_routing_table),
    ("agents/principal-engineer.agent.md", render_agent_bullets),
]


def extract_current(path):
    text = open(path).read()
    i = text.find(BEGIN)
    j = text.find(END)
    if i < 0 or j < 0:
        return None, text
    inner_start = i + len(BEGIN) + 1  # skip the newline right after BEGIN
    inner = text[inner_start:j].rstrip("\n")
    return inner, text


def replace_block(path, rendered):
    text = open(path).read()
    i = text.find(BEGIN)
    j = text.find(END)
    if i < 0 or j < 0:
        print(f"gen-tier-table: markers not found in {path}", file=sys.stderr)
        sys.exit(2)
    new_text = text[:i] + BEGIN + "\n" + rendered + "\n" + text[j:]
    open(path, "w").write(new_text)


if MODE == "--write":
    for path, fn in TARGETS:
        replace_block(path, fn())
        print(f"wrote {path}")
elif MODE == "--check":
    stale = []
    for path, fn in TARGETS:
        current, _ = extract_current(path)
        rendered = fn()
        if current is None:
            stale.append((path, None, rendered))
        elif current != rendered:
            stale.append((path, current, rendered))
    if stale:
        for path, current, rendered in stale:
            print(f"gen-tier-table: {path} is stale; run 'scripts/gen-tier-table.sh --write'",
                  file=sys.stderr)
            if current is not None:
                diff = difflib.unified_diff(current.splitlines(), rendered.splitlines(), lineterm="")
                for line in diff:
                    print(line, file=sys.stderr)
            else:
                print(f"  {path}: BEGIN/END markers not found", file=sys.stderr)
        sys.exit(1)
    else:
        print("gen-tier-table: all 3 surfaces are current")
        sys.exit(0)
else:
    for path, fn in TARGETS:
        print(f"=== {path} ===")
        print(fn())
        print()
PY
