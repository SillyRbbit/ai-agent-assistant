# Execution plan — Increment 3C simulated tool result

Last updated: 2026-07-13

Status: **Complete**

## Gap analysis

The Phase 3 product target requires a complete conversation UI and mocked agent loop. The verified implementation through Increment 3B covers conversation identity, messages, streaming, cancellation, context disclosure, mock tool activity, approval decisions, bounded failure, Retry, and redacted Activity. It does not represent or render a distinct tool result.

| Product capability              | Verified repository state                                                                   | Remaining gap                                                                 |
| ------------------------------- | ------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- |
| Conversation state              | Volatile sessions restore messages, context provenance, and tool activity.                  | No gap for current in-memory ownership.                                       |
| Tool proposal and preview       | A fixed `create_local_task` proposal and exact mock approval preview render.                | No gap for the existing proposal.                                             |
| Approval decisions              | Approve, reject, and edit update tool activity and append a fixed assistant outcome.        | No trusted execution occurs by design.                                        |
| Tool result                     | There is no result type, result state, or result view.                                      | Missing explicit, run-bound presentation after mock approval.                 |
| Provider continuation           | The fixed assistant outcome ends the run after a decision.                                  | Returning results to a provider and continuing model turns remain later work. |
| Real execution and verification | The WebView cannot execute tools, and the Rust tool interfaces are not exposed through IPC. | Must remain out of scope for Phase 3.                                         |

A simulated tool result is the smallest coherent Phase 3C capability. The center-pane product requirements explicitly include tool results, while provider continuation, trusted executor output, and real tools would cross later-phase boundaries. An approve-only fixed result can prove attribution, state ownership, restoration, and presentation without implying that a tool ran.

## Goal and user-visible outcome

After the user chooses `Approve mock`, append a clearly labeled simulated tool result to the owning conversation. The result identifies the mock tool and run, states that no execution occurred, contains no request content, and restores with that volatile conversation. Reject and Edit create no result.

## Scope

- Add a closed typed `MockToolResult` model bound to exact run and conversation IDs.
- Derive the associated tool-activity ID from the validated run ID.
- Use fixed fields only:
  - Tool name: `create_local_task`.
  - Status: `simulated`.
  - Executed: `false`.
  - Summary: fixed copy stating that no local task was created and no data changed.
- Construct a result only for an accepted `Approve mock` decision.
- Store the result only in the owning volatile `ConversationSession`.
- Reject malformed IDs and fail closed if a valid result cannot be constructed.
- Render an accessible `Mock tool result` card with explicit `Simulated` and `No execution` labeling.
- Preserve the existing fixed assistant outcome, tool-activity status, context provenance, Activity redaction, Retry, cancellation, and conversation behavior.

## Explicit non-goals

- Real tool execution, task creation, device access, or side effects.
- Trusted Rust executor results, validation, policy, approval transactions, or audit evidence.
- Returning a result to a model/provider or adding additional model turns.
- Result schemas for arbitrary tools, multiple tools, dynamic payloads, or untrusted external content.
- Tool errors, partial results, timeouts, progress, result expansion, copy, export, or persistence.
- Context source selection, attachments, voice, model providers, gateway access, credentials, OAuth, or networking.
- Rust, IPC, Tauri commands, capabilities, CSP, dependencies, lockfiles, packaging, SQLite, or operating-system permissions.

## Existing behavior and constraints

- React reducer state remains the frontend state architecture under D-014.
- The deterministic mock loop remains frontend-only under D-015 and D-016.
- Conversations and context provenance remain volatile under D-017 and D-018.
- `get_app_info` remains the only custom Tauri command.
- The WebView cannot authorize or execute a local action.
- The existing mock approval is not trusted authorization and executes nothing.
- Activity records remain fixed-copy and cannot receive request text, arguments, or result content.
- Approval decisions are accepted only for the exact active awaiting-approval run and selected conversation.

## Exact proposed files

Create runtime and focused tests:

```text
src/application/mockToolResult.ts
src/application/mockToolResult.test.ts
src/features/conversations/ToolResultCard.tsx
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
docs/increments/03c-simulated-tool-result.md
docs/plans/03c-simulated-tool-result.md
```

No file outside this list may change without stopping for project-owner approval.

## Implementation steps

1. Add the closed result types, strict ID validation, deterministic constructor, fixed summary, and focused pure tests.
2. Extend `ConversationSession` with tool results and include that field in empty-session detection.
3. In the approval reducer, construct and append one result only for `approve`; reject or edit must retain an unchanged result list.
4. Keep duplicate or stale decisions rejected through the existing active-run state guard.
5. Pass the selected session's results through `App` to the conversation workspace.
6. Render a dedicated accessible card after tool activity with explicit simulated and no-execution labeling.
7. Add reducer and interaction tests for approve-only creation, ID attribution, no duplication, privacy, Retry attribution, and conversation restoration.
8. Run focused checks, the complete repository gate, dependency audit, native launch, and manual regression checks.
9. Perform security review and synchronize implementation records with actual evidence.

## Risks and mitigations

