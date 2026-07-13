# Increment 2A — core interfaces and deterministic mocks

## Goal

Establish platform-neutral Rust contracts for the trusted local core before adding SQLite, menu-bar behavior, model networking, macOS permissions, or new Tauri commands.

## Status

Status: **Verified complete**

Verification was completed on the target Apple Silicon Mac after applying formatting and installing the locked npm dependencies.

## Scope

Added architecture-only interfaces and deterministic in-memory or no-op implementations for:

- `AgentProvider`
- `ToolRegistry`
- `PolicyEngine`
- `ApprovalManager`
- `AuditLogger`
- `MemoryStore`
- `PlatformAdapter`

This increment preserves the existing React UI and the existing `get_app_info` IPC behavior.

## Files added

```text
src-tauri/src/agent/mod.rs
src-tauri/src/agent/provider.rs
src-tauri/src/agent/types.rs
src-tauri/src/tools/mod.rs
src-tauri/src/tools/registry.rs
src-tauri/src/tools/types.rs
src-tauri/src/policy/mod.rs
src-tauri/src/policy/engine.rs
src-tauri/src/policy/types.rs
src-tauri/src/approvals/mod.rs
src-tauri/src/approvals/manager.rs
src-tauri/src/approvals/types.rs
src-tauri/src/audit/mod.rs
src-tauri/src/audit/logger.rs
src-tauri/src/audit/types.rs
src-tauri/src/memory/mod.rs
src-tauri/src/memory/store.rs
src-tauri/src/memory/types.rs
src-tauri/src/platform/mod.rs
src-tauri/src/platform/adapter.rs
src-tauri/src/platform/types.rs
```

## Files changed

```text
src-tauri/src/lib.rs
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PROJECT_STATUS.md
TROUBLESHOOTING_LOG.md
docs/increments/02a-core-interfaces.md
```

## Implemented contracts

### AgentProvider

- Accepts an `AgentRequest` containing a run id, user message, and available tool names.
- Returns assistant text or tool-call proposals.
- Includes a deterministic `MockAgentProvider` with keyed responses and explicit forced-error support.

### ToolRegistry

- Registers `ToolDefinition` values.
- Looks up tools by name.
- Rejects unknown tools.
- Lists tools in deterministic name order.
- Exposes name, description, risk class, permission, input schema placeholder, and output schema placeholder.

### PolicyEngine

- Accepts a `PolicyAction` and returns `PolicyDecision`.
- Allows information-only and read-only actions when required permissions are granted.
- Requires approval for reversible local actions and personal-data modifications.
- Denies external/high-impact and prohibited-autonomy actions.
- Denies missing permissions before risk evaluation.

### ApprovalManager

- Creates approval requests with deterministic IDs.
- Lists pending approvals in deterministic order.
- Records approve and reject decisions.
- Rejects unknown or already decided approvals.

### AuditLogger

- Records audit events in deterministic sequence order.
- Provides a no-op implementation for tests or disabled persistence.
- Redacts simple secret-like tokens from audit details.
- Rejects invalid empty event types.

### MemoryStore

- Creates, lists, updates, and deletes memory records.
- Separates `Session`, `Working`, and `Preference` memory types.
- Rejects secret-like content before storage.
- Lists records deterministically.

### PlatformAdapter

- Returns platform-neutral metadata.
- Reports placeholder capability status.
- Lists capability reports deterministically.
- Does not call macOS APIs.

## Verification commands and results

Run on the target Mac:

```bash
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup default 1.90.0

cargo fmt --manifest-path src-tauri/Cargo.toml
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check

cargo clippy --manifest-path src-tauri/Cargo.toml \
  --all-targets \
  --all-features \
  --locked \
  -- -D warnings

cargo test --manifest-path src-tauri/Cargo.toml \
  --all-targets \
  --locked

npm ci
npm run typecheck
npm run build
npm run tauri -- dev
```

Actual results reported from the target Mac:

- `cargo fmt` applied standard Rust formatting.
- `cargo fmt --check` passed after formatting.
- `cargo clippy --all-targets --all-features --locked -- -D warnings` passed.
- `cargo test --all-targets --locked` passed with 25 Rust unit tests and 1 integration test.
- `npm ci` installed the local TypeScript toolchain.
- `npm run typecheck` passed.
- `npm run build` passed.
- `npm run tauri -- dev` launched the app.

## Security review

No new privileged behavior was added:

- No SQLite database.
- No model networking.
- No backend gateway calls.
- No API keys or API-key configuration.
- No OAuth.
- No new Tauri command.
- No macOS permission prompt.
- No Accessibility API.
- No ScreenCaptureKit.
- No Apple Events.
- No shell execution.
- No filesystem, calendar, contacts, reminders, notifications, or clipboard tools.

## Exit criteria

- All seven required interfaces compile.
- Each interface has a deterministic mock, no-op, or in-memory implementation.
- Focused tests cover representative success and failure behavior.
- High-risk and prohibited actions are not silently allowed.
- Unknown tools are rejected.
- Existing UI behavior is preserved.
- Existing `get_app_info` IPC behavior is preserved.
- Required Rust and npm verification passed on the target Mac.
