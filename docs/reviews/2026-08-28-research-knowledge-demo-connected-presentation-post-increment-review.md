# Research/Knowledge connected presentation post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle",
    "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri",
    "cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract",
    "npm run test:frontend",
    "npm run test:repository",
    "npm run test:agent-acceptance",
    "npm run verify",
    "npm run security:scan",
    "git diff --check",
    "npm run tauri -- dev",
    "npm audit --audit-level=low",
    "npm run docs:check",
    "npm run repository:check",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/PROJECT_DIRECTION.md",
    "docs/increments/research-knowledge-demo-connected-presentation.md",
    "docs/plans/2026-08-28-research-knowledge-demo-connected-presentation.md",
    "docs/reviews/2026-08-28-research-knowledge-demo-connected-presentation-post-increment-review.md",
    "docs/reviews/2026-08-28-research-knowledge-demo-connected-presentation-security-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/research_knowledge_demo_lifecycle.rs",
    "src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs",
    "src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs",
    "src/features/command-center/CommandCenterPage.test.tsx",
    "src/features/command-center/CommandCenterPage.tsx",
    "src/features/command-center/ResearchKnowledgeLifecyclePanel.test.tsx",
    "src/features/command-center/ResearchKnowledgeLifecyclePanel.tsx",
    "src/features/command-center/command-center.css",
    "src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts",
    "src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Small when approved tooling can bind the process",
      "milestone": "Before using raw-debug lifecycle behavior as presentation evidence",
      "risk": "The source-current native debug lifecycle and alternate native presentation matrix were not directly observed.",
      "severity": "Advisory",
      "summary": "Approved UI tooling could not bind to the raw source-current debug executable."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Security",
      "effort": "Small to medium",
      "milestone": "Future separately approved capability narrowing",
      "risk": "A compromised WebView can emit the fixed event and force fail-closed recovery, but cannot mutate Rust or presentation state.",
      "severity": "Advisory",
      "summary": "The unchanged core:default capability includes WebView event emission."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "Small per approved guarded-source change",
      "milestone": "Any future lifecycle panel or client increment",
      "risk": "A legitimate guarded-source edit must update its exact reviewed SHA-256 baseline atomically.",
      "severity": "Advisory",
      "summary": "F-12 exact source digests deliberately impose review maintenance."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "Owner decision",
      "milestone": "Before any further source increment",
      "risk": "Starting unselected work would bypass the owner-approved increment order.",
      "severity": "Advisory",
      "summary": "No next source increment is owner-selected or Ready."
    }
  ],
  "increment_id": "research-knowledge-demo-connected-presentation",
  "manual_verification": [
    {
      "check": "Target-Mac development startup and visual no-permission-prompt inspection",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Source-current browser fallback: selected scenario, exact disclosure/proof boundary, mount/unmount, closed unavailable state, structured table, viewport, focus, scroll, topology zoom/reset, theme/reduced-motion query, and console",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Source-current raw-debug native success, failure, and success lifecycle sequence",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Source-current raw-debug native cancellation at research, knowledge, and synthesis plus terminal control closure",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Alternate native theme, reduced motion, page zoom, and native resize",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:frontend",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:repository",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:agent-acceptance",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
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
      "command": "npm run tauri -- dev",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --audit-level=low",
      "required": false,
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
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-28
Increment: research-knowledge-demo-connected-presentation
Branch: codex/research-knowledge-demo-connected-presentation

## Executive summary

`PASS WITH ADVISORIES`. The owner-approved increment implements the smallest
truthful Rust-backed interactive presentation over one existing sealed Research
-> Knowledge workflow. Only the selected scenario mounts a prop-free lifecycle
panel. Explicit no-input actions start, advance, or cancel one volatile
application-owned host; completed epochs privately alternate success and
synthetic synthesis failure, while cancellation consumes no outcome. All
required automated and available target-Mac checks pass. Raw-debug native
interaction and unavailable native presentation checks are `Not run`
advisories.

## Scope and boundaries

The exact 29-path inventory matches the approved Rust lifecycle schedule,
unchanged Tauri contract proof, response-authoritative client, selected-scenario
panel, accessibility/styles, F-12 guard/tests, F-15 current-state records, and
review evidence. No dependency, capability, CSP, permission, configuration,
`src-tauri/src/agent/**`, Conversations, fixture-projection DTO, provider,
model, network, credential, tool, approval dispatch, persistence, filesystem,
background work, durable audit, or device-effect path changed.

The WebView supplies no agent, task, root, run, request, profile, runtime,
workflow, objective, fixture, script, stage, outcome, or runtime event. Rust
owns every trusted identity and fixture. The Command Center fixture controls
remain IPC-free; the read-only projection, lifecycle panel, Conversations mock,
and Rust acceptance workflows remain separate deterministic proofs with the
exact `DEMO MODE · SIMULATED AGENT DATA` disclosure.

## Verification results

- Lifecycle-focused Rust command: Passed, 16 passed, 0 failed, 0 ignored; 253
  library tests filtered out.
- Adapter-focused Rust command: Passed, 6 passed, 0 failed, 0 ignored; 263
  library tests filtered out.
- Public Tauri contract: Passed, 1/1.
- `npm run test:frontend`: Passed, 18 files and 313 tests.
- `npm run test:repository`: Passed, 76/76.
- `npm run test:agent-acceptance`: Passed, 269 library plus 207 selected public
  contracts; 476 passed, 0 failed, 0 ignored.
- `npm run verify`: Passed with 28 hook, 76 repository, 313 frontend, 269 Rust
  library, and 244 Rust integration tests; one opt-in real Hermes probe is
  intentionally ignored. Strict formatting, ESLint, Clippy, typecheck,
  production frontend build, and Tauri release no-bundle build pass without
  warnings.
- `npm run security:scan`, `npm run docs:check`, `npm run repository:check`,
  `git diff --check`, and the session-end inventory: Passed.
- `npm audit --audit-level=low`: Passed with zero vulnerabilities after the
  first sandboxed attempt failed DNS resolution and the same command was
  retried with approved network access.
- Target-Mac `npm run tauri -- dev`: Passed startup and no-permission-prompt
  inspection. The source-current browser fallback passed the selected scenario,
  exact disclosures, mount/unmount, closed unavailable state, structured table,
  viewport/overflow, focus, scroll, topology zoom/reset, current dark theme,
  reduced-motion query, and zero console warning/error checks.
- Raw-debug native success/failure/cancellation, alternate native theme and
  reduced motion, page zoom, and native resize: Not run because approved UI
  tooling could not bind to the raw debug executable. A stale release app was
  explicitly excluded from current-source evidence.
- GitHub Actions: Not run because this branch state is uncommitted and
  unpublished.

## Architecture findings

`PASS WITH ADVISORIES`. One Rust host behind one mutex remains the sole
lifecycle owner. Its private completed-run schedule advances only after returned
terminal completion; cancellation does not consume a slot. Four fixed commands
accept no caller values, and the one event is notification-only. The prop-free
panel is mounted only by the selected scenario, performs one hydration snapshot,
and starts no operation automatically. No new engine, scheduler, timer, worker,
queue, polling, retry, second host, dependency, or generic runtime surface
exists.

F-12 statically pins the sole lifecycle client and panel, exact reviewed source
digests, command/event token locality, no-argument operations, allowed imports,
and prohibited raw Tauri/browser surfaces. It is defense in depth, not runtime
authorization. The current-state architecture and project direction distinguish
this sealed demo exception from the still-unwired generic runtime/catalog and
from the IPC-free Command Center fixture projection.

## Security findings

`PASS WITH ADVISORIES`. Exact returned-runtime identity validation and
rejected-run quarantine remain inside the host start/cleanup path. IPC returns
only a closed content-free v1 DTO/error. The client parses from `unknown`,
enforces exact keys/literals/bounds/journal grammar and requested-operation
successors, and commits state only from a validated command response. Every
parser-valid newer event requires recovery but never commits its state; older,
same-revision, and malformed events are inert.

The unchanged `core:default` capability permits WebView event emission, so a
compromised WebView can force fail-closed recovery. It cannot mutate Rust,
render the forged event, select an outcome, or authorize an operation. No
unsafe Rust, secret, personal data, credential, log, durable audit, SQLite,
filesystem/OS, network, provider, model, tool, approval, permission, or supply
chain expansion exists. The standalone security review records full evidence.

## Code-health findings

`PASS WITH ADVISORIES`. Rust schedule ownership is private and tested across
success/failure/success plus cancellation at every active stage. The Tauri
adapter preserves no-input signatures and closed notification failures. The
client freezes exact DTOs, validates operation-relative successors, rejects
concurrency and late/disposed results, and makes events non-authoritative. The
React panel closes errors, avoids retries/automation, disposes on route exit,
keeps in-flight controls closed, and exposes accessible region/status/action/
journal semantics. Independent re-review found no remaining actionable source
finding.

## Technical debt

- Category: Manual verification. Severity: Advisory. Risk: the source-current
  raw-debug lifecycle and alternate native presentation matrix were not directly
  observed. Effort: Small when approved tooling can bind the process. Milestone:
  before using raw-debug behavior as presentation evidence. Blocks completion:
  No. Blocks next increment: No.
- Category: Security boundary. Severity: Advisory. Risk: unchanged WebView event
  emission can force fail-closed recovery but grants no state or device
  authority. Effort: Small to medium. Milestone: a future separately approved
  capability-narrowing increment. Blocks completion: No. Blocks next increment:
  No.
- Category: Maintainability. Severity: Advisory. Risk: legitimate edits to the
  two guarded lifecycle sources require atomic SHA-256 baseline review. Effort:
  Small per approved change. Milestone: any future panel/client increment.
  Blocks completion: No. Blocks next increment: No.
- Existing/transient verification conditions: the first source verify stopped
  on one unformatted new increment document, the first post-document verify
  stopped because this report did not yet exist, one sandboxed audit lacked DNS,
  and one incorrect diagnostic used an unsupported `--check` flag. Each cause
  is recorded; corrected or approved exact commands pass without weakened
  controls. These are not product defects.

## Roadmap findings

`Blocked`. The active source increment is complete, but no next source increment
is owner-selected or Ready. `NEXT_STEPS.md` is not reordered. The smallest next
action is owner review and, only under separate explicit authorization, commit,
push, squash merge, and exact published-commit GitHub Actions inspection.

## Completion decision

`PASS WITH ADVISORIES`

## Next-increment readiness

`Blocked`. No next source task is authorized. Publication of this verified
increment is an owner decision, not a new implementation increment.

## Exact files changed

- `AGENTS.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/increments/research-knowledge-demo-connected-presentation.md`
- `docs/plans/2026-08-28-research-knowledge-demo-connected-presentation.md`
- `docs/reviews/2026-08-28-research-knowledge-demo-connected-presentation-post-increment-review.md`
- `docs/reviews/2026-08-28-research-knowledge-demo-connected-presentation-security-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/src/research_knowledge_demo_lifecycle.rs`
- `src-tauri/src/research_knowledge_demo_lifecycle_tauri.rs`
- `src-tauri/tests/research_knowledge_demo_lifecycle_tauri_contract.rs`
- `src/features/command-center/CommandCenterPage.test.tsx`
- `src/features/command-center/CommandCenterPage.tsx`
- `src/features/command-center/ResearchKnowledgeLifecyclePanel.test.tsx`
- `src/features/command-center/ResearchKnowledgeLifecyclePanel.tsx`
- `src/features/command-center/command-center.css`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.test.ts`
- `src/infrastructure/tauri/research-knowledge-demo-lifecycle-client.ts`

## Exact commands executed

- `python3 .codex/hooks/post_increment_gate.py begin --increment research-knowledge-demo-connected-presentation`: Passed.
- `npm ci`: Passed; 296 packages installed.
- Pinned toolchain checks: Passed on Node 26.3.0, npm 11.16.0, cargo/rustc
  1.90.0, Darwin 25.6.0 arm64.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`: Passed.
- Focused Rust, TypeScript/Vitest, ESLint, typecheck, Python compile, F-12, and
  repository-health commands: Passed after the final source edit. Final focused
  counts are 16 lifecycle, 6 adapter, 1 Tauri contract, 81 client/panel/page,
  and 76 repository tests.
- `cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle`: Passed.
- `cargo test --manifest-path src-tauri/Cargo.toml research_knowledge_demo_lifecycle_tauri`: Passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test research_knowledge_demo_lifecycle_tauri_contract`: Passed.
- `npm run test:frontend`: Passed, 313/313.
- `npm run test:repository`: Passed, 76/76.
- `npm run test:agent-acceptance`: Passed, 476/476.
- `npm run verify`: the first source attempt Failed only on formatting the new
  increment record; the unchanged retry Passed. After F-15 drafting, one attempt
  Failed only because this report link did not exist yet. The final post-report
  run Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `npm audit --audit-level=low`: the sandboxed attempt Failed with DNS
  `ENOTFOUND`; the approved network retry Passed with zero vulnerabilities.
- `npm run tauri -- dev`: Passed startup. Approved UI tooling could not bind the
  raw debug executable; all processes were stopped afterward.
- `python3 scripts/repository_health.py --check ui-native-boundary`: Failed
  because `--check` is unsupported. The correct
  `python3 scripts/repository_health.py ui-native-boundary` Passed.
- `npm run docs:check`: Passed.
- `npm run repository:check`: Passed.
- `python3 .codex/hooks/session_end_gate.py`: Passed with the exact expected
  inventory and no conflicts.

No commit, push, merge, release, publication, credential, capability, CSP,
permission, dependency, provider, or device-effect action occurred.
