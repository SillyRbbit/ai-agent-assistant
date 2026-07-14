# Execution plan — Increment 3B mock context provenance

Last updated: 2026-07-13

Status: **Complete**

## Gap analysis

The Phase 3 product target requires a complete conversation UI and mocked agent loop. It also requires the user to see what information the assistant used. The verified implementation through Increment 3A has deterministic conversations and a bounded mock run, but it does not disclose run context.

| Product capability    | Verified repository state                                                                            | Remaining gap                                                              |
| --------------------- | ---------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Conversation identity | Volatile sessions own messages and mock tool activity.                                               | No gap for in-memory identity or restoration.                              |
| Mocked agent loop     | Submit, streaming, Stop, approval decisions, failure, Retry, and stale-event rejection are verified. | Additional model turns are not required for the next smallest increment.   |
| Tool activity         | A fixed `create_local_task` proposal and decision state render without execution.                    | A distinct tool-result model remains later work.                           |
| Context provenance    | The workspace mentions context, but no run records or displays what information was used.            | Missing typed, run-bound disclosure.                                       |
| Context controls      | No context source selector exists.                                                                   | Deferred because selection implies broader policy and trust-boundary work. |
| Trusted provenance    | The Rust architecture assigns trusted validation and provenance to the core.                         | Deferred; a WebView-only mock must not claim trusted evidence.             |

Context provenance is the smallest coherent Phase 3B capability. Decision D-017 explicitly identifies conversation identity as a prerequisite for later provenance work, and the product brief requires users to see what information the agent used. A fixed mock disclosure can satisfy the presentation and state-transition gap without introducing real data access, networking, persistence, or execution.

## Goal and user-visible outcome

Add a clearly labeled mock context disclosure for every deterministic run. The user can see that the current request was used and that earlier messages, saved memory, device data, and external services were not used. The disclosure is scoped to its conversation and restored with that volatile session.

## Scope

- Add a closed typed `MockContextProvenance` model bound to exact run and conversation IDs.
- Construct provenance from internal IDs only; do not accept or copy request text into the record.
- Use fixed source categories and fixed status copy:
  - Current request — used.
  - Earlier conversation messages — not used.
  - Saved memory — not used.
  - Device data — not used.
  - External services — not used.
- Append exactly one provenance record when each mock run starts, including each Retry run.
- Store provenance only in the owning volatile `ConversationSession`.
- Render an accessible `Mock context used` disclosure in the conversation timeline.
- Clearly state that the disclosure is a frontend mock and not trusted audit evidence.
- Preserve the current transcript, tool activity, approval, Retry, Activity, cancellation, and session-switching behavior.

## Explicit non-goals

- User-selectable context controls or automatic context discovery.
- Reading prior messages as model context beyond the submitted request.
- Real memory, file, clipboard, calendar, contact, application, device, or external-service data.
- Attachments, voice input, model providers, gateway access, credentials, OAuth, or networking.
- Trusted Rust provenance, audit persistence, SQLite records, or cross-session restoration.
- Tool-result modeling, additional mock tool schemas, real tools, approvals, or execution.
- Rust, IPC, Tauri commands, capabilities, CSP, dependencies, lockfiles, packaging, or operating-system permissions.

## Existing behavior and constraints

- React reducer state remains the frontend state architecture under D-014.
- The deterministic mock loop remains frontend-only under D-015 and D-016.
- Conversation state remains volatile under D-017.
- `get_app_info` remains the only custom Tauri command.
- The WebView cannot authorize or execute a local action.
- Activity records remain fixed-copy and must not receive request text or provenance details.
- Active run events remain bound to exact run and conversation IDs.

## Exact proposed files

Create runtime and focused tests:

```text
src/application/contextProvenance.ts
src/application/contextProvenance.test.ts
src/features/conversations/ContextProvenanceCard.tsx
```

Change runtime and tests:

```text
src/application/conversations.ts
src/application/conversations.test.ts
src/application/state.ts
src/application/state.test.ts
src/App.tsx
src/App.test.tsx
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
docs/increments/03b-mock-context-provenance.md
docs/plans/03b-mock-context-provenance.md
```

No file outside this list may change without stopping for project-owner approval.

## Implementation steps

1. Add the closed provenance types, fixed source list, deterministic constructor, and focused pure tests.
2. Extend `ConversationSession` with provenance records and include that field in empty-session detection.
3. Append one record at mock-run start using only the script run ID and owning conversation ID.
4. Pass the active session records through `App` to the conversation workspace.
5. Render a dedicated accessible disclosure component with bounded fixed copy and explicit mock labeling.
6. Add reducer and interaction tests for attribution, Retry, restoration, privacy, and existing behavior.
7. Run focused checks, the complete repository gate, dependency audit, native launch, and manual regression checks.
8. Perform security review and synchronize implementation records with actual evidence.

## Risks and mitigations

