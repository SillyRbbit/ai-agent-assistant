# F-07 production/development CSP separation post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run test:repository",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "npm run verify",
    "npx prettier --write docs/plans/2026-08-26-production-development-csp-separation-f07.md docs/increments/production-development-csp-separation-f07.md",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "npm run tauri -- dev",
    "npm run tauri -- build --bundles app",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/production-development-csp-separation-f07.md",
    "docs/plans/2026-08-26-production-development-csp-separation-f07.md",
    "docs/reviews/2026-08-26-production-development-csp-separation-f07-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/tauri.conf.json"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Medium rendered UI refactor and regression matrix",
      "milestone": "Future pre-release UI hardening",
      "risk": "A future style injection defect retains a broader presentation surface, without script or device execution authority.",
      "severity": "Advisory",
      "summary": "Inline styles remain permitted for current React and React Flow element-style compatibility."
    }
  ],
  "increment_id": "production-development-csp-separation-f07",
  "manual_verification": [
    {
      "check": "Target-Mac Tauri development process launches with the fixed local Vite server",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Development Vite client connects, Command Center renders, and warning/error console remains empty",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Target-Mac release app launches, reports Local core ready, and renders the nine-role Command Center with exact demo disclosure",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Direct accessibility inspection of the unbundled debug WebView",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npm run test:repository",
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
      "command": "npm run verify",
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
    }
  ]
}
-->

Date: 2026-08-26
Increment: `production-development-csp-separation-f07`
Branch: `codex/security-csp-separation-f07`
Target: macOS 26.6 build 25G72, arm64

## Executive summary

F-07 is complete. Production no longer permits the fixed Vite WebSocket or
inline scripts. Development has a separate exact policy whose only production
widening is `ws://localhost:1420`. Tauri's default asset nonce/hash modification
remains enabled, and inline styles remain for existing React/React Flow element
styles.

The F-12 static UI/native-boundary check now freezes both exact policies and
rejects production/development confusion, a missing development policy, and
disabled Tauri asset-CSP modification. Focused/full automation and target-Mac
development/release smoke evidence pass. Acceptance criteria are met. Quality
gate: `PASS WITH ADVISORIES`.

## Scope and boundaries

The exact change is one Tauri security configuration, its standard-library
static enforcement/tests, and truthful plan/current-state documentation. No
Tauri command, event, capability, permission, IPC, agent connection, provider,
model, credential, tool, approval, persistence, filesystem, dependency,
background, network API, or device-effect path changed.

The WebView remains untrusted. Rust authority and the exact app-info-only invoke
allowlist remain unchanged. The locally bundled `.app` exists only as ignored
verification output and is not tracked or published.

## Verification results

- `npm run test:repository`: Passed, 47/47.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `npm run verify`: Passed on its final complete run. This includes formatting,
  repository policy, strict frontend/Rust lint, 28 hook tests, 47 repository
  tests, 211 frontend tests, 249 Rust library tests, 240 passed public/integration
  tests with one intentional ignored Hermes probe, both frontend builds, and
  the Tauri release no-bundle build.
- `python3 .codex/hooks/session_end_gate.py`: Passed with the exact expected
  tracked/untracked inventory and no conflicts.

Earlier evidence retained for transparency:

- The first `npm run verify` stopped at Markdown formatting for the two new
  records. Prettier corrected only those records.
- The next attempt hit a transient Cargo incremental working-directory error at
  strict Clippy. The exact Clippy command passed unchanged, then the fresh
  complete verification passed. No cache deletion, source workaround, lint
  suppression, or toolchain change occurred.

Manual evidence:

- Passed: `npm run tauri -- dev` launched Vite at the fixed local port and the
  Tauri debug process reached its running state.
- Passed: Browser Control observed `[vite] connected.`, no warning/error logs,
  a rendered Command Center heading, and four exact demo-disclosure occurrences.
- Passed: `npm run tauri -- build --bundles app` produced a local ignored
  `Cortexa.app`; Computer Use inspected `tauri://localhost`, `Local core ready`,
  the Command Center, all nine catalog roles, and the exact
  `DEMO MODE · SIMULATED AGENT DATA` disclosure.
- Not run, optional: direct accessibility inspection of the unbundled debug
  WebView. Computer Use could not enumerate that raw executable.

## Architecture findings

No blocking finding. Tauri configuration remains the single owner of WebView
policy, and the repository-health checker remains a read-only drift guard. The
installed `devCsp` surface avoids a parallel configuration system, custom
loader, new dependency, or coupling to React. F-12 command/capability and
Command Center isolation enforcement remains intact.

## Security findings

No blocking finding. Production loses the local Vite WebSocket and inline-script
surfaces; development gains no source beyond the already fixed local socket.
Both policy objects are exact, wildcard-free, and remotely closed. The checker
also fails when Tauri asset-CSP modification is disabled through the dangerous
disable flag. No model, agent, runtime, or WebView input can select or widen the
policy.

Advisory: `style-src 'unsafe-inline'` remains because existing React/React Flow
elements use style attributes. Removing it requires a separate rendered UI
refactor and is not required for F-07's accepted remediation.

## Code-health findings

No blocking finding. The implementation reuses exact dictionary comparison,
adds focused positive and negative fixtures, and keeps the static checker
standard-library-only. Tests cover swapped production/development privileges,
missing `devCsp`, and disabled Tauri CSP modification. Documentation markers
fail closed on stale F-07 wording.

## Technical debt

- Category: WebView defense in depth.
- Severity: Advisory.
- Summary: inline styles remain permitted for current React/React Flow
  compatibility.
- Risk: a future style injection defect retains a broader presentation surface;
  this does not permit script or device execution.
- Effort: Medium because removal requires rendered topology regression work.
- Milestone: future pre-release UI hardening.
- Blocks completion: No.
- Blocks F-08: No.

No debt was introduced for dependencies, IPC, capabilities, permissions,
credentials, persistence, audit, execution, or native-agent ownership.

## Roadmap findings

`Blocked`. F-07 is complete, but F-08 app-info runtime narrowing has no exact
owner-approved Ready plan. The smallest next action is a separate
documentation/readiness review for F-08 only. Agent IPC and the interactive
demo remain later and unapproved.

## Completion decision

`PASS WITH ADVISORIES`

The advisories are retained inline styles and the split development
process/browser evidence necessitated by the raw debug executable's Computer
Use limitation. Neither hides a failed required command, widens production
policy, or blocks F-08 planning.

## Next-increment readiness

`Blocked`. No exact F-08 implementation plan is owner-approved or Ready.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/increments/production-development-csp-separation-f07.md`
- `docs/plans/2026-08-26-production-development-csp-separation-f07.md`
- `docs/reviews/2026-08-26-production-development-csp-separation-f07-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/tauri.conf.json`

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment production-development-csp-separation-f07`
- `npm run test:repository`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `npm run verify`
- `npx prettier --write docs/plans/2026-08-26-production-development-csp-separation-f07.md docs/increments/production-development-csp-separation-f07.md`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
- `npm run tauri -- dev`
- `npm run tauri -- build --bundles app`
- `python3 .codex/hooks/session_end_gate.py`
- `python3 .codex/hooks/post_increment_gate.py finalize --increment production-development-csp-separation-f07 --report docs/reviews/2026-08-26-production-development-csp-separation-f07-post-increment-review.md`
- `python3 .codex/hooks/post_increment_gate.py status`
