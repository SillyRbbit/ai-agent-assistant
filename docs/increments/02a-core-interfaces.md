# Phase 2 Increment 2A — Platform-neutral core interfaces

Status: Verified complete  
Verified on: 2026-07-09  
Platform: macOS on Apple Silicon  
Scope: Architecture-only Rust core contracts and deterministic mocks

## Goal

Add platform-neutral Rust interfaces and deterministic mock or no-op implementations for:

- AgentProvider
- ToolRegistry
- PolicyEngine
- ApprovalManager
- AuditLogger
- MemoryStore
- PlatformAdapter

This increment creates trusted local-core contracts for later work without adding real model access, backend gateway calls, SQLite, OS automation, privileged permissions, file access, calendar access, contacts, reminders, notifications, clipboard tools, shell execution, or new Tauri commands.

## Files created or changed

Rust source:

```text
src-tauri/src/lib.rs
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

Project memory and increment documentation:

```text
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

- Accepts an agent request.
- Returns assistant text, tool-call requests, or a structured error.
- Includes deterministic mock responses keyed by request content.
- Rejects invalid empty requests.

### ToolRegistry

- Registers tool definitions.
- Looks up tools by name.
- Rejects unknown tools.
- Exposes metadata for name, description, risk class, required permission, and schema placeholder.
- Lists tools in deterministic name order.

### PolicyEngine

- Evaluates proposed actions using deterministic rules.
- Supports allow, require approval, and deny decisions.
- Denies missing required permissions before risk evaluation.
- Denies prohibited autonomy.
- Requires approval for personal-data modification and high-impact risk classes.

### ApprovalManager

- Creates approval requests.
- Records approve and reject decisions.
- Lists pending approvals in deterministic order.
- Rejects unknown or already decided approvals.

### AuditLogger

- Records audit events in deterministic order.
- Provides a no-op logger for future tests that need validation without persistence.
- Redacts simple secret-like key/value tokens.
- Rejects invalid empty event data.

### MemoryStore

- Stores, lists, updates, and deletes memory records.
- Separates session, working, and preference memory.
- Rejects simple secret-like content by default.
- Returns records in deterministic order.

### PlatformAdapter

- Exposes platform-neutral metadata.
- Exposes placeholder capability status for future adapters.
- Does not call macOS APIs.
- Returns deterministic capability reports.

## Shared policy placeholders

Risk classes added:

```text
InformationOnly
ReadOnlyDeviceAccess
ReversibleLocalAction
PersonalDataModification
ExternalOrHighImpactAction
ProhibitedAutonomy
```

Permission placeholders added:

```text
None
Calendar
Reminders
Contacts
Notifications
Files
Accessibility
ScreenRecording
Automation
Microphone
```

## Verification results

The following checks were reported passing locally after applying formatting and installing npm dependencies:

```text
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked
npm run typecheck
npm run build
npm run tauri -- dev
```

Rust tests reported:

```text
25 Rust unit tests passed
1 Rust integration test passed
0 Rust tests failed
```

Frontend verification reported:

```text
TypeScript check passed
Vite production build passed
Native app launched successfully
```

## Security notes

This increment intentionally does not add:

- API keys
- OpenAI or other model networking
- Backend gateway calls
- SQLite
- macOS permissions
- New Tauri commands
- Accessibility
- ScreenCaptureKit
- Apple Events
- Shell execution
- OAuth
- File tools
- Calendar tools
- Contact tools
- Reminder tools
- Notification tools
- Clipboard tools
- Unrestricted command execution

The existing UI and `get_app_info` IPC behavior are preserved.

## Follow-up

Stop after Increment 2A. The next recommended increment is Increment 2B-0: choose and document the SQLite dependency and migration approach before implementing persistence.
