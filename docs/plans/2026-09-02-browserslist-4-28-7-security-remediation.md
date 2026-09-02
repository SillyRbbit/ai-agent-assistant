# Browserslist 4.28.7 security remediation

Status: In progress
Owner: Project owner
Last updated: 2026-09-02
Gate ID: `browserslist-4-28-7-security-remediation`
Branch: `codex/gui-operations-workspace-redesign`
Baseline: `e396529246f96a376fdd959a6ffadb71e996bce4`
Pull request: `#102`

## Goal

Restore the required zero-finding JavaScript dependency audit for PR #102 by
advancing the existing development-only transitive `browserslist` resolution
from 4.28.2 to the patched 4.28.7 release and moving only the support entries
required by its new in-range dependency floors.

## User-visible outcome

None. This is a development-tooling supply-chain correction. The verified GUI,
native application, canonical data, and runtime behavior remain unchanged.

## Scope

- Resolve `browserslist` from 4.28.2 to exactly 4.28.7.
- Permit resolver-generated movement only for the four existing support entries
  whose minimum ranges rise in 4.28.7: `baseline-browser-mapping`,
  `caniuse-lite`, `electron-to-chromium`, and `node-releases`.
- Preserve the existing `update-browserslist-db` resolution, parent dependency
  graph, direct dependency declarations, lockfile version, package manager,
  engine policy, registry, and install-script allowlist.
- Record exact audit, graph, integrity, license, lifecycle-script, build, test,
  security, gate, and PR-check evidence.

## Explicit non-goals

- No direct dependency, parent package, major version, durable override, final
  package-manifest, registry, package manager, engine, install-script allowlist,
  workflow, or audit-policy change.
- No application/test source, Rust, Tauri, IPC, capability, CSP, persistence,
  provider, network, tool, permission, approval, audit, execution, or device-
  authority change.
- No `npm audit --force`, exception, ignore, severity downgrade, threshold
  weakening, hand-edited lock resolution, or CI bypass.
- No unrelated dependency refresh and no merge until the remediation gate is
  complete and valid and every applicable PR check passes on the exact head.

## Existing behavior and constraints

- PR #102 commit `e396529` passes repository policy, documentation, frontend,
  Linux Rust, and target-Mac Rust checks. Its dependency/secret job fails only
  at `npm audit --audit-level=low`.
- The same lockfile exists on the GUI baseline and resolves the sole affected
  node through `@vitejs/plugin-react` -> `@babel/core` ->
  `@babel/helper-compilation-targets` -> `browserslist@4.28.2`.
- The affected node is development-only. `package.json` does not directly
  declare Browserslist, and the existing parent range `^4.24.0` admits 4.28.7.
- Both reported advisories affect Browserslist through 4.28.6 and identify
  4.28.7 as patched. The current CI policy requires a zero-finding npm audit and
  must not be bypassed.

## Current-state evidence

- The branch was clean at
  `e396529246f96a376fdd959a6ffadb71e996bce4` when this gate began.
- The preceding `gui-conversation-enter-graph-wheel-zoom` marker was
  `complete`, `valid: true`, and `PASS WITH ADVISORIES`.
- Local `npm audit --audit-level=low` reproduces one High package-level finding
  containing GHSA-c83g-rgw3-j3cx and GHSA-73wf-gq98-2v4g; `fixAvailable` is
  true.
- Registry metadata reports Browserslist 4.28.7 as MIT, SHA-512 integrity-
  published, compatible with the existing Node engine range, and free of native
  code or a consumer lifecycle script.
- Readiness is **Ready with advisories**. Owner authorization is the explicit
  bounded security-remediation exception to D-111's no-automatic-successor
  state.

## Files expected to change

Dependency resolution:

```text
package-lock.json
```

Plan, security evidence, current state, and closeout:

```text
CHANGELOG.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
ROADMAP.md
SECURITY.md
TROUBLESHOOTING_LOG.md
docs/increments/browserslist-4-28-7-security-remediation.md
docs/plans/2026-09-02-browserslist-4-28-7-security-remediation.md
docs/reviews/2026-09-02-browserslist-4-28-7-security-remediation-post-increment-review.md
```

