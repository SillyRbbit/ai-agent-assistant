# Dependency and Rust audit unblock

Status: Complete pending publication
Owner: Project owner
Last updated: 2026-09-20

## Goal

Clear the current dependency-audit failure with one bounded successor that
combines the already validated JavaScript dependency remediation with an exact
RustSec warning-baseline correction.

## User-visible outcome

Repository and pull-request dependency audits can pass against the current npm
and RustSec advisory data without hiding findings or changing product behavior.

## Scope

- Transfer the already validated package.json and package-lock.json bytes from
  the preserved terminal-failed dependency worktree after proving their base
  blobs still match remote main.
- Update Vitest and its matching package family to 4.1.11,
  baseline-browser-mapping to 2.11.0, and js-yaml to 4.3.2.
- Remove only RUSTSEC-2024-0411 through RUSTSEC-2024-0420 from the exact
  accepted-warning set.
- Add focused tests for the exact eight-warning set and rejection of every
  withdrawn GTK advisory if it reappears.
- Add D-127 and synchronize the bounded current-state documentation.

## Explicit non-goals

- No npm re-resolution, blanket audit fix, independent @vitest/mocker
  override, or unrelated dependency update.
- No Cargo dependency, Rust product, UI, native, workflow, hook, skill,
  harness, permission, or policy weakening.
- No D-125, M1, M2, demo-cleanup implementation, PR #116 update, merge, or
  publication beyond the separately authorized successor PR.
- No claim that GTK 0.18.2 is vulnerability-free.

## Existing behavior and constraints

- Remote main is f176c36cc701b5a296162331cfcb2600b2157663.
- PR #116 remains open at exact head
  92c2e19eb71b08ad7a2996f83e034afe9babd52a with its exact 22-path cleanup
  scope and one failed Dependency and secret audit.
- The preserved dependency-audit-remediation worktree has a valid terminal
  failed / FAIL / Blocked record. This successor does not reopen, rewrite, or
  reclose it.
- The base blobs for package.json and package-lock.json in that worktree match
  current remote main.
- Current cargo-audit 0.22.2 data at advisory database commit
  d5c17953a895cf19e8d3ce66eaa42b6fc1fb16 reports the accepted two
  quick-xml 0.39.4 vulnerabilities and eight current warnings.
- RustSec commit b266fb89baa88c73c6aaa53e0e87509c80bdf962 withdrew the ten
  GTK advisories on 2026-08-14 because the gtk3-rs repository was unarchived,
  README warnings were removed, and development resumed.

## Current-state evidence

- Source package blobs:
  - package.json: 63aa3b902895c55d510d8679cef01d86eca93406
  - package-lock.json: dd30fdbb68ed4aa1b3fa808a1f4f4b23efd2b23a
- Transferred candidate SHA-256:
  - package.json:
    6652d6fd99dc0a34773f127c0b781a4e2c057c8ad50b08d960cf20dce7768423
  - package-lock.json:
    b90e638439baf8183faa7c15576152c7161d6a6001bf4508b8a0dd54b3547adc
- The baseline cargo-audit gate unit suite passed before edits.
- Ordinary gate admission for dependency-and-rust-audit-unblock succeeded in
  the isolated worktree.

## Files expected to change

1. package.json
2. package-lock.json
3. scripts/cargo_audit_gate.py
4. scripts/tests/test_cargo_audit_gate.py
5. DECISIONS.md
6. SECURITY.md
7. TESTING_GUIDE.md
8. CHANGELOG.md
9. HANDOFF.md
10. NEXT_STEPS.md
11. PLANS.md
12. PROJECT_STATUS.md
13. ROADMAP.md
14. TROUBLESHOOTING_LOG.md
15. docs/plans/2026-09-20-dependency-and-rust-audit-unblock.md
16. docs/reviews/2026-09-20-dependency-and-rust-audit-unblock-post-increment-review.md

## Affected components

- JavaScript development/test dependency graph.
- Repository Cargo advisory validation gate and focused tests.
- Current security, testing, decision, status, plan, roadmap, handoff,
  troubleshooting, and changelog records.

## Interfaces and invariants

- ACCEPTED_VULNERABILITIES remains exactly the two current quick-xml 0.39.4
  advisories.
- ACCEPTED_WARNINGS contains exactly the eight current warning tuples.
- Any missing, changed, or new vulnerability or warning still fails closed.
- Every withdrawn GTK advisory is unexpected if cargo-audit reports it again.
- No ignore flag, audit configuration exception, or automatic advisory
  synchronization is introduced.
- Product runtime and trust boundaries remain unchanged.

## Implementation milestones

