# PR #57 transitive development-dependency advisory remediation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-25
Gate ID: `pr57-transitive-advisory-remediation`
Branch: `codex/native-multi-agent-end-to-end-demonstrations`
Baseline: `7b5b7e65d1a06de579d75b0727ecec3638c94fea`
Pull request: `#57`

## Goal

Restore the required zero-finding JavaScript dependency audit for PR #57 by
remediating five package-level findings across six development-only transitive
nodes within their existing parent constraints, without adding a direct
dependency, changing a parent package, or changing production architecture or
behavior.

## User-visible outcome

None. This is a bounded development-tooling supply-chain correction. The
deterministic nine-agent demonstrations, native application, and Command Center
UI remain unchanged.

## Scope

- Resolve both vulnerable `brace-expansion` branches from `1.1.15` to `1.1.18`
  and from `5.0.7` to `5.0.9`.
- Resolve `js-yaml` from `4.2.0` to `4.3.1`.
- Resolve `nanoid` from `3.3.16` to `3.3.18`.
- Resolve `postcss` from `8.5.19` to `8.5.26`.
- Resolve `undici` from `7.28.0` to `7.29.0`.
- Retain the existing direct dependency and dev-dependency versions, existing
  parent graph, lockfile version, peer graph, engine policy, and install-script
  allowlist.
- Record exact audit, lockfile, license, install, test, review, and CI evidence.

## Explicit non-goals

- No major, direct, or parent dependency upgrade.
- No new dependency, override, registry, package-manager, engine, install
  script, lifecycle permission, workflow, or audit-policy change.
- No `npm audit --force`, advisory exception, allowlist, severity downgrade,
  audit-level weakening, lockfile hand-edit that is not resolver-generated, or
  CI bypass.
- No Rust, Tauri, React, fixture, test-contract, agent, runtime, provider, tool,
  approval, policy, audit, memory, document, IPC, capability, permission,
  network, or device-effect implementation change.
- No rewrite of historical dependency evidence. Current-state records receive
  additive superseding evidence only.
- No merge until this increment is complete and valid and every applicable PR
  check passes on the exact final head.

## Existing behavior and constraints

- `package.json` pins the existing direct graph exactly and declares npm
  `>=10 <12`; the active package manager is `npm@11.16.0`.
- The lockfile marks all six nodes associated with the five vulnerable
  package-level findings as development-only transitives. Production
  dependencies are not implicated.
- The required CI job runs `npm audit --audit-level=low`; it must pass without
  an exception.
- Repository install-script policy permits only the unchanged exact
  `esbuild@0.28.1` and `fsevents@2.3.3` scripts.
- The completed portability gate and exact correction head already pass Linux,
  target-Mac, frontend, and documentation validation.
- Native remains sole/default and unwired; Hermes remains Deferred/Blocked.

## Current-state evidence

- The worktree was clean at
  `7b5b7e65d1a06de579d75b0727ecec3638c94fea` when this gate began.
- Fresh `npm audit --audit-level=low --json` reports five vulnerable
  package-level findings across six lockfile nodes: four High and one Moderate,
  zero Critical, all indirect, and all `fixAvailable: true`.
- Vulnerable paths are `eslint` -> `minimatch` -> `brace-expansion`,
  `@eslint/eslintrc` -> `js-yaml`, `typescript-eslint` ->
  `typescript-estree` -> `minimatch` -> nested `brace-expansion`, `vite` ->
  `postcss` -> `nanoid`, and `jsdom` -> `undici`.
- Fresh registry metadata reports the exact in-range wanted resolutions listed
  in Scope. Later majors exist but are explicitly outside scope.
