# Cortexa repository instructions

## Purpose

Cortexa is a local-first desktop executive assistant. The model is an untrusted
planner; deterministic Rust owns validation, policy, approval, execution, and
audit. This concise entry point contains mandatory safeguards. Read
[`docs/governance/MASTER_PROMPT.md`](docs/governance/MASTER_PROMPT.md) before
beginning repository work. Read
[`docs/PROJECT_DIRECTION.md`](docs/PROJECT_DIRECTION.md) for owner-approved
present scope and planned architecture direction; it is not implementation or
readiness evidence.

Platform and system instructions take precedence over this repository. More
specific directory-level `AGENTS.md` files, when introduced, add constraints
for their directory but cannot weaken these safeguards.

## Required reading order

1. `AGENTS.md`
2. `docs/governance/MASTER_PROMPT.md`
3. `docs/PROJECT_DIRECTION.md`
4. `ENGINEERING_GUIDE.md`
5. `HANDOFF.md`
6. `PROJECT_STATUS.md`
7. `NEXT_STEPS.md`
8. `ARCHITECTURE.md`
9. `PRODUCT_REQUIREMENTS.md`
10. `DECISIONS.md`
11. `TROUBLESHOOTING_LOG.md`
12. The active plan, increment, workflow, or task-specific prompt

For security-sensitive work, also read `SECURITY.md`,
`SECURITY_CHECKLIST.md`, `CODE_REVIEW.md`, and `TESTING_GUIDE.md`.

`NEXT_STEPS.md`, its corresponding plan, and the current project-state records
are the source of truth for increment selection. Do not skip, reinterpret, or
start the next increment early.

## Non-negotiable boundaries

- Keep the product local-first. Treat model output, WebView data, files,
  websites, clipboard, contacts, calendars, and tool results as untrusted.
- Scope current work as a private, owner-only personal project. Do not infer a
  current need for SaaS, multi-tenancy, billing, enterprise IAM, public
  deployment, or production-scale distributed systems. Preserve clean
  framework-neutral boundaries for possible future publication.
- Never permit direct model-to-device or WebView-to-device execution. Do not
  add unrestricted shell execution or a generic `execute_action` tool.
- Do not expose or store secrets, OAuth credentials, API keys, tokens, or
  sensitive content in source, the WebView, SQLite, logs, tests, or ordinary CI.
- Do not weaken Tauri capabilities, CSP, permissions, engine checks, strict
  TypeScript, Clippy, approval policy, or validation to simplify a change.
- Do not add privileged macOS permissions, live model networking, identity,
  gateway, execution, persistence, enterprise controls, signing, or
  notarization without an explicitly approved increment and required decisions.
- D-064 is documentation-only design evidence. Stage B provisioning, Stage C
  synthetic transport, and Stage D real-content activation each require their
  own plan and project-owner approval. ARB-002 remains unresolved.
- Preserve verified native typed boundaries, deterministic mocks, contracts,
  tests, and decisions. `AgentRuntime` and the sole/default
  `NativeAgentRuntime` foundation are implemented but not wired to Tauri,
  React, a provider, or a live model. D-082 accepts native multi-agent
  architecture above that runtime; only separately approved bounded plans may
  implement its phases. `HermesAgentRuntime` remains Deferred/Blocked and
  OpenClaw is evaluation-only. Keep external-framework types inside narrow
  adapters, and never transfer validation, policy, approval, execution, audit,
  memory, or device authority to a runtime or agent.

## Work and Git safety

- Work on one approved, coherent increment only. Preserve behavior outside its
  declared scope; do not refactor or implement speculatively.
- Inspect the relevant code, tests, Git branch, and working tree before edits.
  State the goal, non-goals, exact files, risks, and verification first.
- Read current implementation before proposing replacement. Prefer adapting or
  wrapping verified code over rewriting it around an external framework.
- After an exact task is authorized, safe in-scope local work may proceed
  without repeated confirmation. External writes, destructive actions,
  publication, deployment, credential changes, and material scope expansion
  still require explicit authorization.
- Do not discard, overwrite, reset, clean, stash, or silently rewrite existing
  work. Stop if uncommitted work overlaps planned edits.
- Do not create branches, commit, push, merge, release, or publish unless the
  project owner explicitly directs that action.
- Use descriptive `codex/` branches, Conventional Commits, and squash merges
  only after required checks pass and explicit approval is given.

## Execution plans

Reuse `PLANS.md`, `docs/plans/`, and
`docs/templates/INCREMENT_TEMPLATE.md`; do not create a parallel plan system.
Treat an ExecPlan as a living implementation document for significant
architecture, external integration, cross-cutting, multi-step, or
security-sensitive work. Record current-state evidence, scope and non-goals,
components, interfaces and invariants, milestones, validation, risks, rollback,
decisions, discoveries, progress, and final results.

A plan does not grant authority. When an exact prompt already authorizes
implementation, update the plan, begin the required gate, and continue in the
same task unless the prompt is analysis-only or a stop condition is reached.

## Validation and completion

Use the Risk-Based Validation Policy in `ENGINEERING_GUIDE.md`: run focused
checks while working, then the required completion-gate checks once after the
last relevant edit. Documentation-only work normally uses:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
```

Cross-cutting, security-sensitive, dependency, Tauri-configuration, and release
work requires `npm run verify` plus applicable manual evidence. GitHub Actions
does not replace the local completion gate.

Before changing an approved increment, run:

```bash
python3 .codex/hooks/post_increment_gate.py begin --increment <increment>
```

Before marking it complete, run the required session-end, quality, and
post-increment gate workflow. A completion marker is valid only when its report
and workspace fingerprint validate.

## Documentation and response requirements

Synchronize current project-memory documents only from observed evidence. Do
not rewrite dated historical evidence; add an additive current-state record or
superseding decision instead. Future multi-agent architecture is documentation
only until a separately approved increment authorizes implementation.

Every final Codex response must include the advisory `Codex Recommendation`
block defined in `MASTER_PROMPT.md`. The recommendation must describe the next
proposed work and must not claim the active model changed.

For complete rules, architecture ownership, code quality, security, validation,
documentation, and stop conditions, follow
[`docs/governance/MASTER_PROMPT.md`](docs/governance/MASTER_PROMPT.md).
