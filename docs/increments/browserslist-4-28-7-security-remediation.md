# Browserslist 4.28.7 security remediation

Status: Verified complete with advisories
Owner: Project owner
Date: 2026-09-02
Gate ID: `browserslist-4-28-7-security-remediation`
Plan:
[`2026-09-02-browserslist-4-28-7-security-remediation.md`](../plans/2026-09-02-browserslist-4-28-7-security-remediation.md)
Baseline: `e396529246f96a376fdd959a6ffadb71e996bce4`
Pull request: `#102`

## Goal

Clear the required JavaScript audit by resolving the existing development-only
transitive Browserslist node at patched version 4.28.7 and moving only its four
necessary existing browser-data support entries.

## Approved scope

- Resolver-generated lockfile movement for `browserslist`,
  `baseline-browser-mapping`, `caniuse-lite`, `electron-to-chromium`, and
  `node-releases`.
- Exact audit, graph, metadata, clean-install, behavior, security, gate, and
  dual-platform CI evidence.
- Required plan, increment, current-state, security, troubleshooting, and
  consolidated review documentation.

## Explicit boundaries

No direct or parent dependency, final package manifest, durable override,
install-script allowlist, workflow, audit policy, application/test source,
Rust/Tauri source, IPC, capability, persistence, provider, network, permission,
approval, execution, or device authority may change. Do not merge PR #102 until
this gate is complete and valid and every applicable check passes on the exact
head.

## Final status

The npm-generated lockfile moves exactly the five authorized development-only
entries, preserves `update-browserslist-db` 1.2.3 and a byte-identical
`package.json`, and passes a scripts-disabled clean install, exact graph and
lifecycle inspection, full and production-only zero-finding npm audits,
protected-path guards, and complete `npm run verify`. Independent architecture,
security, code, debt, and readiness reviews have no blocking finding.

Published remediation `8570034397e273af660a95af5a62e56f74ddc142` passes
every classifier-selected PR check in CI run `33694943603` and Documentation
run `33694943605`. The linked
[`post-increment review`](../reviews/2026-09-02-browserslist-4-28-7-security-remediation-post-increment-review.md)
records exact evidence and a `PASS WITH ADVISORIES` result. The sole advisory is
D-111's unchanged Blocked successor readiness. The gate is complete and valid;
only final closeout-head checks and the separately authorized squash merge
remain.