- PR #57 dependency job `98035560426` on CI run `32921400121` fails only at the
  npm audit step; repository secret scanning passes.

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
docs/increments/pr57-transitive-advisory-remediation.md
docs/plans/2026-08-25-pr57-transitive-advisory-remediation.md
docs/reviews/2026-08-25-pr57-transitive-advisory-remediation-post-increment-review.md
```

`package.json`, `DECISIONS.md`, `ARCHITECTURE.md`, `SECURITY_CHECKLIST.md`, and
all application source remain unchanged unless evidence invalidates this plan.
Any such discovery requires a plan update and renewed scope review before an
edit.

## Affected components

- npm lockfile resolution for existing lint, build, DOM-test, and TypeScript
  tooling.
- CI dependency-audit evidence.
- Current repository security and increment records.

## Interfaces and invariants

- Direct dependency names and exact versions remain byte-for-byte unchanged.
- Parent packages remain `eslint@9.39.4`, `@eslint/eslintrc@3.3.5`,
  `typescript-eslint@8.64.0`, `vite@7.3.5`, and `jsdom@29.1.1`.
- Both `brace-expansion` major lines remain separate and semver-compatible with
  their existing `minimatch` consumers.
- Every changed lock entry remains development-only and retains registry
  integrity metadata.
- The resolver may change only the six package entries named in Scope; any
  unrelated package movement is a stop condition.
- Install scripts stay disabled during the bounded resolver operation; the
  repository allowlist and package lifecycle policy do not change.
- `npm audit --audit-level=low` must report zero vulnerabilities at completion.
- All product, governance, runtime, and execution boundaries remain unchanged.

## Implementation milestones

- [x] Obtain explicit owner approval and begin the dedicated gate.
- [x] Capture exact audit paths, advisory ranges, and safe in-range candidates.
- [x] Record this plan and increment boundaries before dependency edits.
- [x] Run the npm resolver only for the six named transitive entries with
      install scripts disabled; reject unrelated lockfile movement.
- [x] Verify exact graph, integrity, licenses, manifest/lock synchronization,
      zero audit findings, and complete repository behavior.
- [x] Complete interim independent security, code-health, architecture, debt,
      and readiness reviews before publication.
- [x] Publish the reviewed remediation and require every applicable PR check to
      pass on the exact head.
- [x] Synchronize final evidence and finalize a valid marker before merge.

## Security and privacy considerations

This increment changes fetched development tooling, so registry metadata,
tarball integrity, advisory results, licenses, transitive graph, and lifecycle
scripts are untrusted supply-chain inputs. The bounded update must use the
existing npm registry and lockfile resolver, retain integrity hashes, disable
install scripts during resolution, introduce no package addition or parent
change, and inspect the complete lockfile diff. No secret, token, credential, production content,
personal data, provider call, or external-system write participates. Network
access is limited to public npm metadata and packages required by the approved
dependency workflow.

## Test plan

- Prove only the six named vulnerable resolutions moved and `package.json`
  remained unchanged.
- Prove both installed and locked dependency paths resolve to the expected safe
  versions with no invalid, extraneous, or missing entry.
- Prove all changed entries remain development-only and their resolved registry
  URLs, integrity hashes, and licenses are present and reviewed.
- Run `npm audit --audit-level=low` and production-only audit; require zero
  findings in both.
- Run clean-lock synchronization evidence, frontend lint/tests/build, full Rust
  and repository verification, and the dependency-sensitive completion gate.
- Publish only after independent supply-chain/security review, then require the
  exact published remediation head's dependency, frontend, Linux Rust,
  target-Mac Rust, and documentation checks as classified by repository
  policy. Require the later closeout-docs head's applicable documentation
  check before merge.

## Verification commands

```bash
npm ls brace-expansion js-yaml nanoid postcss undici --all
npm audit --audit-level=low --json
npm audit --omit=dev --audit-level=low --json
npm ci --ignore-scripts
npm run verify
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

The living Progress section records the exact bounded resolver and metadata
commands after they run. Remote evidence requires every applicable PR check on
the exact published remediation head.

## Risks

- A broad resolver operation could update unrelated packages. Control: name
  only the six transitive entries, disable scripts, inspect the full lockfile
  diff, and stop on any unrelated movement.
- A direct override could hide incompatible parent constraints. Control: use
  only in-range resolver results and leave `package.json` unchanged.
- A package may be audit-clean but behaviorally incompatible. Control: retain
  major lines and parent graph, run frontend/tooling tests and the full
  repository verification, then require dual-platform CI.
- Registry or advisory data can change between local and remote checks.
  Control: record exact versions/integrities, rerun the audit at closeout, and
  require the final PR dependency job.
- An install lifecycle could execute unreviewed code. Control: use
  `--ignore-scripts` for resolver and synchronization checks and do not expand
  the existing allowlist.

## Rollback or failure strategy

Before publication, reverse only the resolver-generated lockfile patch and this
increment's evidence edits with targeted patches; do not reset or clean the
worktree. After publication, revert only the bounded remediation commit. A
rollback restores the disclosed audit failure but changes no product data,
credential, migration, or external system. Stop without merging if the
resolver changes an undeclared package, an audit remains nonzero, a required
test fails, or a final PR check is not green.

## Decisions made

- Project-owner approval authorizes this as the second separate PR #57
  remediation after the portability marker became complete and valid.
