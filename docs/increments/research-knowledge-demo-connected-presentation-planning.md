# Research/Knowledge connected-presentation planning increment

Status: Verified complete with advisories
Owner: project owner
Last updated: 2026-08-28

## Goal

Produce one source-aligned, security-bounded ExecPlan for the smallest truthful
Research/Knowledge lifecycle presentation. Reconcile the existing success-only
production host with private fixture-only failure proof before any Command
Center connection is considered.

## Scope

- Add the connected-presentation ExecPlan.
- Reconcile current-state documentation with the existing Tauri adapter and
  unconnected client.
- Define exact future source paths, no-input interfaces, fixed alternating
  terminal schedule, UI disclosure, tests, target-Mac checks, rollback, and
  stop conditions.

## Explicit non-goals

No product source, test, configuration, capability, CSP, dependency, lockfile,
provider, model, network, credential, tool, approval dispatch, persistence,
filesystem, background work, device effect, commit, push, or merge.

## Completion evidence

Documentation validation, repository-policy validation, security scan, diff
hygiene, architecture/security/code review, session-end inspection, and the
post-increment completion marker are required. Target-Mac UI validation is not
run because this increment adds no UI.

## Progress

- 2026-08-28: Owner approved this planning-only increment and a fresh branch
  from squash-merged `origin/main` at `54e10b3` was created.
- 2026-08-28: Gate opened. Source inspection confirmed that current production
  `ResearchKnowledgeDemoHost::new()` is success-only while synthesis failure
  remains private test evidence; existing Tauri commands/client are no-input
  and unconnected.
- 2026-08-28: The source-aligned ExecPlan and current-state reconciliation were
  reviewed against the existing lifecycle core, Tauri adapter, and client.

## Acceptance criteria

- [x] One exact future source ExecPlan exists.
- [x] Current, mocked, planned, and prohibited lifecycle behavior is reconciled.
- [x] The plan prevents caller-selected trusted identity or outcome.
- [x] The plan retains F-01/F-02, F-07, F-08, F-12, and F-15.
- [x] Documentation-only validation and required reviews pass.

## Final results

The planning result is Ready with advisories only for a separately approved
source increment. `git diff --check`, `npm run docs:check`, `npm run
repository:check`, and `npm run security:scan` passed, as did the session-end
and post-increment gates; the final marker is valid with `PASS WITH
ADVISORIES`. No target-Mac UI check ran because no UI changed. Advisory: the
alternating success/failure schedule is a proposed, private future
implementation detail and needs fresh owner approval, exact contract tests, and
a new gate before source work.