- **Mock evidence is mistaken for trusted provenance:** label the view `Mock context used` and state that it is not trusted audit evidence.
- **Request content leaks into metadata or Activity:** make the constructor accept IDs only; use fixed display copy; assert the request is absent from provenance and Activity.
- **A record is attributed to the wrong conversation:** bind it to both run and conversation IDs, store it through the existing selected-session update, and test two-session restoration.
- **Retry overwrites or duplicates prior evidence:** assign each Retry a fresh run ID and append exactly one fresh record.
- **An empty session is incorrectly reused:** count provenance as session content in `isConversationSessionEmpty`.
- **UI density regresses at minimum size:** use a compact disclosure component and include native minimum-window checks.
- **Scope drifts into real context collection:** keep source values fixed and add no external input, IPC, permission, persistence, or selector.

## Security and privacy considerations

- Provenance remains untrusted presentation state in volatile WebView memory.
- The record contains opaque internal IDs, fixed source labels, and fixed statuses only.
- Request text, tool arguments, tool results, errors, file paths, and personal content are excluded.
- A displayed `used` status conveys mock data flow only and grants no permission or approval authority.
- No native command, database record, network call, credential, platform API, or operating-system permission is introduced.

## Test plan

- The provenance constructor rejects blank or malformed run and conversation IDs.
- A valid record has deterministic IDs, exact fixed source order, and one `used` source.
- The constructor API and output contain no request text.
- Initial and newly created conversations start without provenance records.
- Each submit appends exactly one record to the active conversation at run start.
- Streaming, Stop, completion, failure, and approval decisions do not duplicate that record.
- Retry appends one fresh record with the new run ID and retains the original record.
- Conversation selection restores only that session's provenance records.
- A provenance-bearing session is not treated as empty.
- The accessible disclosure renders all fixed statuses and explicit mock labeling.
- Request text remains absent from provenance disclosure and Activity.
- Existing conversation, streaming, Stop, approval, failure, Retry, stale-event, Activity, navigation, diagnostics, and native-route tests remain green.

## Verification commands

```bash
npx vitest run src/application/contextProvenance.test.ts src/application/conversations.test.ts src/application/state.test.ts src/App.test.tsx
npm run typecheck
npm run lint:frontend
npm run verify
npm audit --audit-level=low
git diff --check
npm run tauri -- dev
```

Manual native checks:

1. Submit a request and confirm one compact `Mock context used` disclosure appears with only the current request marked used.
2. Confirm the disclosure does not repeat the request text and explicitly says it is not trusted audit evidence.
3. Complete a decision, create a second conversation, submit another request, and confirm disclosures restore only with their owning conversations.
4. Confirm Stop and all mock decisions do not duplicate a disclosure.
5. Confirm layout at the normal and minimum supported window sizes.
6. Confirm Activity redaction, conversation controls, native New Request, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts remain intact.

## Rollback or failure strategy

Remove the provenance model and component, restore the prior `ConversationSession` shape and workspace props, and retain every verified Increment 3A behavior. Do not weaken privacy, ID binding, session guards, or trust labeling to make the increment pass.

## Acceptance criteria

- [x] Every submitted mock run appends exactly one deterministic provenance record bound to its run and conversation.
- [x] Retry appends one new record and preserves the earlier run record.
- [x] The record contains only opaque IDs and fixed source/status values; request text and personal content are absent.
- [x] The UI shows the current request as used and all other modeled sources as not used.
- [x] The UI explicitly labels the disclosure as mock-only and not trusted audit evidence.
- [x] Conversation switching restores only the owning session's provenance records.
- [x] Existing mock-loop, conversation, cancellation, Retry, approval, stale-event, Activity, navigation, diagnostics, and native-route behavior remains correct.
- [x] No dependency, Rust, IPC, Tauri, capability, CSP, persistence, credential, network, packaging, or permission expansion occurs.
- [x] Focused checks, `npm run verify`, dependency audit, native launch, security review, documentation synchronization, and project-owner manual checks pass.
- [x] No file outside the approved list changes.

## Approval gate

The project owner approved this exact file plan before implementation. Implementation and every automated and manual acceptance gate are complete.

## Actual results

Implemented the closed fixed-copy provenance model, conversation ownership, run-start append, accessible disclosure, styles, and focused validation, privacy, Retry, and restoration tests. No file outside the approved list changed.

Automated and native launch evidence:

```text
targeted Vitest: 4 files, 66 tests passed
npm run typecheck: passed
npm run lint:frontend: passed
npm run verify: passed
full Vitest: 8 files, 92 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release no-bundle build: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
native Tauri development launch: passed
storage startup: idempotent, 2 migrations already applied
```

The project owner confirmed the context disclosure, request-content exclusion, mock-only trust labeling, session restoration, no-duplication behavior, minimum-window layout, and existing native regressions passed. Increment 3B is verified complete on the target Mac.
