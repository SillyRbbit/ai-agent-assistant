# F-07 production/development CSP separation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-26

## Goal

Separate production and development Tauri CSPs so release builds do not retain
the Vite development WebSocket or inline-script allowances.

## Approved scope

- Narrow production `connect-src` and `script-src`.
- Add an exact local-development `devCsp`.
- Update the F-12 static guard and focused tests atomically.
- Verify target-Mac development and release rendering, app-info, and Command
  Center behavior.
- Reconcile only affected current-state documentation.

## Non-goals

No command, event, capability, permission, IPC, agent UI, provider, model,
credential, tool, execution, approval, persistence, filesystem, dependency,
background autonomy, or device effect is added.

## Baseline

- Branch: `codex/security-csp-separation-f07`
- Commit: `2a2fa01f7632177b75c4b87268dea867af606f36`
- Prior marker: `runtime-start-containment-f01-f02`, complete and valid.
- Gate: `production-development-csp-separation-f07`, complete and valid.

## Acceptance

- [x] Production has no Vite WebSocket or inline-script allowance.
- [x] Development adds only the fixed local Vite WebSocket source.
- [x] Exact positive and adversarial static checks pass.
- [x] Required automated and target-Mac manual checks pass.
- [x] Security, architecture, code-health, debt, and readiness review complete.
- [x] Project memory, report, and deterministic completion marker are valid.

## Results

The paired policies, static regression guard, and documentation reconciliation
are complete. Focused repository tests pass 47/47 and complete `npm run verify`
passes. Target-Mac development launch plus rendered Vite/HMR smoke and direct
native release app-info/Command Center inspection pass. Computer Use could not
enumerate the raw debug executable, so direct debug-WebView accessibility
inspection remains an advisory. The quality result is `PASS WITH ADVISORIES`.

## Evidence

See
[`2026-08-26-production-development-csp-separation-f07.md`](../plans/2026-08-26-production-development-csp-separation-f07.md).
