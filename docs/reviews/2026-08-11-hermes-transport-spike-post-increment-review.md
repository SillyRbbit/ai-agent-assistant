# Hermes transport spike post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "/usr/bin/python3 -m py_compile src-tauri/tests/fixtures/hermes_tui_gateway_stub.py",
    "env PYTHONPYCACHEPREFIX=/private/tmp/cortexa-hermes-spike-pycache /usr/bin/python3 -m py_compile src-tauri/tests/fixtures/hermes_tui_gateway_stub.py",
    "cargo fmt --manifest-path src-tauri/Cargo.toml",
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked",
    "cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings",
    "./node_modules/.bin/prettier --write docs/plans/2026-08-11-hermes-transport-spike.md docs/spikes/HERMES_TRANSPORT_SPIKE.md",
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
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md",
    "docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md",
    "docs/increments/hermes-transport-spike.md",
    "docs/plans/2026-08-11-hermes-transport-spike.md",
    "docs/reviews/2026-08-11-hermes-transport-spike-post-increment-review.md",
    "docs/spikes/HERMES_TRANSPORT_SPIKE.md",
    "src-tauri/tests/fixtures/hermes_tui_gateway_stub.py",
    "src-tauri/tests/hermes_transport_spike.rs"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Architecture",
      "effort": "a separately owner-selected documentation-only ADR revision and fresh readiness review",
      "milestone": "before accepting the multi-runtime ADR or implementing any runtime adapter",
      "risk": "the Proposed ADR's preferred raw TUI-gateway stdio mechanism is not a supported public production contract at the evaluated release",
      "severity": "High",
      "summary": "The Proposed multi-runtime ADR cannot be accepted unchanged."
    },
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "separate transport selection, immutable provenance, whole-process containment, descendant ownership, restricted capability, secrets, packaging, and target-platform evidence",
      "milestone": "before installing, starting, or integrating a real Hermes runtime",
      "risk": "a real Hermes process would retain host filesystem, network, process, Keychain, tool, configuration, skill, MCP, and descendant authority that the cooperative fixture cannot deny",
      "severity": "High",
      "summary": "Fixture process hygiene is not a containment boundary."
    }
  ],
  "increment_id": "hermes-transport-spike",
  "manual_verification": [
    {
      "check": "Official tagged release, package metadata, programmatic guide, raw TUI source, client launch path, and security model were inspected for Hermes Agent 0.20.0 / v2026.8.3 / commit 3c27eb6234bf91b8ceee9e9071591b31e9b148cb.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "The complete diff contains only the two test-only experimental files and evidence documents; no production source, dependency, manifest, lockfile, feature, UI, IPC, permission, runtime behavior, or ADR status changed.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Execute the ignored real-Hermes version probe against an installed, contained, operator-supplied binary.",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "env PYTHONPYCACHEPREFIX=/private/tmp/cortexa-hermes-spike-pycache /usr/bin/python3 -m py_compile src-tauri/tests/fixtures/hermes_tui_gateway_stub.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked",
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
Increment: hermes-transport-spike
Branch: main

## Executive summary

The isolated spike is complete. Official pinned upstream evidence disproves raw
TUI-gateway stdio as the selected supported production contract for Hermes
Agent `0.20.0` / `v2026.8.3`: no public raw-gateway launcher, initial
version/capability negotiation, or gateway-shutdown RPC exists. The deterministic
fixture proves bounded host mechanics only. Hermes was neither installed nor
executed, native behavior is unchanged, and the Proposed ADR remains Proposed.
**PASS WITH ADVISORIES**.

## Scope and boundaries

The increment added one Unix-only Rust integration-test target, one cooperative
Python fixture, the required spike report, additive assessment/ADR notices, and
plan/current-memory/review evidence. The harness is not linked into the
application. It uses direct fixed-argument child creation, cleared and isolated
test state, bounded NDJSON/stdout, count-only stderr, closed errors, deadlines,
terminal cancellation, and direct-child cleanup.

The exact 13-path inventory stayed within plan. There is no Hermes dependency,
feature, package, manifest, lockfile, production source, UI, Tauri IPC,
capability, permission, runtime selector, provider, credential, networking,
external write, or ADR acceptance.

## Verification results

Final focused verification reports seven ordinary spike tests passed and one
explicit real-Hermes version probe ignored. The ten-test native gateway contract
passes. Strict all-target/all-feature Clippy, frontend checks/tests/build,
complete Rust tests, Tauri no-bundle release build, repository health, security
scan, documentation checks, formatting, protected-production-path assertion,
and deterministic session inventory pass.

The real-Hermes probe was not run by design because Hermes is absent and no
whole-process-contained execution was approved. No manual desktop exercise was
required because no application behavior, source, configuration, or UI changed.

Intermediate failures were not suppressed:

- The first Python syntax command failed before reading the fixture because
  macOS Python attempted to create a cache outside the sandbox. The isolated
  `PYTHONPYCACHEPREFIX` command passed.
- Initial format checks identified new-file formatting and passed after exact
  Rust/Prettier formatting.
- Focused fixture tests exposed macOS-injected environment keys and `/var` path
  aliasing; the final closed-name and canonical-path evidence passes.
- Initial strict Clippy found one needless test-helper lifetime; the corrected
  final command passes.
- The first `npm run verify` stopped at the documented Prettier failure; the
  final full verification passes after formatting and the reviewed fixes.

## Architecture findings

`$architecture-review`: no completion-blocking finding.

- The fixture and supervisor live entirely under `src-tauri/tests/`; no runtime
  abstraction or shipping composition root was created.
- Application, native gateway, provider transport, policy, approval, audit,
  tools, memory, secrets, and platform ownership remain unchanged.
- The spike distinguishes the internal raw TUI module from supported public
  launchers and does not select ACP or `hermes serve` automatically.
- The assessment notice preserves historical evidence while superseding only
  its raw-stdio preference. The ADR remains Proposed.

High next-increment advisory: the Proposed ADR cannot be accepted unchanged.

## Security findings

`$security-review`: no completion-blocking finding; real Hermes execution remains
blocked by a High advisory.

Ordinary tests launch only the repository fixture through a canonicalized fixed
interpreter and fixed arguments. Prompt text crosses stdin JSON, never argv or a
shell. Frames, counts, deadlines, identifiers, methods/events, stderr, and
version output are bounded. Error and debug surfaces retain closed codes/counts,
not prompt, frame, stderr, environment values, or child diagnostics. The command
starts from `env_clear`; tests close key names and verify isolated paths without
printing their values. Every exercised terminal path reaps the direct child.

These controls are hygiene, not containment. They do not prevent absolute
filesystem, network, process, Keychain, or descendant access and do not prove a
real Hermes restricted profile. No real process/session/model/provider test ran.

## Code-health findings

`$code-review`: no unresolved finding.

Focused review initially found that late output was labeled rather than rejected
through terminal state, the fixed Python path would reject common Linux
symlinks, and environment evidence was too open. The corrected harness now
returns `LateOutput` from terminal-aware acceptance, canonicalizes only the fixed
fixture interpreter while keeping the real probe strict, and uses a closed set
of explicit/system-injected key names plus value-free canonical-path equality.
Re-review confirms all three findings resolved. Strict Clippy and tests pass.

## Technical debt

`$technical-debt`: None introduced. The intentionally disposable harness is
test-only, dependency-free, excluded from production, and has a bounded rollback.
Its Unix/system-Python limitation and lack of containment are explicitly recorded
as spike limitations and next-runtime blockers, not hidden production debt.

## Roadmap findings

`$readiness-review`: **Blocked**.

No runtime or product increment is Ready. The smallest possible follow-up is a
separately owner-selected documentation-only revision of the Proposed
multi-runtime ADR. That revision must compare native-only architecture with
supported public surfaces without silently selecting ACP or `hermes serve`.
Any later executable proposal needs its own accepted decision, readiness review,
plan, provenance, containment, capability, secrets, packaging, and target-system
evidence.

## Completion decision

**PASS WITH ADVISORIES**

The advisories block subsequent Hermes/runtime implementation, not completion of
the requested isolated negative-result spike.

## Next-increment readiness

**Blocked** — do not install or run Hermes or implement a runtime adapter. The
only identified follow-up is an owner-selected documentation-only ADR revision.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`
- `docs/architecture/HERMES_INTEGRATION_ASSESSMENT.md`
- `docs/increments/hermes-transport-spike.md`
- `docs/plans/2026-08-11-hermes-transport-spike.md`
- `docs/reviews/2026-08-11-hermes-transport-spike-post-increment-review.md`
- `docs/spikes/HERMES_TRANSPORT_SPIKE.md`
- `src-tauri/tests/fixtures/hermes_tui_gateway_stub.py`
- `src-tauri/tests/hermes_transport_spike.rs`

## Exact commands executed

- `/usr/bin/python3 -m py_compile src-tauri/tests/fixtures/hermes_tui_gateway_stub.py`
  — failed before file examination because the default macOS Python cache path
  was sandbox-denied.
- `env PYTHONPYCACHEPREFIX=/private/tmp/cortexa-hermes-spike-pycache /usr/bin/python3 -m py_compile src-tauri/tests/fixtures/hermes_tui_gateway_stub.py`
  — passed.
- `cargo fmt --manifest-path src-tauri/Cargo.toml` — passed and formatted the
  test target.
- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check` — initially found
  formatting drift; final run passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --test hermes_transport_spike --locked`
  — intermediate environment/canonicalization assertions failed and were fixed;
  final result passed, 7 passed and 1 ignored.
- `cargo test --manifest-path src-tauri/Cargo.toml --test gateway_request_contract --locked`
  — passed, 10 passed.
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings`
  — initially found one needless lifetime; final run passed.
- `./node_modules/.bin/prettier --write docs/plans/2026-08-11-hermes-transport-spike.md docs/spikes/HERMES_TRANSPORT_SPIKE.md`
  — passed and formatted the two documents.
- `npm run verify` — initially stopped at Prettier; final post-fix run passed,
  including frontend, Rust, Tauri, repository, and audit checks.
- `npm run docs:check` — passed.
- `npm run repository:check` — passed.
- `npm run security:scan` — passed.
- `git diff --check` — passed.
- `git diff --exit-code -- src src-tauri/src src-tauri/Cargo.toml src-tauri/Cargo.lock package.json package-lock.json src-tauri/tauri.conf.json src-tauri/capabilities .github .codex .agents`
  — passed with no protected production/configuration change.
- `python3 .codex/hooks/session_end_gate.py` — passed with no conflicts or
  suspicious paths.
- `python3 .codex/hooks/post_increment_gate.py status` — active before
  finalization; required complete and valid after finalization.
