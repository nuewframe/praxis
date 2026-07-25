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

# Declared-legitimate semver locations, as "path<TAB>lineRegex".
# An empty lineRegex allows the whole path; a non-empty one allows ONLY lines
# matching it, so a file may host policy examples and still be guarded against
# stale version claims.
audit_allow_entries() {
  jq -r '.audit.allow[] | "\(.path)\t\(.lines // "")"' "$CONFIG" 2>/dev/null
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

  local -a scan_paths=() allow_entries=() declared_paths=()
  while IFS= read -r p; do [[ -n "$p" ]] && scan_paths+=("$p"); done < <(audit_scan_paths)
  while IFS= read -r e; do [[ -n "$e" ]] && allow_entries+=("$e"); done < <(audit_allow_entries)
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
    # Skip declared-legitimate locations. A path with no lineRegex is allowed
    # wholesale; a path WITH one allows only matching lines, so any other semver
    # literal in that same file is still reported.
    local ap alines
    for entry in "${allow_entries[@]}"; do
      IFS=$'\t' read -r ap alines <<< "$entry"
      if [[ "$rel_path" == "$ap" || "$rel_path" == "$ap"/* ]]; then
        if [[ -z "$alines" ]]; then skip=1; break; fi
        if [[ "$content" =~ $alines ]]; then skip=1; break; fi
      fi
    done
    [[ "$skip" -eq 1 ]] && continue

    if [[ "$found" -eq 0 ]]; then
      echo "UNDECLARED version literals found:"
      found=1
    fi
    echo "  $match"
  done < <(cd "$REPO_ROOT" && grep -rnE --include='*.md' --include='*.json' --include='*.yaml' \
             --exclude-dir=node_modules --exclude-dir=.git \
             "$semver" "${scan_paths[@]}" 2>/dev/null || true)

  if [[ "$found" -eq 1 ]]; then
    echo ""
    echo "package.json is the single source of truth for the plugin version."
    echo "Fix by one of:"
    echo "  1. Remove the literal — prose rarely needs to state a version at all."
    echo "  2. Add the file to .version-bump.json \"files\" so it is synced mechanically."
    echo "  3. Add it to .version-bump.json \"audit.allow\" WITH a reason, if the literal"
    echo "     is an instructional example or host-repo template content. Prefer a narrow"
    echo "     \"lines\" regex over allowing a whole file — a file that legitimately hosts"
    echo "     examples can still develop a stale version claim."
    return 1
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

case "${1:-}" in
  --check)
    cmd_check
    ;;
  --audit)
    cmd_audit
    ;;
  --help|-h|"")
    echo "Usage: bump-version.sh <new-version> | --check | --audit"
    echo ""
    echo "  <new-version>  Bump all declared files to the given version"
    echo "  --check        Show current versions, detect drift"
    echo "  --audit        Check + scan repo for undeclared version references"
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
