# Research/Knowledge demo projection contract post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment research-knowledge-demo-projection-contract",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npx vitest run src/infrastructure/tauri/research-knowledge-demo-projection-client.test.ts src/features/command-center/components/ResearchKnowledgeDemoProjectionPanel.test.tsx src/features/command-center/CommandCenterPage.test.tsx src/App.test.tsx",
    "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_projection --lib --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_projection_contract --locked",
    "python3 -m unittest scripts.tests.test_repository_health -v",
    "python3 scripts/repository_health.py ui-native-boundary",
    "npm run verify",
    "npm run tauri -- dev",
    "npm run tauri -- build",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/increments/research-knowledge-demo-projection-contract-planning.md",
    "docs/increments/research-knowledge-demo-projection-contract.md",
    "docs/plans/2026-08-27-research-knowledge-demo-projection-contract.md",
    "docs/reviews/2026-08-27-research-knowledge-demo-projection-contract-planning-post-increment-review.md",
    "docs/reviews/2026-08-27-research-knowledge-demo-projection-contract-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/lib.rs",
    "src-tauri/src/research_knowledge_demo_projection.rs",
    "src-tauri/tests/research_knowledge_demo_projection_contract.rs",
    "src/App.test.tsx",
    "src/App.tsx",
    "src/features/command-center/CommandCenterPage.test.tsx",
    "src/features/command-center/CommandCenterPage.tsx",
    "src/features/command-center/command-center.css",
    "src/features/command-center/components/ResearchKnowledgeDemoProjectionPanel.test.tsx",
    "src/features/command-center/components/ResearchKnowledgeDemoProjectionPanel.tsx",
    "src/infrastructure/tauri/research-knowledge-demo-projection-client.test.ts",
    "src/infrastructure/tauri/research-knowledge-demo-projection-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low",
      "milestone": "When approved tooling supports the raw debug process or host page zoom",
      "risk": "Direct debug-WebView accessibility binding and host/browser page zoom lack tool-observed evidence, although source-current browser and release-native checks cover the changed view.",
      "severity": "Advisory",
      "summary": "Two optional target-Mac evidence checks were unavailable and remain Not run."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Separate investigation",
      "milestone": "Before any distribution or release increment",
      "risk": "A future distributable DMG cannot rely on this attempt; the source-current app and required no-bundle binary are valid, but packaging failed in bundle_dmg.sh.",
      "severity": "Advisory",
      "summary": "The non-required full bundle attempt failed during DMG packaging after producing the local app bundle."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner decision and documentation planning",
      "milestone": "Before any lifecycle source work",
      "risk": "Starting lifecycle commands or events without a separately reviewed plan would expand the trusted IPC boundary beyond this read-only prerequisite.",
      "severity": "Advisory",
      "summary": "No volatile lifecycle successor plan is approved or Ready."
    }
  ],
  "increment_id": "research-knowledge-demo-projection-contract",
  "manual_verification": [
    {
      "check": "Target-Mac development launch and source watch",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Source-current release explicit refresh, exact DTO, disclosure, and separate-proof copy",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Light and dark themes",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Reduced motion on and off",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Viewport, keyboard focus, scroll ownership, and in-app topology zoom/reset",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native resize 1040x700 to 803x563 and restored 1040x700",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Direct accessibility binding to the raw debug executable",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Host/browser page zoom",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "npx vitest run src/infrastructure/tauri/research-knowledge-demo-projection-client.test.ts src/features/command-center/components/ResearchKnowledgeDemoProjectionPanel.test.tsx src/features/command-center/CommandCenterPage.test.tsx src/App.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_projection --lib --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_projection_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -m unittest scripts.tests.test_repository_health -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 scripts/repository_health.py ui-native-boundary",
      "required": true,
      "status": "Passed"
    },
    {"command": "npm run verify", "required": true, "status": "Passed"},
    {
      "command": "npm run tauri -- build",
      "required": false,
      "status": "Failed"
    },
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-27
Increment: research-knowledge-demo-projection-contract
Branch: `main`

## Executive summary

The first Rust-to-WebView Research/Knowledge prerequisite is complete. One
explicit argument-free query returns only a finite application-owned synthetic
projection; it starts no workflow and confers no authority. All required
automated and target-Mac checks pass. The result is `PASS WITH ADVISORIES` for
two unavailable optional UI-tooling checks, one non-required DMG packaging
failure, and the intentionally blocked lifecycle successor.

## Scope and boundaries

The 27-file workspace inventory contains the prior planning records, the exact
Rust DTO/query and public contract, one fail-closed Tauri client, one
explicit-refresh Command Center panel, focused tests, F-12 static protection,
security/architecture/current-state reconciliation, and this report. No
capability, CSP, permission, dependency, lockfile, provider, model, network,
credential, tool, approval dispatch, persistence, filesystem, background,
generic workflow, durable audit, or device-effect surface changed.

## Verification results

- Focused frontend: `Passed`, 69/69.
- Rust projection module: `Passed`, 3/3.
- Exact public Rust projection contract: `Passed`, 1/1.
- Repository-health suite: `Passed`, 32/32.
- Exact UI/native boundary: `Passed`.
- Complete `npm run verify`: `Passed`; hook tests 28/28, repository tests
  57/57, frontend tests 247/247 across 16 files, Rust library tests 252/252,
  Rust integration tests 242/242, and one explicit opt-in Hermes version probe
  ignored. Frontend production build and Tauri release no-bundle build passed.
- Documentation, repository policy, secret scan, diff, and session-end checks:
  `Passed`.
- Target-Mac development launch, release explicit refresh, disclosure, themes,
  reduced motion, viewports, focus, scroll, topology zoom/reset, and native
  resize: `Passed`.
- Direct raw-debug accessibility binding and host/browser page zoom: `Not run`;
  approved tooling was unavailable and both checks are non-required.
- Full DMG packaging: `Failed`, non-required, after the source-current `.app`
  was created. The required no-bundle build passed; no artifact was published.

## Architecture findings

`PASS`. Rust privately owns the fixed DTO and exports only one no-argument
query. The client adapter contains the Tauri import, narrows from `unknown`, and
injects the loader into a scenario-local UI. The native projection never enters
the frontend topology/activity fixture or the sealed D-086 workflow. Coupling,
portability, dependencies, and authority ownership remain bounded.

## Security findings

`PASS`. No caller identity or content crosses IPC. Rust returns one closed
serializable error; the client reconstructs fresh frozen values and exposes
fixed unavailable copy. Late/stale replies are discarded. F-12 recursively
pins capability files, exact config/security keys, the two-command handler,
both invokes, the sole menu event, all six emitter APIs, and forbidden network,
storage, provider, runtime, tool, filesystem, and state imports. No secret,
unsafe Rust, log, audit content, permission, CSP, plugin, event, or effect was
added.

## Code-health findings

`PASS`. Closed enums, fixed-size Rust arrays, exact object-key validation,
Unicode bounds, exhaustive malformed-response tests, explicit refresh,
concurrency disabling, stale-result clearing, late-reply rejection, and
accessible loading/status attributes are covered. Independent review found no
blocking correctness, accessibility, privacy, test, or documentation defect.

## Technical debt

No debt was introduced by the read-only projection. The DMG packaging failure
is an existing/out-of-scope build-tooling advisory to investigate before any
distribution increment. It does not block this increment or a future
documentation plan. Effort: separate investigation; milestone: before
distribution.

## Roadmap findings

`Blocked`. This prerequisite is complete, but no exact volatile lifecycle plan
is owner-approved or Ready. The smallest next action is a separate
documentation/readiness run; it must preserve application-owned identities,
closed lifecycle commands/events, cancellation cleanup, late-event rejection,
and the same no-provider/no-effect boundaries.

## Completion decision

`PASS WITH ADVISORIES`.

## Next-increment readiness

`Blocked`. No source increment is Ready. Plan and approve the volatile
lifecycle prerequisite separately before starting a gate or implementation.

## Exact files changed

The machine manifest lists the complete 27-file tracked and untracked
workspace inventory. No generated build output is included.

## Exact commands executed

The machine manifest records every required verification command and the two
runtime/build commands relevant to target-Mac evidence. No commit, push, merge,
reset, rebase, release, deployment, credential, or publication command ran.
