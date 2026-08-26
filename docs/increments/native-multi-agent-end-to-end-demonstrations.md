# Native multi-agent end-to-end demonstrations

Status: Verified complete with advisories; marker complete and valid
Owner: Project owner
Date: 2026-08-25
Gate ID: `native-multi-agent-end-to-end-demonstrations`
Plan:
[`2026-08-11-multi-agent-end-to-end-demonstrations.md`](../plans/2026-08-11-multi-agent-end-to-end-demonstrations.md)
Review:
[`2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md`](../reviews/2026-08-25-native-multi-agent-end-to-end-demonstrations-post-increment-review.md)
Baseline: synchronized `main` at
`527f0f4bafc4e263221e1ca246bc3e49600e0a26`
Target evidence: macOS 26.6 (build 25G72), Apple silicon (`arm64`)

## Goal

Prove the implemented native nine-agent architecture through deterministic,
non-destructive demonstrations and one canonical acceptance command, without
adding architecture or bypassing governance.

## Implemented acceptance surface

- Added `npm run test:agent-acceptance`, which composes the full Rust library
  unit suite and the exact ten public native-agent contract binaries.
- Strengthened only the rejected branch of the existing macOS approval unit.
  It now proves the exact approval is cleared, the originating task remains
  Running and controlled, full live-root attribution and the exact action/policy
  are preserved, the audit lifecycle is `ApprovalResolved`, the decision is
  `Rejected`, execution is `NotAttempted`, and no audit error exists.
- Strengthened the existing approved-document contract. The live Knowledge
  context submits one deterministic application test value through the real
  proposal API; the record remains process-local `ProposedShared`, unapproved,
  and absent from Personal synthesis after root completion.
- Added separate demonstration, fixture, and acceptance/evidence artifacts:
  - [`NATIVE_MULTI_AGENT_DEMONSTRATIONS.md`](../demos/NATIVE_MULTI_AGENT_DEMONSTRATIONS.md)
  - [`NATIVE_MULTI_AGENT_FIXTURES.md`](../demos/NATIVE_MULTI_AGENT_FIXTURES.md)
  - [`NATIVE_MULTI_AGENT_ACCEPTANCE.md`](../demos/NATIVE_MULTI_AGENT_ACCEPTANCE.md)

No production runtime/workflow behavior changed. Both Rust edits are test-only;
one is inside an existing `#[cfg(test)]` module and one is a public integration
contract.

## Demonstration results

All twelve demonstrations pass under the owner-approved acceptance scope.
Demo 7 validates the non-executable checkpoint-denial branch and the separate
safe manual-dispatch branch without bridging them. The absent combined chain
remains an explicit advisory and grants no execution authority.

1. Personal Assistant direct response without delegation.
2. Personal -> Research -> Knowledge -> Personal synthesis with exact source
   provenance and truthful partial failure.
3. Explicit temporary approved-document reading, Knowledge processing,
   proposal-only reusable knowledge, and versioned application review before
   any shared-memory update.
4. Personal -> Coding -> QA -> Security -> Personal proposal-only engineering
   review over a synthetic repository catalog.
5. Personal -> Cloud -> QA -> Security -> Personal assessment over static
   synthetic Terraform and architecture data.
6. Personal -> Systems -> QA -> Security -> Personal analysis over sanitized
   service, log, and recovery fixtures.
7. **Pass with advisory:** typed Workflow Automation proposals, strict
   application validation, non-executable tool/checkpoint values, and separate
   take-once manual dispatch of safe A-D proposals into existing sealed fixture
   selectors. No approval-authorized transition connects those branches.
8. Bounded same-thread retained-run multiplexing with attribution, canonical
   ordering, cancellation, partial failure, all three failure policies, and
   synthesis.
9. Unknown/ineligible specialist tool policy denial with typed attributed audit
   and no execution.
10. Task-bound simulated macOS approval rejection with pending cleanup,
    complete audit, and no action.
11. Child-first root cancellation, retained-run cleanup, late-event rejection,
    and pending-approval reconciliation.
12. Typed specialist/runtime failure with controlled fallback or partial
    synthesis and no application crash.

## Fixture versus live behavior

