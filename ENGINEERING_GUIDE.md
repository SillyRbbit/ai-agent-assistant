# Cortexa engineering guide

Status: Authoritative engineering operating model
Last updated: 2026-07-15

## Mission

Cortexa is a local-first desktop executive assistant. The model is an untrusted
planner; deterministic Rust code owns validation, policy, approval, execution,
and audit. Engineering work must improve the product without weakening that
boundary or representing planned capability as implemented.

This guide defines how the repository is engineered. `AGENTS.md` contains the
short mandatory rules for coding assistants. This guide provides the complete
operating model for maintainers, contributors, and assistants.

## Documentation authority

When documents differ, use this order:

1. Accepted decisions in `DECISIONS.md` for durable architecture and policy.
2. `SECURITY.md` and `SECURITY_CHECKLIST.md` for security invariants.
3. `PRODUCT_REQUIREMENTS.md` for normalized product requirements.
4. `ARCHITECTURE.md` for the current implementation and approved target
   boundaries.
5. `ROADMAP.md`, `PROJECT_STATUS.md`, and `NEXT_STEPS.md` for current milestone,
   capability, and queue state.
6. The approved active plan and increment record for the bounded task.
7. `HANDOFF.md` for the current workspace and exact continuation instructions.

`docs/product/PRODUCT_BRIEF.md` preserves the owner-supplied inception brief.
`docs/product/ARCHITECTURE_BASELINE.md` preserves the target architecture
baseline. Completed plans, increment records, reviews, changelog entries, and
troubleshooting entries are historical evidence and may describe the state at
their recording time. Do not rewrite that evidence to conceal later changes;
add a superseding decision or current-state update instead.

## Repository structure

| Path               | Responsibility                                                                                 |
| ------------------ | ---------------------------------------------------------------------------------------------- |
| `src/`             | React presentation, volatile application state, and typed Tauri clients                        |
| `src-tauri/src/`   | Trusted Rust core, Tauri startup, storage, menu lifecycle, and transport-free agent boundaries |
| `src-tauri/tests/` | Public Rust integration and contract tests                                                     |
| `.github/`         | Review ownership, issue/PR templates, dependency proposals, and read-only quality workflows    |
| `.codex/hooks/`    | Shared safe repository inspection, completion guardrails, and tests; not a product boundary    |
| `.agents/skills/`  | Repository-scoped assistant procedures                                                         |
| `scripts/`         | Deterministic local verification and repository-health tooling                                 |
| `assets/branding/` | Canonical Cortexa identity assets                                                              |
| `docs/branding/`   | Brand and presentation standards                                                               |
| `docs/product/`    | Inception requirements and target architecture sources                                         |
| `docs/plans/`      | Proposed, ready, active, and completed bounded plans                                           |
| `docs/increments/` | Increment completion and checkpoint records                                                    |
| `docs/reviews/`    | Post-increment machine-readable and human-readable reviews                                     |
| `docs/github/`     | Licensing status, label taxonomy, and milestone mapping                                        |
| `docs/workflows/`  | Session and troubleshooting runbooks                                                           |
| `prompts/`         | Categorized copy-paste increment, review, workflow, and authoring-template prompts             |

Compatibility identifiers retain the historical `ai-agent-assistant` name.
Human-facing product text uses `Cortexa`; see D-026.

## Architecture principles

- Local-first by default. External processing must be explicit and minimized.
- The model, WebView, gateway, external content, and tool results are untrusted.
- Only narrow typed IPC commands may enter trusted Rust.
- Validation, policy, approval, execution, and audit are separate transitions.
- No proposal, policy result, approval presentation, approval resolution, or
  audit receipt is execution authority.
- Fail closed on unknown tools, fields, states, identities, or evidence.
- Keep portable domain behavior outside operating-system adapters.
- Add capability in independently testable increments with conservative limits.
- Label documentation as current, mocked, planned, or prohibited.
- Prefer deleting misleading unused abstractions over preserving speculative
  architecture.

See `ARCHITECTURE.md` for the implemented module map and current-versus-future
boundaries.

## General coding standards

- Read surrounding code and tests before changing behavior.
- Make the smallest coherent change that satisfies approved acceptance criteria.
- Preserve strict compiler, linter, formatter, CSP, capability, and engine
  settings.
