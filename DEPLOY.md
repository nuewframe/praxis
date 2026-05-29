# Deploy / Release Guide

This document describes the end-to-end process for releasing a new version of the Praxis plugin.

---

## Prerequisites

- `git` and [`gh`](https://cli.github.com/) installed and authenticated (`gh auth status`)
- `jq` installed (used by the version-bump script)
- Push access to `nuewframe/praxis` on GitHub

---

## Release types and versioning

Praxis follows [Semantic Versioning](https://semver.org/):

| Type | When to use | Example |
|------|-------------|---------|
| **patch** (x.y.**Z**) | Bug fixes, docs, non-breaking tweaks | `0.1.2 → 0.1.3` |
| **minor** (x.**Y**.0) | New skills, new guardrails, additive features | `0.1.3 → 0.2.0` |
| **major** (**X**.0.0) | Breaking changes to skill interfaces or manifest format | `0.x.x → 1.0.0` |

---

## Step-by-step release process

### 1. Create a release branch

```bash
# patch fix
git checkout main && git pull
git checkout -b fix/<short-description>

# feature / minor
git checkout -b feat/<short-description>
```

### 2. Make and commit changes

Make code changes, then commit with a [Conventional Commits](https://www.conventionalcommits.org/) message:

```bash
git add <files>
git commit -m "fix(scope): short description"
# or
git commit -m "feat(scope): short description"
```

### 3. Bump the version

Use the provided script to update all versioned manifests atomically:

```bash
bash scripts/bump-version.sh <new-version>
# e.g. bash scripts/bump-version.sh 0.1.4
```

Files updated by the script (declared in `.version-bump.json`):

- `package.json`
- `.claude-plugin/plugin.json`
- `.claude-plugin/marketplace.json`
- `.cursor-plugin/plugin.json`
- `.codex-plugin/plugin.json`
- `gemini-extension.json`

Verify no undeclared files were missed (the script runs an audit automatically).

### 4. Update CHANGELOG.md

Add a new section under `## [Unreleased]` following [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
## [0.1.4] — YYYY-MM-DD

### Fixed
- ...

### Added
- ...

### Changed
- ...
```

Move the content to replace `## [Unreleased]` and leave a fresh empty `## [Unreleased]` at the top.

### 5. Commit the release preparation

```bash
git add .
git commit -m "chore(release): prepare v<new-version>"
```

### 6. Push branch and open a PR

```bash
git push -u origin <branch-name>

gh pr create \
  --title "<type>(<scope>): <description> (v<new-version>)" \
  --body "Describe what changed and why." \
  --base main
```

### 7. Merge the PR

```bash
gh pr merge <PR-number> --squash --delete-branch --admin
git checkout main && git pull
```

### 8. Tag the release

```bash
git tag -a v<new-version> -m "chore(release): v<new-version>"
git push origin v<new-version>
```

### 9. Create a GitHub release

```bash
gh release create v<new-version> \
  --title "v<new-version> — <short title>" \
  --notes "## What's Changed

### Fixed / Added / Changed
- ...

## Full Changelog
https://github.com/nuewframe/praxis/blob/main/CHANGELOG.md"
```

---

## Validation before releasing

Run the plugin self-test to catch frontmatter, JSON, YAML, and version-drift issues:

```bash
bash scripts/validate-plugin.sh
```

Fix any reported errors before pushing.

---

## Plugin distribution

Praxis is distributed as a Claude Code plugin via the `nuewframe-marketplace` registry. After a GitHub release is published, the marketplace picks up the new version from the `v<version>` tag automatically — no separate publish step is required.

The `package.json` is present for tooling compatibility (npm `files` list governs what ships) but Praxis is **not** currently published to the npm registry. If that changes, add `npm publish --access public` after step 9.

---

## Quick reference

```bash
# Full patch release in one sequence
NEW=0.1.4
git checkout main && git pull
git checkout -b fix/my-fix
# ... make changes ...
bash scripts/bump-version.sh $NEW
# ... update CHANGELOG.md ...
git add . && git commit -m "chore(release): prepare v$NEW"
git push -u origin fix/my-fix
gh pr create --title "fix: ... (v$NEW)" --base main
gh pr merge --squash --delete-branch --admin
git checkout main && git pull
git tag -a v$NEW -m "chore(release): v$NEW" && git push origin v$NEW
gh release create v$NEW --title "v$NEW — ..." --notes "..."
```
