#!/usr/bin/env bash
# check-design-approval-gate.sh
#
# The mechanical Design Approval gate (course-correction plan, Phase C1 —
# "a demonstrated runtime-enforcement bridge"). `create-sprint`'s Major-tier
# path requires two things to exist before implementer mode may proceed: (1)
# the referenced ADR's status line reads `**Status:** Accepted`, and (2) the
# sprint file's Design Approval section carries a genuinely signed block, not
# leftover template placeholders. Until this script existed, both were
# entirely agent-attested — nothing mechanically stopped a push if either was
# missing. This is the one gate Praxis makes fail closed today, wired into
# verify.sh and therefore into the optional pre-push git hook, with no
# orchestration runtime required.
#
# Unlike most other check-*.sh probes in this repo (which default to
# warn-first, mode-promoted via a project JSON config), this script is
# DELIBERATELY HARD-FAIL BY DESIGN with no warn mode and no opt-out config.
# It exists specifically to demonstrate genuine fail-closed enforcement of a
# gate that was previously agent-attested only. Do not add a warn mode to
# this script without re-litigating that decision.
#
# Section format this script parses — the exact contract defined in
# `skills/create-sprint/SKILL.md`'s "## Design Approval (Major-tier sprints
# only)" section. A sprint file's Design Approval section either:
#   (a) contains the literal string `n/a (tier: Trivial)` or
#       `n/a (tier: Standard)` (case-insensitive) — not Major-tier, skip; or
#   (b) contains a fenced block of the form:
#         Approver: <name or role>
#         Date: YYYY-MM-DD
#         ADR(s): ADR.<ID> (status: Accepted)
#         Notes: <one-line summary of what was approved>
#       — Major-tier, must be genuinely filled in (no template placeholders
#       left), and must reference an ADR file that actually exists and whose
#       own `**Status:**` line reads exactly `Accepted`.
#
# Sprint files are named `SPRINT.<id>-<slug>.md` (progress ledgers are the
# separate `SPRINT.<id>-<slug>.ledger.md` — excluded, not sprint files). ADR
# files are named `ADR.<id>-<slug>.md`. Paths are project-configurable
# (`paths.sprints`, `paths.adr` in praxis.config.yaml), so this script scans
# the whole tree under the given root for these filename patterns rather than
# hardcoding a path — mirroring check-sprint-id-collision.sh and
# check-seam-contract-parity.sh.
#
# Usage:
#   scripts/check-design-approval-gate.sh [scan-root]   (default: .)
#
# Compatible with bash 3.2+ (macOS default). No associative arrays.
#
# Exit codes:
#   0 — every Major-tier sprint found is genuinely signed against an Accepted
#       ADR (or no sprint files exist at all)
#   1 — at least one Major-tier sprint failed a check (placeholder text left
#       unfilled, referenced ADR missing, or referenced ADR not Accepted)
#   2 — invocation error

set -u

ROOT="${1:-.}"
ROOT="${ROOT%/}"

if [[ ! -d "$ROOT" ]]; then
  echo "check-design-approval-gate: error: not a directory: $ROOT" >&2
  exit 2
fi

# ---- Locate sprint files -----------------------------------------------------
#
# SPRINT.<id>-<slug>.md, excluding *.ledger.md progress ledgers.

set +e
SPRINT_FILES=$(find "$ROOT" -type f -name 'SPRINT.*.md' ! -name '*.ledger.md' 2>/dev/null)
FIND_RC=$?
set -e 2>/dev/null || true

if [[ $FIND_RC -ne 0 ]]; then
  echo "check-design-approval-gate: error scanning $ROOT for sprint files" >&2
  exit 2
fi

if [[ -z "$SPRINT_FILES" ]]; then
  echo "check-design-approval-gate: no sprint files under $ROOT; nothing to check"
  exit 0
fi

# ---- Helpers ------------------------------------------------------------------

# Extract the Design Approval section's body: from the heading line to the
# next '## ' heading (or EOF). Uses awk for portability (bash 3.2 has no
# reliable multi-line regex slicing).
extract_design_approval_section() {
  local file="$1"
  awk '
    /^## Design Approval \(Major-tier sprints only\)/ { found=1; next }
    found && /^## / { found=0 }
    found { print }
  ' "$file"
}

# Case-insensitive, whitespace-tolerant check for the n/a tier markers.
is_non_major() {
  local body="$1"
  printf '%s' "$body" | tr '[:upper:]' '[:lower:]' | grep -Eq 'n/a[[:space:]]*\(tier:[[:space:]]*(trivial|standard)\)'
}

TOTAL_MAJOR=0
FAILURES=''

# ---- Check each sprint file ---------------------------------------------------

