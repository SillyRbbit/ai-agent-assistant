# Execution plan — Increment 3D bounded mock-loop completion

Last updated: 2026-07-14

Status: **Complete**

## Completion gap analysis

The Phase 3 target is a complete conversation UI and mocked agent loop. The verified implementation through Increment 3C covers volatile conversations, deterministic streaming, Stop, fixed context provenance, one mock tool proposal, mock approval decisions, one fixed simulated result, bounded failure, Retry, stale-event rejection, and redacted Activity. The loop stops at the result: it does not render a distinct final answer after that result, and its conservative limits are not represented as one closed contract.

| Product requirement                      | Verified repository state                                                                                         | Remaining Phase 3 gap                                                                                 |
| ---------------------------------------- | ----------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| Return a tool result to the provider     | Increment 3C records a fixed simulated result after `Approve mock`; no provider is called.                        | A real return remains Phase 4 or later. Phase 3 can represent only a deterministic mock continuation. |
| Continue to final text                   | The approve outcome is stored with ordinary assistant messages and renders before the separately rendered result. | Missing a distinct final answer rendered immediately after its exact result.                          |
| Maximum model turns                      | The deterministic first response is bounded by fixed chunks.                                                      | No explicit run-level limit includes the post-result mock turn.                                       |
| Maximum tool calls                       | The current state machine has one proposal and accepts one decision.                                              | The one-call limit is structural but not declared with the other loop limits.                         |
| Retry attempts                           | A failed run can be retried with a fresh run ID.                                                                  | Repeated injected failures can continue offering Retry without an explicit attempt cap.               |
| Tool timeout, files, search, and network | No real tool, file, search, or network operation exists.                                                          | Record zero capacity so unsupported operations cannot be mistaken for latent Phase 3 behavior.        |
| Output length                            | Existing text is deterministic and finite.                                                                        | Record and enforce one conservative character ceiling for mock assistant output.                      |

Phase 3 should not close at Increment 3C. One final frontend-only Increment 3D is required to pair each approved simulated result with a deterministic final answer and make the existing mocked loop limits explicit and enforceable. Production provider continuation, verified executor results, and real operations remain later-phase work.

## Goal and user-visible outcome

After `Approve mock`, render the existing fixed simulated tool result followed immediately by one distinct fixed final answer bound to the same run, conversation, and result. Permit at most one Retry after a failed attempt and expose one closed conservative limit contract for the mocked loop without adding provider, network, tool, or native behavior.

## Scope

- Add a frozen `MOCK_LOOP_LIMITS` contract:
  - Maximum consecutive model turns: `2` — initial deterministic response plus one deterministic post-result final answer.
  - Maximum tool calls per run: `1`.
  - Maximum retry attempts: `1`.
  - Maximum network requests: `0`.
  - Tool timeout milliseconds: `0` because no tool executes.
  - Maximum file bytes: `0` because files are unavailable.
  - Maximum search results: `0` because search is unavailable.
  - Maximum assistant output characters per turn: `512`.
- Add a closed `MockFinalAnswer` model constructed from validated run, conversation, and result IDs only.
- Derive the final-answer ID from the run ID and require the exact `${runId}-result` result binding.
- Use fixed final-answer copy that states the action was simulated, no local task was created, and no data changed.
- Mark the final answer as deterministic frontend mock output and model turn `2`.
- Store final answers only in their owning volatile `ConversationSession`.
- On an accepted approve decision, construct and append the simulated result and its paired final answer atomically; fail closed if either identity is invalid or mismatched.
- Render each final answer immediately after its exact result.
- Track retry attempt `0` for initial submission and `1` for the only allowed Retry.
- After a retried run fails, retain the bounded failed state but do not offer another Retry.
- Preserve Reject and Edit fixed assistant outcomes and preserve Stop without a final answer.
- Preserve current cancellation, stale-event rejection, Activity redaction, conversation ownership, navigation, diagnostics, storage, lifecycle, capability, CSP, and permission behavior.

## Explicit non-goals

- A real or production provider continuation, second network request, or streamed second response.
- Returning result content to a provider or claiming provider-generated text.
- Real tools, trusted execution, policy authorization, approval transactions, executor output, or audit evidence.
- Arbitrary tool-result or final-answer content, dynamic external payloads, or more than one tool call.
- A generic run engine, general timeline refactor, configurable limits, or user-facing retry settings.
- Tool progress, timeout handling, file input, search, attachments, voice, or context source selection.
- Model gateways, OpenAI Responses integration, API keys, credentials, OAuth, cloud accounts, or networking.
- Rust, IPC, Tauri commands, capabilities, CSP, dependencies, lockfiles, packaging, SQLite, persistence, or operating-system permissions.

