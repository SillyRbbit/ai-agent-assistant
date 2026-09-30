# Undici audit remediation

Date: 2026-09-29. Increment: `undici-audit-remediation`.
Branch: `codex/undici-audit-remediation`.
Baseline: `14e9a6a51a292fccaa867b5666dd382768262abd`.

## Goal and current-state evidence

Repair the new npm audit finding with undici 7.29.1, preserving the merged
Anthropic/local Agents and accepted Hermes fixture repairs. Main's locked
undici 7.29.0 predates PR #123. CI run 36645343675 failed only Dependency and
secret audit; frontend and both Rust jobs passed. Existing acceptance is
historical evidence, not fresh acceptance of this dependency patch.

Official [release](https://github.com/nodejs/undici/releases/tag/v7.29.1) and
[registry metadata](https://registry.npmjs.org/undici/7.29.1) identify the patch.
jsdom 29.1.1 allows `^7.25.0`; undici retains Node `>=20.18.1` and has no runtime
dependencies. Node 26.3.0 satisfies current requirements. Pinned cargo-audit
0.22.2 was installed in owner-authorized isolated temporary tooling using the
existing Rust 1.90.0 toolchain. No global toolchain/settings were changed.

## Exact scope and non-goals

Exactly nine paths:

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-09-29-undici-audit-remediation.md`
- `docs/reviews/2026-09-29-undici-audit-remediation-post-increment-review.md`
- `package-lock.json`

Only the version, resolved URL and integrity of `node_modules/undici` may change.
Preserve package.json and every other lock entry. No overrides, blanket audit
fixes, source/test/fixture changes, Cargo changes, governance amendments,
workflow changes, UI work, live requests or Git publication.

## Invariants, preservation and rollback

Three existing valid checkouts, their tracked/nonignored bytes, modes, HEADs,
statuses and raw gate states are frozen in external preservation evidence.
All 36 prunable registry entries remain. New root entries preserve old bodies
as exact historical suffixes. Predecessor reports and all protected bytes remain
unchanged. No failed record is reopened. On failure stop and preserve the
truthful result; no automatic rollback or successor.

## Admission and milestones

- Preservation preflight and exact baseline passed.
- Ordinary begin passed before tracked edits; no predecessor gate state copied.
- Three-field lock patch applied; documentation records are initially pending.
- Required validation, engineering review and completion remain pending.

## Required validation

Clean `npm ci`; exact `npm ls jsdom undici --all` and semantic lock comparison;
full and production npm audits with threshold low; focused Agents frontend
regressions; full `npm run verify`; pinned cargo-audit 0.22.2 against current
RustSec data and unchanged repository gate; documentation, repository,
security, whitespace, nine-path/suffix/protected-byte and registry checks;
session, architecture, security, code-health, debt and readiness reviews;
exact twelve-section report validation; ordinary finalization; complete/valid
status; full-payload Stop. No required live/manual product check is introduced
by this development dependency patch. Existing live advisories remain.

Use installed Xcode and macOS SDK 27.0 process-locally for native builds.
Rust application verification runs offline with locked Cargo dependencies.
No missing toolchain installation is authorized. npm installation and advisory
retrieval use only their explicitly authorized network access.

## Risks, decisions and stop conditions

The patch contains security and HTTP behavior fixes used by jsdom; fresh tests
and a clean install must establish compatibility. Do not infer the Rust
provider transport is repaired by this Node development dependency update.
Stop on ref/evidence drift, package incompatibility, another dependency change,
new advisory findings, failed validation, downloads outside authorization,
security failure, missing prerequisites or scope expansion. Retain D-127,
D-128, provider/runtime live-success and Codex-isolation advisories; keep
D-125/M1/M2 parked.

## Progress and final results

Implementation applied; required acceptance is pending. A later additive
result in this same plan must identify actual checks and completion status.
Publication requires separate owner authorization and exact-head CI review.

## Observed validation failure and disposition

Clean install and both npm audits passed with zero findings; installed jsdom
29.1.1 resolves only undici 7.29.1. Focused Agents tests passed 18/18.
The pinned Rust audit gate passed the exact two accepted quick-xml findings and
eight warning records. Full verification passed linting, 74 hook tests,
85 repository tests, 431 frontend tests, 368 Rust library tests, integration
suites (ACP 7/7; Hermes 13 passed with its real-runtime probe intentionally
ignored), type checking and frontend builds. Its required Tauri no-bundle build
failed with exit 1: E0463 could not find zerofrom_derive and E0432 followed in
zerofrom 0.1.8. This is observed compiler evidence, not a confirmed root cause.
No repair, repeat build or application suite follows. Result: FAIL / Blocked.
Passing finalization and complete status will not be attempted. Only truthful
report/documentation, scope, preservation and terminal-failure/Stop validation
remain for this stopped increment. No live activity or Git publication occurred.

Failure-disposition documentation, repository, security, whitespace, exact scope, preservation, registry, session and report-schema checks passed. The frozen report remains FAIL / Blocked; ordinary terminal disposition and Stop are validated afterward against immutable report bytes. No passing finalization is requested.
