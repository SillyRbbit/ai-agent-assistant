# Browserslist 4.28.7 security remediation post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment browserslist-4-28-7-security-remediation",
    "npm audit --audit-level=low --json",
    "npm ls browserslist baseline-browser-mapping caniuse-lite electron-to-chromium node-releases update-browserslist-db --all",
    "npm view browserslist@4.28.7 version license engines dependencies dist.integrity dist.tarball --json",
    "npm update --package-lock-only --ignore-scripts browserslist baseline-browser-mapping caniuse-lite electron-to-chromium node-releases",
    "npm install --package-lock-only --ignore-scripts --no-save browserslist@4.28.7 baseline-browser-mapping@2.10.44 caniuse-lite@1.0.30001806 electron-to-chromium@1.5.393 node-releases@2.0.51 update-browserslist-db@1.2.3",
    "npm install --package-lock-only --ignore-scripts",
    "npm ci --ignore-scripts",
    "npm audit --omit=dev --audit-level=low --json",
    "git diff --exit-code e396529246f96a376fdd959a6ffadb71e996bce4 -- package.json",
    "git diff --exit-code e396529246f96a376fdd959a6ffadb71e996bce4 -- src src-tauri .github package.json",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py",
    "gh pr checks 102 --watch --interval 10",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment browserslist-4-28-7-security-remediation --report docs/reviews/2026-09-02-browserslist-4-28-7-security-remediation-post-increment-review.md",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/browserslist-4-28-7-security-remediation.md",
    "docs/plans/2026-09-02-browserslist-4-28-7-security-remediation.md",
    "docs/reviews/2026-09-02-browserslist-4-28-7-security-remediation-post-increment-review.md"
  ],
  "findings": [],
  "increment_id": "browserslist-4-28-7-security-remediation",
  "manual_verification": [
    {
      "check": "Independent readiness review of the bounded dependency remediation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent architecture review of the complete dependency diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent security and code review of the complete dependency diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Independent technical-debt and successor-readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Every classifier-selected PR check on remediation commit 8570034397e273af660a95af5a62e56f74ddc142",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm ci --ignore-scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm ls browserslist baseline-browser-mapping caniuse-lite electron-to-chromium node-releases update-browserslist-db --all",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low --json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --omit=dev --audit-level=low --json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code e396529246f96a376fdd959a6ffadb71e996bce4 -- package.json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code e396529246f96a376fdd959a6ffadb71e996bce4 -- src src-tauri .github package.json",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "gh pr checks 102 --watch --interval 10",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-02
Increment: Browserslist 4.28.7 security remediation for PR #102
Branch: `codex/gui-operations-workspace-redesign`
Baseline: `e396529246f96a376fdd959a6ffadb71e996bce4`

## Executive summary

The bounded lockfile remediation resolves both High Browserslist advisories by
advancing the existing development-only transitive from 4.28.2 to the first
patched release, 4.28.7. Exactly four existing support nodes move to the floors
published by 4.28.7. `package.json`, direct and parent dependencies, lockfile
topology, install-script allowlist, audit policy, source, and product behavior
remain unchanged.

Local supply-chain and complete behavioral verification pass, independent
review has no blocking finding, and every classifier-selected check passes on
exact published remediation
`8570034397e273af660a95af5a62e56f74ddc142`. The completion result is `PASS
WITH ADVISORIES`; the sole advisory is D-111's unchanged Blocked successor
readiness. The deterministic finalizer accepted the report and live status is
complete with a valid workspace fingerprint. PR #102 remains open. Its
separately authorized squash merge is a publication operation, not another
increment, and remains conditional on the final closeout head's checks.

## Scope and boundaries

The final lockfile changes only these five existing top-level development
nodes:

- `browserslist` 4.28.2 -> 4.28.7
- `baseline-browser-mapping` 2.10.38 -> 2.10.44
- `caniuse-lite` 1.0.30001799 -> 1.0.30001806
- `electron-to-chromium` 1.5.375 -> 1.5.393
- `node-releases` 2.0.47 -> 2.0.51

`update-browserslist-db@1.2.3` remains unchanged. The dependency path remains
`@vitejs/plugin-react@4.7.0` -> `@babel/core@7.29.7` ->
`@babel/helper-compilation-targets@7.29.7` -> Browserslist. The parent range
`^4.24.0` admits 4.28.7.

`package.json` is byte-for-byte identical to baseline. There is no direct
Browserslist declaration, durable override, package addition/removal, nested
resolution, install-policy change, or parent movement. No application, test,
Rust/Tauri, workflow, IPC, persistence, provider, network, permission,
approval, audit, execution, or device-authority path changed.

The overall increment contains 12 unique paths: the published lockfile plus 11
plan, security, current-state, troubleshooting, and review documents. Because
remote proof required publishing the lockfile first, the machine manifest
correctly records the final 11-document Git change set present at finalization.

## Resolver guard and provenance

The first named package-lock-only update was not sufficiently exact: npm chose
Browserslist 4.28.8, later versions of the four support packages, and
`update-browserslist-db@1.3.2`. The scope guard stopped the operation, and a
targeted patch restored only that resolver-created delta. An exact `--no-save`
package request did not replace the broad lock selection and was also discarded.

Temporary exact resolver inputs then constrained npm to Browserslist 4.28.7 and
the four authorized floors while `npm install --package-lock-only
--ignore-scripts` generated the final metadata. The temporary inputs were
removed immediately. No override remains, `package.json` matches baseline, and
the subsequent scripts-disabled clean install resolves the exact ordinary
graph through the unchanged parent constraints. This is a resolver-generated
lockfile, not a hand-authored resolution.

## Verification results

- Scripts-disabled clean install: `Passed`; 296 packages installed, 297
  audited, zero vulnerabilities.
- Exact installed graph: `Passed`; all six relevant nodes resolve at the
  expected versions with no invalid, missing, or extraneous package.
- Lock and installed metadata: `Passed`; the five moved nodes are
  development-only, SHA-512 integrity-bound, registry-resolved, compatible
  with supported Node 22/24/26, and have no install lifecycle hook.
- License review: `Passed`; Browserslist is MIT, baseline-browser-mapping is
  Apache-2.0, caniuse-lite is CC-BY-4.0, electron-to-chromium is ISC, and
  node-releases is MIT. These license families were already present.
- Full npm audit: `Passed`; zero Critical, High, Moderate, Low, or total
  vulnerabilities.
- Production-only npm audit: `Passed`; zero vulnerabilities.
- Dependency counts remain 29 production, 327 development, 63 optional, 8
  peer, and 355 total.
- Manifest, source, native, and workflow guards: `Passed`; no diff from
  baseline in `package.json`, `src/`, `src-tauri/`, or `.github/`.
- Complete `npm run verify`: `Passed`; formatting, repository policy, strict
  frontend/Rust lint, 74 hook tests, 80 repository tests, 22 frontend files and
  370 tests, Rust library/integration tests, typecheck, Vite builds, and Tauri
  release no-bundle build passed. The one intentional opt-in Hermes probe was
  ignored.
- CI run `33694943603`, classifier job `100461696363`: `Passed` in 27s.
- CI run `33694943603`, Linux Rust job `100461917260`: `Passed` in 5m21s.
- CI run `33694943603`, target-Mac Rust job `100461917294`: `Passed` in 2m45s.
- CI run `33694943603`, dependency/secret job `100461917273`: `Passed` in
  3m45s, including secret scan, zero-finding npm audit, and the unchanged
  accepted Rust advisory-baseline gate.
- CI run `33694943603`, frontend job `100461917319`: `Passed` in 1m14s.
- Documentation run `33694943605`, job `100461695987`: `Passed` in 30s.
- Final documentation, repository, security-pattern, whitespace, session-end,
  and marker checks: `Passed`.

## Architecture findings

`PASS`. No Critical, High, Medium, Low, or architecture-specific Advisory
finding remains. Package count and topology are unchanged, supported Node
targets remain compatible, and no module ownership or trust boundary moved.

## Security findings

`PASS`. GHSA-c83g-rgw3-j3cx / CVE-2026-73089 and
GHSA-73wf-gq98-2v4g / CVE-2026-73088 are resolved without an exception,
threshold change, install hook, native module, durable override, or new
authority path. Full and production-only npm audits are zero. The accepted Rust
advisory baseline remains separately governed and unchanged.

## Code-health findings

`PASS`. The exact graph is valid through the unchanged parent range, complete
frontend/Rust/Tauri verification passes, and final project memory corrects the
interim publication wording identified during review. No code-health finding
remains.

## Technical debt

No technical debt was introduced. The vulnerable Browserslist transitive is
resolved rather than deferred. Existing GUI harness, Graph composition, and
legend-wording advisories remain historical, nonblocking, and unchanged by
this dependency increment.

## Roadmap findings

The remediation is complete and creates no successor authority. D-111's ten
product/operational blockers remain unchanged, no new plan is owner-selected or
Ready, and next-increment readiness is `Blocked`. Only the already authorized
PR #102 squash merge may proceed after the final closeout-head checks pass.

## Completion decision

`PASS WITH ADVISORIES`

All required local, independent, and exact-remediation-head evidence passes.
No Critical, High, Medium, Low, dependency-specific Advisory, security,
architecture, code-health, or introduced-debt finding remains. The sole
advisory is that next-increment readiness stays Blocked under D-111.

## Next-increment readiness

`Blocked`. Perform only the already authorized PR #102 squash merge after the
valid marker and final closeout publication checks are confirmed. Begin no new
implementation without separate owner selection, approval, and readiness
review.

## Exact files changed

The post-increment finalizer's complete current Git change set is:

1. `CHANGELOG.md`
2. `HANDOFF.md`
3. `NEXT_STEPS.md`
4. `PLANS.md`
5. `PROJECT_STATUS.md`
6. `ROADMAP.md`
7. `SECURITY.md`
8. `TROUBLESHOOTING_LOG.md`
9. `docs/increments/browserslist-4-28-7-security-remediation.md`
10. `docs/plans/2026-09-02-browserslist-4-28-7-security-remediation.md`
11. `docs/reviews/2026-09-02-browserslist-4-28-7-security-remediation-post-increment-review.md`

Published remediation `8570034` contains the other overall increment path,
`package-lock.json`.

## Exact commands executed

The machine manifest records every required local, remote, and deterministic
gate command and its passing status. The Resolver guard and provenance section
also records both rejected resolver attempts so future work does not repeat the
over-broad update.