`package.json`, `DECISIONS.md`, `ARCHITECTURE.md`,
`SECURITY_CHECKLIST.md`, workflows, hooks, application source, and Rust/Tauri
source remain unchanged. Any contrary discovery stops the increment.

## Affected components

- npm lockfile resolution for existing Babel/Vite development tooling.
- CI dependency-audit evidence.
- Current repository security and increment records.

## Interfaces and invariants

- Direct dependency names and versions remain byte-for-byte unchanged.
- Parent packages and the `browserslist` parent constraint remain unchanged.
- Every moved node remains development-only, registry-resolved, integrity-
  bound, license-reviewed, engine-compatible, and without a new lifecycle
  script.
- The resolver may move only Browserslist and the four named support entries;
  any package addition, removal, nested topology change, or unrelated movement
  is a stop condition.
- `npm audit --audit-level=low` and production-only audit must report zero
  vulnerabilities at completion.
- The previously verified GUI and all product/trust boundaries remain
  byte-for-byte unchanged.

## Implementation milestones

- [x] Obtain explicit owner approval, complete readiness review, and begin the
      dedicated gate
- [x] Capture the exact audit path, advisories, patched version, and required
      support floors
- [x] Record plan/increment boundaries before dependency resolution
- [x] Run the bounded npm resolver with install scripts disabled and inspect
      every lockfile movement
- [x] Verify the exact installed graph, metadata, zero audits, and complete
      repository behavior
- [ ] Complete independent architecture, security, code, debt, and readiness
      reviews
- [ ] Publish the remediation and require every applicable exact-head PR check
      to pass
- [ ] Synchronize final evidence and finalize a valid completion marker before
      squash merge

## Security and privacy considerations

Registry metadata, package tarballs, advisories, integrity values, and lifecycle
metadata are untrusted supply-chain inputs. Use the existing npm registry and
resolver, disable lifecycle scripts during resolution and clean installation,
retain SHA-512 integrity data, inspect every changed lock entry, and require
zero audit findings. No credential, secret, personal content, application data,
provider call, or external-system mutation participates. Network access is
limited to public npm and GitHub metadata required for this approved workflow.

## Test plan

- Prove `package.json`, parent packages, workflows, and application/native
  source remain unchanged.
- Prove only the five allowlisted lock entries move and record their exact
  versions, integrity, license, engine, dependency, and script metadata.
- Run a scripts-disabled clean install and prove the installed graph has no
  invalid, missing, or extraneous package.
- Require full and production-only npm audits to report zero vulnerabilities.
- Run the complete dependency-sensitive repository verification and all
  documentation, security, diff, session, review, and post-increment gates.
- Require every classifier-selected PR check on the exact published remediation
  head before finalization and every final closeout-doc check before merge.

## Verification commands

```bash
npm ls browserslist baseline-browser-mapping caniuse-lite electron-to-chromium node-releases update-browserslist-db --all
npm audit --audit-level=low --json
npm audit --omit=dev --audit-level=low --json
npm ci --ignore-scripts
git diff --exit-code e396529246f96a376fdd959a6ffadb71e996bce4 -- package.json
git diff --exit-code e396529246f96a376fdd959a6ffadb71e996bce4 -- src src-tauri .github package.json
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
gh pr checks 102 --watch --interval 10
python3 .codex/hooks/post_increment_gate.py status
```

## Risks

- Resolver drift could update unrelated packages. Control: name only the
  allowlisted entries, disable scripts, inspect the complete lockfile diff, and
  stop on any unrelated node.
- Browserslist 4.28.7 raises four support floors. Control: permit only those
  necessary in-range data-package movements and record their exact metadata.
- A clean audit does not prove compatibility. Control: run clean install, exact
  graph inspection, full frontend/Rust/Tauri verification, and dual-platform
  CI.
