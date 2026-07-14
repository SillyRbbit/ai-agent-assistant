# Phase 3 Increment 3A — in-memory conversation sessions

Last updated: 2026-07-13

Status: **Verified complete**

## Goal

Replace the single volatile transcript with deterministic in-memory conversation sessions and expose New conversation plus idle conversation selection in the sidebar.

## Implemented behavior

- One selected empty conversation exists at startup.
- Sessions use deterministic `conversation-N` IDs.
- First requests produce whitespace-normalized titles capped at 48 Unicode code points including any ellipsis.
- Messages and mock tool activity are stored only in their conversation.
- Sidebar history renders newest-first with accessible creation and selection controls.
- New conversation reuses the existing empty session and creates one only when none exists.
- Selection restores the exact session transcript and tool activity.
- Active and retryable runs carry the owning conversation ID.
- Creation and selection are disabled and reducer-rejected during streaming and pending approval.
- Native New Request creates/selects an empty session while idle and only focuses the current conversation while busy.
- Leaving a failed conversation clears Retry eligibility.

## Files created

```text
src/application/conversations.ts
src/application/conversations.test.ts
docs/increments/03a-in-memory-conversation-sessions.md
docs/plans/03a-in-memory-conversation-sessions.md
```

## Files changed

```text
src/application/state.ts
src/application/state.test.ts
src/App.tsx
src/App.test.tsx
src/components/ApplicationSidebar.tsx
src/features/conversations/ConversationWorkspace.tsx
src/styles.css
AGENTS.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
```

No other source, dependency, lockfile, Rust, Tauri, capability, CSP, storage, packaging, credential, network, or permission file changed.

## Focused coverage

- Invalid and deterministic conversation ordinals.
- Whitespace-normalized and Unicode-safe bounded titles.
- Empty-session detection and reuse.
- Initial active conversation and conversation-scoped transcript updates.
- Newest-first history and transcript/tool-activity restoration.
- Busy-state creation and selection rejection.
- Run-event and Retry conversation binding.
- Idle and busy native New Request behavior.
- Existing streaming, Stop, approval, failure, Retry, stale-event, Activity-redaction, navigation, diagnostics, permissions, and menu routing.

## Automated verification evidence

```text
npm run typecheck: passed
npm run lint:frontend: passed
targeted Vitest: 3 files, 57 tests passed
npm run verify: passed
full Vitest: 7 files, 83 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release no-bundle build: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
```

## Security review

- The WebView gains no executor or authorization path.
- Conversation content remains in volatile frontend memory and is not copied into Activity or logs.
- Async run events require both the active run ID and owning conversation ID to match valid state.
- Retry cannot migrate a request to another conversation.
- New conversation and selection fail closed during streaming or pending approval.
- No production model, network, credential, OAuth, real tool, trusted approval, persistence, dependency, IPC, Tauri, Rust, capability, CSP, packaging, or operating-system permission change was added.

## Native verification status

The native Tauri development app compiled and launched. Storage startup remained idempotent with two migrations already applied.

The project owner confirmed:

- Initial and minimum-window conversation layout passed.
- Two-conversation creation, transcript restoration, and tool-card restoration passed.
- Existing empty-session reuse passed without duplicate blanks.
- Busy-state control disabling and re-enabling passed.
- Menu-bar New Request idle and busy behavior passed.
- Streaming, Stop, approval decisions, Activity redaction, navigation, diagnostics, close/reopen/Dock/quit, storage, and no-permission-prompt checks passed.

Increment 3A is verified complete on the target Mac.
