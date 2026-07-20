#!/usr/bin/env bash
# check-anti-dumping.sh
#
# Fails if any forbidden "dumping ground" file or folder names appear in the
# configured scan paths. Configuration is read from .anti-dumping.json at the
# repository root.
#
# Config schema (.anti-dumping.json):
#   {
#     "scanPaths":      ["src/**", "services/**"],   // glob roots to walk
#     "forbiddenNames": ["utils", "helpers", ...],     // case-insensitive base names
#     "allowPatterns":  ["\\.service\\.(ts|js)$"],     // regex escape-hatches
#     "exemptions":     ["src/legacy/services"]        // paths to skip entirely
#   }
#
# Framework collisions:
#   Some frameworks ship conventional names that overlap our forbidden list
#   (e.g., Angular `services/`, NestJS `*.service.ts`, MVC `controllers/`,
#   `models/`, `views/`, Rust `lib.rs`, Django `models.py`/`views.py`). Two
#   escape hatches are supported:
#     - `exemptions` for whole-tree opt-out at known framework directories.
#     - `allowPatterns` for regex escape-hatches against the BASENAME.
#
#   allowPatterns match a basename, so they exempt a framework-mandated FILE
#   while still blocking the dumping-ground DIRECTORY of the same name.
#   Documented per-ecosystem defaults (add the ones for your stack):
#     Rust           -> "^lib\\.rs$"                    (crate root; blocks lib/)
#     Django/Python  -> "^models\\.py$", "^views\\.py$"  (per-app; blocks models//views/)
#     NestJS         -> none needed: `user.service.ts` does not start with
#                       `service`, so it never matches; only a bare `services/`
#                       dir is blocked.
#   A framework whose idiom is a whole directory (Angular `services/`, ASP.NET
#   MVC `Controllers/`) uses `exemptions` for that specific path instead.
#
# Exit codes:
#   0 — no violations
#   1 — violations found
#   2 — configuration error
#
# Dependencies: bash 3.2+ (macOS default), find, grep, jq.

set -euo pipefail

CONFIG_FILE='.anti-dumping.json'
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT"

if ! command -v jq >/dev/null 2>&1; then
  echo 'error: jq is required but not installed' >&2
  exit 2
fi

if [[ ! -f "$CONFIG_FILE" ]]; then
  cat >&2 <<EOF
error: $CONFIG_FILE not found at repository root.

Create one with:

{
  "scanPaths": ["src/**"],
  "forbiddenNames": [
    "utils", "util", "helpers", "helper",
    "common", "shared", "misc", "general", "lib",
    "controllers", "services", "models", "views", "handlers"
  ],
  "allowPatterns": [],
  "exemptions": []
}
EOF
  exit 2
fi

# Parse config (portable for bash 3.2 on macOS — no `mapfile`).
SCAN_PATHS=()
while IFS= read -r line; do SCAN_PATHS+=("$line"); done < <(jq -r '.scanPaths[]'      "$CONFIG_FILE")
FORBIDDEN_NAMES=()
while IFS= read -r line; do FORBIDDEN_NAMES+=("$line"); done < <(jq -r '.forbiddenNames[]' "$CONFIG_FILE")
EXEMPTIONS=()
while IFS= read -r line; do EXEMPTIONS+=("$line"); done < <(jq -r '.exemptions[]?'    "$CONFIG_FILE")
ALLOW_PATTERNS=()
while IFS= read -r line; do ALLOW_PATTERNS+=("$line"); done < <(jq -r '.allowPatterns[]?' "$CONFIG_FILE")

if [[ ${#SCAN_PATHS[@]} -eq 0 ]]; then
  echo "error: $CONFIG_FILE has no 'scanPaths'" >&2
  exit 2
fi

if [[ ${#FORBIDDEN_NAMES[@]} -eq 0 ]]; then
  echo "error: $CONFIG_FILE has no 'forbiddenNames'" >&2
  exit 2
fi

# Build a regex of forbidden base names. Match the bare name, or the name
# with any extension (e.g., 'utils', 'utils.ts', 'utils.py', 'helpers.go').
FORBIDDEN_REGEX="^($(IFS='|'; echo "${FORBIDDEN_NAMES[*]}"))(\..+)?$"

# Build the prune args for exempted paths.
PRUNE_ARGS=()
if [[ ${#EXEMPTIONS[@]} -gt 0 ]]; then
  for ex in "${EXEMPTIONS[@]}"; do
    [[ -z "$ex" ]] && continue
    PRUNE_ARGS+=(-path "./$ex" -prune -o)
  done
fi

# Always prune universal build/dependency/VCS directories. These are never
# source code we author and are noise in every ecosystem (JS, Python, Rust,
# Go, Java). Add to .anti-dumping.json `exemptions` for project-specific
# legacy paths; this list is for tool-managed directories only.
DEFAULT_EXCLUDE_DIRS=(
  '.git'
  'node_modules'
  'dist'
  'build'
  'out'
  'target'
  'vendor'
  '.venv'
  'venv'
  '__pycache__'
  '.next'
  '.nuxt'
  '.svelte-kit'
  '.turbo'
  '.cache'
  'coverage'
)
for d in "${DEFAULT_EXCLUDE_DIRS[@]}"; do
  PRUNE_ARGS+=(-name "$d" -prune -o)
done

violations=()
total_scanned=0

for root in "${SCAN_PATHS[@]}"; do
  # Strip glob suffixes (`/**`, `/*`) — we walk recursively.
  base="${root%%/\**}"
  base="${base%/}"
  [[ -z "$base" ]] && base='.'

  if [[ ! -d "$base" ]]; then
    continue
  fi

  while IFS= read -r path; do
    total_scanned=$((total_scanned + 1))
    name="$(basename "$path")"
    name_lc="$(echo "$name" | tr '[:upper:]' '[:lower:]')"
    if [[ "$name_lc" =~ $FORBIDDEN_REGEX ]]; then
      # Honor allowPatterns regex escape-hatches before flagging.
      allowed=0
      for ap in "${ALLOW_PATTERNS[@]+"${ALLOW_PATTERNS[@]}"}"; do
        [[ -z "$ap" ]] && continue
        if [[ "$name" =~ $ap ]]; then allowed=1; break; fi
      done
      [[ $allowed -eq 0 ]] && violations+=("$path")
    fi
  done < <(
    find "./$base" \
      ${PRUNE_ARGS[@]+"${PRUNE_ARGS[@]}"} \
      \( -type f -o -type d \) -print 2>/dev/null || true
  )
done

if [[ ${#violations[@]} -eq 0 ]]; then
  echo "anti-dumping: clean (${total_scanned} entries scanned, ${#FORBIDDEN_NAMES[@]} forbidden names, ${#ALLOW_PATTERNS[@]} allow patterns, ${#EXEMPTIONS[@]} exemptions)"
  exit 0
fi

echo "anti-dumping: ${#violations[@]} violation(s) found" >&2
echo >&2
for v in "${violations[@]}"; do
  echo "  - $v" >&2
done
echo >&2
cat >&2 <<EOF
Forbidden 'dumping ground' names create catch-all files and folders that
accumulate unrelated logic. See the praxis plugin's
'capability-driven-guardrails' instruction for what to do instead:

  - Move the code into the single capability that uses it, OR
  - Extract into a narrow, named technical module (e.g., pkg/encryption/), OR
  - Duplicate it if the lifecycles of the consuming capabilities diverge.

To exempt a legacy path during a migration, add it to .anti-dumping.json
under 'exemptions' and remove the exemption when the migration completes.
EOF

exit 1