- Advisory data may change again between local and remote checks. Control:
  rerun audits at closeout and require the final PR dependency job.

## Rollback or failure strategy

Before publication, reverse only the resolver-generated lockfile patch and this
increment's evidence edits with targeted patches; never reset or clean the
workspace. After publication, revert only the bounded remediation commit. A
rollback restores the disclosed audit failure but changes no product data or
external system. Stop without merging if the resolver leaves scope, an audit
remains nonzero, a required test fails, or any final PR check is not green.

## Decisions made

- The owner explicitly authorized this separate PR #102 dependency-security
  increment after the GUI commit's audit failure.
- Use a resolver-generated, in-range transitive update; do not leave a direct
  dependency or durable override and do not change the parent graph.
- No durable architecture or product-policy decision changes. PR #102 may be
  squash-merged only after the valid remediation gate and exact-head checks
  pass.

## Discoveries

- Browserslist 4.28.7 retains the MIT license and Node engine range but raises
  minimum ranges for four existing browser-data support packages, so an exact
  one-node-only lockfile update is not valid.
- An unconstrained package-lock-only update selected Browserslist 4.28.8,
  refreshed the four support packages beyond the required floors, and moved
  `update-browserslist-db` to 1.3.2. The scope guard rejected and fully reversed
  that generated delta before verification or publication.
- npm did not preserve exact transitive selections from `--no-save` arguments.
  Temporary exact resolver overrides therefore supplied the five authorized
  versions to `npm install --package-lock-only --ignore-scripts`; they were
  removed immediately. The final `package.json` is byte-for-byte identical to
  baseline, the resulting lockfile has no override or nested topology, and a
  subsequent scripts-disabled clean install validates the ordinary parent
  ranges without an override.

## Progress

- 2026-09-02: PR #102 published GUI commit `e396529`; all checks except the
  dependency/secret audit passed.
- 2026-09-02: Local audit reproduced the newly disclosed High finding, registry
  metadata confirmed 4.28.7 as the patched version, the owner approved a
  bounded remediation, readiness returned Ready with advisories, and gate
  `browserslist-4-28-7-security-remediation` began on a clean branch.
- 2026-09-02: The scope guard rejected an over-broad resolver result, then the
  exact scripts-disabled resolution moved only Browserslist to 4.28.7 and the
  four required support entries to 2.10.44, 1.0.30001806, 1.5.393, and 2.0.51.
  `update-browserslist-db` remains 1.2.3 and `package.json` remains unchanged.
- 2026-09-02: Scripts-disabled `npm ci`, exact graph inspection, metadata and
  lifecycle review, full and production-only zero-finding npm audits, protected-
  path guards, diff hygiene, and complete `npm run verify` pass locally.

## Acceptance criteria

- [x] `package.json`, direct and parent dependencies, workflows, application
      source, and native source remain unchanged
- [x] Only Browserslist and the four necessary support entries move, with no
      package addition/removal or topology drift
- [x] Every moved node remains development-only, integrity-bound, compatible,
      license-reviewed, and free of a new lifecycle script
- [x] Installed graph is valid and full/production npm audits report zero
      vulnerabilities
- [ ] Complete local verification and independent reviews have no blocking
      finding
- [ ] Every applicable PR check passes on the exact published remediation and
      final closeout heads
- [ ] Final report passes and the completion marker is complete and valid
      before squash merge

## Final results

The bounded lockfile implementation and complete local verification pass.
Independent reviews, publication, exact-head CI, final documentation, and gate
finalization remain pending.

## Documentation updates

- [ ] `HANDOFF.md`
- [ ] `PROJECT_STATUS.md`
- [ ] `NEXT_STEPS.md`
- [ ] `PLANS.md`
- [ ] `ROADMAP.md`
- [ ] `SECURITY.md`
- [ ] `DECISIONS.md` reviewed; no durable decision expected
- [ ] `CHANGELOG.md`
- [ ] `TROUBLESHOOTING_LOG.md`
- [ ] Consolidated post-increment review