- [x] Verify remote baseline, PR #116, all worktrees, predecessor failure,
      package blobs, current advisories, and isolated branch.
- [x] Begin ordinary gate admission and transfer exact package bytes.
- [x] Update the exact Rust warning set and focused regression tests.
- [x] Complete documentation synchronization and all required local checks.
- [ ] Finalize a valid passing post-increment record.
- [ ] Commit, push, open the combined PR, and inspect exact-head CI.

## Security and privacy considerations

This is a supply-chain and fail-closed audit-baseline change. It adds no
credential, network path, product authority, filesystem permission, model data,
IPC, runtime capability, or device access. The npm registry and RustSec network
access occur only in explicit verification. Existing accepted findings remain
visible debt.

## Test plan

1. Prove the exact transferred package bytes and dependency versions.
2. Run focused and complete repository tests.
3. Run clean installation, exact tree inspection, full and production npm
   audits, and secret scanning.
4. Run pinned cargo-audit 0.22.2 against current RustSec data and validate the
   exact output through the repository gate.
5. Run complete verification, documentation/repository checks, exact-scope and
   diff checks, session inventory, independent reviews, and the post-increment
   gate.
6. After publication, inspect every exact-head CI job and stop on any failure.

## Verification commands

```bash
python3 -m unittest scripts.tests.test_cargo_audit_gate -v
python3 -m unittest discover -s scripts/tests -p 'test_*.py' -v
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
npm ci
npm ls vitest @vitest/mocker baseline-browser-mapping js-yaml
npm run test:frontend
npm audit --audit-level=low
npm audit --omit=dev --audit-level=low
/private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit audit --json --file src-tauri/Cargo.lock
python3 scripts/cargo_audit_gate.py <report> --cargo-audit-exit 1
npm run security:scan
npm run verify
npm run docs:check
npm run repository:check
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Risks

- Vitest 4 can change discovery or mock behavior; any required source,
  configuration, or test repair outside this scope stops the increment.
- Current advisory data can add or change a finding; any new advisory stops the
  increment.
- Documentation or completion tooling can reveal scope drift; any path outside
  the exact 16-path ceiling stops the increment.

## Rollback or failure strategy

On any required failure, preserve the isolated worktree and record a truthful
terminal failure without weakening a gate or repairing outside scope. Do not
modify the two preserved checkouts or PR #116.

## Decisions made

- Use the byte-identical JavaScript candidate already validated in the
  predecessor worktree.
- Reconcile the Rust baseline to current RustSec output without an ignore or
  generic synchronization mechanism.
- Keep the combined audit successor separate from the completed demo cleanup.

## Discoveries

- RustSec withdrew eleven related gtk3-rs advisories, including
  RUSTSEC-2024-0410; the Cortexa baseline contained only the ten IDs
  RUSTSEC-2024-0411 through RUSTSEC-2024-0420, so only those ten require removal.
- Withdrawal of an unmaintained advisory is maintenance-status evidence, not a
  vulnerability assessment of Cortexa's locked GTK 0.18.2 crates.

## Progress

- 2026-09-20: Verified all baselines and entered ordinary admission.
- 2026-09-20: Transferred package files byte-for-byte and passed the focused
  ten-test Cargo audit gate suite.
- 2026-09-20: Clean installation, exact dependency inspection, frontend,
  repository and hook tests, both npm audits, pinned current Cargo audit and
  exact gate, secret scan, and complete verification passed. The first complete
  verification attempt stopped at plan formatting; the exact in-scope
  Prettier output was applied and the second attempt passed.

## Acceptance criteria

- [x] The exact 16-path scope contains no unrelated change.
- [x] The package graph resolves the four named packages at their approved
      versions with no peer error.
- [x] The two npm audits, secret scan, and exact Cargo audit gate pass.
- [x] Frontend discovery and all repository behavior checks pass.
- [x] D-127 and current documentation remain truthful and additive.
- [ ] Session, quality, report, and post-increment gates pass.
- [ ] The isolated branch is committed and pushed, one PR into main is open,
      and all exact-head CI jobs are inspected without merging.

## Final results

Candidate quality result: PASS WITH ADVISORIES. All required local checks
passed. The accepted two Rust vulnerabilities and eight warnings remain visible
debt, and PR #116 retains its pending native GUI advisory. Final completion is
authoritative only after the ordinary gate reports complete and valid.

## Documentation updates

- [x] HANDOFF.md
- [x] PROJECT_STATUS.md
- [x] NEXT_STEPS.md
- [x] DECISIONS.md
- [x] CHANGELOG.md
- [x] TROUBLESHOOTING_LOG.md
- [ ] Post-increment review