- Workflow input and output are deterministic repository fixtures.
- Public workflows use test-only `MockAgentRuntime`. Its descriptor reports the
  sole closed `Native` runtime identity; it is not a configured provider.
- `NativeAgentRuntime` remains the sole/default wrapper and has no provider or
  live model.
- Production task, orchestrator, schema, policy, approval, bounded document,
  memory, and volatile audit code is exercised where documented.
- No real governed tool executes. All execution dispositions are
  `NotAttempted`.
- The approval result is a deterministic simulated macOS dialog result passed
  through the real trusted source-resolution path; no dialog is displayed.
- No live retrieval, repository, cloud, system, service, browser, credential,
  provider, or device operation occurs. Cargo is locked but not offline, so a
  cold local cache may fetch locked crates; normal build and temporary-fixture
  I/O are test-harness effects, not assistant operations.
- The Command Center remains a separate simulated frontend projection and is
  not backend workflow evidence.

## Exact verification evidence

Focused approval unit:

- 1 passed, 0 failed, 0 ignored; 248 filtered out.

Canonical acceptance command:

- Rust library units: 249 passed.
- Ten public native-agent contract suites: 198 passed.
- Total: 447 passed, 0 failed, 0 ignored.

Complete `npm run verify`:

- formatting and repository health: passed;
- frontend and Rust lint: passed;
- hook tests: 28 passed;
- repository-workflow tests: 38 passed;
- frontend: 211 passed across 13 files;
- Rust library: 249 passed;
- Rust integration: 232 passed and one intentional opt-in Hermes probe ignored;
- Rust total: 481 passed, 0 failed, 1 ignored;
- TypeScript typecheck and Vite production build: passed;
- Tauri release no-bundle build: passed.

## Architecture and governance result

The acceptance evidence confirms:

- all nine definitions and exact activation metadata;
- application-only task creation and specialist delegation denial;
- Workflow Automation's zero executable tool-step limit;
- QA's lack of approval authority and Security's lack of authorization
  authority;
- unknown-tool, identity-forgery, memory, document, destructive cloud, and
  destructive systems fail-closed behavior;
- exact task/agent/run attribution in bounded process-local records;
- Native as sole/default and unwired; and
- Hermes absent and Deferred/Blocked.

## Explicit advisories

1. No configured Native provider or live model exists.
2. No real governed tool executor exists.
3. Demo 7's two owner-approved branches remain separate: D-090 checkpoint
   denial does not authorize safe manual A-D dispatch, no bridge exists, and
   executable tool steps remain zero.
4. Approval rejection returns a typed task-bound application resolution but is
   not automatically injected into runtime text and does not terminalize the
   root task.
5. Audit is bounded and process-local, not durable or unified.
6. D-091 is same-thread event multiplexing, not provider/CPU concurrency.
7. No connected backend UI exists; screenshots cannot prove the Rust paths.
8. Hermes remains deferred/blocked.

These advisories block live/executable/production capability claims. The suite
is complete evidence for all twelve demonstrations under the owner-approved
scope, without claiming that Demo 7 contains a combined approval-to-dispatch
chain.

## Owner-approved Demo 7 clarification

D-093 accepts checkpoint denial and safe manual A-D dispatch as two separate
Demo 7 branches. D-090 still sets executable tool steps to zero, checkpoint
proposals still issue no dispatch token, and the safe A-D dispatch path remains
a separate application-owned branch. The absent combined chain is an advisory,
not a completion blocker. Adding any connection would require a separately
approved decision and ExecPlan; this increment adds none. Fresh closeout passes,
and the deterministic marker is complete and fingerprint-valid.

## Screenshot and captured-state decision

No new screenshot is tracked. The repository's backend workflows have no
connected UI, and a Command Center screenshot would show a separate simulated
projection. Exact test output, structured state assertions, this increment
record, and the post-increment report are the appropriate captured evidence.

## Rollback

Remove the npm alias, revert both test-only assertion changes, remove the three
additive evidence artifacts, and restore the current-state documentation
passages. Existing individual contracts and all production architecture remain
intact. No data, migration, credential, external resource, or device state
requires rollback.
