# Cortexa master instructions

Status: Authoritative repository-wide Codex instruction detail
Last updated: 2026-07-20

## 1. Purpose

This document supplies the durable detail behind the concise root `AGENTS.md`.
It helps Codex make bounded, evidence-based changes without weakening Cortexa's
local-first trust boundaries or treating plans as implemented capability.

## 2. Applicability

Apply these instructions to every repository task unless a higher-precedence
instruction prevents it. Task-specific prompts, plans, and directory-level
instructions may add constraints but do not weaken this document.

## 3. Instruction precedence

Use this order:

1. Platform and system instructions.
2. Root `AGENTS.md`.
3. This master document.
4. Current-state records: `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `HANDOFF.md`,
   and accepted decisions in `DECISIONS.md`.
5. The approved task-specific skill, prompt, plan, or increment record.
6. More-specific directory-level `AGENTS.md` files, when present.

This is instruction precedence, not a claim that every current-state document
has equal architecture authority. Follow `ENGINEERING_GUIDE.md` for
documentation authority and preserve dated historical evidence.

## 4. Required startup procedure

Before work, read the root required reading order, inspect the branch and
working tree, reconcile the handoff with repository evidence, identify the one
approved task, inspect applicable implementation and tests, and run the
smallest useful baseline check. Do not discard, overwrite, stash, reset, or
clean user work. Stop when overlapping changes, a failed baseline, or an
unresolved architecture decision prevents safe work.

## 5. Current-increment selection

Continue only the approved increment. Do not skip increments, promote future
roadmap ideas to authorization, or start the next increment automatically.
Select work from `NEXT_STEPS.md` only when it is Ready, unless the project owner
explicitly requests another bounded task. Preserve the approved roadmap; record
new evidence without silently reprioritizing it.

## 6. Scope control

State the goal, non-goals, exact expected files, risks, verification, manual
gates, and rollback before editing. Prefer the smallest change that satisfies
the accepted criteria. Stop for approval before expanding a declared file,
capability, permission, dependency, data, or trust boundary. Do not perform
unrelated refactors or speculative implementation.

## 7. Change-planning requirements

Separate documentation work from behavior changes. Revalidate claims against
source and tests rather than documentation alone. Identify dependent behavior,
compatibility constraints, owner decisions, and whether the task is blocked.
For documentation-only work, do not imply product capability, deployment,
operational evidence, or external approval exists.

## 8. Architecture principles

Keep the product local-first. The model, provider output, WebView, files,
websites, clipboard, connected-system data, and tool results are untrusted.
Trusted Rust owns validation, deterministic policy, approval, restricted
execution, cancellation, and audit. Preserve narrow IPC, closed contracts,
portable domain boundaries, and the prohibition on direct model-to-device
execution. `ARCHITECTURE.md` distinguishes current, mocked, planned, and
prohibited behavior.

## 9. Code-quality rules

Follow existing module ownership, strict TypeScript, focused React components,
typed IPC clients, and Rust portability conventions. Add focused success,
failure, boundary, and regression tests when behavior changes. Do not use
unnecessary abstractions, `any`, broad public interfaces, or style-only rewrites
that expand risk without current value.

## 10. Error-handling rules

Use typed errors and explicit narrowing at external boundaries. Production Rust
must not use `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!` unless a
narrow documented exception already applies. Preserve cancellation, timeout,
redaction, and failure-state behavior. Never expose raw upstream errors,
credentials, or sensitive content.

## 11. Dependency rules

Prefer existing dependencies and exact pinned versions. Add a production
dependency only for a demonstrated current need, with an accepted decision and
focused verification. Do not update lockfiles incidentally, install dependencies
for documentation work, or use dependencies to bypass a security boundary.

## 12. Security and credential handling

Follow `SECURITY.md` and `SECURITY_CHECKLIST.md`. Never place credentials,
tokens, secrets, authorization codes, sensitive personal data, or raw content
in source, WebView, SQLite, logs, tests, ordinary CI, or documentation. Do not
weaken CSP, capabilities, permissions, approval binding, policy, or audit
boundaries. D-060 through D-064 remain decision/evidence gates; no live
identity, gateway, provider, or external processing path is authorized by
documentation alone.

## 13. Git safety

Inspect status, staged files, untracked files, and conflicts before edits and
before publication. Preserve user changes. Use descriptive `codex/` branches,
Conventional Commits, and PR descriptions with Purpose, Files changed, Testing
performed, Breaking changes, and Next increment. Commit, push, create or merge
PRs, release, or publish only with explicit project-owner direction. Use squash
merge after all required checks pass.

## 14. Testing and validation

Use the Risk-Based Validation Policy in `ENGINEERING_GUIDE.md` and
`TESTING_GUIDE.md`. During implementation, batch related changes and run the
smallest relevant checks. After the final relevant edit, run the required
completion-gate verification once. Documentation-only changes normally require
Markdown, links, paths, scope, and repository-policy checks; they do not require
application builds unless executable tooling or an approved plan requires them.
Cross-cutting, security-sensitive, dependency, Tauri, storage, policy,
approval, IPC, or release work requires the complete applicable validation.
Report every command and actual result, including checks not run and why.

## 15. Documentation updates

Update only the current records affected by observed evidence: `HANDOFF.md`,
`PROJECT_STATUS.md`, `NEXT_STEPS.md`, `PLANS.md`, `CHANGELOG.md`, and accepted
decisions when durable policy changes. Preserve dated plans, increments,
reviews, and troubleshooting evidence; use additive corrections or superseding
records instead of rewriting history. Keep links and path references valid.

## 16. Status and handoff requirements

Record the actual branch, worktree state, completed work, remaining work, files
changed, commands and results, blockers, risks, and an exact next prompt. Do
not call partial verification complete. Distinguish Passed, Failed, Not run, and
Manual verification pending. Current project state belongs in the root
project-memory documents, not in copied prompt boilerplate.

## 17. Future multi-agent boundaries

Future approved work may document specialized roles, a supervisor, typed agent
messages, shared workflow state, policy-controlled tools, human approvals,
verification, retry/cancellation, resumable workflows, and observability.
None is currently authorized. Do not implement or invoke product-level
multi-agent behavior, orchestration, supervisors, planners, executors,
verifiers, agent-to-agent communication, autonomous planning, recursive
delegation, subagents for application implementation, parallel agent execution,
workflow engines, event buses, queues, persistent agent tasks, databases,
background workers, provider integration, new Tauri commands, IPC, UI, cloud,
containers, distributed services, or speculative dependencies. Documentation is
not implementation authorization.

## 18. Codex model-and-effort policy

Recommend a model and effort level for the next proposed work; recommendations
are advisory and do not switch the active model.

- Documentation and routine planning: prefer **GPT-5.6 Terra**, when available.
  Use Low effort for wording or narrow status corrections and Medium effort for
  architecture documentation, prompt governance, inventories, plans, or
  multi-file requirements analysis.
- Implementation: prefer **GPT-5.6 Sol**, when available. Use Medium effort
  for localized code changes, contained bug fixes, focused tests, and bounded
  refactors. Use High effort for features, cross-module/public interfaces,
  Rust/Tauri, IPC, authentication, authorization, persistence, concurrency,
  security-sensitive work, difficult debugging, or material regression risk.
- Use Extra High or Max only for major migrations, difficult root-cause work
  after lower efforts fail, high-risk security work, complex recovery or
  concurrency behavior, or broad architectural impact. Length alone is not a
  reason.
- If the named model is unavailable, recommend the closest available model and
  state that it is a fallback. Do not recommend Ultra while multi-agent or
  subagent execution is outside the approved scope.

## 19. Required completion report

Every final response separates completed work, files changed, validation,
unchanged behavior, deferred work, risks, unresolved issues, current status,
and the recommended next action. Append exactly:

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Codex Recommendation

Work category:
Recommended model:
Recommended effort:
Recommendation applies to:
Why:
Scope warning:
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

`Recommendation applies to` must be one of: `continue in the current thread`,
`switch before the next implementation run`, `start a separate documentation
run`, `start a separate review run`, or `no additional run is currently
required`.

## 20. Stop conditions

Stop and request direction when a required owner decision, baseline, scope,
security boundary, manual gate, operational evidence, or external authority is
missing. Do not fabricate success, accept hidden risk, create deployment
resources, or begin a later increment. Before completion, inspect the complete
diff and use the required session-end, quality, and post-increment workflows.