- Use domain types and structured parsers instead of ad hoc string protocols.
- Keep APIs narrow and make invalid states difficult to represent.
- Avoid unrelated refactors, generated output, and metadata churn.
- Keep GitHub workflows read-only, secret-free, digest-pinned, and incapable of
  committing, pushing, publishing, deploying, signing, or auto-merging.
- Add comments only when they explain a non-obvious invariant or boundary.
- Never claim a command passed unless its actual output was observed.

## Rust standards

- Keep `unsafe_code = "forbid"` unless a separately accepted decision defines a
  reviewed boundary.
- Production code must not use `unwrap`, `expect`, `panic!`, `todo!`,
  `unimplemented!`, or `dbg!`.
- Return closed typed errors with useful context and no secrets or raw personal
  content.
- Validate data at IPC, gateway, schema, policy, approval, storage, and adapter
  boundaries.
- Consume values when a transition must be one-time; reject replay explicitly.
- Keep deterministic policy independent of model explanations.
- Keep platform-specific code behind target-gated adapters.
- Use transactions and prepared values for SQLite writes. Enable foreign keys
  on every connection and preserve migration checksums and ordering.
- Add focused unit tests beside private implementation and public contract tests
  under `src-tauri/tests/`.

## TypeScript and React standards

- Preserve all strict TypeScript options and ESLint's zero-warning policy.
- Do not use `any`. Narrow external, event, and IPC values from `unknown`.
- Isolate typed Tauri clients under `src/infrastructure/tauri/`.
- Keep authorization and operating-system execution out of the WebView.
- Keep components focused on presentation; place state transitions in
  application modules and reducers.
- Model loading, success, failure, retry, cancellation, and empty states where
  they apply.
- Maintain accessible names, focus behavior, semantic controls, and keyboard
  interaction.
- Test user-visible behavior and meaningful state transitions rather than
  implementation details.

## Error handling

- Errors must be typed at production boundaries and closed at trust boundaries.
- Preserve actionable local context without forwarding raw provider responses,
  headers, prompts, arguments, stack traces, credentials, or personal content.
- Distinguish validation, policy denial, approval rejection, cancellation,
  expiry, limits, storage failure, and transport failure.
- Cancellation must be idempotent and terminal. Late events or outcomes fail.
- Do not silently retry mutations after an unknown result.
- Tests may use assertions and panic-style helpers where idiomatic, but
  production paths may not.

## Dependency policy

- Prefer the standard library and existing dependencies.
- Production dependencies require a concrete need, maintenance and license
  review, native-build and transitive-impact review, an accepted decision, exact
  version pinning, lockfile updates, and full verification.
- Development dependencies require the same justification except when an
  existing approved tool already supplies the capability.
- Do not weaken audits or ignore advisories without an explicit bounded decision.
- Dependency upgrades are separate increments from feature work unless the
  dependency change is inseparable from the approved goal.
- Dependabot may propose review branches for npm, Cargo, and GitHub Actions. A
  proposal is not approval: no repository workflow auto-merges or writes it to
  `main`.

## Git and branch strategy

- Start from clean synchronized `main` unless an approved reconstruction plan
  says otherwise.
- Use one descriptive capability-based branch per increment. Codex-created
  branches use the `codex/` prefix; names such as
  `codex/feature/add-agent-memory-store`,
  `codex/fix/srm-health-check-timeout`, or
  `codex/meta-codex-automation-quality-gates` identify the work directly.
- Do not mix unrelated changes or rewrite another contributor's work.
- Use Conventional Commits whose subject explains the bounded capability and
  reason. Do not use generic names such as `update`, `changes`, `misc`, `temp`,
  or `final`.
- Use descriptive pull-request titles. Every pull-request description includes
  Purpose, Files changed, Testing performed, Breaking changes, and Next
  increment.
- Commit only after verification and explicit project-owner direction.
- Push, open a pull request, merge, tag, or publish only when explicitly asked.
- After all checks pass and publication is explicitly approved, push the branch,
  create the pull request, and merge it with a squash merge.
- CODEOWNERS and workflow results support review but do not prove remote branch
  protection, required reviews, release approval, or increment completion.

## Definition of Ready

An increment is Ready only when:

