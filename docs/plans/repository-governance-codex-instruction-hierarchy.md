# Repository governance - Codex instruction hierarchy

Status: Verified complete with advisories
Owner: Project owner
Date: 2026-07-20
Gate ID: `repository-governance-codex-instruction-hierarchy`
Baseline: clean synchronized `main` at `36ce9ab`

## Goal

Create a durable Codex instruction hierarchy with a concise root entry point,
one detailed master instruction document, preserved reusable prompts, and an
advisory model-and-effort policy. Correct only the live ARB-002A publication
wording needed to keep current state accurate.

## Scope

Create:

- `docs/governance/MASTER_PROMPT.md`
- `docs/plans/repository-governance-codex-instruction-hierarchy.md`
- `docs/increments/repository-governance-codex-instruction-hierarchy.md`
- `docs/reviews/2026-07-20-repository-governance-codex-instruction-hierarchy-post-increment-review.md`

Modify only the concise root instructions, detailed engineering/Codex usage
guidance, the existing prompt index and twenty-three prompt/template assets,
live project-memory records, `ROADMAP.md`, `docs/plans/README.md`, and the
new D-065 decision record.

The exact approved inventory is the four created files above plus these
thirty-five modified files:

`AGENTS.md`, `ASSISTANT_USAGE.md`, `CHANGELOG.md`, `DECISIONS.md`,
`ENGINEERING_GUIDE.md`, `HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`,
`PROJECT_STATUS.md`, `ROADMAP.md`, `docs/plans/README.md`, `prompts/README.md`,
the six files under `prompts/increments/`, the eight files under
`prompts/reviews/`, the three files under `prompts/templates/`, and the six
files under `prompts/workflows/`.

## Non-goals

- No application source, tests, dependencies, lockfiles, build/runtime/Tauri
  configuration, IPC, UI, database, deployment, CI, hook, skill, secret, or
  production behavior change.
- No prompt deletion, rename, relocation, historical evidence rewrite, model
  switching automation, or multi-agent/subagent implementation.
- No change to the approved product roadmap or authorization of Stage B, Stage
  C, Stage D, ARB-002 runtime work, or another increment.

## Risks and controls

- A concise root file could omit a critical safety rule: retain essential
  local-first, credential, destructive-Git, scope, and completion safeguards in
  `AGENTS.md`.
- A master document could duplicate drifting policy: reference authoritative
  project documents rather than reproduce them.
- Prompt changes could weaken a task-specific boundary: add only a concise
  governing preamble and retain goals, inputs, validation, and stop conditions.
- Current-state repair could rewrite history: correct only live status wording
  and preserve dated plans, reports, and handoff evidence.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
test "$(find prompts -type f -name '*.md' | wc -l | tr -d ' ')" = "24"
test "$(rg -l 'Follow the root AGENTS.md and docs/governance/MASTER_PROMPT.md' prompts | wc -l | tr -d ' ')" = "23"
test "$(wc -l < AGENTS.md | tr -d ' ')" -le 160
git diff --diff-filter=D -- prompts
git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Rollback

Before publication, restore only this declared documentation scope. After
publication, revert the one bounded documentation commit and supersede D-065
additively if policy changes. No runtime, deployment, credential, data, or
product rollback applies.
