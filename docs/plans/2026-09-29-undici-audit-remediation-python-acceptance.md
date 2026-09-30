# Python-corrected undici audit remediation acceptance

Date: 2026-09-29
Increment: `undici-audit-remediation-python-acceptance`
Branch: `codex/undici-audit-remediation-python-acceptance`

## Objective and current evidence

Complete local acceptance of the existing undici 7.29.1 patch from main
`14e9a6a51a292fccaa867b5666dd382768262abd` without changing dependency resolution,
fixtures, product code or governance. Both terminal failed predecessors remain
immutable. The latest full verify failed when a clean PATH selected Python 3.9.6
instead of installed Python 3.12.1. The new sanitized subprocess preflight proved
npm selects Python 3.12.1 and supports strict zip before admission.

The earlier native E0463 root cause and successful remediation are preserved
historical evidence: LLVM debug stripping misaligned a procedural macro's
LINKEDIT string pool, rejected by macOS 27. The supported process-local Cargo
build-dependency stripping override passed the native build. Do not repeat that
diagnosis or claim this successor's acceptance until its full verification passes.

## Exact scope and non-goals

Thirteen cumulative paths: package-lock.json; CHANGELOG.md, HANDOFF.md,
NEXT_STEPS.md, PLANS.md, PROJECT_STATUS.md, TROUBLESHOOTING_LOG.md; both original
2026-09-29 undici-audit-remediation and undici-audit-remediation-acceptance
plan/review pairs; and this Python-acceptance plan/review pair.
Only the six root documents and this new pair may be edited. Preserve historical
root bodies as exact suffixes and lockfile/predecessor evidence bytes exactly.
No product, fixture, Cargo, hook, workflow, policy, script, configuration,
permission, dependency-version or framework changes. No publication or live use.

## Components, interfaces and invariants

The transitive package remains owned by jsdom 29.1.1. Only undici version,
resolved URL and integrity differ from main; every unrelated lock entry and
package.json must match main. No execution, credential or model authority changes.
Preserve all five predecessor worktrees, their changes/raw states/markers and
artifacts, external evidence and all 36 prunable entries. Use machine-generated
preservation records and one new ordinary gate without copied state.

## Environment and reproducible validation

Use an explicit credential-free subprocess environment with installed Rust 1.90.0
first in PATH, then `/Library/Frameworks/Python.framework/Versions/3.12/bin`,
`/usr/local/bin`, `/opt/homebrew/bin` and system tool directories. Verify npm's
Python subprocess route before checks. Set process-local Xcode developer directory,
SDKROOT to its MacOSX27.0.sdk, MACOSX_DEPLOYMENT_TARGET=14.0,
CARGO_NET_OFFLINE=true and an isolated ignored Cargo target directory.
No installation of toolchains or persistent configuration change is authorized.

Required acceptance commands include clean `npm ci`, exact dependency inspection,
full and production npm audits, official metadata verification, pinned
cargo-audit 0.22.2 against current RustSec data and the unchanged exact repository
audit gate. Audit retrieval may use the network; Cargo builds stay locked/offline.

Run the complete command after applicable document formatting:

```sh
npm run verify -- -- --locked --offline --config 'profile.release.build-override.strip="none"'
```

This includes formatting, repository checks, strict lint, hook/repository,
frontend/Rust tests, type checking, frontend build and the native no-bundle build.
Avoid redundant standalone suites already covered by this command. Then require
final documentation/repository/security/whitespace validation, exact scope and
historical/protected byte checks, session-end and quality reviews, twelve-section
report validation, ordinary finalize, complete/valid status and full-payload Stop.
Only actual executed checks may be recorded Passed.

## Milestones

1. Verify baseline, all predecessor evidence and tooling; freeze preservation.
2. Create the single isolated branch, transfer eleven paths and ordinarily admit.
3. Add the exact eight-document successor delta and run acceptance validation.
4. Complete architecture/security/code-health/debt/readiness reviews; freeze report.
5. Finalize only after required checks pass, then verify status and full Stop.

Milestones 1 through 3 passed. Reviews and final documentation/gate validation
are recorded in the report; finalization and Stop follow its freeze.

## Failure handling, risks and rollback

Keep recoverable process-local environment, formatting, assertion/documentation
and report-schema failures in this active increment. Capture evidence, identify
the smallest authorized correction and revalidate only affected checks before
broader completion. Do not repeat identical failed probes without changed inputs
or diagnostic visibility. Do not create a recovery chain or change a gate.

Hard stops: unexplained external drift, admission rejection, destructive action,
unavailable required tooling, new security findings, scope expansion, unauthorized
external/provider access, terminal-history changes or gate weakening. Preserve
all work on a hard stop; never reset, clean or overwrite predecessors.

## Advisories and decisions

Retain D-127's exact accepted two quick-xml vulnerabilities/eight warning tuples;
a passing audit gate is not vulnerability-free Rust. Retain D-128 custody/abort,
native/provider/runtime live-success and Codex-isolation advisories. D-125/M1/M2
remain parked. The native workaround is invocation-local; default stripped builds
are not claimed fixed. Local completion cannot prove remote CI or grant publication.

## Results and next authority

Fresh full verification passed with the corrected Python route and the documented
Cargo stripping override, including strict lint, 74 hook tests, 85 repository
tests, 431 frontend tests, 368 Rust library tests, integration suites (ACP 7/7;
Hermes 13 passed and one intentionally ignored real-runtime probe), frontend
build and Tauri native release build. Clean install, tree/official metadata,
both npm audits and pinned Cargo audit/repository gate passed. The current Rust
baseline remains exactly two accepted vulnerabilities and eight warning records.
No repeated diagnosis or product/fixture repair was needed in this increment.

The report records fresh documentation/security/preservation/session checks as
they actually pass, then the schema/finalization/status/Stop evidence is frozen
externally. A completion marker is valid only when the gate confirms it. Both
failed predecessors remain immutable. Publication needs separate owner approval
and CI/review for the eventual published commit; no remote validation is claimed.
