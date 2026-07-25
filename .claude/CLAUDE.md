# Praxis

Praxis is a portable agent plugin that fuses **lean wave-based product delivery** with **Principal Engineer discipline**. This `CLAUDE.md` is the bootstrap entry for Claude Code sessions that load this repo directly (without going through the plugin marketplace).

## Bootstrap

Load the canonical skill index, personas, always-on guardrails, and skill triggers from:

[`skills/using-praxis/SKILL.md`](../skills/using-praxis/SKILL.md)

For the method, its doctrine, and governance: [`docs/project-context.md`](../docs/project-context.md).

## Precedence

The host repository's own `.github/`, `.claude/`, or workspace instructions always override anything in this plugin. Praxis sets defaults; the host repo wins.

In a monorepo, a package tier (`<pkg>/.praxis/context.md`, `<pkg>/.github/`) sits above the repository — nearest declaration wins, and a package states only what differs.