while IFS= read -r f; do
  [[ -z "$f" ]] && continue
  rel="${f#"$ROOT"/}"

  SECTION=$(extract_design_approval_section "$f")

  if [[ -z "$SECTION" ]]; then
    FAILURES="${FAILURES}${rel}: no '## Design Approval (Major-tier sprints only)' section found — cannot determine tier or approval status"$'\n'
    continue
  fi

  if is_non_major "$SECTION"; then
    # Trivial or Standard tier: not subject to this gate.
    continue
  fi

  TOTAL_MAJOR=$((TOTAL_MAJOR + 1))
  SPRINT_OK=1

  # --- Extract the signed fields (first match of each, tolerant of leading
  #     whitespace inside the fenced block). ---
  APPROVER_LINE=$(printf '%s\n' "$SECTION" | grep -E '^[[:space:]]*Approver:' | head -n1)
  DATE_LINE=$(printf '%s\n' "$SECTION" | grep -E '^[[:space:]]*Date:' | head -n1)
  ADRS_LINE=$(printf '%s\n' "$SECTION" | grep -E '^[[:space:]]*ADR\(s\):' | head -n1)
  NOTES_LINE=$(printf '%s\n' "$SECTION" | grep -E '^[[:space:]]*Notes:' | head -n1)

  if [[ -z "$APPROVER_LINE" || -z "$DATE_LINE" || -z "$ADRS_LINE" || -z "$NOTES_LINE" ]]; then
    FAILURES="${FAILURES}${rel}: Major-tier sprint is missing one or more of the required Approver:/Date:/ADR(s):/Notes: lines in its Design Approval block"$'\n'
    continue
  fi

  # --- Placeholder detection ---
  if printf '%s' "$APPROVER_LINE" | grep -Fq '<name or role>'; then
    FAILURES="${FAILURES}${rel}: Design Approval 'Approver:' field is still the template placeholder '<name or role>' — not a genuine signature"$'\n'
    SPRINT_OK=0
  fi
  if printf '%s' "$DATE_LINE" | grep -Fq 'YYYY-MM-DD'; then
    FAILURES="${FAILURES}${rel}: Design Approval 'Date:' field is still the template placeholder 'YYYY-MM-DD' — not a genuine signature"$'\n'
    SPRINT_OK=0
  fi
  if printf '%s' "$ADRS_LINE" | grep -Fq 'ADR.<ID>'; then
    FAILURES="${FAILURES}${rel}: Design Approval 'ADR(s):' field is still the template placeholder 'ADR.<ID>' — not a genuine signature"$'\n'
    SPRINT_OK=0
  fi
  if printf '%s' "$NOTES_LINE" | grep -Fq '<one-line summary of what was approved>'; then
    FAILURES="${FAILURES}${rel}: Design Approval 'Notes:' field is still the template placeholder '<one-line summary of what was approved>' — not a genuine signature"$'\n'
    SPRINT_OK=0
  fi

  [[ $SPRINT_OK -eq 0 ]] && continue

  # --- Extract ADR id(s) from the ADR(s): line, pattern ADR.<id-token>. ---
  ADR_IDS=$(printf '%s\n' "$ADRS_LINE" | grep -Eo 'ADR\.[A-Za-z0-9._-]+' | sed -E 's/^ADR\.//' | sort -u)

  if [[ -z "$ADR_IDS" ]]; then
    FAILURES="${FAILURES}${rel}: could not parse any ADR.<ID> reference out of the 'ADR(s):' line: ${ADRS_LINE}"$'\n'
    continue
  fi

  while IFS= read -r adr_id; do
    [[ -z "$adr_id" ]] && continue

    ADR_MATCHES=$(find "$ROOT" -type f -name "ADR.${adr_id}-*.md" 2>/dev/null)

    if [[ -z "$ADR_MATCHES" ]]; then
      FAILURES="${FAILURES}${rel}: references ADR.${adr_id} but no matching ADR file (ADR.${adr_id}-*.md) exists in the repo"$'\n'
      continue
    fi

    # If multiple files somehow match, check each; any non-Accepted status fails.
    while IFS= read -r adr_file; do
      [[ -z "$adr_file" ]] && continue
      adr_rel="${adr_file#"$ROOT"/}"

      STATUS_LINE=$(grep -E '^\*\*Status:\*\*' "$adr_file" | head -n1)
      STATUS_VALUE=$(printf '%s' "$STATUS_LINE" | sed -E 's/^\*\*Status:\*\*[[:space:]]*//')

      if [[ -z "$STATUS_VALUE" ]]; then
        FAILURES="${FAILURES}${rel}: ADR.${adr_id} (${adr_rel}) has no machine-readable '**Status:**' line — Major-tier sprint cannot proceed"$'\n'
      elif [[ "$STATUS_VALUE" != "Accepted" ]]; then
        FAILURES="${FAILURES}${rel}: ADR.${adr_id} status is '${STATUS_VALUE}', not Accepted — Major-tier sprint cannot proceed"$'\n'
      fi
    done <<< "$ADR_MATCHES"
  done <<< "$ADR_IDS"

done <<< "$SPRINT_FILES"

# ---- Report -------------------------------------------------------------------

if [[ -z "$FAILURES" ]]; then
  N_SPRINTS=$(printf '%s\n' "$SPRINT_FILES" | grep -c .)
  echo "check-design-approval-gate: clean ($N_SPRINTS sprint file(s) scanned, $TOTAL_MAJOR Major-tier, all genuinely signed against Accepted ADRs)"
  exit 0
fi

{
  N=$(printf '%s' "$FAILURES" | grep -c .)
  echo "check-design-approval-gate: $N Design Approval violation(s) found — this gate is HARD-FAIL, not warn-first"
  echo
  printf '%s' "$FAILURES" | while IFS= read -r line; do
    [[ -n "$line" ]] && echo "  - $line"
  done
  echo
  cat <<'EOF'
Every Major-tier sprint's "## Design Approval (Major-tier sprints only)"
section must contain a genuinely signed Approver:/Date:/ADR(s):/Notes: block
(no leftover template placeholders), referencing an ADR file that exists and
whose own **Status:** line reads exactly "Accepted". This is the one gate
Praxis makes fail closed today without an orchestration runtime — there is no
warn mode and no opt-out config for it. Fix the sprint's Design Approval
section or get the referenced ADR to Accepted status before proceeding.
EOF
} >&2

exit 1
