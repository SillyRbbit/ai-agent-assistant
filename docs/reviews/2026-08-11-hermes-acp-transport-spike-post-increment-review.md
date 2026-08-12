# Hermes ACP transport spike post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 -m py_compile src-tauri/tests/fixtures/hermes_acp_server_stub.py",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
    "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/adr/ADR-HERMES-ACP-TRANSPORT.md docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/increments/hermes-acp-transport-spike.md docs/plans/2026-08-11-hermes-acp-transport-spike.md docs/plans/2026-08-11-hermes-agent-runtime-adapter.md docs/reviews/2026-08-11-hermes-acp-transport-spike-post-increment-review.md docs/spikes/HERMES_ACP_TRANSPORT_SPIKE.md",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri/src src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/adr/ADR-HERMES-ACP-TRANSPORT.md",
    "docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md",
    "docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md",
    "docs/increments/hermes-acp-transport-spike.md",
    "docs/plans/2026-08-11-hermes-acp-transport-spike.md",
    "docs/plans/2026-08-11-hermes-agent-runtime-adapter.md",
    "docs/reviews/2026-08-11-hermes-acp-transport-spike-post-increment-review.md",
    "docs/spikes/HERMES_ACP_TRANSPORT_SPIKE.md",
    "src-tauri/tests/fixtures/hermes_acp_server_stub.py",
    "src-tauri/tests/hermes_acp_transport_spike.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "an upstream/enforced conversation-only ACP construction, separately reviewed immutable build, and a new owner-approved spike",
      "milestone": "before any renewed Hermes transport or adapter increment",
      "risk": "Hermes can execute privileged terminal, filesystem, browser, memory, skill, code, and delegation capabilities before Cortexa can validate and authorize them",
      "severity": "Critical",
      "summary": "Pinned ACP hardcodes privileged internal tools without an application-owned pre-execution gate."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "an owner-supplied fully content-manifested immutable interpreter/runtime containing the exact ACP SDK",
      "milestone": "before any real Hermes command or import",
      "risk": "the supplied candidate cannot be bound to a complete immutable runtime identity and lacks the pinned ACP distribution",
      "severity": "Critical",
      "summary": "Complete candidate provenance and required ACP packaging are absent."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "a separately approved target-platform containment and lifecycle design with containment-wide descendant cleanup",
      "milestone": "before any real external-runtime lifecycle test",
      "risk": "ACP cancellation and EOF do not prove shutdown or cleanup of Hermes-owned tool processes and detached descendants",
      "severity": "High",
      "summary": "No eligible whole-process containment and shutdown boundary was established for the evaluated candidate and path."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Technical debt",
      "effort": "one bounded documentation-only current-state reconciliation",
      "milestone": "before relying on root runtime wording as current implementation evidence",
      "risk": "root AGENTS.md and docs/PROJECT_DIRECTION.md retain pre-existing wording that describes AgentRuntime and NativeAgentRuntime as conceptual despite the published native foundation",
      "severity": "Advisory",
      "summary": "Pre-existing runtime current-state wording remains stale."
    }
  ],
  "increment_id": "hermes-acp-transport-spike",
  "manual_verification": [
    {
      "check": "Exact pinned source identity, tag, commit, clean state, origin, archive and critical-file hashes, installed dependency inventory, and ACP source symbols were reviewed without executing candidate code.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Pinned-source review established that the hardcoded ACP toolset lacks a supported true zero-tool mode and applied the mandatory NO-GO condition before real execution.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "The exact changed inventory contains only ADR/decision/plan/report/current-memory paths plus the isolated test target and deterministic fixture; production source, Native, manifests, dependencies, UI, IPC, capabilities, and external state remain unchanged.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "A real pinned Hermes ACP process demonstrates startup, one session, prompt, cancellation, shutdown, and whole-process containment.",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "A dedicated provider credential supports a real text turn.",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "python3 -m py_compile src-tauri/tests/fixtures/hermes_acp_server_stub.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked",
      "required": true,
      "status": "Passed"
    },
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
      "command": "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
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
      "command": "git diff --exit-code -- src src-tauri/src src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents",
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
Increment: hermes-acp-transport-spike
Branch: main

## Executive summary

The owner-selected isolated ACP evaluation reached an evidence-backed **NO GO**
for Hermes Agent `0.20.0` / `v2026.8.3`. ACP is materially stronger than raw
TUI-gateway stdio as a wire contract, but pinned source hardcodes privileged
Hermes-owned tool execution without a supported conversation-only mode or a
Cortexa-owned gate before every effect. The supplied candidate also lacks
complete immutable runtime provenance and the pinned ACP SDK. The mandatory
stop condition fired before any real candidate command or import.

Five deterministic fixture tests pass and prove bounded host mechanics only.
The ACP ADR is Rejected under D-081, the WebSocket NO-GO is preserved, Native
remains sole/default, the adapter remains Draft/Blocked, and Prompt 4D was not
started. The safely completed negative evaluation receives **PASS WITH
ADVISORIES**; that engineering result is distinct from the transport verdict.

## Scope and boundaries

The approved goal was to evaluate exact pinned ACP without connecting Hermes to
production behavior. The 17-path inventory contains ADRs, the living plan,
decision/current-memory closeout, one Rust integration-test target, and one
deterministic Python fixture. No `src-tauri/src`, React, Tauri command or
capability, manifest, lockfile, dependency, runtime selector, provider,
credential, or Native path changed.

The fixture never imports or executes Hermes and uses no provider, model,
network, shell, tool, memory, credential, or normal profile. A real Hermes
version/check/start/session test was prohibited once Milestone 0 and capability
containment failed.

## Verification results

The ACP fixture syntax and five focused tests pass. Those tests cover bounded
version/check probes, initialization, one fake session/text turn, request
correlation, protocol-only stdout, bounded stderr, startup and in-run timeouts,
malformed/oversized output, unknown methods, privileged tool and permission
messages, wrong identity, crash/EOF handling, ACP notification cancellation,
late-output rejection, environment isolation, and direct-child reap. The prior
raw-stdio fixture regression passes seven ordinary tests with its one
explicitly ignored real probe; the Native runtime contract passes 20/20.

Full verification passes, including formatting, repository health, strict
frontend/Rust lint, 124 frontend tests, the complete Rust suite, frontend build,
and Tauri no-bundle release build. Documentation, repository, secret, protected
production-path, diff, and session-end checks pass. The candidate itself was
not executed; every real-runtime/containment check is truthfully Not run.

## Architecture findings

`$architecture-review`: one Critical next-increment blocker and one High
next-increment blocker, neither blocking the safe negative closeout.

ACP provides a coherent process wire, but its implementation owns the agent's
privileged execution loop. Host-side rejection of an observed tool or
permission message cannot restore Cortexa's pre-execution governance after an
effect has begun. Cooperative cancel and EOF also do not define
containment-wide cleanup. Consequently ACP cannot implement D-079's runtime
boundary at this pinned release. Native and every production ownership boundary
remain unchanged.

## Security findings

`$security-review`: Critical next-increment blockers establish the NO-GO.

- Every ACP session hardcodes terminal/process, filesystem, browser, memory,
  skills, code-execution, and delegation capabilities inside Hermes. No
  supported true zero-tool ACP mode or application-owned pre-execution gate was
  found.
- The candidate's virtual environment and external Python runtime lack a
  complete immutable content manifest, and the pinned optional ACP SDK is not
  installed.
- Normal ACP construction can enter Hermes configuration, environment,
  credentials, persistence, MCP, memory, skills, plugins, network, filesystem,
  and subprocess paths. Cancellation does not prove descendant cleanup, and
  stderr may contain sensitive prompt/error material.

No candidate execution occurred, so no secret, credential, profile, state,
network, Keychain, package-manager, or tool interaction was exposed.

## Code-health findings

`$code-review`: no remaining actionable finding. The test target is isolated,
uses direct argv with no shell interpolation, clears the environment, bounds
frames/counts/stderr/deadlines, reaps the direct child, and uses closed redacted
errors. Cancellation is modelled as an ACP notification, terminal late output
is rejected by state, and both startup and in-run timeouts are exercised.

The deterministic fixture is explicitly not presented as Hermes conformance,
provenance, OS containment, or process-tree evidence. No production code was
added or modified.

## Technical debt

- **Critical / Security:** pinned ACP's broad internal tool authority blocks
  every adapter or renewed transport increment. Remediation requires an
  upstream/enforced conversation-only construction and a new owner-approved
  immutable spike; it does not block preserving this negative result.
- **Critical / Security:** incomplete runtime provenance and missing ACP SDK
  block every real candidate command. Remediation requires a complete immutable
  interpreter/runtime artifact; installing into this candidate is forbidden.
- **High / Architecture:** no eligible whole-process containment and shutdown
  design was established for the evaluated candidate and path. A separate
  target-platform design and adversarial proof are required before any renewed
  external-runtime test.
- **Advisory / Documentation:** pre-existing root instruction and project-
  direction wording still describes the now-published Native runtime foundation
  as conceptual. A later bounded documentation-only reconciliation should
  correct that factual drift; it does not authorize or unblock Hermes.

## Roadmap findings

`$readiness-review`: **Blocked**. The three evaluated Hermes mechanisms—raw
TUI-gateway stdio, managed `hermes serve` WebSocket, and ACP—are rejected for
the exact pinned release under their evaluated conditions. No replacement
transport is selected, and the adapter remains Draft/Blocked. Native remains
the only runtime.

## Completion decision

**PASS WITH ADVISORIES.** The prove-or-disprove objective reached its mandated
negative answer without weakening controls or executing the ineligible
candidate. Critical/High findings block later Hermes work, not preservation of
the verified negative evidence.

## Next-increment readiness

**Blocked.** No Hermes implementation or additional transport plan is Ready.
The smallest safe future action is an owner-selected documentation/architecture
decision, and only if upstream evidence supplies a conversation-only mode plus
a fully content-manifested artifact and exact containment proposal. Do not
automatically evaluate another mechanism.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/adr/ADR-HERMES-ACP-TRANSPORT.md`
- `docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/increments/hermes-acp-transport-spike.md`
- `docs/plans/2026-08-11-hermes-acp-transport-spike.md`
- `docs/plans/2026-08-11-hermes-agent-runtime-adapter.md`
- `docs/reviews/2026-08-11-hermes-acp-transport-spike-post-increment-review.md`
- `docs/spikes/HERMES_ACP_TRANSPORT_SPIKE.md`
- `src-tauri/tests/fixtures/hermes_acp_server_stub.py`
- `src-tauri/tests/hermes_acp_transport_spike.rs`

## Exact commands executed

- `python3 -m py_compile src-tauri/tests/fixtures/hermes_acp_server_stub.py` —
  Passed.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check` — Passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test hermes_acp_transport_spike --locked`
  — Passed: 5 passed, 0 failed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked`
  — Passed: 7 passed, 0 failed, 1 ignored.
- `cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked`
  — Passed: 20 passed, 0 failed.
- `cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked`
  — Passed.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
  — Passed.
- `./node_modules/.bin/prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/adr/ADR-HERMES-ACP-TRANSPORT.md docs/adr/ADR-HERMES-SERVE-WEBSOCKET-TRANSPORT.md docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md docs/increments/hermes-acp-transport-spike.md docs/plans/2026-08-11-hermes-acp-transport-spike.md docs/plans/2026-08-11-hermes-agent-runtime-adapter.md docs/reviews/2026-08-11-hermes-acp-transport-spike-post-increment-review.md docs/spikes/HERMES_ACP_TRANSPORT_SPIKE.md`
  — Passed; all listed files were formatted.
- `npm run verify` — Passed, including 124 frontend tests, the complete Rust
  suite, frontend build, and Tauri no-bundle release build.
- `npm run docs:check` — Passed.
- `npm run repository:check` — Passed.
- `npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `git diff --exit-code -- src src-tauri/src src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents`
  — Passed; no protected production/dependency/configuration path changed.
- `python3 .codex/hooks/session_end_gate.py` — Passed with the exact expected
  17-path dirty inventory and no conflicts.
- `python3 .codex/hooks/post_increment_gate.py status` — Active before
  finalization.
