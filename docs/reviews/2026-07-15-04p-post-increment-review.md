# Increment 4P post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git log -3 --oneline --decorate",
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/post_increment_gate.py begin --increment 04p",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo fmt --manifest-path src-tauri/Cargo.toml",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::function_call_validation::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test policy_input_binding --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "npm run verify",
    "npm audit --audit-level=low",
    "git diff --diff-filter=U --name-only",
    "if rg -n --hidden --glob '!.git/**' --glob '!node_modules/**' --glob '!src-tauri/target/**' --glob '!dist/**' --glob '!.codex/state/**' --glob '!**/__pycache__/**' -- '(sk-(?:proj-)?[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16}|-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----)' .; then exit 1; else rg_status=$?; test \"$rg_status\" -eq 1; fi",
    "git diff --exit-code HEAD -- package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json src-tauri/capabilities SECURITY.md CODE_REVIEW.md TROUBLESHOOTING_LOG.md docs/product docs/workflows .codex",
    "npm run format:check",
    "git diff --check",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment 04p --report docs/reviews/2026-07-15-04p-post-increment-review.md"
  ],
  "files_changed": [
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/04p-schema-bound-initial-gateway-events.md",
    "docs/plans/04p-schema-bound-initial-gateway-events.md",
    "docs/plans/README.md",
    "docs/reviews/2026-07-15-04p-post-increment-review.md",
    "src-tauri/src/agent/gateway_request.rs",
    "src-tauri/tests/gateway_request_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low for any unsupported external consumer to adopt InitialGatewayEvent and InitialGatewayTurnError",
      "milestone": "Before publishing the Rust crate or supporting external consumers",
      "risk": "Narrowing InitialGatewayTurn frame results from raw normalized events and bare protocol errors may require migration for a theoretical unsupported external Rust consumer.",
      "severity": "Advisory",
      "summary": "The intentional bound-turn event and error API narrowing may affect a theoretical unsupported external consumer."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "Low; require future initial transport code to own InitialGatewayTurn rather than reconstruct lower-level validation",
      "milestone": "Before authenticated gateway transport or live provider traffic",
      "risk": "Lower-level normalized events, raw calls, registries, and validators remain public for focused tests, so future code could bypass schema-bound turn ownership unless transport integration is reviewed.",
      "severity": "Advisory",
      "summary": "Lower-level raw protocol and registry APIs remain intentionally public outside the bound initial-turn path."
    }
  ],
  "increment_id": "04p",
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
      "command": "git diff --diff-filter=U --name-only",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "if rg -n --hidden --glob '!.git/**' --glob '!node_modules/**' --glob '!src-tauri/target/**' --glob '!dist/**' --glob '!.codex/state/**' --glob '!**/__pycache__/**' -- '(sk-(?:proj-)?[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16}|-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----)' .; then exit 1; else rg_status=$?; test \"$rg_status\" -eq 1; fi",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "git diff --exit-code HEAD -- package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json src-tauri/capabilities SECURITY.md CODE_REVIEW.md TROUBLESHOOTING_LOG.md docs/product docs/workflows .codex",
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
Increment: 04p
Branch: `main`

## Executive summary

Increment 4P is complete within its exact two-file source/test and 11-file
closeout scope. `InitialGatewayTurn` now owns the exact local schema registry and
returns only closed events whose function variant contains a locally
`SchemaValidatedFunctionCall`. Local schema rejection is typed, content-free,
and terminal at the wrapper. Every acceptance criterion is met. The result is
`PASS WITH ADVISORIES`; the advisories record intentional public API narrowing
and the preserved lower-level raw protocol/registry boundary.

## Verification results

Passed:

- mandatory 04p gate state began before source edits;
- rustfmt and Clippy with warnings denied;
- six request, 18 protocol, six independent function-validation, nine tool, eight
  public gateway-contract, and two policy-binding tests;
- complete `npm run verify` with 17 hook, 124 frontend, 92 Rust library, and 19
  Rust integration tests plus lint, typecheck, builds, and Tauri release
  no-bundle;
