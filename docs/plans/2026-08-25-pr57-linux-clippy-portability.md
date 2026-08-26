# PR #57 Linux Clippy portability remediation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-25
Gate ID: `pr57-linux-clippy-portability`
Branch: `codex/native-multi-agent-end-to-end-demonstrations`
Baseline: `28a0c46b24d2865a165dc6e11ec082e6dec61fae`
Pull request: `#57`

## Goal

Restore strict Linux all-target Clippy for PR #57 by aligning private
conditional-compilation scope with existing macOS-only consumers, without
changing any public contract, authorization boundary, credential behavior, or
target-Mac behavior.

## User-visible outcome

None. This is a repository portability correction. The deterministic native
nine-agent demonstrations and the application UI remain unchanged.

## Scope

- Gate the test-only `AgentExecutionDisposition` import to macOS, matching its
  sole macOS-only test consumer.
- Gate `ApprovalSubjectKey::matches_source` to macOS, matching the already
  macOS-only trusted approval-source resolver.
- Compile the private Cloudflare credential reader, validation helpers,
  constants, and zeroizing byte wrapper only on macOS or in tests.
- Preserve the public non-macOS credential probe and its typed
  `UnsupportedPlatform` fail-closed result.
- Record the Linux CI diagnosis, validation, and bounded closeout evidence.

## Explicit non-goals

- No `allow(dead_code)`, lint suppression, workflow weakening, or CI bypass.
- No public API, approval identity, policy, audit, task, runtime, or execution
  behavior change.
- No credential label, Keychain access, secret lifecycle, error, dependency,
  capability, permission, entitlement, network, IPC, or Tauri change.
- No real Cloudflare credential ingestion. D-069 and D-070 remain controlling.
- No dependency advisory remediation in this increment; the five newly
  reported npm transitives are a separately approved next increment.
- No change to the completed D-093 plan, report, or its recorded pre-gate
  validity. Beginning this new gate necessarily replaces the live ignored
  workflow marker state.
- No merge until this increment, the separate dependency increment, and all
  required PR checks pass.

## Existing behavior and constraints

- PR #57 correction commit `6b2675343db8518587068e7175ce0cec9d2f6107`
  passes documentation, frontend, Linux Rust, and target-Mac Rust validation.
- Linux Rust validation run `32917746165`, job `98027487903`, fails only at
  strict all-target Clippy on target-conditional private items.
- The public Cloudflare probe returns `UnsupportedPlatform` outside macOS and
  must remain available and fail closed.
- D-084 keeps agent approval non-executing and application-owned. The proposed
  correction changes compile visibility only.
- D-069 and D-070 keep real Cloudflare credential ingestion blocked.
- TS-016 establishes the precedent: target-gate private macOS-only support;
  never suppress strict Linux Clippy.

## Current-state evidence

- The worktree was clean at `28a0c46` when this gate began.
- The prior `native-multi-agent-end-to-end-demonstrations` gate was complete
  and fingerprint-valid before `pr57-linux-clippy-portability` began.
- On the baseline, four PR checks passed while Linux Rust validation and the
  independent dependency audit failed. The correction head now passes every
  check applicable to this increment; only the separately scoped dependency
  audit remains red.
- Local `npm audit` reproduces five vulnerable transitive packages, but no
  dependency file is changed here.

## Files expected to change

Implementation:

```text
src-tauri/src/agent/orchestrator.rs
src-tauri/src/approvals/manager.rs
src-tauri/src/credentials/cloudflare_access.rs
```

Plan, evidence, and closeout:

```text
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/pr57-linux-clippy-portability.md
docs/plans/2026-08-25-pr57-linux-clippy-portability.md
docs/reviews/2026-08-25-pr57-linux-clippy-portability-post-increment-review.md
```

`DECISIONS.md` does not change unless implementation review discovers a public
contract, credential lifecycle, dependency, permission, or authority change.

## Affected components

- Agent orchestrator test module imports.
- Private trusted approval-source identity comparison.
- Private macOS Keychain proof helpers and their unit-test seam.
- Repository troubleshooting and increment evidence.

## Interfaces and invariants

- All public Rust signatures remain unchanged.
- macOS compiles and exercises the same approval and Keychain paths.
- Tests on every target retain the private Cloudflare fake-reader seam.
- Non-macOS production compilation omits only private helpers that cannot be
  called there; the public probe still returns `UnsupportedPlatform`.
- Approval identity checks, redacted errors, fixed credential labels,
  validation bounds, and zeroization behavior remain unchanged wherever their
  consumers exist.
- Native remains sole/default and unwired; no model, provider, tool, executor,
  or external operation is added.

## Implementation milestones

- [x] Obtain explicit owner approval and begin the dedicated gate.
- [x] Record the exact plan and non-goals before source edits.
- [x] Apply only the three conditional-compilation corrections.
- [x] Run focused macOS approval and credential tests plus strict local checks.
- [x] Run complete local verification and independent reviews.
- [x] Publish an interim reviewed commit and obtain the exact Linux CI result.
- [x] Synchronize final evidence and finalize a valid marker. Closeout
      publication follows under the existing owner authorization.

