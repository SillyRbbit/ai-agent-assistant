# Research/Knowledge connected presentation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-28

## Goal

Connect only the selected Command Center Research and Knowledge scenario to the
existing no-input volatile lifecycle client, while preserving Rust-owned
workflow identities, the closed DTO, fail-closed cleanup, and the visibly
simulated proof boundary.

## User-visible outcome

An explicit Start action begins one application-owned synthetic lifecycle.
Advance and Cancel manually step only that active lifecycle. Completed runs
alternate privately between deterministic success and the existing synthetic
synthesis failure; cancellation does not select or consume an outcome.

## Scope

- Add one prop-free, selected-scenario-only lifecycle panel.
- Reuse the four existing no-input lifecycle commands and one notification.
- Add a private volatile completed-run schedule to the existing host.
- Extend F-12 for the exact client consumer, panel mount, and closed controls.
- Add focused Rust, Tauri, React, integration, and adversarial static tests.

## Explicit non-goals

No provider, model, network, credential, tool execution, approval dispatch,
policy change, persistence, filesystem access, durable audit, background
autonomy, timer, worker, queue, generic workflow engine, capability, CSP,
permission, dependency, device effect, packaging, signing, or publication.

## Existing behavior and constraints

The application already owns one mutex-held volatile host, four commands that
accept no WebView values, one content-free v1 notification, and one client that
runtime-narrows `unknown`. The Command Center projection, Conversations mock,
and Rust acceptance workflows remain separate deterministic proofs.

## Current-state evidence

Implementation began on clean source-current `origin/main` under the active
`research-knowledge-demo-connected-presentation` gate. Baseline lifecycle,
client/Command Center, and repository-health tests passed before source edits.

## Files expected to change

Only the exact source, test, plan, increment, review, and current-state records
listed in the active ExecPlan may change.

## Affected components

The volatile lifecycle host, its unchanged Tauri adapter contract, the exact
lifecycle client, selected Command Center presentation, and F-12 static guard.

## Interfaces and invariants

- The WebView supplies no agent, task, run, profile, runtime, workflow,
  objective, fixture, script, stage, or outcome value.
- First completed run succeeds; second completed run uses the application-owned
  synthesis failure; completed outcomes then repeat. Cancellation does not
  advance this schedule.
- Start is explicit. There is no automatic start, poll, retry, timer, worker,
  alternate consumer, or recovery control.
- Commands remain responses for their calls; events remain notification-only.
- Errors and cleanup state close to one fixed unavailable message.
- State is process-local and no effect or durable record is produced.

## Implementation milestones

- [x] Add the private completed-epoch schedule and focused Rust/Tauri proof.
- [x] Add and verify the selected-scenario lifecycle panel.
- [x] Extend and verify the F-12 adversarial boundary guard.
- [x] Complete target-Mac, full verification, review, and documentation gates.

## Security and privacy considerations

The WebView remains untrusted and receives only bounded lifecycle vocabulary,
epoch/revision counters, and an eight-entry content-free journal. The existing
F-01 returned-identity check and F-02 cleanup quarantine remain in the native
start path. No content, raw error, credential, path, URL, source, finding,
output, runtime identity, audit entry, or device authority enters the DTO.

## Test plan

Prove completed-run schedule continuity across cancellation at every stage,
success/failure/success, unchanged no-input Tauri signatures, exact DTO and
late-event handling, selected-scenario mounting, all control states, explicit
actions only, closed failures, disposal, accessibility, and adversarial F-12
rejection of alternate consumers or expanded browser/native surfaces.

## Verification commands

```bash
cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle
cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri
cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract
npm run test:frontend
npm run test:repository
npm run test:agent-acceptance
npm run verify
npm run security:scan
git diff --check
```

## Risks

The main risks are outcome selection leaking across IPC, schedule advancement
on cancellation or rejected steps, a second client consumer, stale native data,
automatic execution, raw-error disclosure, or misleading claims that separate
proofs are integrated.

## Rollback or failure strategy

Stop on any scope broadening or failed invariant. Before publication, rollback
is limited to the exact changed paths listed in the active plan; do not reset,
clean, stash, or discard unrelated work. All runtime state disappears when the
process exits.

## Decisions made

No durable architecture or policy decision changed. The private alternating
schedule is the smallest application-owned fixture mechanism approved by this
increment and is not exposed as an interface.

## Discoveries

The existing private synthesis-failure path can be compiled into the host
without changing the runtime, orchestrator, Tauri adapter, DTO, or agent
modules. A completed-terminal transition, not presentation-epoch parity, must
own schedule advancement because cancelled starts consume epochs.

## Progress

- 2026-08-28: Owner approval recorded; clean baseline reconciled and gate
  opened.
- 2026-08-28: Private schedule and schedule-continuity tests implemented;
  focused Rust lifecycle and public Tauri contract checks pass.
- 2026-08-28: Selected-scenario panel and F-12 adversarial coverage completed.
  Review found and closed notification-authority and alternate raw-Tauri bypass
  paths before the final gate.
- 2026-08-28: Focused Rust/Tauri, 81-test frontend, 76-test repository, agent
  acceptance, security, audit, and complete verification commands passed.
- 2026-08-28: Target-Mac development startup and source-current browser fallback
  passed. Approved tooling could not bind to the raw debug executable, so
  native lifecycle interaction and unavailable native appearance/zoom/resize
  checks are `Not run` advisories.

## Acceptance criteria

- [x] Only explicit user action can start or step the lifecycle.
- [x] No caller-selected trusted identity, fixture, stage, script, or outcome
      crosses IPC or the feature boundary.
- [x] Success, synthetic failure, cancellation, late-event rejection, and
      closed errors have deterministic proof.
- [x] Exact disclosure and separate-proof statement remain visible.
- [x] Complete automated, target-Mac, review, documentation, and gate evidence
      supports the final result.

## Final results

`PASS WITH ADVISORIES`. The bounded source slice meets its automated acceptance
criteria without adding an identity/outcome selector or any prohibited external
surface. Complete verification passes with 28 hook, 76 repository, 313
frontend, 269 Rust library, and 244 Rust integration tests; one opt-in real
Hermes probe is intentionally ignored. Security scan, zero-finding npm audit,
builds, and diff checks pass.

Target-Mac `npm run tauri -- dev` startup and source-current browser fallback
checks pass. Native success/failure/cancellation interaction, alternate native
theme/reduced-motion, page zoom, and native resize are `Not run` because
approved tooling could not bind to the raw debug executable. No GitHub Actions
result exists because the branch is uncommitted and unpublished.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` not required; no durable decision changed
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] `ARCHITECTURE.md`, `PRODUCT_REQUIREMENTS.md`, `SECURITY.md`,
      `SECURITY_CHECKLIST.md`, and `TESTING_GUIDE.md`
