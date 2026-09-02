# Personal Assistant v0 signing-security prerequisite planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Increment: `personal-assistant-v0-signing-security-prerequisite-planning`
Predecessor: D-097 terminal failure with D-098 schema-v3 disposition

## Goal

Define the smallest truthful documentation outcome that makes future signing
security work understandable and safely sequenced without performing any
Apple, Keychain, signing, build, provider, credential, network, or product
operation. This plan is not an operational signing plan and does not make one
Ready.

## User-visible outcome

The repository will contain a privacy-safe prerequisite map for any future
Personal Assistant v0 signing proof. It distinguishes known facts from Failed,
Pending, and Not-run evidence, and identifies separately approvable work needed
before an operation could be considered.

## Scope

1. Preserve immutable predecessor facts: D-097 remains `failed` / `FAIL` /
   `Blocked` with no completion marker; the screenshot/privacy requirement is
   Failed; `getpwuid`/`opendirectoryd` is Manual verification pending; scoped
   identity visibility is not signing evidence; signing remains Not run.
2. Record D-098 accurately: schema-v3 permits this exact successor and carries
   closed lineage. It is workflow-integrity evidence, not authentication,
   durable audit, product authority, or operational authorization.
3. Define four future prerequisites, all Proposed or Blocked:
   - **P1 evidence privacy protocol:** categorical evidence only; no screenshot,
     identifier, raw output, certificate metadata, or account detail in chat,
     Git, logs, or reports.
   - **P2 account-directory boundary:** a contained resolver or separate owner
     acceptance of the exact account-record, directory-service, cache/socket/
     log, process-metadata, and possible network boundary. Never rerun the
     consumed query.
   - **P3 build-child containment:** a no-new-dependency fail-closed control
     for npm/Cargo/Tauri/frontend descendants. Output roots and process groups
     alone are insufficient; outside-root writes, undeclared network activity,
     and escaped children must deny progression.
   - **P4 immutable signing contract:** application-owned team binding,
     fingerprint-to-leaf verification, fixed disposable artifact, prompt-stop
     policy, bounded cleanup, and a no-argument sanitizer with closed redacted
     outcomes.
4. Reconcile only the exact D-098 fifteen-path documentation ceiling.

## Explicit non-goals

- No reclassification, remediation, waiver, deletion, replacement, or
  reclosure of D-097 or D-098 evidence.
- No Keychain query, certificate/key action, private-key use, signing,
  notarization, Gatekeeper, installer, distribution, or target-Mac operation.
- No product source, dependency, lockfile, hook, workflow, Tauri, CSP,
  capability, permission, provider, model, credential, filesystem,
  persistence, tool, or network change.
- No branch, commit, push, merge, pull request, release, publication, or
  external-system action.

## Existing behavior and constraints

- The D-098 lineage names only this increment and limits its final tracked
  change set to fifteen documentation paths.
- The ignored state is same-user writable and checkout-local. Its hashes detect
  ordinary drift; they are not authentication or durable audit evidence.
- D-096 governs future evidence: technical non-extractability, historical
  absence of export, and exclusive custody remain `not_proven`.
- Current product behavior is unchanged.

## Current-state evidence

- Before this plan began, refreshed `main`, `HEAD`, and `origin/main` were all
  `7fd4812fb02de7fee19e53a50b8f7bf96bd38709`, with ahead/behind `0/0`.
- The predecessor report digest is
  `712df03ac11cff026be4badad2c8d22a023377f40c99981b90937c95ae165087` and its
  reconstructed v2 state digest is
  `dfa11a0784c10edffab71e0ae1859db397d47ec035801f69a249bdded8ff4dc3`.
- The valid D-098 disposition is `PASS WITH ADVISORIES` / `Ready with
advisories` and has recovery report digest
  `db0de5f25087cd4d0465d35d591b6a3b235af1a2c482b5306317390ca36dbc73`.

## Files expected to change

Only the D-098 allowlisted paths: `ARCHITECTURE.md`, `CHANGELOG.md`,
`DECISIONS.md`, `HANDOFF.md`, `NEXT_STEPS.md`, `PLANS.md`,
`PROJECT_STATUS.md`, `ROADMAP.md`, `SECURITY.md`, `SECURITY_CHECKLIST.md`,
`TESTING_GUIDE.md`, `TROUBLESHOOTING_LOG.md`, this plan, the matching increment
record, and the matching post-increment review.

## Affected components

| Component                    | Effect                                                         |
| ---------------------------- | -------------------------------------------------------------- |
| Repository governance        | Carries D-098 lineage into one bounded documentation closeout. |
| Security evidence            | Separates historical facts from future prerequisites.          |
| Product runtime              | None.                                                          |
| Target Mac and Apple systems | None. All operational checks remain Not run.                   |

