# D-107 late-result rejection contract decision

Status: Complete (`PASS WITH ADVISORIES`) — proposed D-117 pending owner acceptance
Owner: Project owner
Last updated: 2026-09-02
Baseline: `787255354995f941980138adc3d69beef104ff07`
Branch: `codex/d107-late-result-rejection-contract-decision`
Increment: `d107-late-result-rejection-contract-decision` (complete)
Predecessor: proposed D-116; D-113 through D-116 remain Proposed and
non-controlling

## Goal

Perform one repository-only, documentation-only assessment of D-107's
`late_result_rejection_contract`: whether current source proves that trusted
Rust can reject a private-key identity/sign/verify result after the owning
attempt has become terminal, without mutating terminal state or granting retry,
fallback, reuse, presentation, evidence, or cleanup authority.

The current source supports only the closed negative disposition
`late_result_rejection_not_accepted`. No private-key attempt host exists, and
discarding an application-visible result would not prove that the underlying
synchronous private-key operation stopped. This now-approved increment records
that bounded negative result as proposed D-117 after revalidating this exact
source baseline and the required preservation checks; it does not accept the
proposal.

## User-visible outcome

None. This plan and its approved decision increment are governance
documentation only. They add no application behavior, UI, provider, identity,
private-key, signing, filesystem, network, persistence, process, device, or
external-system capability.

## Scope

1. Review D-096 through D-116, the frozen D-107 candidate, current repository
   source, and existing deterministic late-event tests without running an
   identity or signing operation.
2. Define conceptual `LateResultRejectionPolicyV1` with exactly three closed
   governance dispositions:
   - `late_result_rejection_documented`;
   - `late_result_rejection_not_accepted`; and
   - `boundary_failed`.
3. Record that the current source assessment selects only
   `late_result_rejection_not_accepted`; missing, ambiguous, contradictory, or
   drifted facts select `boundary_failed`.
4. Define the invariants and adversarial evidence a future positive assessment
   would require without designing or implementing the missing host.
5. Preserve D-096 through D-116, including D-097's failed record, GUI D-112
   byte-for-byte, D-107's immutable 8 documented / 11 unproved record, D-108's
   additive 9/10 interpretation, accepted D-109 through D-111, proposed D-113
   through D-116, all ten blockers, the unadmitted candidate, and `Blocked`
   operational readiness.
6. In this approved decision increment, reconcile only current publication
   wording for squash-merged reconciliation commit
   `787255354995f941980138adc3d69beef104ff07`; do not rewrite its historical
   plan, increment, review, or pre-publication evidence.

## Explicit non-goals

- No Keychain, certificate, private-key, signing, Apple/Xcode, target-Mac
  identity, build, provider, product, credential, or state-changing external-
  system access. Read-only Git synchronization with `origin` is the sole
  external contact and only verifies the explicitly requested baseline.
- No product, Rust, TypeScript, test-source, dependency, lockfile,
  configuration, workflow, hook, capability, permission, entitlement, CSP,
  runner, or toolchain change.
- No choice of worker, thread, process, channel, callback, async runtime, FFI,
  containment primitive, cleanup mechanism, or production architecture.
- No claim that result rejection cancels or stops a synchronous operation,
  prevents a signature or prompt, proves quiescence, releases ownership, or
  dispositions `hard_deadline_cancellation_contract`,
  `cleanup_quarantine_contract`, or `platform_effect_contract`.
- No acceptance of proposed D-113 through D-116, no positive D-117 decision,
  no D-100 evidence, no candidate admission, and no successor readiness.
- No caller-, model-, WebView-, environment-, account-, path-, profile-,
  runtime-, agent-, workflow-, task-, run-, clock-, or result-selected trusted
  identity or policy value.
- No additional branch or gate, and no commit, push, merge, release,
  publication, decision acceptance, or successor start.

## Existing behavior and constraints

- D-107 records `late_result_rejection_contract` as `contract_unproven`
  because no private-key attempt host exists and rejecting a late application
  result would not prove that late private-key use stopped.
- Proposed D-116 records that synchronous identity/sign/verify APIs expose no
  hard deadline or cancellation parameter. A timer, dropped future or receiver,
  worker, or late-result filter does not stop an in-flight operation or prove
  cleanup and quiescence.
- Production source contains no `SecIdentity`, private-key signature, or paired
  verification attempt host for the frozen
  `in_process_security_framework_ephemeral_challenge_proof_v1` candidate.
  Pinned Security.framework crates and the separate fixed-label generic-
  password reader do not supply that host.