- **A simulated result is mistaken for executor evidence:** encode `executed: false`, label it `Simulated` and `No execution`, and state that it is frontend mock data.
- **Reject or Edit incorrectly produces a result:** branch only on the closed `approve` decision and test all three outcomes.
- **Duplicate decisions append duplicate results:** retain the existing awaiting-approval guard and test a repeated decision is rejected.
- **A result is attributed to the wrong run or conversation:** bind validated run and conversation IDs, derive the proposal ID, store through the owning session update, and test restoration.
- **Request or preview content leaks into result state:** make the constructor accept IDs only and use fixed tool, status, execution, and summary values.
- **Result content reaches Activity:** leave Activity construction unchanged and retain redaction tests.
- **The type becomes a premature generic executor contract:** support only the fixed `create_local_task` mock and defer arbitrary payloads or external data.
- **UI density regresses:** use a compact card and verify normal and minimum supported window sizes.

## Security and privacy considerations

- The result is volatile untrusted WebView presentation, not trusted executor or audit output.
- The constructor contains no request, argument, preview, error, path, or arbitrary-content input.
- React renders fixed text without an HTML injection path.
- `Approve mock` remains a presentation-only decision and cannot invoke IPC or a tool.
- No result content is copied into Activity or logs.
- No native command, database record, network call, credential, platform API, or operating-system permission is introduced.

## Test plan

- The result constructor rejects blank or malformed run and conversation IDs.
- A valid result has deterministic result and proposal IDs, `create_local_task`, `simulated`, `executed: false`, and exact fixed summary copy.
- The constructor API and output contain no request-content field.
- Initial and newly created conversations start without tool results.
- Approve appends exactly one result to the active conversation.
- Reject and Edit append no result.
- A repeated, stale, or mismatched decision cannot append a result.
- An approved retried run attributes its result to the fresh run ID without duplicating the user message.
- Conversation selection restores only that session's results.
- A result-bearing session is not treated as empty.
- The accessible card shows tool name, run ID, `Simulated`, `No execution`, and fixed summary.
- Request text is absent from the result card and Activity.
- Existing messages, provenance, tool activity, streaming, Stop, approval, failure, Retry, stale-event, navigation, diagnostics, and native-route tests remain green.

## Verification commands

```bash
npx vitest run src/application/mockToolResult.test.ts src/application/conversations.test.ts src/application/state.test.ts src/App.test.tsx
npm run typecheck
npm run lint:frontend
npm run verify
npm audit --audit-level=low
git diff --check
npm run tauri -- dev
```

Manual native checks:

1. Submit a request, choose `Approve mock`, and confirm exactly one compact `Mock tool result` card appears.
2. Confirm the card shows `create_local_task`, the run ID, `Simulated`, `No execution`, and fixed no-change copy without repeating the request.
3. Confirm Reject and Edit produce no result card.
4. Complete approved runs in two conversations and confirm each result restores only with its owning conversation.
5. Confirm Stop produces no result and the normal and minimum supported layouts remain usable.
6. Confirm context disclosure, tool activity, Activity redaction, conversation controls, native New Request, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts remain intact.

## Rollback or failure strategy

Remove the simulated-result model and card, restore the prior `ConversationSession` shape and workspace props, and retain every verified Increment 3B behavior. Do not weaken approval-state guards, ID validation, privacy, or no-execution labeling to make the increment pass.

## Acceptance criteria

- [x] Approve appends exactly one deterministic simulated result bound to the active run, conversation, and proposal.
- [x] Reject, Edit, Stop, stale events, and invalid decisions append no result.
- [x] Retry attributes an approved result to the fresh run ID without duplicating the request.
- [x] The result contains only validated opaque IDs and fixed fields; request text and personal content are absent.
- [x] The UI explicitly shows `Simulated`, `No execution`, and fixed no-change copy.
- [x] Conversation switching restores only the owning session's results.
- [x] Existing mock-loop, conversation, context, cancellation, approval, Activity, navigation, diagnostics, and native-route behavior remains correct.
- [x] No dependency, Rust, IPC, Tauri, capability, CSP, persistence, credential, network, packaging, or permission expansion occurs.
- [x] Focused checks, `npm run verify`, dependency audit, native launch, security review, documentation synchronization, and project-owner manual checks pass.
- [x] No file outside the approved list changes.

## Approval gate

The project owner approved this exact file plan before implementation. Implementation and every automated and manual acceptance gate are complete.

## Actual results

Implemented the closed fixed-result model, exact proposal binding, approve-only conversation storage, accessible result card, styles, and focused validation, decision, Retry, privacy, and restoration tests. No file outside the approved list changed.

Automated and native launch evidence:

```text
targeted Vitest: 4 files, 70 tests passed
npm run typecheck: passed
npm run lint:frontend: passed
npm run verify: passed
full Vitest: 9 files, 104 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release no-bundle build: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
native Tauri development launch: passed
storage startup: idempotent, 2 migrations already applied
```

The initial full-gate run stopped at Prettier wrapping, and the next run stopped at one optional-chaining lint preference. Both were corrected within the approved files before the complete gate passed. They were development feedback, not repository setup or runtime defects, so `TROUBLESHOOTING_LOG.md` remains unchanged.

The project owner confirmed approve-only result creation, fixed simulated and no-execution presentation, request-content exclusion, no result after Reject/Edit/Stop, session restoration, normal and minimum-window layout, and existing native regressions passed. Increment 3C is verified complete on the target Mac.
