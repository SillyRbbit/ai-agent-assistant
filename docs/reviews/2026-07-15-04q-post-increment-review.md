# Increment 4Q post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git log -3 --oneline --decorate",
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/post_increment_gate.py begin --increment 04q",
    "cargo fmt --manifest-path src-tauri/Cargo.toml",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "npm run verify",
    "npm audit --audit-level=low",
    "npx prettier --write AGENTS.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md docs/increments/04q-terminally-release-initial-function-call.md docs/plans/04q-terminally-release-initial-function-call.md docs/plans/README.md docs/reviews/2026-07-15-04q-post-increment-review.md",
    "git diff --diff-filter=U --name-only",
    "if rg -n --hidden --glob '!.git/**' --glob '!node_modules/**' --glob '!src-tauri/target/**' --glob '!dist/**' --glob '!.codex/state/**' --glob '!**/__pycache__/**' -- '(sk-(?:proj-)?[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16}|-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----)' .; then exit 1; else rg_status=$?; test \"$rg_status\" -eq 1; fi",
    "git diff --exit-code HEAD -- package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json src-tauri/capabilities SECURITY.md CODE_REVIEW.md TROUBLESHOOTING_LOG.md docs/product docs/workflows .codex",
    "npm run format:check",
    "git diff --check",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment 04q --report docs/reviews/2026-07-15-04q-post-increment-review.md"
  ],
  "files_changed": [
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/04q-terminally-release-initial-function-call.md",
    "docs/plans/04q-terminally-release-initial-function-call.md",
    "docs/plans/README.md",
    "docs/reviews/2026-07-15-04q-post-increment-review.md",
    "src-tauri/src/agent/gateway_request.rs",
    "src-tauri/tests/gateway_request_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low for any unsupported external consumer to handle the optional event result",
      "milestone": "Before publishing the Rust crate or supporting external consumers",
      "risk": "Narrowing InitialGatewayTurn frame results to Option may require migration for a theoretical unsupported external Rust consumer and None must be interpreted only as a pending function call.",
      "severity": "Advisory",
      "summary": "The intentional optional-event API narrowing may affect a theoretical unsupported external consumer."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "Low; require future initial transport code to own InitialGatewayTurn",
      "milestone": "Before authenticated gateway transport or live provider traffic",
      "risk": "Lower-level protocol and registry APIs remain public for fixtures, so future code could bypass terminal-release ownership unless transport integration is reviewed.",
      "severity": "Advisory",
      "summary": "Lower-level protocol and registry APIs remain intentionally public outside the bound initial-turn path."
    }
  ],
  "increment_id": "04q",
  "manual_verification": [],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked",
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
      "command": "npm audit --audit-level=low",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run format:check",
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

Date: 2026-07-15
Increment: 4Q
Branch: `main` with verified uncommitted changes

## Executive summary

Increment 4Q is complete within its exact two-file source/test scope.
`InitialGatewayTurn` privately retains one schema-validated function call after
its non-terminal frame returns `None`. Accepted terminal completion releases the
exact call once; accepted failure and successful cancellation discard it;
transactional protocol errors retain it for the correct terminal frame. Text and
local schema-failure behavior remain unchanged.

Every required automated check passed. No manual gate is required, the complete
13-path change set is within declared scope, and no Critical or High blocking
finding remains. The quality result is `PASS WITH ADVISORIES`.

## Verification results

Passed:

- Rust formatting check.
- Six request unit tests.
- Eighteen gateway protocol unit tests.
- Six function-call validation unit tests.
- Nine tool unit tests.
- Nine public gateway-request contract tests.
- Two public policy-input binding tests.
- Clippy for all targets and features with warnings denied.
- Complete `npm run verify`: 17 hook, 124 frontend, 92 Rust library, and 20
  Rust integration tests plus lint, typecheck, Vite builds, and Tauri release
  no-bundle.
