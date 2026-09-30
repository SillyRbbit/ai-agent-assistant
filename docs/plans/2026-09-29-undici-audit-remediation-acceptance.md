# Undici audit remediation acceptance

Date: 2026-09-29
Increment: `undici-audit-remediation-acceptance`
Branch: `codex/undici-audit-remediation-acceptance`

## Goal and current evidence

Accept the unchanged nine-path undici 7.29.1 candidate in a separately approved
ordinary increment from main `14e9a6a51a292fccaa867b5666dd382768262abd`.
The predecessor remains failed / FAIL / Blocked / valid. Its required full
verification historically failed E0463/E0432. The preserved external repair
identified LLVM debug stripping producing a misaligned LINKEDIT string pool;
macOS 27's loader rejected the procedural macro. A process-local Cargo
build-dependency strip override corrected the import and native build.
These are inherited observations, not newly executed acceptance checks.

## Exact scope and non-goals

Cumulative scope is eleven paths: package-lock.json; CHANGELOG.md, HANDOFF.md,
NEXT_STEPS.md, PLANS.md, PROJECT_STATUS.md, TROUBLESHOOTING_LOG.md; the original
2026-09-29-undici-audit-remediation plan/review pair; and this acceptance
plan/review pair. Only the six root documents and this new pair may be edited.
Preserve root-document bodies as historical suffixes, original plan/review,
lockfile bytes, package.json and all unrelated product/test/Cargo/governance,
workflow, policy and dependency bytes. No resolution, fixture work, diagnosis,
app launch, provider requests, toolchain installation or publication.

## Interfaces, invariants and risks

The patch stays transitive through jsdom 29.1.1; undici's Node requirement and
semver range must be satisfied. No runtime/permission/interface changes occur.
Native build requires verified installed Rust 1.90.0 and Xcode/SDK 27.0,
process-local deployment target 14.0, offline locked Cargo and
`--config 'profile.release.build-override.strip="none"'`. Do not persist config
or claim default builds fixed. Preserve all old worktrees, artifacts, raw states,
reports, external evidence and all 36 already-prunable registry entries.

## Admission and milestones

1. Verify live main and source records; machine-freeze preservation evidence.
2. Create the one isolated branch/worktree, transfer exactly nine paths without
   gate state, byte-verify, and ordinary begin. Completed.
3. Added acceptance records and executed checks once. Full verify failed at
   repository Python selection; application-suite and native-build phases were
   not reached. Stop condition honored.
4. Passing finalization is prohibited. Validate truthful failure report and
   preserve FAIL / Blocked through ordinary terminal disposition.

## Required validation

- Clean locked npm ci; official undici 7.29.1 metadata and package requirements;
  exact npm tree plus semantic lock diff proving no unrelated version changes.
- Full and production npm audits at low threshold; no new npm findings.
- Focused Agents frontend tests, then full npm run verify with the locked/offline
  Cargo stripping override passed through to Tauri's native build.
- Pinned cargo-audit 0.22.2 with current RustSec data and the unchanged repository
  Cargo gate: preserve its exact accepted quick-xml baseline and eight warnings;
  any new/missing/drifted advisory blocks acceptance.
- Targeted formatting, docs:check, repository:check, security:scan, whitespace,
  exact eleven-path scope, historical suffixes, protected bytes, all predecessor
  fingerprints/raw states/artifacts/evidence and registry preservation.
- Session-end, quality reviews for architecture/security/code health/debt/readiness,
  exact twelve-section report schema, ordinary finalization, complete/valid status,
  and full-payload Stop after report freeze.

## Advisories and stop conditions

Retain D-127, D-128, unverified provider/runtime live success and Codex isolation;
D-125/M1/M2 remain parked. Stop on drift, failed checks, new advisory findings,
incompatible requirements, admission rejection or scope expansion. Preserve a
truthful failure without changing predecessor records, bypassing gates or
starting an automatic successor. No commit, push or publication.

## Rollback and next authority

Preserve all changes on failure; no reset, clean or destructive rollback.
Publication needs explicit owner authorization and exact-head CI/review.

## Actual progress and final results

Admission and byte-preserved transfer passed. Clean npm ci, both npm audits
(zero vulnerabilities), metadata/requirements, exact dependency checks, focused
Agents tests (18/18), and pinned current RustSec/repository gate passed (the exact
accepted two quick-xml vulnerabilities and eight warnings). Required full verify
failed with exit 1 at repository_health.py: Python 3.9.6 does not support
zip(..., strict=True). The task-owned clean PATH omitted the installed Python
3.12 location. Format checks passed before the failure. Lint, full application
suites, type check/frontend build and native build were Not run in this successor.
No rerun or source repair follows. Prior native success remains inherited evidence;
it does not replace the failed acceptance command. FAIL / Blocked; no passing
finalization or completion marker. The existing Python 3.12 route is used only
for truthful failure-disposition checks, which must be recorded as actual
results. All old checkouts/records/artifacts and the lock patch remain unchanged.
