# Brace-expansion audit remediation

Date: 2026-09-29
Increment: `brace-expansion-audit-remediation`
Branch: `codex/brace-expansion-audit-remediation`

## Objective and current evidence

Repair the npm audit blocker on PR #126 without repeating the completed undici
remediation. Required main is `14e9a6a51a292fccaa867b5666dd382768262abd` and required PR head is `a14b8ce0c022234ceb04558b090adeb79c5fa9ac`.
The completed source remains complete/valid, PASS WITH ADVISORIES. Its CI passed
Documentation, classification, Frontend, Linux Rust and Target-Mac Rust; npm audit
failed on three brace-expansion denial-of-service advisories. Secret scanning
passed; later Rust audit steps were skipped. This does not invalidate the source's
historical local audit evidence or establish a product/native defect.

## Exact scope and non-goals

Nine successor paths: package-lock.json; CHANGELOG.md, HANDOFF.md, NEXT_STEPS.md,
PLANS.md, PROJECT_STATUS.md, TROUBLESHOOTING_LOG.md; this plan and its review.
Exactly fifteen cumulative changed paths against main. Preserve all six inherited
plan/review files and all other product, test, Cargo, configuration, workflow,
hook, skill, governance and security-policy bytes. Preserve every root-document
body as an exact historical suffix. No publication, launch or provider requests.

## Components, interfaces and invariants

Update only version, resolved URL and registry integrity in two lock entries:
node_modules/brace-expansion 1.1.18 to 1.1.21, and
node_modules/@typescript-eslint/typescript-estree/node_modules/brace-expansion
5.0.9 to 5.0.12. Existing minimatch 3.1.5 requires ^1.1.7; minimatch 10.2.5 requires
^5.0.5. Official metadata confirms both ranges, unchanged dependencies, engine
requirements and module exports. Preserve package.json, undici 7.29.1, parent
packages and every unrelated lock field; no audit fix or dependency override.

Official advisories: [CPU complexity](https://github.com/advisories/GHSA-q2hr-2g5m-vwhr),
[nested recursion](https://github.com/advisories/GHSA-qhr7-859c-m2p7), and
[comma parsing](https://github.com/advisories/GHSA-6j4f-fj2g-mc7p).
The targets cover all three; earlier patches leave an advisory unresolved.

## Milestones and validation

1. Verify refs, six existing worktrees/36 prunable entries, source completion and
   terminal predecessors; freeze preservation and official metadata. Passed.
2. Create only the authorized isolated worktree from PR head without gate state
   or ignored outputs; ordinarily admit before edits. Passed.
3. Apply exact lock patch and additive documents; clean npm ci, exact tree and
   full/production audits, bounded offline package and consumer regressions.
4. Focused frontend lint/format, then full npm run verify with locked/offline
   Cargo and the validated process-local stripping override.
5. Pinned cargo-audit 0.22.2/current RustSec and unchanged repository gate;
   documentation/repository/secret/whitespace, scope/preservation/registry,
   session/quality review, twelve-section report schema, ordinary finalization,
   complete/valid status and full-payload Stop.

## Environment and portability

Preflight verified Python 3.12.1 through npm, Node 26.3.0, npm 11.16.0, Rust 1.90.0,
installed Xcode/SDK 27.0 and cargo-audit 0.22.2. Use explicit credential-free
process-local PATH, DEVELOPER_DIR, SDKROOT, MACOSX_DEPLOYMENT_TARGET=14.0,
CARGO_NET_OFFLINE=true and this worktree's isolated ignored target/acceptance.
Use `profile.release.build-override.strip="none"`; default stripped native builds
are not claimed repaired. Do not install toolchains or change global settings.

## Risks, failure handling and rollback

Upstream fixes bound pathological brace input; ordinary alternatives, ranges,
escaping/nesting and both minimatch consumers require fresh checks. Keep any
adversarial check bounded by a child deadline. Local checks do not prove remote CI.
Preserve all artifacts and failed results. Recover ordinary in-scope environment,
formatting, assertion and report-schema issues in this increment; do not repeat
identical failed probes or create a recovery chain. Investigate new security
findings safely within authorization; report unrelated findings without scope
expansion. Stop for unexplained drift, admission rejection, incompatible targets,
necessary scope expansion, unavailable prerequisites needing installation,
destructive action, unauthorized access, unsafe output or gate weakening.
No reset, clean, terminal-record rewrite, publication or automatic rollback.

## Decisions, advisories and results

Retain D-127's exact two accepted quick-xml vulnerabilities/eight warnings, D-128
custody/abort and native/provider/runtime live-success/Codex-isolation advisories.
D-125/M1/M2 remain parked. Existing completed and failed records are immutable.
Fresh clean install, exact dependency/requirements checks, full and production
npm audits (zero vulnerabilities), 42 package/consumer assertions with bounded
advisory checks, focused lint/format and full verification all passed. Full
verification includes application/native builds and unchanged integration suites.
The current RustSec database is f23b768236fe2880e4cfa167da662cad8ca79240; the
unchanged audit gate accepts exactly two vulnerabilities and eight warnings.
First documentation/repository/secret/whitespace/session checks passed. Final
review, documentation/preservation and schema checks are required after this
evidence update; ordinary finalize/status/full Stop follow only actual passes.
No application code, fixture, production permissions or remote state changed.
Historical PR CI is retained; local acceptance is distinct from remote CI.
