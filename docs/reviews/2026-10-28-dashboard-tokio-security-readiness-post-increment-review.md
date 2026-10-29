# Dashboard Tokio security readiness post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dashboard-tokio-security-readiness",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "DECISIONS.md",
    "docs/plans/2026-10-28-dashboard-current-main-integration.md",
    "docs/reviews/2026-10-28-dashboard-tokio-security-readiness-post-increment-review.md"
  ],
  "findings": [],
  "verification": [
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
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-10-28
Increment: dashboard-tokio-security-readiness
Branch: codex/dashboard-main-integration

## Executive summary

Owner-approved preparation for the narrow Tokio1.52.3 to1.53.3 correction. Earlier npm repair and all historical reports remain immutable.

## Scope and boundaries

Only src-tauri/Cargo.toml exact Tokio pin, the corresponding necessary Cargo.lock resolution and additive current governance records. Preserve product/tests/workflows/audit-gate baseline, npm graph/lock, original dashboard checkout/index/state, historical helpers/failures/seals and other worktrees. No new helper/framework, native app launch, retained cleanup or later milestone.

## Verification results

Five documentation/repository/security/whitespace/session preparation checks establish only normal admission readiness. No changed-Cargo verification is claimed yet. Prior npm install/activation/SDK and unchanged source evidence is reused with exact provenance.

## Architecture findings

The minimum forward patched Tokio version is1.53.3. Fixed registry metadata declares rust-version1.71, compatible with the existing pinned1.90 toolchain. Preserve runtime features and all application ownership/IPC boundaries; reject unrelated dependency resolution.

## Security findings

CI exposed RUSTSEC-2026-0386 in exact Tokio1.52.3 after npm audit passed with zero vulnerabilities. Official RustSec lists forward patched versions>=1.53.3. Standard Cargo uses the isolated existing Cargo home, public crates.io only if required source is absent, strict TLS and lock checksums; subsequent full verification is offline. The existing strict audit gate and accepted historical baseline remain unchanged. No exploitability or live/native readiness inference.

## Code-health findings

Inspect every Cargo delta and preserve all unrelated locked entries. No product/test adaptation, generic override, parser/helper or audit waiver. Fresh required verification establishes changed-graph compatibility; historical full results retain original identities.

## Technical debt

Advisory: native app, VoiceOver, hosted authentication and live-provider QA remain unverified/outside scope. Existing bundle/experimental-storage/ignored-test advisories remain. Step B Blocked, exec Deferred, oldest cleanup and historical volume continuity Unverified remain.

## Roadmap findings

Continue only the already-approved PR158 finish after required current results. Archive/supervisor/census mechanisms stay parked. No later milestone selected.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready with advisories for this approved correction and conditional Git/PR finish only; no publication waiver.

## Exact files changed

- `DECISIONS.md`
- `docs/plans/2026-10-28-dashboard-current-main-integration.md`
- `docs/reviews/2026-10-28-dashboard-tokio-security-readiness-post-increment-review.md`

## Exact commands executed

- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `python3 -B .codex/hooks/session_end_gate.py`: Passed.