- In-range transitive resolution is preferred over direct overrides or parent
  upgrades. No durable architecture or policy decision changes.
- PR #57 may be squash-merged only after this gate and exact-head checks pass.

## Discoveries

- Advisory data advanced after the Command Center checkpoint: current fixed
  floors are `brace-expansion@1.1.18` and `5.0.9`, `js-yaml@4.3.1`,
  `nanoid@3.3.18`, `postcss` above `8.5.22`, and `undici@7.29.0`.
- The current parent constraints already admit safe patch/minor resolutions;
  no direct or parent upgrade is presently required.

## Progress

- 2026-08-25: Gate `pr57-transitive-advisory-remediation` began on clean
  baseline `7b5b7e65d1a06de579d75b0727ecec3638c94fea` after the first remediation
  marker and its exact-head documentation workflow passed.
- 2026-08-25: Captured fresh audit and registry evidence and recorded this plan
  before dependency edits.
- 2026-08-25: Ran
  `npm update --package-lock-only --ignore-scripts brace-expansion js-yaml nanoid postcss undici`.
  The resolver changed exactly the six declared lockfile nodes and their own
  metadata; no package was added or removed, and `package.json` and every parent
  remain unchanged.
- 2026-08-25: `npm ci --ignore-scripts` installed the exact graph. `npm ls`
  reports `brace-expansion@1.1.18` and `5.0.9`, `js-yaml@4.3.1`,
  `nanoid@3.3.18`, `postcss@8.5.26`, and `undici@7.29.0` with no invalid or
  extraneous package. Every node remains development-only, MIT, integrity-bound,
  engine-compatible, and without an install hook. The only install-script
  nodes remain the existing allowed `esbuild@0.28.1` and optional
  `fsevents@2.3.3`.
- 2026-08-25: Full and production-only npm audits both report zero
  vulnerabilities with the unchanged 29 production, 327 development, 63
  optional, 8 peer, and 355 total dependency counts. Complete `npm run verify`
  passes against the clean installed graph. Independent architecture and
  security/code reviews pass with no remaining finding after two wording
  corrections. Exact-head CI remains pending.
- 2026-08-25: Published reviewed remediation
  `c3cc49ee28444397ac957d7279ddcfb3ce608548`. CI run `32923751481` passed
  classifier job `98042347127` in 9s, target-Mac Rust job `98042378918` in
  2m12s, Linux Rust job `98042378943` in 6m38s, frontend job `98042378946` in
  1m10s, and dependency/secret-audit job `98042378964` in 4m39s. Documentation
  run `32923751571`, job `98042347401`, passed in 27s. The dependency job
  passed repository secret scanning, the full npm audit, and the unchanged
  accepted Rust advisory-baseline gate.
- 2026-08-25: The completion report is `PASS WITH ADVISORIES`; all local,
  independent, and exact-remediation-head evidence passes with no remaining
  dependency, security, architecture, or code-health finding. The sole
  advisory is that no next implementation plan is owner-selected or Ready.
  Final current-tree documentation, repository, security, whitespace, and
  session-end checks pass. The deterministic marker is complete and valid.

## Acceptance criteria

- [x] `package.json` and every direct/parent dependency remain unchanged.
- [x] Only the six declared lockfile package entries move to the exact safe
      in-range versions, with development flags and integrity metadata intact.
- [x] Installed and locked dependency graphs contain no invalid, missing,
      extraneous, or vulnerable named resolution.
- [x] JavaScript full and production-only audits report zero vulnerabilities.
- [x] Install-script policy, licenses, engine policy, and lockfile version are
      unchanged or explicitly verified compatible.
- [x] Complete local verification and independent security/architecture/code/
      debt/readiness reviews contain no blocking finding.
- [x] Every applicable source check passes on the exact published remediation
      head.
- [x] The final report is `PASS WITH ADVISORIES` and the marker is complete and
      valid before merge.

## Final results

The resolver changed only the six declared development-only nodes. The manifest
and parent graph remain unchanged; exact metadata and installed-graph checks,
both zero-finding npm audits, complete `npm run verify`, and independent review
pass. The exact published remediation head passes every classifier-selected PR
check, including the unchanged accepted Rust advisory baseline. The completion
report is `PASS WITH ADVISORIES`; the sole advisory is that no next
implementation plan is owner-selected or Ready. The deterministic marker is
complete and valid. Only the separately authorized squash merge remains after
the closeout-docs check.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `DECISIONS.md` reviewed; no durable decision changed
- [x] `CHANGELOG.md`
- [x] `TROUBLESHOOTING_LOG.md`