- `RuntimeRun::accept_event` and `NativeAgentRun` reject terminal, foreign, and
  malformed agent-runtime events. That application-owned agent event boundary
  is not a private-key result boundary.
- `PersonalAssistantV0Host` owns volatile generation and terminal state, but
  its late-event ingress and results are deterministic test fixtures; no
  production provider-response ingress exists.
- `ResearchKnowledgeDemoHost` is a sealed, manually stepped synthetic workflow
  with no private-key operation. Its epoch, cancellation, and quarantine tests
  are separate deterministic proofs.
- Gateway, approval, orchestration, and demo late-event handling cannot be
  generalized into signing evidence. Reuse requires a separately approved
  source design and tests; documentation similarity grants no authority.
- D-100 permits only one source-minimized three-field evidence record from a
  separately approved operation. The policy labels in this plan are governance
  vocabulary, not D-100 outcomes or runtime/IPC values.

## Current-state evidence

- A read-only fetch established clean synchronized `main` at
  `787255354995f941980138adc3d69beef104ff07`; local `HEAD`, local `main`, and
  `origin/main` matched with ahead/behind `0/0` before this draft.
- The plan was drafted in a clean detached worktree so the original checkout's
  30 user-owned untracked ` 2.*` paths remain untouched. After approval, the
  exact branch was created in that isolated worktree and the gate began.
- `git fsck --no-progress` exited zero; only recoverable dangling objects were
  reported.
- Repository searches found no existing late-result decision plan, no frozen
  candidate host, and no production/test/example Rust identity/sign/verify
  operation for this candidate.
- Existing source and tests demonstrate only domain-specific agent/demo event
  rejection and fixture-only Personal Assistant late events.
- Repository-pinned Node 26.3.0 and npm 11.16.0 were active. Offline,
  scripts-disabled installation used the existing lockfile and reported zero
  known vulnerabilities from available metadata.
- Baseline `npm run docs:check` passed before this file was added.
- Baseline `HANDOFF.md` and `NEXT_STEPS.md` contained pre-merge wording for the
  reconciliation branch. Git proves the squash merge at `7872553`; this
  increment corrects only current memory and leaves historical artifacts
  unchanged.
- No target-derived, Keychain, certificate, private-key, signing, Apple/Xcode,
  provider, product, or operational external evidence was collected. Read-only
  Git synchronization was the sole external contact.

## Files expected to change

The completed planning phase created exactly one path:

1. `docs/plans/2026-09-02-d107-late-result-rejection-contract-decision.md`