- network-enabled npm audit with zero vulnerabilities; and
- conflict, secret, whitespace, exact-scope, generated-output, complete-diff,
  architecture, code-health, security, preserved-boundary, and documentation
  reviews.

Failed and resolved:

- The first sandboxed 04p begin could not write ignored state; the approved
  elevated retry succeeded before source edits.
- The first rustfmt check found only integration-test import wrapping; `cargo fmt`
  corrected it inside the approved test path, and the final check passed.
- The first public contract compile found two identity assertions still matching
  bare protocol errors. They now match `InitialGatewayTurnError::Protocol`, and
  the rerun passed all eight tests.
- The first sandboxed npm audit could not resolve the registry or write npm logs;
  the approved network-enabled retry passed with zero vulnerabilities.

Checks not run:

- No native application or interaction check was required because there is no
  production caller, Tauri route, WebView behavior, native API, network,
  credential, persistence, or permission change.
- No Rust dependency audit was required because manifests and lockfiles are
  unchanged.

Manual verification pending: none.

## Architecture findings

No blocking finding. The bound turn reuses the existing validator and local
function-call validator, derives one private registry from the same fixed schema
array, and converts protocol events with an exhaustive match. It adds no
coordinator, transport, continuation, policy, approval, audit, dispatch, or
execution abstraction.

Advisory: lower-level `GatewayStreamValidator`, `ValidatedGatewayEvent`,
`UntrustedFunctionCall`, registry, and validation APIs remain public for focused
tests and independently verified downstream boundaries. Future initial transport
code must own `InitialGatewayTurn` rather than reconstructing those steps.

## Security findings

No blocking finding. Raw function argument JSON is consumed immediately inside
the bound turn. The exact registry, tool identity, version, typed arguments, risk,
and permission derive only from trusted Rust catalog values. Local schema failure
closes the wrapper before policy can receive a call, rejects late frames, and
makes cancellation a no-op. Output and arguments remain redacted from debug and
errors.

No model or WebView authority, Tauri IPC, capability, CSP, unsafe Rust, SQLite,
filesystem, Keychain, credential, network, policy allowance, approval, audit,
dispatch, executor, entitlement, or permission boundary changed. A schema-valid
call remains non-authorizing.

## Code-health findings

No blocking finding. The implementation uses closed typed events and errors,
preserves non-cloneable/non-serializable content-bearing values, and avoids
duplicating protocol or schema logic. Public tests cover both tools, locally
derived classification, protocol transactionality, local schema rejection,
terminal state, text/failure events, cancellation, and redaction. No dependency,
production panic-style shortcut, dead runtime wiring, or unrelated abstraction
was added.

## Technical debt

Advisory: the bound turn's public event/error API is intentionally narrowed. Risk
is limited to a theoretical unsupported external consumer because the crate is
unpublished and every repository caller is migrated; effort is low; milestone is
before publishing or supporting external consumers; it blocks neither completion
nor later bounded planning.

Advisory: lower-level raw protocol and registry APIs remain public. Risk is a
future initial transport bypassing bound-turn schema ownership; effort is low;
milestone is before authenticated transport; O-006 and O-007 still block live
traffic, and this advisory does not block 4P completion.

## Roadmap findings

No later implementation item is Ready, and this review does not reorder
`NEXT_STEPS.md`. O-006 and O-007 continue to block live gateway/provider traffic.
The project owner must explicitly direct 4P publication before later planning or
implementation.

## Completion decision

`PASS WITH ADVISORIES`. Every required automated check passed, no manual check is
required, the complete 13-path change set was reviewed, no Critical or High
blocking issue remains, and D-037 plus project memory match the implementation.
This result does not authorize commit, push, merge, live traffic, execution, or
another increment.

## Next-increment readiness

`Blocked`. Increment 4P is complete, but no later increment is Ready. Exact next
task: wait for explicit project-owner direction to commit, push, and merge
Increment 4P.

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
resolved sandbox begin/audit failures and assertion migration are recorded above
and in `HANDOFF.md`.
