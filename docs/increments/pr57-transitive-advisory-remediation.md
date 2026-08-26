# PR #57 transitive development-dependency advisory remediation

Status: Active
Owner: Project owner
Date: 2026-08-25
Gate ID: `pr57-transitive-advisory-remediation`
Plan:
[`2026-08-25-pr57-transitive-advisory-remediation.md`](../plans/2026-08-25-pr57-transitive-advisory-remediation.md)
Baseline: `7b5b7e65d1a06de579d75b0727ecec3638c94fea`
Pull request: `#57`

## Goal

Clear the required npm audit by advancing only six vulnerable development-only
transitive lockfile entries to safe versions admitted by their existing parent
constraints.

## Approved scope

- `brace-expansion` `1.1.15` -> `1.1.18` and `5.0.7` -> `5.0.9`.
- `js-yaml` `4.2.0` -> `4.3.1`.
- `nanoid` `3.3.16` -> `3.3.18`.
- `postcss` `8.5.19` -> `8.5.26`.
- `undici` `7.28.0` -> `7.29.0`.
- Exact lockfile, audit, license, integrity, behavior, security, and dual-runner
  evidence.

## Explicit boundaries

No direct or parent dependency, major version, override, package manifest,
install-script allowlist, workflow, audit policy, product source, governance,
runtime, provider, tool, approval, IPC, permission, network capability, or
external effect is authorized. Do not merge PR #57 until this gate is complete
and valid and every applicable exact-head check passes.

## Current status

The gate and plan are active. Baseline audit evidence reported five vulnerable
package-level findings across six lockfile nodes—four High and one Moderate—all
indirect development transitives. The resolver advanced exactly those six
nodes to the approved safe in-range versions with no manifest or parent change,
package addition/removal, or install-policy change. A scripts-disabled clean
install, exact graph and metadata inspection, full and production-only
zero-finding audits, and complete `npm run verify` pass. Independent
security/code and architecture reviews find no blocker. Exact-head PR checks
remain pending, so the gate stays Active and PR #57 remains unmerged.