The owner-approved active decision increment has exactly these fifteen
documentation paths as its ceiling:

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/d107-late-result-rejection-contract-decision.md`
14. `docs/plans/2026-09-02-d107-late-result-rejection-contract-decision.md`
15. `docs/reviews/2026-09-02-d107-late-result-rejection-contract-decision-post-increment-review.md`

Any other tracked or gate-visible untracked path is a stop condition.

## Affected components

- Conceptual governance vocabulary for one D-107 blocker.
- Durable proposed D-117 decision lineage under this owner-approved increment;
  the proposal remains non-controlling.
- Current architecture, security, testing, roadmap, status, handoff,
  troubleshooting, and plan memory under this exact-scope documentation
  increment.

No runtime, adapter, key, process, IPC, WebView, provider, persistence,
filesystem, network, approval, audit, tool, or device component is affected.

## Interfaces and invariants

`LateResultRejectionPolicyV1` is conceptual documentation only:

| Condition                                                                                                      | Closed disposition                   |
| -------------------------------------------------------------------------------------------------------------- | ------------------------------------ |
| Current source proves every positive invariant below for the exact frozen candidate                            | `late_result_rejection_documented`   |
| Current source establishes no complete private-key attempt host and therefore no complete late-result boundary | `late_result_rejection_not_accepted` |
| Any required fact is unavailable, ambiguous, contradictory, unbounded, or drifted                              | `boundary_failed`                    |

The current expected selection is exactly
`late_result_rejection_not_accepted`. A future source change is not grounds to
infer a positive selection; it stops this plan and requires a new bounded
review.

The following invariants describe evidence required before any future positive
selection. They are not implemented or proved today:

1. One explicit foreground owner action may create at most one attempt. Trusted
   Rust alone issues one non-reusable opaque attempt handle and monotonic
   attempt epoch before the first effect; no caller supplies or selects either
   value. The signing identity remains a separately prebound application-owned
   capability whose provenance, signer binding, account scope, and non-export
   properties remain governed by their independent unresolved contracts; this
   plan neither issues nor proves that identity.
2. One trusted-Rust state owner serializes result admission and terminalization
   at one linearization point. Exactly one transition may win; terminal state
   is monotonic and immutable.
3. A result may be admitted exactly once only through an ownership-bound
   ingress for the exact live attempt handle and epoch, expected operation
   phase, and nonterminal state. Caller- or result-supplied identity, handle,
   epoch, timestamp, or other correlation data is never authority.
4. Unknown, malformed, foreign, prior-epoch, duplicate, cancelled, failed,
   completed, locally expired, or otherwise post-terminal results are rejected
   before they can authorize or trigger any additional application-owned state,
   journal, evidence, UI, IPC, readiness, follow-on operation, or external
   dispatch mutation. Rejection makes no claim that prior or in-flight OS or
   private-key effects were prevented.
5. Rejection never reverses the terminal result, emits success, marks a check
   Passed, creates D-100 evidence, starts a replacement, or triggers fallback,
   retry, reuse, or concurrent private-key work.
6. The attempt lease and any uncertain operation ownership remain private and
   retained for the separate cleanup/quarantine owner. Rejected result material
   remains adapter-private and follows a separately proved bounded disposal
   path; neither disposal nor rejection releases operation ownership or proves
   cleanup or quiescence.
7. No signature bytes, key/certificate/identity metadata, native error,
   result-supplied or native timestamp, raw diagnostic, arbitrary text, or
   result payload crosses the adapter through DTOs, logs, debug output, errors,
   documentation, tests, or ordinary CI. Exposed errors remain closed and
   redacted.
8. Any later approved result-ingress mechanism and retained state must have
   fixed bounds. Capacity, closure, poisoned or failed owner state, sequence
   exhaustion, clock ambiguity, and ownership ambiguity fail closed without a
   second attempt.
9. Result rejection is independent of hard cancellation, interaction denial,
   cleanup/quarantine, and platform effects. It cannot prove the synchronous
   operation stopped, undo an operation, prevent prompts, or establish absence
   of OS-managed effects.

## Documentation milestones for the approved increment

- [x] Reconfirm clean synchronized baseline, exact decision numbering, and the
      unchanged status of D-113 through D-116.
- [x] Create the approved `codex/` branch and run exactly one `begin` command
      for `d107-late-result-rejection-contract-decision`.
- [x] Re-run the repository-only source assessment without target or signing
      operations and select exactly one closed disposition.
- [x] Because the negative result remains supported, add proposed D-117 under
      this documentation increment; implementation approval does not accept
      D-117 or D-113 through D-116.
- [x] Synchronize exactly the twelve authorized current-memory documents,
      including the observed reconciliation publication state, while
      preserving all ten blockers and `Blocked` readiness.
- [x] Run exact-scope, documentation, repository, security, preservation,
      protected-path, independent documentation/security/readiness, session,
      and post-increment checks.
- [x] Stop for owner review; do not publish or begin another increment.

## Security and privacy considerations

### Threats

- **Discard-as-cancellation laundering:** a dropped result, future, receiver,
  or worker is misrepresented as proof that private-key use stopped.
- **Terminal/result race:** a check-then-commit race permits both cancellation
  and completion, reverses terminal state, or emits two outcomes.
- **Cross-attempt or ABA replay:** an old result is accepted by a replacement
  attempt that reused a handle, epoch, or slot.
- **Caller-selected correlation:** a model, WebView, runtime, environment, or
  caller chooses an identity or timestamp that trusted Rust accepts as
  authority.
- **Duplicate or forged completion:** duplicate, malformed, or foreign results
  mutate state, evidence, UI, audit, or readiness.
- **Ownership release:** result rejection clears the lease or quarantine before
  the underlying operation is proved quiescent and permits replacement or
  concurrent key use.
- **Retry or fallback laundering:** a rejected result starts another attempt,
  chooses another identity/algorithm, or downgrades policy.
- **Sensitive late-result leakage:** signature, certificate, identity, key, or
  native-error material reaches a log, DTO, debug formatter, report, or test.
- **Unbounded retention:** result queues or tombstones grow without a fixed
  ceiling, or capacity failure defaults open.
- **Fixture transference:** Personal Assistant, Research/Knowledge, gateway,
  approval, or orchestration tests are claimed as private-key boundary proof.
- **Contract collapse:** late-result rejection is used to disposition deadline,
  cancellation, interaction, cleanup/quarantine, or platform-effect blockers.

### Security review disposition

The documentation plan is acceptable only with the current negative result and
the strict contract separation above. Any positive/admitting result on the
current source is `Blocked` and requires plan revision. No new permission,
dependency, credential, identity, platform operation, or authority is proposed.

## Test plan

### Repository-only decision checks

- Confirm the policy exposes exactly the three listed governance dispositions
  and that unknown, missing, contradictory, or drifted evidence selects
  `boundary_failed`.
- Confirm the current repository selection is negative, not D-100 evidence,
  runtime state, or a universal impossibility claim.
- Confirm no candidate attempt host or private-key result ingress exists.
- Confirm every existing late-event proof is explicitly scoped to its current
  agent, demo, gateway, approval, orchestration, or fixture boundary.
- Confirm D-107 remains byte-for-byte 8/11, D-108 remains additively 9/10, all
  ten blocker names remain exact, D-113 through D-116 remain Proposed, and
  operational readiness remains `Blocked`.
- Confirm no protected source, dependency, configuration, workflow, hook,
  capability, permission, or toolchain path changes.

### Required future implementation evidence, not run by this increment

These cases define the minimum deterministic tests for any later source design;
they do not authorize that design or count as current evidence:

- exact active-attempt preterminal result admits once;
- result after success, failure, cancellation, or local deadline is rejected;
- wrong identity, prior epoch, malformed result, and duplicate result reject;
- result-versus-terminal races prove exactly one linearized winner in both
  deterministic orders;
- if a separately approved later design permits a subsequent attempt, a
  prior-epoch result cannot mutate it; this test grants no restart, replacement,
  or persistence authority;
- mechanism-specific capacity, closure, owner-failure, clock, and sequence
  faults defined by a separately approved design fail closed and retain
  uncertain operation ownership;
- late success, failure, and native-error content remain adapter-private and
  redacted;
- rejection cannot clear a lease/quarantine, emit evidence, mark Passed, retry,
  fall back, or change any independent blocker.

## Verification commands

The planning phase ran:

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git status --short --branch
```

