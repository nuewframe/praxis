# Installing Praxis for OpenCode

## Prerequisites

- [OpenCode.ai](https://opencode.ai) installed

## Installation

Add Praxis to the `plugin` array in your `opencode.json` (global or project-level):

```json
{
  "plugin": ["praxis@git+https://github.com/nuewframe/praxis.git"]
}
```

Restart OpenCode. The plugin installs through OpenCode's plugin manager and registers all skills.

Verify by asking: *"Tell me about your praxis."*

OpenCode uses its own plugin install. If you also use Claude Code, Codex, Cursor, Gemini CLI, or another harness, install Praxis separately for each one.

## Pinning a version

```json
{
  "plugin": ["praxis@git+https://github.com/nuewframe/praxis.git#v0.1.0"]
}
```

## Usage

Use OpenCode's native `skill` tool:

```
use skill tool to list skills
use skill tool to load praxis/create-wave
```

The `using-praxis` bootstrap is injected automatically into the first user message of every session, so personas, guardrails, and the skill index are available without any extra step.

## Updating

OpenCode installs Praxis through a git-backed package spec. Some OpenCode and Bun versions pin the resolved git dependency in a lockfile or cache, so a restart may not pick up the newest commit. If updates do not appear, clear OpenCode's package cache or reinstall the plugin.

## Troubleshooting

### Plugin not loading

1. Check logs: `opencode run --print-logs "hello" 2>&1 | grep -i praxis`
2. Verify the plugin entry in your `opencode.json`.
3. Make sure you are running a recent version of OpenCode.

### Windows install issues

Some Windows OpenCode builds have upstream installer issues with git-backed plugin specs (cache paths for `git+https` URLs, Bun not finding `git.exe`). If OpenCode cannot install the plugin, try installing with system npm and pointing OpenCode at the local package:

```powershell
npm install praxis@git+https://github.com/nuewframe/praxis.git --prefix "$HOME\.config\opencode"
```

Then use the installed package path in `opencode.json`:

```json
{
  "plugin": ["~/.config/opencode/node_modules/praxis"]
}
```

### Skills not found

1. Use the `skill` tool to list what's discovered.
2. Confirm the plugin is loading (see above).
