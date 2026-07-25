#!/usr/bin/env bash
#
# bump-version.sh — single-source the plugin version.
#
# package.json is the source of truth. Every file listed in .version-bump.json
# "files" is synced from it mechanically; no other file may state the version.
#
# Usage:
#   bump-version.sh <new-version>   Sync all declared files to new version
#   bump-version.sh --check         Report declared versions (detect drift)
#   bump-version.sh --audit         Fail on any undeclared version literal
#
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG="$REPO_ROOT/.version-bump.json"

if [[ ! -f "$CONFIG" ]]; then
  echo "error: .version-bump.json not found at $CONFIG" >&2
  exit 1
fi

# --- helpers ---

# Read a dotted field path from a JSON file.
# Handles both simple ("version") and nested ("plugins.0.version") paths.
read_json_field() {
  local file="$1" field="$2"
  # Convert dot-path to jq path: "plugins.0.version" -> .plugins[0].version
  local jq_path
  jq_path=$(echo "$field" | sed -E 's/\.([0-9]+)/[\1]/g' | sed 's/^/./' | sed 's/\.\././g')
  jq -r "$jq_path" "$file"
}

# Write a dotted field path in a JSON file, preserving formatting.
write_json_field() {
  local file="$1" field="$2" value="$3"
  local jq_path
  jq_path=$(echo "$field" | sed -E 's/\.([0-9]+)/[\1]/g' | sed 's/^/./' | sed 's/\.\././g')
  local tmp="${file}.tmp"
  jq "$jq_path = \"$value\"" "$file" > "$tmp" && mv "$tmp" "$file"
}

# Read the list of declared files from config.
# Outputs lines of "path<TAB>field"
declared_files() {
  jq -r '.files[] | "\(.path)\t\(.field)"' "$CONFIG"
}

# Read the audit exclude patterns from config.
audit_excludes() {
  jq -r '.audit.exclude[]' "$CONFIG" 2>/dev/null
}

# Paths scanned for stray version literals.
audit_scan_paths() {
  jq -r '.audit.scan[]' "$CONFIG" 2>/dev/null
}

# --- commands ---

cmd_check() {
  local has_drift=0
  local versions=()

  echo "Version check:"
  echo ""

  while IFS=$'\t' read -r path field; do
    local fullpath="$REPO_ROOT/$path"
    if [[ ! -f "$fullpath" ]]; then
      printf "  %-45s  MISSING\n" "$path ($field)"
      has_drift=1
      continue
    fi
    local ver
    ver=$(read_json_field "$fullpath" "$field")
    printf "  %-45s  %s\n" "$path ($field)" "$ver"
    versions+=("$ver")
  done < <(declared_files)

  echo ""

  # Check if all versions match
  local unique
  unique=$(printf '%s\n' "${versions[@]}" | sort -u | wc -l | tr -d ' ')
  if [[ "$unique" -gt 1 ]]; then
    echo "DRIFT DETECTED — versions are not in sync:"
    printf '%s\n' "${versions[@]}" | sort | uniq -c | sort -rn | while read -r count ver; do
      echo "  $ver ($count files)"
    done
    has_drift=1
  else
    echo "All declared files are in sync at ${versions[0]}"
  fi

  return $has_drift
}

# Cache of exempt line numbers for the file most recently asked about, so a file
# with many matches costs one python invocation rather than one per match.
EXEMPT_CACHE=""
EXEMPT_CACHE_PATH=""
BAD_MARKERS=""

exempt_lines_for() {
  local path="$1"
  [[ "$path" == "$EXEMPT_CACHE_PATH" ]] && return 0
  EXEMPT_CACHE_PATH="$path"
  EXEMPT_CACHE=""
  local out
  out=$(cd "$REPO_ROOT" && python3 scripts/citation_scan.py "$path" \
          --marker praxis:allow-version-literal \
          --pattern '[0-9]+\.[0-9]+\.[0-9]+' 2>/dev/null || true)
  local kind n rest
  while read -r kind n rest; do
    case "$kind" in
      EXEMPT) EXEMPT_CACHE="$EXEMPT_CACHE $n" ;;
      BADMARKER) BAD_MARKERS="${BAD_MARKERS}  $path:$n: $rest"$'\n' ;;
    esac
  done <<< "$out"
}