This approved decision increment additionally runs an exact fifteen-path
equality check, historical-decision preservation checks, protected-path checks,
independent architecture/security/documentation/code-health/technical-debt/
readiness review, `python3 .codex/hooks/session_end_gate.py`, and the
post-increment workflow.

`npm run verify`, application tests and builds, `npm audit --audit-level=low`,
Keychain, certificate, private-key, signing, Apple/Xcode, target-Mac identity/
signing/launched-product/device-effect, provider, product, and operational
network/external-system checks are `Not run` by design. Read-only Git
synchronization is recorded separately as the sole external contact.

## Manual gates

- Confirm D-097, D-107 8/11, D-108 9/10, GUI D-112, accepted D-109 through
  D-111, and proposed D-113 through D-116 are preserved.
- Confirm the exact ten blockers remain unproved and readiness remains
  `Blocked`.
- Confirm no existing deterministic late-event test is represented as
  private-key evidence.
- Confirm proposed D-117 is closed, negative, non-authorizing, and does not
  accept D-113 through D-116 by implication.
- Confirm no target-derived value, secret, account identifier, path,
  certificate/key/signature material, native error, or personal data enters the
  diff or report.
- Confirm no operational, signing, provider, product, or target-system check
  ran, read-only Git synchronization was the sole external contact, and no
  successor began.

## Risks

- A generic event-rejection abstraction could hide missing key-use ownership
  and transfer authority from an unrelated runtime or fixture.
- Ambiguous words such as “cancelled,” “discarded,” or “stopped” could overstate
  what application-side rejection proves about a synchronous OS operation.
- Reusing an opaque handle without an attempt-private epoch could permit ABA
  result confusion.
- Treating a terminal flag check as atomic admission could conceal a race.
- Treating result destruction as cleanup could erase the separate retained-
  ownership and quiescence requirements.
- Adding D-117 after proposed predecessors could imply their acceptance unless
  proposal status and non-supersession are explicit.
- Updating stale publication wording could accidentally rewrite historical
  evidence rather than only current memory.
- A baseline change could introduce a new decision number, source host, or
  authority edge and invalidate this plan.

## Rollback or failure strategy

