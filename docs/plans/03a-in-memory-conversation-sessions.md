# Execution plan — Increment 3A in-memory conversation sessions

Last updated: 2026-07-13

Status: **Complete**

## Gap analysis

The Phase 3 product target is a complete conversation UI and mocked agent loop. The verified Phase 2 implementation already supplies most of the loop and center-pane behavior:

| Product capability               | Verified repository state                                                            | Gap                                                                         |
| -------------------------------- | ------------------------------------------------------------------------------------ | --------------------------------------------------------------------------- |
| User and assistant messages      | Deterministic in-memory messages render in the center pane.                          | No gap for one transcript.                                                  |
| Streaming and cancellation       | Fixed chunks stream progressively; Stop cancels the driver and rejects stale events. | No gap for the mock driver.                                                 |
| Tool activity and action preview | `create_local_task` mock activity and exact non-executing approval preview render.   | A distinct verified tool-result view remains later work.                    |
| Error and Retry                  | Bounded failure copy and deterministic Retry are covered.                            | No gap for one active transcript.                                           |
| Activity visibility              | Fixed-copy, redacted events render newest-first.                                     | It is intentionally volatile presentation, not the later trusted audit log. |
| Conversation history             | State contains one global `messages` array and one global `toolActivities` array.    | Missing conversation identity, list, selection, and transcript restoration. |
| New conversation                 | Native New Request routes to Conversations and clears only the draft.                | Missing creation of a separate conversation.                                |
| Context controls and provenance  | Not implemented.                                                                     | Later bounded Phase 3 work after conversation identity exists.              |
| Attachments and voice            | Not implemented.                                                                     | Deferred pending trust, file-scope, and permission design.                  |

Conversation sessions are the smallest missing capability because they satisfy an explicit MVP requirement, correct the misleading New Request behavior, and establish a stable UI boundary for later context provenance without adding networking, trusted execution, or persistence.

## Goal and user-visible outcome

Add deterministic volatile conversation sessions. The user can start a new conversation, see in-memory conversation titles in the sidebar, switch among idle conversations, and recover each session's messages and mock tool activity until the WebView reloads.

## Scope

- Introduce a typed `ConversationSession` model with IDs such as `conversation-1`, bounded title, messages, and mock tool activity.
- Start with one selected empty conversation.
- Derive the title from the first normalized request, collapse whitespace, and cap it at 48 Unicode code points including a trailing ellipsis when truncated. Empty sessions use `New conversation`.
- Store mock messages and tool activity in the selected conversation instead of global transcript arrays.
- Add an accessible New conversation control and newest-first volatile conversation list to the sidebar.
- Make the existing native `new_request` route create or select an empty conversation while idle.
- Select an existing empty conversation instead of creating another; create a new empty conversation only when none exists.
- Restore a selected session's messages and tool activity.
- Disable and reducer-reject conversation creation or switching while a run is streaming or awaiting approval.
- Preserve global redacted Activity events and monotonic run/activity ordinals across conversations.

## Explicit non-goals

- SQLite or other conversation persistence.
- Conversation deletion, rename, search, export, pinning, synchronization, or draft-per-conversation behavior.
- Preserving Retry eligibility after leaving a failed conversation.
- Production model or gateway integration.
- Additional model turns, tool schemas, real tools, trusted approvals, or execution.
- Context controls, provenance presentation, file attachment, or voice input.
- Rust, Tauri IPC, capabilities, CSP, dependencies, lockfiles, packaging, credentials, network access, or operating-system permissions.

## Existing behavior and constraints

- React `useReducer` and context remain the state architecture under D-014.
- The deterministic frontend mock loop remains under D-015 and D-016.
- `get_app_info` remains the only custom Tauri command.
- The WebView remains presentation-only and cannot authorize or execute a local action.
- All conversation state remains volatile and clears on WebView reload.
- Driver cleanup continues to own cancellation; navigation must not orphan an active callback stream.

## Exact proposed files

Create:

```text
src/application/conversations.ts
src/application/conversations.test.ts
```

Change runtime and tests:

```text
src/application/state.ts
src/application/state.test.ts
src/App.tsx
src/App.test.tsx
src/components/ApplicationSidebar.tsx
src/features/conversations/ConversationWorkspace.tsx
src/styles.css
```

Update implementation records only after approval and implementation:

```text
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
docs/increments/03a-in-memory-conversation-sessions.md
docs/plans/03a-in-memory-conversation-sessions.md
```

No file outside this list may change without stopping for project-owner approval.

## Implementation steps

1. Add the typed volatile conversation model, deterministic constructors, bounded title derivation, and focused pure tests.
2. Refactor reducer transcript updates so messages and tool activity are always scoped to the selected conversation.
3. Add closed New conversation and select conversation actions with idle-only guards and duplicate-empty-session prevention.
4. Bind native `new_request` to the same idle-only creation rule while preserving route focus.
5. Render the New conversation control and newest-first session list in the sidebar with active and disabled states.
6. Pass only the selected session to the existing conversation workspace and expose its title without changing mock execution semantics.
7. Add reducer and component tests for creation, title bounds, selection, restoration, busy-state rejection, native routing, failures, Retry, Activity, and existing navigation.
8. Run focused checks, the complete repository gate, dependency audit, native launch, and manual regression checks.
9. Perform security review and synchronize all implementation records with actual evidence.