- its prerequisite state is verified in the repository;
- one bounded goal and user-visible outcome are stated;
- exact implementation and closeout files are declared;
- risks, security impact, non-goals, verification, manual gates, and rollback
  are documented;
- unresolved architecture decisions do not block the work;
- the project owner has selected the plan and approval is still required before
  editing.

Only the first Ready item in `NEXT_STEPS.md` may be implemented unless the
project owner explicitly selects another bounded task.

## Increment workflow

1. Use `$session-start` or `$resume-session` and reconcile memory with Git.
2. Read the required documents, source boundaries, and relevant tests.
3. Confirm a clean tree and pass the smallest useful baseline.
4. State the goal, non-goals, exact files, risks, verification, and rollback.
5. Wait for project-owner approval.
6. Begin the mandatory gate with
   `python3 .codex/hooks/post_increment_gate.py begin --increment <id>`.
7. Implement only the approved scope and run focused checks while working.
8. Run complete relevant verification and any required manual checks.
9. Run `python3 .codex/hooks/session_end_gate.py` and resolve any conflict or
   unexpected path.
10. Run `$quality-gate` to compose architecture, security, code-health,
    technical-debt, and roadmap-readiness review against the complete diff.
11. Synchronize project memory and write the increment and review records only
    after evidence is collected.
12. Run `$post-increment-gate` and require a valid PASS or PASS WITH ADVISORIES
    marker.
13. Stop. Do not begin the next increment, commit, or publish automatically.

## Code-review workflow

Review findings before summaries and order them by severity. A review must
cover correctness, trust boundaries, privacy, errors and cancellation, tests,
portability, dependencies, and documentation. Every finding names a path,
concrete impact, evidence, and smallest safe correction.

Critical or High findings, merge conflicts, failed required checks, or pending
required manual checks block completion. Advisories may remain only when their
risk and follow-up are explicit. Follow `CODE_REVIEW.md` and
`SECURITY_CHECKLIST.md`.

## Definition of Done

An increment is complete only when:

- every acceptance criterion is met;
- all required automated and manual checks passed;
- the complete diff matches the approved scope;
- no Critical or High finding remains;
- security and privacy boundaries remain intact;
- project memory and current-state documentation match the repository;
- a post-increment report records exact files, commands, results, risks, and
  next readiness;
- the gate reports the expected increment complete and valid.

Publication is a separate operation. A verified uncommitted increment is not
published until its commit and remote state are confirmed.

## Session start and end

At session start, follow `.agents/skills/session-start/SKILL.md`: read the
required memory in order, inspect Git and the toolchain, reconcile actual state,
select one goal, and do not edit before scope approval.

At session end, follow `.agents/skills/session-end/SKILL.md`: run final checks,
inventory the repository with `.codex/hooks/session_end_gate.py`, review the
complete diff, classify passed/failed/not-run/manual checks, update memory,
record the exact next task and resume prompt, and leave no required process
running. Do not mark incomplete work complete.

## Release process

Releases are separate approved increments. They must follow
`RELEASE_CHECKLIST.md`, resolve release-blocking open decisions, verify a clean
tagged commit, audit dependencies and secrets, build release artifacts, and run
target-platform installer, signing, notarization, launch, upgrade, and rollback
checks. Start release notes from
`docs/templates/RELEASE_NOTES_TEMPLATE.md`. Current signing, notarization, and
production distribution remain planned, not implemented.

## Documentation requirements

- Update current-state documents when capability or queue state changes.
- Append decisions, changelog entries, and troubleshooting history; do not erase
  them.
- Use repository-relative links and verify every referenced path.
- Keep active prompt references aligned with `prompts/README.md`; preserve old
  paths only where they are dated migration or historical evidence.
- Keep prompts, repository skills, human workflow runbooks, prompt templates,
  and document templates conceptually distinct.
- Separate implemented facts from mocked, planned, prohibited, and unverified
  behavior.
- Record exact commands and actual results, including checks not run.
- Keep external decks, screenshots, and diagrams subordinate to repository facts
  and the canonical branding guidance.
- Keep issue labels and milestones subordinate to `NEXT_STEPS.md`, `ROADMAP.md`,
  approved plans, and verified completion evidence.
- Use Mermaid for maintainable architecture flows and `$branding` for branded
  visual artifacts.
