# Increment 4O post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git log -1 --oneline --decorate",
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/post_increment_gate.py begin --increment 04o",
    "npm run typecheck",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_request::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked agent::gateway_protocol::",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked tools::",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "npm run verify",
    "npm audit --audit-level=low",
    "git diff --diff-filter=U --name-only",
    "if rg -n --hidden --glob '!.git/**' --glob '!node_modules/**' --glob '!src-tauri/target/**' --glob '!dist/**' --glob '!.codex/state/**' --glob '!**/__pycache__/**' -- '(sk-(?:proj-)?[A-Za-z0-9_-]{20,}|AKIA[0-9A-Z]{16}|-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----)' .; then exit 1; else rg_status=$?; test \"$rg_status\" -eq 1; fi",
    "git diff --exit-code HEAD -- package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock src-tauri/tauri.conf.json src-tauri/capabilities SECURITY.md CODE_REVIEW.md TROUBLESHOOTING_LOG.md docs/product docs/workflows .codex",
    "npm run format:check",
    "git diff --check",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment 04o --report docs/reviews/2026-07-15-04o-post-increment-review.md"
  ],
  "files_changed": [
    "AGENTS.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/increments/04o-bound-initial-gateway-turn.md",
    "docs/plans/04o-bound-initial-gateway-turn.md",
    "docs/plans/README.md",
    "docs/reviews/2026-07-15-04o-post-increment-review.md",
    "src-tauri/src/agent/gateway_request.rs",
    "src-tauri/tests/gateway_request_contract.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Code health",
      "effort": "Low for any unsupported external consumer to adopt InitialGatewayTurn",
      "milestone": "Before publishing the Rust crate or supporting external consumers",
      "risk": "Making InitialGatewayRequest private narrows a previously public Rust API even though the crate is not published and repository callers are migrated.",
      "severity": "Advisory",
      "summary": "The intentional raw-request API narrowing may affect a theoretical unsupported external consumer."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Architecture",
      "effort": "Low; require future initial transport code to accept InitialGatewayTurn rather than IDs and a detached validator",
      "milestone": "Before authenticated gateway transport or live provider traffic",
      "risk": "GatewayStreamValidator::new remains public for protocol fixtures, so future code could bypass bound-turn construction unless transport ownership is reviewed.",
      "severity": "Advisory",
      "summary": "The lower-level validator remains intentionally public outside the initial-turn construction path."
    }
  ],
  "increment_id": "04o",
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
Increment: 04o
Branch: `main`

## Executive summary

Increment 4O is complete within its exact two-file source/test and 11-file
closeout scope. It replaces detached public initial-request construction with one
transport-free `InitialGatewayTurn` that privately owns request bytes and a
response validator derived from the same IDs and exact local tool catalog. Every
acceptance criterion is met. The result is `PASS WITH ADVISORIES`; the advisories
record intentional public API narrowing and the preserved lower-level validator
boundary.

## Verification results

Passed:

- mandatory 04o gate state began before source edits;
- rustfmt and Clippy with warnings denied;
- six preserved request, 18 protocol, nine tool-catalog, and six public bound-turn
  contract tests;
- complete `npm run verify` with 17 hook, 124 frontend, 92 Rust library, and 17
  Rust integration tests plus lint, typecheck, builds, and Tauri release
  no-bundle;
- network-enabled npm audit with zero vulnerabilities; and
- conflict, secret, whitespace, exact-scope, generated-output, complete-diff,
  architecture, code-health, security, preserved-boundary, and documentation
  reviews.

Failed and resolved:

- The first sandboxed 04o begin could not write ignored state; the approved
  elevated retry succeeded before source edits.
- The first sandboxed npm audit could not resolve the registry or write npm logs;
  the approved network-enabled retry passed with zero vulnerabilities.
- Code-health review found a test helper translating a request-construction error
  into an unrelated protocol error. The helper now returns the original typed
  error before final verification.
- The first gate finalization rejected the unsupported `Compatibility` finding
  category before writing a marker. The advisory now uses the allowed
  `Code health` category, and refinalization passed.

Checks not run:

- No native application or interaction check was required because there is no
  production caller, Tauri route, WebView behavior, native API, network, or
  permission change.
- No Rust dependency audit was required because manifests and lockfiles are
  unchanged.

Manual verification pending: none.

## Architecture findings

No blocking finding. `InitialGatewayTurn` stays inside the existing request
module, owns exactly one request and validator, and exposes only the four planned
operations. It adds no coordinator, retry scheduler, deadline owner, continuation
contract, transport, or replacement protocol abstraction.

Advisory: `GatewayStreamValidator::new` remains public for protocol fixtures and
independently verified downstream boundaries. Future initial transport code must
accept the bound turn rather than reconstructing a detached validator.

## Security findings

No blocking finding. Request and response IDs enter once through trusted Rust;
allowed names and version derive only from the exact local catalog. Selected text
appears only in borrowed request bytes. Debug and errors retain no selected
content, request body, or identities. Mismatched IDs, unknown tools, wrong
versions, cancellation, and late frames fail closed.

No model or WebView authority, Tauri IPC, capability, CSP, unsafe Rust, SQLite,
filesystem, Keychain, credential, network, policy, approval, audit, dispatch,
executor, entitlement, or permission boundary changed. A validated event remains
non-authorizing and must pass all later trusted boundaries.

## Code-health findings

No blocking finding. The implementation is non-cloneable, uses typed closed
errors, delegates the existing validator without duplicating its state machine,
and keeps exact serialization tests private to the module. Public tests cover the
new API and adversarial configuration cases. The review-found test error mapping
was corrected before final verification. No dependency, production panic-style
shortcut, dead runtime wiring, or unrelated abstraction was added.

## Technical debt

Advisory: making `InitialGatewayRequest` private may affect an unsupported external
consumer. Risk is limited because the crate is unpublished and repository callers
are migrated; effort is low; milestone is before publishing or supporting
external consumers; it blocks neither completion nor later bounded planning.

Advisory: the lower-level validator constructor remains public. Risk is a future
transport bypassing bound-turn construction; effort is low; milestone is before
authenticated transport; it blocks live traffic under O-006 but not 4O.

## Roadmap findings

No later implementation item is Ready, and this review does not reorder
`NEXT_STEPS.md`. O-006 and O-007 continue to block live gateway/provider traffic.
The project owner must explicitly direct 4O publication before later planning or
implementation.

## Completion decision

`PASS WITH ADVISORIES`. Every required automated check passed, no manual check is
required, the complete 13-path change set was reviewed, no Critical or High
blocking issue remains, and D-036 plus project memory match the implementation.
This result does not authorize commit, push, merge, live traffic, execution, or
another increment.

## Next-increment readiness

`Blocked`. Increment 4O is complete, but no later increment is Ready. Exact next
task: wait for explicit project-owner direction to commit, push, and merge
Increment 4O.

## Exact files changed

The 13 paths in the machine manifest are the complete tracked and untracked
change set. Source and test work is limited to the approved two paths. The other
paths are approved planning, closeout, decision, and review documentation. No
preserved product, security, workflow, dependency, Tauri, storage, or permission
boundary changed.

## Exact commands executed

The machine manifest records the material Git and gate inspection, gate begin,
baseline and focused tests, formatting, Clippy, full verification, dependency
audit, conflict, secret, preserved-boundary, diff, and finalization commands. The
resolved sandbox begin/audit failures and test-helper correction are recorded
above and in `HANDOFF.md`.
