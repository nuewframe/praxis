# Praxis — Agent Bootstrap

This repository ships the **Praxis** plugin: lean wave-based product delivery unified with Principal Engineer discipline.

If your harness auto-loads `AGENTS.md` (Codex, Aider, Sourcegraph Amp, and others), this file is your entry point. Read the bootstrap skill below before doing anything else.

## Bootstrap

- **Skill index, personas, always-on guardrails, and skill triggers:** [`skills/using-praxis/SKILL.md`](./skills/using-praxis/SKILL.md)
- **Plugin governance and scope rules:** [`project-context.md`](./project-context.md)

## Personas

- Product Manager — [`agents/product-manager.agent.md`](./agents/product-manager.agent.md)
- Product Designer — [`agents/product-designer.agent.md`](./agents/product-designer.agent.md)
- Principal Engineer — [`agents/principal-engineer.agent.md`](./agents/principal-engineer.agent.md)

## Always-on guardrails

- Lean delivery — [`instructions/lean-delivery-guardrails.instructions.md`](./instructions/lean-delivery-guardrails.instructions.md)
- Capability-driven engineering — [`instructions/capability-driven-guardrails.instructions.md`](./instructions/capability-driven-guardrails.instructions.md)
- Code contribution intake — [`instructions/code-contribution-intake.instructions.md`](./instructions/code-contribution-intake.instructions.md)

## Precedence

The host repository's own `.github/`, `.claude/`, or workspace instructions always override anything in this plugin. Praxis sets defaults; the host repo wins.
