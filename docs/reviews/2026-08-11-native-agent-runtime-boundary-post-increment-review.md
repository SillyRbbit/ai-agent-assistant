# Native agent runtime boundary post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "python3 .codex/hooks/post_increment_gate.py begin --increment native-agent-runtime-boundary",
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/session_end_gate.py",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::tests::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::tests::",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
    "npm run verify",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/native-agent-runtime-boundary.md",
    "docs/plans/2026-08-11-native-agent-runtime-boundary.md",
    "docs/reviews/2026-08-11-native-agent-runtime-boundary-post-increment-review.md",
    "src-tauri/src/agent/mod.rs",
    "src-tauri/src/agent/native_runtime.rs",
    "src-tauri/src/agent/runtime.rs",
    "src-tauri/tests/agent_runtime_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Code health",
      "effort": "Small separately approved documentation-only edit and documentation gate",
      "milestone": "Before any later external-runtime implementation",
      "risk": "Future agents may read the root planned-concepts wording as current absence and make an inaccurate architecture or readiness decision.",
      "severity": "Advisory",
      "summary": "Root AGENTS.md still describes AgentRuntime and NativeAgentRuntime as planned concepts even though this separately authorized increment now implements the native foundation."
    }
  ],
  "increment_id": "native-agent-runtime-boundary",
  "manual_verification": [
    {
      "check": "No manual application check is required because the Rust foundation remains unwired to Tauri and React and the existing visible mock is unchanged.",
      "required": false,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
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
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked",
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
    }
  ]
}
-->

Date: 2026-08-11
Increment: native-agent-runtime-boundary
Branch: main

## Executive summary

Implemented the owner-approved application-owned runtime foundation and the
sole/default `NativeAgentRuntime` wrapper over the unchanged native initial-turn
path. A private deterministic `MockAgentRuntime` tests the same closed event and
lifecycle contract. Native request serialization, event validation, local tool
schema, policy, approval, audit, and cancellation behavior remain delegated to
the verified native boundary. The complete automated suite and independent
reviews pass. No Hermes, provider, process, network, dependency, credential,
Tauri/React wiring, selector, automatic fallback, or visible behavior was
added. The result is `PASS WITH ADVISORIES` only because root `AGENTS.md` retains
pre-implementation planned-concepts wording outside this increment's approved
file inventory.

## Scope and boundaries

The exact 13-path inventory matches the approved source, test, architecture,
plan, increment, project-memory, and review scope. New product code is limited
to `agent::runtime`, `agent::native_runtime`, and two module exports. The one new
integration test contains the private mock. Existing native source/tests,
frontend, Tauri IPC/capabilities/configuration, manifests, lockfile, workflows,
dependencies, credentials, storage, and platform code remain unchanged.

The common contract owns only a closed descriptor, typed bounded start request,
redacted run correlation, one untrusted event sink, closed acceptance/failure,
status, and exact idempotent cancellation. `NativeAgentRun` owns exactly one
`InitialGatewayTurn`. The shared lane privately translates only supported
application-owned events through the unchanged validator. The concrete lane
preserves exact governance-bearing native results. The two lanes are mutually
exclusive and neither grants execution, approval, policy, audit, provider,
network, or platform authority.

## Verification results

- Passed: gate `native-agent-runtime-boundary` began from clean synchronized
  `main` at `701c061` and remained the sole active increment until finalization.
- Passed: `cargo fmt --check`, all-target/all-feature `cargo check`, and strict
  all-target/all-feature Clippy.
- Passed: all-target Rust tests — 150 passed, 0 failed, with one explicitly
  opt-in real-Hermes version probe ignored as designed.
- Passed: the focused runtime contract — 20 passed, 0 failed — plus the
  unchanged public gateway contract — 10 passed, 0 failed.
- Passed: `npm run verify`, including Prettier, repository checks, ESLint, 28
  hook tests, 38 repository tests, 124 frontend tests, 101 Rust library tests,
  all Rust integration tests, TypeScript checking, frontend production build,
  and the Tauri release build without bundling.
- Passed: documentation formatting/links, repository health, secret scan,
  whitespace/error diff check, and conflict-free session-end inventory.
- Passed as not required: no manual application check. No shipping application
  path or user-visible behavior changed.

## Architecture findings