## Existing behavior and constraints

- React reducer state remains the frontend state architecture under D-014.
- The deterministic loop remains frontend-only under D-015 and D-016.
- Conversations, context provenance, and simulated results remain volatile under D-017 through D-019.
- `get_app_info` remains the only custom Tauri command.
- The WebView cannot authorize or execute a local action.
- Activity records remain fixed-copy and cannot receive request, result, error, or final-answer content.
- Approval decisions are accepted only for the exact active awaiting-approval run and selected conversation.
- Result construction already fails closed for malformed or mismatched run, conversation, and proposal IDs.
- The production deterministic driver does not intentionally fail; retry limits are verified with injected drivers.

## Exact proposed files

Create runtime and focused tests:

```text
src/application/mockLoop.ts
src/application/mockLoop.test.ts
src/features/conversations/FinalAnswerMessage.tsx
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
docs/increments/03d-bounded-mock-loop-completion.md
docs/plans/03d-bounded-mock-loop-completion.md
```

No file outside this list may change without stopping for project-owner approval.

## Implementation steps

1. Add the frozen loop-limit contract, retry-attempt helper, final-answer type, strict identifier validation, exact result binding, fixed copy, output-ceiling enforcement, and focused pure tests.
2. Extend `ConversationSession` with final answers and include them in empty-session detection.
3. Add retry-attempt identity to active and retryable runs; start submissions at attempt `0` and allow only attempt `1` to be created by Retry.
4. On approve, construct the existing result and its exact final answer before changing state; append both only if every run, conversation, proposal, and result identity matches.
5. Preserve the existing fixed Reject and Edit outcomes; Approve uses the paired final answer instead of the earlier generic approval outcome message.
6. Pass the selected conversation's final answers through `App` to the workspace.
7. Pair final answers by exact result ID and render each directly after its result with accessible deterministic-mock labeling.
8. Add state and interaction tests for result/final ordering, exact identity, no content leakage, all non-approve paths, one-retry enforcement, stale events, and conversation restoration.
9. Run focused checks, the complete repository gate, dependency audit, native launch, and manual regression checks.
10. Perform code and security review and synchronize implementation records with actual evidence.

## Risks and mitigations

- **The final answer is mistaken for provider output:** label it deterministic frontend mock output and use fixed copy that makes no provider claim.
- **The final answer appears before or apart from its result:** require the exact result ID and render the pair in one result-order mapping; verify adjacent DOM order.
- **Separate session arrays drift:** construct and append result plus final answer atomically after validating every identity.
- **Retry has an off-by-one error:** model initial attempt `0`, permit only retry attempt `1`, and test that a second failure has no Retry control.
- **A third model turn or second tool call is introduced accidentally:** expose fixed limits and retain the closed one-proposal, one-decision state transition with focused no-duplication tests.
- **Request, result, or personal content leaks into the answer:** accept identifiers only in the constructor and use fixed copy under the output ceiling.
- **Unsupported limits imply dormant capabilities:** use explicit zero values for tool timeout, files, search, and network and add no corresponding operation path.
- **Activity receives new content:** leave Activity construction unchanged and retain redaction tests.
- **UI density or accessibility regresses:** use existing message typography, add an accessible label, and verify normal and minimum supported window sizes.

## Security and privacy considerations

- The final answer and limits are untrusted volatile WebView state, not trusted provider, executor, policy, or audit output.
- The final-answer constructor accepts no request, argument, preview, result payload, error, path, or arbitrary-content input.
- React renders fixed text without an HTML injection path.
- `Approve mock` remains presentation-only and cannot invoke IPC or a tool.
- Retry creates a fresh run ID but never duplicates the user message or exceeds one retry attempt.
- No new content is copied into Activity or logs.
- No native command, database record, network call, credential, platform API, or operating-system permission is introduced.

## Test plan

- `MOCK_LOOP_LIMITS` has the exact frozen values recorded in this plan.
- Every fixed first-turn and final-turn output remains within the 512-character per-turn ceiling.
- Final-answer construction rejects blank or malformed run and conversation IDs and mismatched result IDs.
- A valid final answer has deterministic IDs, model turn `2`, fixed mock source, and exact fixed copy.
- The final-answer constructor API and output contain no request or result-content field.
- Initial and newly created conversations start without final answers.
- Approve appends exactly one result and one matching final answer to the active conversation.
- The result renders immediately before its exact final answer.
- Reject, Edit, Stop, failure, stale events, invalid decisions, and duplicate decisions append no final answer.
- The first failed attempt offers Retry; failure of retry attempt `1` does not.
- Retry does not duplicate the user message and an approved retry binds result and final answer to the fresh run.
- Conversation selection restores only that session's result/final pairs.
- A final-answer-bearing session is not treated as empty.
- Request text is absent from the result, final answer, and Activity presentation.
- Existing messages, provenance, tool activity, streaming, Stop, approval, failure, Retry, stale-event, navigation, diagnostics, and native-route tests remain green.

