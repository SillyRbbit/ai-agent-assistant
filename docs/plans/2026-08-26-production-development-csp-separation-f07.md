# F-07 production/development CSP separation

Status: Verified complete with advisories
Owner: Project owner
Last updated: 2026-08-26

## Goal

Separate the Tauri production and development Content Security Policies so the
production WebView no longer permits the Vite development WebSocket or inline
scripts.

## User-visible outcome

The target-Mac development app and release build render and retain their
existing app-info and Command Center behavior while the production WebView has
a narrower network and script policy.

## Scope

- Remove `ws://localhost:1420` from production `connect-src`.
- Remove `'unsafe-inline'` from production and development `script-src`.
- Add an exact `devCsp` whose only production-policy widening is the local Vite
  WebSocket source.
- Retain `'unsafe-inline'` for `style-src` because current React/React Flow
  rendering uses element style attributes.
- Update the F-12 repository-health guard and focused tests to enforce both
  exact CSPs.
- Reconcile current architecture and project-memory statements from observed
  evidence.

## Explicit non-goals

- No Tauri command, event, capability, permission, or IPC change.
- No agent UI connection, runtime wiring, provider, model, external framework,
  network API, credential, tool, approval dispatch, persistence, filesystem
  access, background work, or device effect.
- No React/CSS refactor to remove current element style attributes.
- No dependency, lockfile, GitHub Actions, signing, notarization, or release
  packaging change.
- No F-08 runtime IPC narrowing or later interactive-demo implementation.

## Existing behavior and constraints

- `src-tauri/tauri.conf.json` currently applies one CSP to development and
  production, including `ws://localhost:1420` and inline-script permission.
- The pinned Tauri 2 schema and resolved crates support `app.security.devCsp`.
- Tauri's default asset CSP nonce/hash modification remains enabled.
- `get_app_info` remains the only custom invoke command, and the main window
  retains only `core:default` capability permission.
- F-12 intentionally freezes the pre-F-07 CSP and must be updated atomically.

## Current-state evidence

- Clean synchronized baseline: `2a2fa01f7632177b75c4b87268dea867af606f36`.
- Prior `runtime-start-containment-f01-f02` marker: complete and valid with
  `PASS WITH ADVISORIES`.
- Baseline repository tests pass 44/44; repository, documentation, and diff
  checks pass.
- The generated frontend HTML uses external module and stylesheet assets.
- `OperationalTopologyAdapter.tsx` contains two current React element style
  attributes, so removing inline styles is outside this bounded change.

## Files expected to change

- `src-tauri/tauri.conf.json`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `ARCHITECTURE.md`
- `docs/plans/2026-08-26-production-development-csp-separation-f07.md`
- `docs/increments/production-development-csp-separation-f07.md`
- `HANDOFF.md`
- `PROJECT_STATUS.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `CHANGELOG.md`
- `docs/reviews/2026-08-26-production-development-csp-separation-f07-post-increment-review.md`

`DECISIONS.md` changes only if implementation evidence requires a new durable
policy decision. `TROUBLESHOOTING_LOG.md` changes only if a failure requires a
durable troubleshooting record.

## Affected components

- Tauri production and development WebView security configuration.
- Static UI/native-boundary repository health checks.
- Current-state architecture and handoff evidence.

## Interfaces and invariants

- Production `connect-src` is exactly `'self' ipc: http://ipc.localhost`.
- Development `connect-src` adds only `ws://localhost:1420`.
- Both script policies are exactly `'self'`, without inline scripts, eval,
  wildcard, data, remote, or development sources.
- Existing font, image, style, and default sources remain unchanged.
- Tauri asset CSP modification is not disabled.
- A missing, malformed, swapped, or broadened production/development policy
  fails repository health.
- No model, runtime, agent, WebView content, or caller selects either policy.

## Implementation milestones

- [x] Start the deterministic increment gate from the clean synchronized base.
- [x] Add exact production and development CSP configuration.
- [x] Update the static guard and adversarial tests.
- [x] Run focused checks and inspect the complete source/configuration diff.
- [x] Complete target-Mac development, release, app-info, and Command Center
      manual checks.