## Risks and mitigations

- **Stale events update the wrong conversation:** reject creation and selection while a run is active; keep exact run-ID checks; test matching and stale events after attempted navigation.
- **Messages and tool activity cross conversation boundaries:** centralize immutable selected-session updates in application state and test two distinct transcripts.
- **Blank-session clutter:** select the existing empty session and never retain more than one empty conversation.
- **Unbounded or misleading titles:** normalize once, cap titles at 48 Unicode code points including any ellipsis, and use `New conversation` only for empty sessions.
- **Retry targets another conversation:** clear Retry eligibility when leaving or replacing the failed active conversation and reducer-reject invalid Retry actions.
- **Sensitive content reaches Activity:** keep Activity construction unchanged and continue testing that request text is absent.
- **Reducer regression from state reshaping:** preserve run/activity ordinals and cover streaming, Stop, approval, failure, Retry, and menu routing in focused tests before the full gate.

## Security and privacy considerations

- Conversation titles and transcripts remain only in WebView memory and are not logged or copied into Activity.
- No new IPC payload, command, native event value, database record, file access, network request, credential, or permission is introduced.
- Sidebar selection is navigation only and conveys no approval authority.
- Busy-state guards prevent navigation from detaching UI state from a live driver or pending mock approval.
- Mock approval remains explicitly non-executing and untrusted.

## Test plan

- Conversation constructor rejects invalid ordinals and creates IDs such as `conversation-1`.
- Title derivation trims input, collapses whitespace, applies the 48-code-point maximum including any ellipsis, and does not mutate message content.
- Initial state has one selected empty conversation.
- First submission titles and updates only the selected conversation.
- New conversation preserves the prior transcript and selects an existing empty conversation before creating another.
- Selection restores the correct messages and tool activity.
- New and select actions are rejected during streaming and approval states.
- Native New Request uses the same idle-only semantics.
- Failed Retry remains scoped and cannot migrate a request across conversations.
- Existing Stop, stale-event, approval, Activity-redaction, diagnostics, navigation, and lifecycle-facing tests remain green.

## Verification commands

```bash
npx vitest run src/application/conversations.test.ts src/application/state.test.ts src/App.test.tsx
npm run typecheck
npm run lint:frontend
npm run verify
npm audit --audit-level=low
git diff --check
npm run tauri -- dev
```

Manual native checks:

1. Start one request, complete a mock decision, create a second conversation, and confirm both transcripts restore correctly.
2. Confirm New conversation reuses the existing empty session and never leaves multiple blank entries.
3. Confirm New conversation and conversation selection are unavailable during streaming and pending approval.
4. Confirm the menu-bar New Request action follows the same rule.
5. Confirm streaming, Stop, failure/Retry tests, approval decisions, Activity redaction, Settings diagnostics, close/reopen/Dock/quit, idempotent storage startup, and no-permission-prompt behavior remain intact.

## Rollback or failure strategy

Restore the single-transcript reducer and existing sidebar props, remove the conversation model and list UI, and retain all verified Phase 2 mock-driver, Activity, Rust, Tauri, storage, security, and lifecycle behavior. Do not weaken busy-state guards or persistence restrictions to make the increment pass.

## Acceptance criteria

- [x] One selected volatile conversation exists at startup.
- [x] First submission assigns a title capped at 48 Unicode code points including any ellipsis and stores transcript state only in that conversation.
- [x] New conversation preserves completed prior transcripts and never leaves more than one empty session.
- [x] Selecting an idle conversation restores its messages and mock tool activity.
- [x] Creation and selection fail closed while streaming or awaiting approval.
- [x] Native New Request follows the same deterministic idle-only rule.
- [x] Streaming, Stop, failure, Retry, approval, stale-event, and Activity-redaction behavior remains correct in automated coverage.
- [x] Conversation data remains volatile and no sensitive content is added to Activity or logs.
- [x] No unapproved file changes.
- [x] Focused checks, `npm run verify`, dependency audit, native launch, security review, and documentation synchronization pass.
- [x] Project-owner native interaction and layout checks pass.

## Approval gate

Approved by the project owner before implementation. No file outside the approved list changed.

## Actual results

Implemented the approved volatile conversation model, reducer scoping, sidebar history, native route behavior, accessibility labels, styles, and focused tests. Active and retryable runs now require their conversation ID to match the selected conversation before state changes are accepted.

Automated evidence:

```text
targeted Vitest: 3 files, 57 tests passed
npm run verify: passed
full Vitest: 7 files, 83 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release no-bundle build: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
native Tauri development launch: passed
storage startup: idempotent, 2 migrations already applied
```

The project owner confirmed the complete native interaction and layout checklist passed. Increment 3A is verified complete.