## Verification commands

```bash
npx vitest run src/application/mockLoop.test.ts src/application/conversations.test.ts src/application/state.test.ts src/App.test.tsx
npm run typecheck
npm run lint:frontend
npm run verify
npm audit --audit-level=low
git diff --check
npm run tauri -- dev
```

Manual native checks:

1. Submit a request, choose `Approve mock`, and confirm one fixed simulated result is followed immediately by one `Mock final answer`.
2. Confirm both records use the same run and conversation and the answer states that no local task was created and no data changed.
3. Confirm the request text is absent from the result and final-answer presentation.
4. Confirm Reject, Edit, and Stop produce no final answer.
5. Complete approved runs in two conversations and confirm each result/final pair restores only with its owning conversation.
6. Confirm normal and minimum supported layouts remain usable.
7. Confirm context disclosure, tool activity, Activity redaction, conversation controls, native New Request, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts remain intact.

The one-retry cap is an automated injected-driver check because the production deterministic driver does not intentionally fail.

## Rollback or failure strategy

Remove the final-answer model and component, restore the prior conversation and retry state shapes, and retain every verified Increment 3C behavior. Do not weaken ID validation, one-shot approval guards, cancellation, Activity redaction, no-execution labeling, or zero-capability boundaries to make the increment pass.

## Acceptance criteria

- [x] One closed conservative limit contract records and enforces the exact Phase 3 mock-loop limits.
- [x] Approve appends exactly one deterministic final answer bound to the exact run, conversation, and simulated result.
- [x] The result renders immediately before its matching final answer.
- [x] Reject, Edit, Stop, failure, stale events, invalid decisions, and duplicate decisions append no final answer.
- [x] Initial attempt `0` permits at most retry attempt `1`; a second failure offers no further Retry.
- [x] The final answer contains only validated opaque IDs and fixed copy; request, result, and personal content are absent.
- [x] The UI explicitly identifies deterministic mock output and makes no provider or execution claim.
- [x] Conversation switching restores only the owning result/final pairs.
- [x] Existing mock-loop, conversation, context, result, cancellation, approval, Activity, navigation, diagnostics, and native-route behavior remains correct in automated tests.
- [x] No dependency, Rust, IPC, Tauri, capability, CSP, persistence, credential, network, packaging, or permission expansion occurs.
- [x] Focused checks, `npm run verify`, dependency audit, native launch, code review, security review, documentation synchronization, and project-owner manual checks pass.
- [x] No file outside the approved list changes.

## Approval gate

The project owner approved this exact file plan before implementation. Stop and ask before expanding scope remains in force.

## Actual results

Implemented the frozen limit contract, Unicode code-point output enforcement, one-retry attempt state, one-tool-call guard, fixed result-bound final-answer model, atomic conversation update, adjacent accessible rendering, and focused regression coverage. Approve now uses the distinct final answer instead of the earlier generic outcome; Reject and Edit retain fixed outcomes.

Automated evidence:

```text
targeted Vitest: 4 files, 81 tests passed
npm run typecheck: passed
npm run lint:frontend: passed
npm run verify: passed
full Vitest: 10 files, 124 tests passed
Rust library tests: 50 passed
Rust integration tests: 6 passed
Vite production build: passed
Tauri release no-bundle build: passed
npm audit --audit-level=low: 0 vulnerabilities
git diff --check: passed
code review: passed with no findings
security review: passed with no findings
native Tauri development launch: passed
storage startup: idempotent, 0 migrations applied and 2 already applied
```

The first full-gate run stopped at Prettier wrapping in four approved source files. Formatting was applied, and the complete gate then passed. The first audit attempt was blocked by sandbox DNS; the approved network-enabled retry reported zero vulnerabilities. Neither event was a repository setup, build, or runtime defect, so `TROUBLESHOOTING_LOG.md` remains unchanged.

Native development launch passed. The project owner confirmed result/final ordering, run identity, request-content exclusion, no final answer after Reject/Edit/Stop, per-conversation restoration, normal and minimum-window layout, existing context and Activity behavior, native routing, Settings diagnostics, lifecycle behavior, storage startup, and absence of permission prompts all pass. Increment 3D and Phase 3 are verified complete.