- [x] Run full verification, reviews, documentation synchronization, and gate
      finalization.

## Security and privacy considerations

This increment narrows the untrusted WebView boundary. It creates no new input,
output, data store, credential path, permission, executor, audit channel, or
network operation. Production loses a local-network destination and inline
script execution surface. Development retains only the fixed local Vite
WebSocket needed for HMR. The retained inline-style allowance is explicit and
does not authorize script execution.

## Test plan

- Accept the exact paired production/development policy.
- Reject a production Vite WebSocket source.
- Reject production or development inline-script permission.
- Reject a missing, malformed, or broadened development policy.
- Preserve all existing UI/native import, capability, and command guard tests.
- Build and launch both target-Mac development and release variants.
- Smoke the app-info connection and deterministic Command Center rendering.

## Verification commands

```bash
npm run test:repository
npm run repository:check
npm run security:scan
npm run verify
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Required target-Mac manual checks:

- `npm run tauri -- dev` launches, renders, and connects the Vite client without
  a CSP error.
- The development Command Center renders through the exact Vite URL with the
  HMR client connected and no warning/error console output.
- `npm run tauri -- build --no-bundle` succeeds, and a local ignored release
  app launches with app-info and Command Center behavior intact.
- The effective production policy contains no Vite WebSocket or inline-script
  allowance.

## Risks

- An incorrect production policy can blank the release WebView or break IPC.
- An incorrect development policy can prevent Vite loading or HMR connection.
- Removing inline scripts can expose an undocumented runtime requirement.
- Removing inline styles without a separate UI refactor would break current
  topology rendering, so that hardening remains out of scope.

## Rollback or failure strategy

Before publication, revert only this increment's bounded files. After
publication, use a narrow corrective revert. Never reset, rebase, clean, or
discard unrelated work. Stop instead of broadening production CSP when either
development or release verification fails.

## Decisions made

- Use Tauri's installed `devCsp` configuration surface; add no new config file,
  script, dependency, or custom loader.
- Retain inline styles for current element-style compatibility and remove
  inline scripts from both policies.

## Discoveries

- The local Vite configuration uses fixed port 1420; its optional remote-host
  port behavior is already unsupported by the existing CSP and is not widened.
- Computer Use can inspect the locally bundled release `.app` but cannot
  enumerate Tauri's unbundled debug executable. Development evidence therefore
  combines the successful Tauri process launch with Browser Control's rendered
  Vite/HMR/console smoke rather than direct debug-WebView accessibility output.
- One full verification attempt hit a transient Cargo incremental-cache
  working-directory error. The exact strict Clippy command and a fresh complete
  verification passed unchanged; no cache deletion or source workaround was
  used.

## Progress

- 2026-08-26: Owner approved the exact readiness-reviewed plan. Created the
  bounded branch and began gate `production-development-csp-separation-f07`.
- 2026-08-26: Implemented the exact paired policies and static regression
  checks. Focused/full automation and target-Mac development/release evidence
  pass; documentation and consolidated closeout are synchronized.

## Acceptance criteria

- [x] Production CSP contains no development WebSocket source or inline-script
      allowance.
- [x] Development CSP differs from production only by the fixed local Vite
      WebSocket source.
- [x] Static checks enforce both exact policies and their negative cases.
- [x] Target-Mac development launch/HMR/Command Center and direct release
      app-info/Command Center checks pass without observed CSP violations.
- [x] Complete required verification and review pass.
- [x] Current-state documentation and the deterministic completion marker are
      synchronized and valid.

## Final results

Production `connect-src` is exactly `'self' ipc: http://ipc.localhost` and both
script policies are exactly `'self'`. Development adds only the fixed local
Vite WebSocket. The static guard and three new adversarial tests enforce the
separation and default Tauri asset-CSP modification. Focused tests pass 47/47,
complete verification passes, and target-Mac development/release smoke evidence
passes. The result is `PASS WITH ADVISORIES` for retained inline styles, split
debug-process/browser evidence, and the absence of a separately approved Ready
F-08 plan.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `CHANGELOG.md`
- [x] `ARCHITECTURE.md`
- [x] Increment and post-increment review records