cmd_audit() {
  # First run check (declared manifests in sync?)
  cmd_check || true
  echo ""

  # Scan for ANY semver literal in the declared scan paths. This deliberately
  # does NOT search for the *current* version: a stale claim (e.g. a doc still
  # saying 0.3.0 after the repo moved to 0.4.0) is exactly the drift that
  # searching for the current string can never find.
  #
  # package.json is the single source of truth. No file outside .version-bump.json's
  # "files" list may state the plugin's version. Legitimate semver literals
  # (instructional examples, host-repo template content) must be declared in
  # audit.allow with a reason — a reviewed exception, never silence.

  local -a scan_paths=() declared_paths=()
  while IFS= read -r p; do [[ -n "$p" ]] && scan_paths+=("$p"); done < <(audit_scan_paths)
  while IFS=$'\t' read -r path _field; do declared_paths+=("$path"); done < <(declared_files)

  if [[ ${#scan_paths[@]} -eq 0 ]]; then
    echo "audit: no scan paths configured in .version-bump.json (audit.scan)" >&2
    return 1
  fi

  echo "Audit: scanning for undeclared version literals in: ${scan_paths[*]}"
  echo ""

  local found=0
  local semver='[0-9]+\.[0-9]+\.[0-9]+'

  while IFS= read -r match; do
    [[ -z "$match" ]] && continue
    local rel_path="${match%%:*}"
    rel_path="${rel_path#./}"
    local rest="${match#*:}"          # "<lineno>:<content>"
    local content="${rest#*:}"

    # Skip files whose version IS mechanically managed.
    local skip=0
    for dp in "${declared_paths[@]}"; do
      [[ "$rel_path" == "$dp" ]] && skip=1 && break
    done
    local lineno="${rest%%:*}"
    if [[ "$skip" -eq 0 ]]; then
      # Declared exceptions (ADR.260725): a literal inside a fence, blockquote,
      # or inline code span is a citation by position; a citation in running
      # prose carries an inline marker with a mandatory reason. Both layers come
      # from scripts/citation_scan.py — one implementation, so Layer 1 cannot
      # diverge from check #14's fence rule.
      exempt_lines_for "$rel_path"
      case " $EXEMPT_CACHE " in *" $lineno "*) skip=1 ;; esac
    fi
    [[ "$skip" -eq 1 ]] && continue

    if [[ "$found" -eq 0 ]]; then
      echo "UNDECLARED version literals found:"
      found=1
    fi
    echo "  $match"
  done < <(cd "$REPO_ROOT" && grep -rnE --include='*.md' --include='*.json' --include='*.yaml' \
             --exclude-dir=node_modules --exclude-dir=.git \
             "$semver" "${scan_paths[@]}" 2>/dev/null || true)

  # An exemption that explains nothing is itself the defect (ADR.260725).
  if [[ -n "$BAD_MARKERS" ]]; then
    echo ""
    echo "UNEXPLAINED exemption markers found:"
    printf '%s' "$BAD_MARKERS"
    found=1
  fi

  if [[ "$found" -eq 1 ]]; then
    echo ""
    if [[ "$AUDIT_REPORT_ONLY" -eq 1 ]]; then
      echo "(--report-only: the rules above are NOT authoritative; exit code forced to 0)"
      return 0
    fi
    echo "package.json is the single source of truth for the plugin version."
    echo "Fix by one of:"
    echo "  1. Remove the literal — prose rarely needs to state a version at all."
    echo "  2. Add the file to .version-bump.json \"files\" so it is synced mechanically."
    echo "  3. Put the literal inside a fenced block, blockquote, or inline code span"
    echo "     if it is a citation — those are recognised structurally, no config needed."
    echo "  4. If it must sit in running prose, declare it on the line above:"
    echo "     <!-- praxis:allow-version-literal reason=\"why this cites rather than claims\" -->"
    echo "     The reason is mandatory; an empty one fails."
    return 1
  fi

  if [[ "$AUDIT_REPORT_ONLY" -eq 1 ]]; then
    echo "(--report-only) No undeclared version literals."
    return 0
  fi
  echo "No undeclared version literals. package.json remains the single source."
  return 0
}

cmd_bump() {
  local new_version="$1"

  # Validate semver-ish format
  if ! echo "$new_version" | grep -qE '^[0-9]+\.[0-9]+\.[0-9]+'; then
    echo "error: '$new_version' doesn't look like a version (expected X.Y.Z)" >&2
    exit 1
  fi

  echo "Bumping all declared files to $new_version..."
  echo ""

  while IFS=$'\t' read -r path field; do
    local fullpath="$REPO_ROOT/$path"
    if [[ ! -f "$fullpath" ]]; then
      echo "  SKIP (missing): $path"
      continue
    fi
    local old_ver
    old_ver=$(read_json_field "$fullpath" "$field")
    write_json_field "$fullpath" "$field" "$new_version"
    printf "  %-45s  %s -> %s\n" "$path ($field)" "$old_ver" "$new_version"
  done < <(declared_files)

  echo ""
  echo "Done. Running audit to check for missed files..."
  echo ""
  cmd_audit
}

# --- main ---

AUDIT_REPORT_ONLY=0

case "${1:-}" in
  --check)
    cmd_check
    ;;
  --audit)
    # --audit [--report-only] [--rules old|new]
    shift || true
    while [[ $# -gt 0 ]]; do
      case "$1" in
        --report-only) AUDIT_REPORT_ONLY=1 ;;
        *) echo "error: unknown --audit option '$1'" >&2; exit 1 ;;
      esac
      shift
    done
    cmd_audit
    ;;
  --help|-h|"")
    echo "Usage: bump-version.sh <new-version> | --check | --audit [--report-only]"
    echo ""
    echo "  <new-version>  Bump all declared files to the given version"
    echo "  --check        Show current versions, detect drift"
    echo "  --audit        Check + scan repo for undeclared version references"
    echo "  --report-only  Report what would fail, always exit 0"
    exit 0
    ;;
  --*)
    echo "error: unknown flag '$1'" >&2
    exit 1
    ;;
  *)
    cmd_bump "$1"
    ;;
esac