Before approval and `begin`, the safe rollback was to leave this uncommitted
plan unused or remove only this plan file with explicit owner direction. That
planning-phase option is historical; do not reset, clean, stash, rewrite
history, or touch the original checkout's user-owned files.

During this approved increment, stop on any failed required check or scope drift
and preserve the evidence. Correct only within the exact owner-approved scope
or record a truthful terminal failure. After publication, rollback requires a
separate owner-approved documentation revert; historical records remain
immutable.

## Stop conditions

Stop and request owner direction if any of the following occurs:

- `main`, decision numbering, D-107/D-108 counts, a predecessor status, or the
  exact ten-blocker set differs from this baseline;
- the assessment requires Keychain, certificate, private-key, signing,
  Apple/Xcode, target-Mac, provider, product, operational network, or external-
  system evidence beyond the approved read-only Git synchronization;
- a worker, process, FFI, async runtime, channel, dependency, cleanup design, or
  product interface must be selected;
- a positive disposition is proposed without an existing application-owned
  attempt host and deterministic proof of every positive invariant;
- late-result rejection is used as proof of cancellation, interaction denial,
  cleanup/quarantine, or platform-effect safety;
- any protected or non-documentation path changes, the exact path ceiling
  expands, or sensitive/target-derived data appears;
- any required validation fails or a manual gate remains unresolved;
- publication, decision acceptance, operational readiness, or a successor is
  requested without separate explicit authorization.

## Decisions made

- Use a distinct conceptual `LateResultRejectionPolicyV1`; do not generalize an
  agent/demo event validator into private-key authority.
- Preserve the late-result, hard-cancellation, cleanup/quarantine, and platform-
  effect contracts as independent conjunctive blockers.
- Treat the current repository result as
  `late_result_rejection_not_accepted`, not a universal impossibility claim.
- The planning draft reserved D-117 as the next non-colliding number; this
  approved increment records it as Proposed but does not accept it.

## Discoveries

- No current source owns the frozen private-key attempt or a result-ingress
  boundary for it.
- The strongest analogous Personal Assistant late-event test is fixture-only,
  and the production API exposes no corresponding response-frame ingress.
- The reconciliation was squash-merged at `7872553`, but its four new entries
  intentionally remain Proposed. This increment corrects the stale current
  pre-publication queue wording without changing historical evidence.
- A positive late-result boundary would still leave hard cancellation,
  cleanup/quarantine, and platform effects independently unproved.

## Progress

- 2026-09-02: Read the complete required project-memory and security/testing
  chain, inspected synchronized source, preserved the original checkout, and
  drafted this plan without a branch or gate.
- 2026-09-02: Current source assessment selected the expected closed negative
  disposition and defined the evidence ceiling for the then-unapproved
  documentation increment.
- 2026-09-02: The owner approved the exact plan; synchronized refs remained at
  the recorded baseline, the approved branch was created in the isolated
  worktree, and the gate began for exactly this increment.
- 2026-09-02: Reconfirmed the source-only negative disposition, added proposed
  D-117, and synchronized the authorized documentation set.
- 2026-09-02: Corrected initial independent-review findings, ran every required
  documentation-tier, scope, preservation, session, and completion check, and
  recorded `PASS WITH ADVISORIES` with a complete valid marker.

## Acceptance criteria

- [x] The owner explicitly approves this exact documentation-only plan and its
      15-path ceiling.
- [x] A clean synchronized baseline and non-colliding D-117 slot are reconfirmed
      before `begin`.
- [x] Exactly one closed disposition is recorded from repository source without
      target-derived evidence.
- [x] D-097, D-107/D-108 counts, D-109 through D-116 status and lineage, all ten
      blockers, and `Blocked` readiness remain preserved.
- [x] Every required command and manual gate is recorded as Passed, Failed, Not
      run, or Pending without treating an analogous fixture as proof.
- [x] No operational, source, dependency, configuration, permission, signing,
      provider, product, or external-system authority is added.
- [x] This approved increment stops for owner review without publication or
      successor start.

## Final results

The exact fifteen-path documentation increment is complete with `PASS WITH
ADVISORIES` and a valid local completion marker. Current source supports
`late_result_rejection_not_accepted`; proposed D-117 remains non-controlling,
and the operational candidate and every successor remain `Blocked`. No
publication or operational/state-changing external-system work began;
read-only Git synchronization with `origin` was the sole external contact.

## Documentation updates for the approved increment

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Increment record and consolidated post-increment review