## Interfaces and invariants

- D-097 report/state evidence is immutable and this plan cannot add its
  completion marker or change `FAIL`/Blocked.
- The active gate must keep schema-v3 `predecessor_disposition` for this exact
  increment; closeout rejects any path outside the recorded allowlist.
- A future trusted identity, team, fingerprint, bundle, Keychain, runtime,
  profile, task, run, or workflow must be application-owned, never caller or
  WebView selected.
- A future sanitizer emits only a bounded categorical DTO. It never emits raw
  account, path, certificate, label, fingerprint, command output, private-key
  material, password, provider credential, or screenshot.
- Any prompt, ambiguity, timeout, malformed output, unexpected filesystem or
  network effect, unresolved directory boundary, or unproven child quiescence
  denies progression without retry.

## Implementation milestones

- [x] Confirm D-098 lineage and begin from clean synchronized `main`.
- [x] Create the increment record and this closed-scope ExecPlan.
- [x] Reconcile project-memory, security, testing, troubleshooting, and
      roadmap records without changing historical evidence.
- [x] Run documentation-tier validation and independent reviews.
- [x] Create the exact review report; final validation and gate finalization
      remain pending.

## Security and privacy considerations

| Threat                                            | Required control                                                     |
| ------------------------------------------------- | -------------------------------------------------------------------- |
| Historical privacy failure is softened            | Preserve Failed status in every record and review.                   |
| Pending directory boundary is treated as accepted | Keep it Manual verification pending.                                 |
| Documentation is mistaken for authority           | State no-operation boundaries and list target-Mac checks as Not run. |
| Lineage is mistaken for audit/authentication      | Limit claims to checkout-local workflow drift detection.             |
| Future trusted selector reaches caller            | Require fixed application-owned inputs and no-argument interfaces.   |
| Evidence leaks through prose                      | Use only categorical evidence.                                       |

## Test plan

- Inspect gate status before and after work for the exact active increment and
  preserved lineage.
- Confirm Git reports only the fifteen allowed documentation paths, with no
  source, generated, credential, certificate, database, log, or workflow path.
- Verify every record retains Failed/Pending/Not-run labels and makes no
  Keychain, signing, build, or external-operation claim.
- Run documentation formatting/link, repository-health, secret-scan,
  protected-path, and whitespace checks.
- Target-Mac, Apple, Keychain, signing, build, provider, network, and product
  checks are Not run: prohibited and irrelevant to this documentation scope.

## Verification commands

Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
`git diff --check`, the protected-path `git diff --exit-code` proof,
`python3 .codex/hooks/session_end_gate.py`, and
`python3 .codex/hooks/post_increment_gate.py status`.

## Risks

- Passing documentation evidence could be mistaken for D-097 remediation; all
  summaries must keep the outcomes distinct.
- Text cannot establish target-Mac, Keychain, signing, or containment behavior;
  those checks remain Not run.
- The ignored state is not durable audit evidence.

## Rollback or failure strategy

If validation fails, keep this increment active and correct only allowlisted
documentation after identifying the cause. If truthful closeout cannot be
reached, use the normal terminal-failed report path; never alter D-097/D-098
evidence or delete ignored state. A future rollback reverts only this
documentation increment and preserves predecessor evidence.

## Decisions made

- D-099 permits documentation of P1–P4 but authorizes none of their operations.

## Discoveries

- The exact D-098 active lineage enforces a fifteen-path closeout ceiling.
- The first truthful output is an evidence map, not a signing workflow.

## Progress

- 2026-09-01: Owner authorized the exact successor `begin` from clean,
  synchronized `main`; no prohibited operation ran.
- 2026-09-01: Owner approved this documentation-only implementation.

## Acceptance criteria

- [x] The plan preserves D-097/D-098 and historical Failed/Pending/Not-run
      evidence.
- [x] Scope is exactly documentation-only and within the fifteen paths.
- [x] P1–P4 separate privacy, directory, containment, and signer work.
- [x] Every affected record is reconciled from observed evidence.
- [x] Documentation-tier checks and independent reviews pass.
- [x] The review inventory matches the exact allowlist; gate finalization is
      the immediate final action after the recorded passing checks.

## Final results

`PASS WITH ADVISORIES` for the documentation plan. It preserves D-097/D-098
and maps P1–P4 without operational authority. No target-Mac, Apple, Keychain,
certificate, signing, build, credential, provider, network, or product action
has run or is authorized by this plan.

## Documentation updates

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
- [x] Increment record and ExecPlan