## Security and privacy considerations

The change touches approval and credential trust-boundary files but adds no
authority or data flow. Review must prove that conditional compilation cannot
remove macOS identity checks, expose a credential value, weaken secret
validation or zeroization, or make the non-macOS public probe succeed. No
secret, personal production data, Keychain item, provider request, or network
operation is used.

## Test plan

- Exercise the macOS-only approval-resolution regression using its existing
  simulated native-dialog outcome.
- Exercise all Cloudflare fake-reader units and the public credential-boundary
  integration test.
- Run Rust formatting, strict all-target/all-feature Clippy, all-target tests,
  and the complete repository verification on the target Mac.
- After an explicitly authorized interim commit and push, require the exact
  Linux Rust validation job to pass. Linux evidence does not replace target-Mac
  evidence.

## Verification commands

```bash
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo test --manifest-path src-tauri/Cargo.toml --locked --lib agent::orchestrator::tests::agent_approval_source_resolution_retains_origin_without_execution
cargo test --manifest-path src-tauri/Cargo.toml --locked --lib credentials::cloudflare_access::tests
cargo test --manifest-path src-tauri/Cargo.toml --locked --test cloudflare_access_credential_boundary
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Remote publication verification requires the `Linux Rust validation` check for
the published correction to pass. The dependency-and-secret-audit job is
expected to remain blocked only by the separately scoped npm advisories until
the next increment.

## Risks

- Over-gating the Cloudflare public API would remove non-macOS fail-closed
  behavior. Control: gate only private helpers and run the public boundary test.
- Using macOS-only rather than `any(target_os = "macos", test)` on private
  credential helpers would remove Linux unit coverage. Control: retain the
  test configuration explicitly.
- Gating the wrong governance import could hide Linux-visible assertions.
  Control: gate only `AgentExecutionDisposition`; keep
  `AgentApprovalAuditDisposition` cross-target.
- Remote Linux validation requires a published commit. Control: keep the gate
  active with truthful pending evidence, publish only the reviewed bounded
  patch, then finalize after the exact job passes.

## Rollback or failure strategy

Before publication, revert only this increment's exact paths. After
publication, revert the bounded remediation commit. Rollback restores the
strict Linux Clippy failure but changes no data, credential, migration, or
external state. If any focused or complete check exposes behavioral change,
stop without starting dependency remediation or merging PR #57.

## Decisions made

- Project-owner approval authorizes this as a separate increment rather than
  reopening the completed D-093 gate.
- Existing D-069, D-070, and D-084 remain unchanged; no new durable decision is
  required for private conditional-compilation alignment.
- The npm advisory repair is a second, separately gated increment.

## Discoveries

- The PR failure includes one test import warning in the active branch and
  latent target-conditional approval/credential warnings already present on the
  baseline. All have the same private compile-scope cause.
- Secret scanning passes. The failed security job is an npm registry advisory
  result, not a detected repository secret.

## Progress

- 2026-08-25: Owner approved two sequential merge-remediation increments.
- 2026-08-25: Began gate `pr57-linux-clippy-portability` on clean commit
  `28a0c46` and recorded this plan before source edits.
- 2026-08-25: Applied the attribute-only correction. Focused approval and
  credential tests, strict Clippy, all-target Rust, and complete `npm run
verify` pass. Independent architecture, security, and code review found no
  source finding.
- 2026-08-25: Published reviewed correction
  `6b2675343db8518587068e7175ce0cec9d2f6107`. CI run `32921400121` passed
  Linux Rust job `98035560462` in 6m55s, target-Mac Rust job `98035560489` in
  2m18s, and frontend job `98035560481` in 57s. Documentation run
  `32921400102`, job `98035529472`, passed in 26s. Dependency job
  `98035560426` failed only on the separately scoped five development
  transitive advisories.
- 2026-08-25: Final quality, documentation, repository, security, whitespace,
  and session-end checks passed. The completion decision is `PASS WITH
ADVISORIES`; the sole merge-blocking advisory is the separately approved
  dependency remediation.

## Acceptance criteria

- [x] The diff changes only private conditional-compilation visibility and
      declared evidence files.
- [x] Public non-macOS credential probing still fails closed with
      `UnsupportedPlatform`.
- [x] Target-Mac approval and credential behavior remains covered and passing.
- [x] Strict local Clippy, all-target tests, and complete verification pass.
- [x] Published Linux Rust validation passes with no warning suppression.
- [x] Security, architecture, code-health, debt, and readiness reviews contain
      no blocking finding.
- [x] The final report is `PASS WITH ADVISORIES` and the marker is
      complete and valid.

## Final results

The exact three-file compile-scope correction is verified locally and on both
required remote Rust targets. Linux strict Clippy and all-target tests pass
without suppression, target-Mac behavior passes, and the public non-macOS
credential boundary remains fail-closed. Independent review found no source,
architecture, security, or code-health finding. The result is `PASS WITH
ADVISORIES` because five development-only npm transitives remain for the next
separately approved gate and still block PR #57 merge.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md` reviewed; no durable decision changed
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