- Network-enabled npm audit with zero vulnerabilities.
- Conflict, secret, preserved-boundary, formatting, whitespace, exact-scope,
  complete-diff, code, security, and documentation checks.

The first sandboxed gate-begin attempt could not write ignored local state; the
approved elevated retry succeeded before source edits. The first sandboxed npm
audit could not resolve the registry or write npm logs; the approved
network-enabled retry passed with zero vulnerabilities.

Checks not run:

- No native application or interaction check was required because there is no
  production caller, Tauri route, WebView behavior, native API, network,
  credential, persistence, capability, entitlement, or permission change.
- No Rust dependency audit was required because manifests and lockfiles are
  unchanged.

Manual verification pending: none.

## Architecture findings

No blocking finding. The change keeps pending-call lifecycle inside the existing
bound turn and reuses the verified protocol and schema validators. It adds no
coordinator, transport, continuation, policy, approval, audit, dispatch, or
execution abstraction.

Advisory: lower-level protocol and registry APIs remain public for focused
fixtures and independently verified boundaries. Future initial transport code
must own `InitialGatewayTurn` rather than reconstructing early release.

## Security findings

No blocking finding. A model or gateway proposal cannot leave the bound turn
before terminal response completion. Pending arguments are already bounded and
typed, remain ephemeral and inaccessible, and are omitted from debug, errors,
logs, audit, persistence, and IPC. Failure and cancellation discard the pending
call before terminal state rejects late frames. Protocol rejection remains
transactional and cannot create early authority.

No model or WebView authorization, Tauri IPC, capability, CSP, unsafe Rust,
SQLite, filesystem, Keychain, credential, network, policy allowance, approval,
audit, dispatch, executor, entitlement, or permission boundary changed. Terminal
completion and schema validity remain non-authorizing.

## Code-health findings

No blocking finding. The implementation adds one private optional field and one
closed optional-event transition without duplicating protocol state. Public tests
cover both tools, exact release, malformed/mismatched/out-of-sequence retention,
failure and cancellation discard, text completion, schema rejection, terminal
state, classification, and redaction. No production panic-style shortcut,
dependency, dead runtime wiring, or unrelated abstraction was added.

## Technical debt

Advisory: `InitialGatewayTurn::accept_frame` intentionally returns an optional
event. Risk is limited to a theoretical unsupported external consumer and future
misinterpretation of `None`; effort is low; milestone is before publishing or
supporting external consumers; it blocks neither completion nor later bounded
planning.

Advisory: lower-level protocol and registry APIs remain public. Risk is a future
initial transport bypassing terminal-release ownership; effort is low; milestone
is before authenticated transport; O-006 and O-007 still block live traffic, and
this advisory does not block 4Q completion.

## Roadmap findings

No later implementation item is Ready, and this review does not reorder
`NEXT_STEPS.md`. O-006 and O-007 continue to block live gateway/provider traffic.
The project owner must explicitly direct 4Q publication before later planning or
implementation.

## Completion decision

`PASS WITH ADVISORIES`. Every required automated check passed, no manual check is
required, the complete 13-path change set was reviewed, no Critical or High
blocking issue remains, and D-038 plus project memory match the implementation.
This result does not authorize commit, push, merge, live traffic, execution, or
another increment.

## Next-increment readiness

`Blocked`. Increment 4Q is complete, but no later increment is Ready. Exact next
task: wait for explicit project-owner direction to commit, push, and merge
Increment 4Q.

## Exact files changed

The 13 paths in the machine manifest are the complete tracked and untracked
change set. Source and test work is limited to the approved two paths. The other
paths are approved planning, closeout, decision, and review documentation. No
preserved product, security, workflow, dependency, Tauri, storage, capability,
entitlement, or permission boundary changed.

## Exact commands executed

The machine manifest records the material Git and gate inspection, gate begin,
focused tests, formatting, Clippy, complete verification, dependency audit,
conflict, secret, preserved-boundary, diff, and finalization commands. The
resolved sandbox begin and audit failures are recorded above and in `HANDOFF.md`.