PASS. Independent review initially found that a descriptor/start/cancel-only
shape was not a usable D-079 runtime boundary. The corrected implementation adds
one closed application-owned event sink shared by Native and the private mock.
`RuntimeRunIdentity` supports events arriving after the consuming start call
without retaining selected content. Fixed capabilities are truthful; Native
reports streaming text and does not report shared tool proposals. Mixed lanes
fail closed in both directions. No provider/runtime conflation, framework type
leak, generic RPC, speculative session registry, async dependency, selector, or
automatic fallback was introduced.

## Security findings

PASS. Runtime events remain untrusted and traverse the unchanged native
identity, sequence, state, size, content, local-schema, policy, approval, and
audit boundaries. Tool proposals are not accepted on Native's shared lane; the
concrete lane alone retains governance-bearing results. Generic cancellation
closes a nonterminal stream or delegates exact audited pending-approval
termination without returning approval/audit data. IDs, selected text, output
text, call IDs, and arguments are bounded and redacted from Debug/errors. No
unsafe Rust, secret, credential, environment, filesystem, database, IPC,
permission, CSP, operating-system, process, network, provider, model, or
execution path was added.

## Code-health findings

PASS with one documentation advisory. The contract uses application-owned
newtypes, closed enums, typed errors, fixed capability storage, and no production
`unwrap`, `expect`, `panic!`, `todo!`, `unimplemented!`, or `dbg!`. Native logic
is delegated rather than copied. The private mock uses the common contract and
terminalizes controlled failures and capability contradictions. Focused tests
cover identity, availability, health, capabilities, post-start correlation,
text/failure lifecycle, every failure-code mapping, invalid transitions,
limits, redaction, lane isolation, native policy/approval parity, audited
cancellation, deterministic mock behavior, and late/duplicate terminal input.

Advisory: root `AGENTS.md` still labels the runtime names planned concepts. The
exact owner-authorized prompt superseded that implementation prerequisite for
this increment, but updating repository-governance instructions was outside the
approved path inventory and should be a separate documentation-only change.

## Technical debt

None introduced in product source. The unwired/no-coordinator state, absence of
a runtime selector, and blocked Hermes work are explicit non-goals rather than
hidden incomplete behavior. The stale root-instruction wording is recorded as a
documentation advisory, not implementation debt.

## Roadmap findings

The later contained Hermes serve/WebSocket spike remains `Blocked`. This native
foundation must first be published to a clean synchronized baseline, root
governance wording should be reconciled through a separately approved
documentation change, and a fresh security/readiness review must verify the
exact immutable Hermes distribution, installed extras, no-install/no-update
controls, secret-source denial, endpoint/Unix-socket containment, detached-
descendant cleanup, and target-Mac evidence. The Draft Hermes adapter remains
Blocked on a passing spike and separately approved dependencies/scope. No next
implementation is Ready automatically.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked

The exact next task is owner review and, only with separate owner direction,
commit/push publication of this verified native boundary as one bounded commit.
Do not begin the contained Hermes spike or adapter. Before any later external-
runtime implementation, perform a separately scoped governance-document sync
for root `AGENTS.md` and a fresh security/readiness review.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/increments/native-agent-runtime-boundary.md`
- `docs/plans/2026-08-11-native-agent-runtime-boundary.md`
- `docs/reviews/2026-08-11-native-agent-runtime-boundary-post-increment-review.md`
- `src-tauri/src/agent/mod.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `src-tauri/src/agent/runtime.rs`
- `src-tauri/tests/agent_runtime_contract.rs`

## Exact commands executed

- Passed: `git status --short --branch`
- Passed: `python3 .codex/hooks/post_increment_gate.py begin --increment native-agent-runtime-boundary`
- Passed: `python3 .codex/hooks/post_increment_gate.py status`
- Passed: `python3 .codex/hooks/session_end_gate.py`
- Passed: `cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked`
- Passed: `cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::tests::`
- Passed: `cargo test --manifest-path src-tauri/Cargo.toml --test agent_runtime_contract --locked`
- Passed: `cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::tests::`
- Passed: `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
- Passed: `cargo check --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked`
- Passed: `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
- Passed: `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked`
- Passed: `npm run verify`
- Passed: `npm run docs:check`
- Passed: `npm run repository:check`
- Passed: `npm run security:scan`
- Passed: `git diff --check`
