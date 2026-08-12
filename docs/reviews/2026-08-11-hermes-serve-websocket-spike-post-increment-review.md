# Hermes serve WebSocket spike post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
    "npm run docs:check",
    "./node_modules/.bin/prettier --write docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md",
    "./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/increments/hermes-serve-websocket-spike.md docs/plans/2026-08-11-hermes-serve-websocket-spike.md docs/reviews/2026-08-11-hermes-serve-websocket-spike-post-increment-review.md docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md",
    "npm run verify",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri/src src-tauri/tests src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents",
    "pgrep -af '[h]ermes serve|hermes-agent/.venv/bin/hermes'",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/hermes-serve-websocket-spike.md",
    "docs/plans/2026-08-11-hermes-serve-websocket-spike.md",
    "docs/reviews/2026-08-11-hermes-serve-websocket-spike-post-increment-review.md",
    "docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "owner-supplied fully content-manifested immutable distribution and interpreter, or a separately approved provenance/packaging increment",
      "milestone": "before any real Hermes process launch",
      "risk": "the supplied virtual environment and externally located owner-writable Python runtime cannot be bound to the approved complete-runtime identity",
      "severity": "Critical",
      "summary": "Complete immutable interpreter/runtime provenance is absent."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "an upstream or exactly pinned reviewed Hermes change with supported explicit disable controls, followed by a fresh plan and source/security review",
      "milestone": "before any real Hermes process launch or adapter implementation",
      "risk": "the pinned release unconditionally initializes update, dotenv/managed-secret, credential, plugin, skill, and privileged default-tool paths that the approved conversation-only spike prohibits",
      "severity": "Critical",
      "summary": "Pinned startup has no supported closed no-update/no-credential/no-plugin/zero-tool mode."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "a separately owner-approved target-Mac containment design and adversarial proof, potentially including a reviewed helper or different mechanism",
      "milestone": "before any contained real-runtime transport test",
      "risk": "deprecated sandbox-exec alone cannot prove the required dynamic listener, package-manager execution, Unix-socket, or detached-descendant membership and cleanup boundaries",
      "severity": "Critical",
      "summary": "No eligible whole-process containment mechanism was selected."
    }
  ],
  "increment_id": "hermes-serve-websocket-spike",
  "manual_verification": [
    {
      "check": "The supplied candidate matches Hermes Agent 0.20.0, exact tag v2026.8.3, exact commit 3c27eb6234bf91b8ceee9e9071591b31e9b148cb, clean source state, source archive digest, three critical-file hashes, and 61 installed package metadata records.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Milestone 0 reviewed complete immutable runtime provenance, determined that the supplied manifest does not cover the virtual environment and external interpreter, and applied the approved pre-launch stop condition.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Pinned-source review determined that supported complete no-update, no-dotenv/managed-secret, no-credential-keepalive, no-plugin/skill, and true zero-tool controls are absent and applied the approved pre-launch stop condition.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Target-Mac review determined that sandbox-exec does not prove the required listener/egress, package-manager/updater, Unix-socket, Keychain, and detached-descendant guarantees and applied the approved pre-launch stop condition.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "The real pinned Hermes process completes the approved credential-free fake-provider lifecycle and containment negatives.",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "The complete diff is documentation-only, contains no local user path or credential, changes no source, test, manifest, lockfile, dependency, runtime, UI, or capability, and leaves no persistent external integration state.",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
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
      "command": "git diff --exit-code -- src src-tauri/src src-tauri/tests src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents",
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

Date: 2026-08-11
Increment: hermes-serve-websocket-spike
Branch: main

## Executive summary

The isolated selected-transport spike stopped at Milestone 0 before any real
Hermes launch or harness implementation. Candidate source identity, critical
hashes, package metadata, and sanitized module discovery passed. Complete
runtime provenance, pinned startup suppression, and exact target-Mac
containment did not pass. Acceptance criteria are unmet, the selected contained
integration is NO-GO under D-080's approved conditions. The negative-result
increment safely achieved its prove-or-disprove objective, so its engineering
quality result is **PASS WITH ADVISORIES** while every later Hermes increment
remains Blocked.

This is a safe negative result: Native remains sole/default, no application or
persistent external/integration state changed, no credential was accessed, the
disposable preflight directory was removed, and Prompt 4D was not started.

## Scope and boundaries

The approved goal was to prove or disprove managed local `hermes serve` plus a
closed TUI-gateway JSON-RPC/WebSocket projection without connecting Hermes to
the application. Milestone 0 was explicitly load-bearing: complete immutable
runtime provenance and an eligible containment/no-update/no-secret/no-tool plan
had to pass before a server could launch.

The exact 10-path change set contains only the plan, sanitized negative report,
blocked increment/review records, architecture status, and current project
memory. The planned Rust test and Python fixture were not created. No production
source, test, dependency, manifest, lockfile, Tauri configuration/capability,
UI, provider, runtime selector, process, socket, or external write entered
scope.

## Verification results

The prior raw-stdio fixture regression passed 7 tests with its version-only real
probe ignored. The native runtime contract passed 20/20. Full `npm run verify`
passed, including formatting, repository health, strict lint/Clippy, 124
frontend tests, the complete Rust suite, frontend build, and Tauri no-bundle
release build. Documentation, repository, security, protected-path, diff, and
session-end checks passed after final formatting.

The first documentation check correctly found formatting in the new spike
report; the exact Prettier correction ran and the final check passed. No failure
was suppressed.

The new WebSocket target and opt-in real-Hermes test were not created or run
because the Milestone 0 stop condition fired. Reviews conclusively recorded the
absent whole-runtime provenance, supported startup suppression, and exact
containment controls. Those findings block a positive transport result and all
later Hermes work; they do not block completion of this safely stopped negative
evaluation.

## Architecture findings

`$architecture-review`: **Critical next-increment-blocking finding**.

The planned transport remains isolated from `AgentRuntime`, Native, policy,
approval, audit, tools, memory, secrets, Tauri, and React. However, no eligible
whole-process ownership/cleanup abstraction was demonstrated. `sandbox-exec`
inherits policy into descendants but provides no complete containment identity,
membership enumeration, containment-wide kill, or detached-descendant reap.
Its static profile also cannot restrict a port-zero child to exactly its
reported inbound listener.

The finding establishes the spike's NO-GO result and blocks every renewed spike
or adapter increment. It does not justify a silent switch to ACP or another
transport.

## Security findings

`$security-review`: **Critical next-increment-blocking findings**.

- The supplied runtime identity is incomplete: one of 4,077 virtual-environment
  files is hashed, while the external owner-writable Python interpreter,
  standard library, installed package contents, and editable loaders are not
  content-manifested.
- Pinned startup unconditionally enters update-prefetch,
  dotenv/managed-secret, credential-keepalive, skill, plugin, and privileged
  default-tool initialization paths without a supported complete disable mode.
- `sandbox-exec` cannot distinguish allowed interpreter execution from
  `python -m pip`, prove blanket Unix-socket denial, or account for and clean all
  detached descendants.

A credential-free loopback fake provider is source-supported, but it cannot
cure these blockers. The correct security action was to stop before launch.

## Code-health findings

`$code-review`: no code was created or changed. The negative report distinguishes
static source evidence from dynamic evidence, redacts the local user path and
raw environment values, records every unrun stage, and avoids implying that a
WebSocket handshake, cancellation, shutdown, or containment behavior was
observed.

## Technical debt

No product technical debt was introduced. The negative evaluation exposes three
pre-existing external-integration liabilities: incomplete artifact provenance,
unavoidable broad startup initialization, and the absence of an eligible
target-Mac containment mechanism. Each is Critical and blocks every renewed
Hermes spike and adapter increment, but does not block preserving this completed
negative-result evidence. Each needs a separately owner-approved architecture/
security milestone rather than in-place workaround code.

## Roadmap findings

`$readiness-review`: **Blocked**.

The Draft `HermesAgentRuntime` plan cannot become Ready. D-080 remains the
accepted conditional evaluation direction, but its currently supplied
candidate/mechanism failed Milestone 0. The smallest responsible follow-up is
an owner-approved documentation/ADR amendment deciding whether to require an
upstream/pinned build with explicit closed startup controls and complete
runtime provenance, evaluate a different containment boundary, or separately
reconsider ACP. No implementation increment is Ready.

## Completion decision

**PASS WITH ADVISORIES**

The spike verdict remains **FAIL / NO-GO** for this candidate under the approved
D-080 conditions. The quality result is separate: source/provenance/containment
review reached an evidence-backed negative answer, obeyed the pre-launch stop
condition, preserved Native, and completed the required documentation and
verification. Critical findings block all subsequent Hermes work, not closure
of this bounded negative-result increment.

## Next-increment readiness

**Blocked** — do not launch Hermes, create the WebSocket harness, approve the
adapter, begin Prompt 4D, switch transports, or change Native. Await an exact
owner-approved documentation/ADR amendment.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/increments/hermes-serve-websocket-spike.md`
- `docs/plans/2026-08-11-hermes-serve-websocket-spike.md`
- `docs/reviews/2026-08-11-hermes-serve-websocket-spike-post-increment-review.md`
- `docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md`

## Exact commands executed

- `cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked`
  — passed: 7 passed, 0 failed, 1 ignored.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked`
  — passed: 20 passed, 0 failed.
- `npm run docs:check` — first run found new-report formatting; final run passed.
- `./node_modules/.bin/prettier --write docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md`
  — passed.
- `./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/increments/hermes-serve-websocket-spike.md docs/plans/2026-08-11-hermes-serve-websocket-spike.md docs/reviews/2026-08-11-hermes-serve-websocket-spike-post-increment-review.md docs/spikes/HERMES_SERVE_WEBSOCKET_SPIKE.md`
  — passed after the complete report was added.
- `npm run verify` — passed.
- `npm run repository:check` — passed.
- `npm run security:scan` — passed.
- `git diff --check` — passed.
- `git diff --exit-code -- src src-tauri/src src-tauri/tests src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents`
  — passed; no protected implementation/configuration path changed.
- `pgrep -af '[h]ermes serve|hermes-agent/.venv/bin/hermes'` — returned no
  matching process.
- `python3 .codex/hooks/session_end_gate.py` — passed.
- `python3 .codex/hooks/post_increment_gate.py status` — active before final
  post-increment finalization.

The candidate provenance and sanitized isolated import-discovery commands are
recorded as manual evidence without embedding the local user path or environment
values in repository history.
