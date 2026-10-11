# A concrete prompt-to-delivery walkthrough

Example: in **Collaboration**, choose Research and ask: “Compare these two supplied
maintenance notes, identify uncertainties and produce a reviewable summary.” Select
existing synthetic Knowledge passages as sources. This is an explanatory example, not
a newly executed request or a fabricated result. The retained native acceptance covers
the same fixed Research route separately on direct OpenAI and Codex.

## 1. Choose the route and inspect saved bots

The owner chooses Research; a model does not route the request. Rust's fixed sequence is
Personal Assistant → Research → Knowledge & Document → Personal Assistant. Saved
profiles determine each stage's runtime/model/effort and presentation identity. A
nickname does not change the canonical role. The UI does not infer authority from the
word “Research” or from the Conductor mascot. [ROUTES](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration.rs#L30) [PROFILE](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_preferences.rs#L19) [COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172)

## 2. Select and review context

Knowledge search is local case-insensitive AND keyword matching over current-version
passages. The owner selects explicit sources with document ID, version, passage, line
range and SHA-256 attribution. Native validation rechecks source bytes and version before
preview and Start. The model will receive these excerpts, not an entire vault. Saved
private bot notes are excluded from every collaboration participant. [KNOW](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/knowledge_tauri.rs#L80) [KNOW_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/knowledge.rs#L83) [CTX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat.rs#L56)

## 3. Preview and acknowledge sharing

`prepare_collaboration` builds the stage roster from saved profiles, checks readiness
without dispatching a generation, and returns a serial-bound preview with maximum calls.
Research has four calls. Start supplies only the expected room/run IDs, preview serial
and acknowledgement. Rust rechecks profile revisions and source identity, acquires the
global generation lease and persists the running room before starting work. If context
changed, it rejects stale approval; it does not quietly send a different request.
This approval covers provider sharing, not a patch or consequential device action.
[COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172)

## 4. What each bot receives and returns

| Stage                       | Inputs                                                               | Expected observable output                                                       | What you see                                                              |
| --------------------------- | -------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------- |
| Personal Assistant, opening | Objective, selected sources, its captured settings and role          | Version-1 handoff framing the review, with summary/findings/evidence/limitations | Stage running, provisional text, then validated handoff                   |
| Research                    | Objective/sources plus application-selected prior validated handoffs | Analysis of supplied evidence and explicit uncertainties                         | Attributed source labels; no claim that external browsing happened        |
| Knowledge & Document        | Supplied material and preceding valid results                        | Organized evidence-backed synthesis or draft material                            | Reviewable text and source attribution; no automatic Knowledge Save       |
| Personal Assistant, closing | Validated upstream material within bounded context                   | Consolidated final advice with limitations                                       | Completed room if all stages complete; otherwise partial/failed/cancelled |

The actual prompt builder serializes stage ID, participant, objective, sources and
prior handoffs. Outputs must match version, stage and agent, allowed status, size and
known source labels. Rust validates structure/reference membership; it does not verify
that the model's factual conclusions are correct. A partial handoff stops the normal
route, and a failed stage does not silently retry or switch providers. [ROUTES](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration.rs#L30) [COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172)

## 5. Execution differs by connection

**OpenAI API:** native code gets the opted-in session key, sends a bounded text-only
Responses request to the fixed HTTPS endpoint, validates SSE and maps fixed error
categories. The WebView never receives the key. **Codex:** native code launches the
reviewed executable in an ephemeral working directory with cleared environment and a
dedicated file-store home, validates account/model/thread metadata and rejects tools.
Both normalize accepted output to Started/Delta/Completed, but their authentication,
accounting, retention and child-cleanup behavior are different. [API](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/personal_assistant_direct.rs#L31) [CODEX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/codex_connection.rs#L156) [ADAPTER](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_adapter.rs#L16)

## 6. Streaming is observable output, not hidden reasoning

Collaboration updates and persists provisional stage output as it arrives. The shared
frontend store polls while rooms are running/queued; Command Center derives cards and
edges from the same accepted snapshots. A graph arrow is a dependency/handoff, not a
thought. Diagnostics record first-text, terminal and cleanup categories with hashed
request identifiers, not the message or raw provider data. [COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172) [SNAP](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/collaboration/collaborationSnapshots.ts#L14) [GRAPH](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/command-center/collaborationProjection.ts#L39) [DIAG](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/diagnostics.rs#L39)

## 7. Stop, finish and recover

Stop aborts/joins the application task and preserves accepted partial text. Saved room
status becomes cancelled; queued stages cannot begin afterward. A restarted app marks
unfinished stored work interrupted instead of resuming behind the owner's back. A
provider failure remains failed with a useful fixed category. Completion requires an
explicit validated terminal result and transport cleanup; partial text alone is not
completion. A new run is an explicit action with fresh preview, within room/run limits.
[ROOM_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/collaboration.rs#L9) [COORD](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/collaboration_tauri.rs#L172) [HOST](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/personal_assistant_v0.rs#L246)

## 8. Delivery and optional owner Save

The completed room contains captured participant settings, objective/source snapshots,
stage results and handoffs. The owner may inspect graph/room history and request a
Knowledge draft from a selected valid handoff. That draft is labeled model-generated;
its source attribution and limitations remain visible. Saving/exporting it requires an
owner action. No coding changes, tests, cloud mutations or workflow scheduling occur.
[KNOW](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/knowledge_tauri.rs#L80) [ROOM_UI](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/collaboration/CollaborationPage.tsx#L21) [ROOM_DB](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/storage/collaboration.rs#L9)

## The simpler single-bot route

Bots → selected profile → fresh conversation → acknowledged Send uses
`start_agent_conversation` / `send_agent_message` instead of a room. Native code captures
one saved revision, builds untrusted context plus completed history, validates the
selected adapter and polls snapshots every 250 ms while busy. Memory Off removes the
private note from request context. Successful completed turns can support follow-up;
stopped/error contexts require a new conversation. The transcript is volatile and does
not become a Knowledge item unless an explicit separate implemented owner path is used.
The older Conversations mock and fixed native sample are different surfaces.
[CHAT](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat_tauri.rs#L247) [CTX](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src-tauri/src/agent_chat.rs#L56) [UI](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/agents/AgentsPage.tsx#L48) [MOCK](https://github.com/SillyRbbit/ai-agent-assistant/blob/bc12776412c717613de1fdc42c38bf3312493726/src/features/conversations/ConversationWorkspace.tsx#L57)

## How to read the verification

Code establishes these branches and guards. Retained automated suites establish their
specified test behavior. Retained native QA establishes selected API/Codex executions,
not every bot/model/account/OS combination. Owner-reported six-screen aesthetic approval
is a judgment, distinct from native metadata/direct Computer Use. The final A/B recheck
specifically proves the corrected Codex credential store and saved-low/restart path.
Neither screenshots nor a graph establish private model reasoning or precise backend
race order. See the evidence register in the index and each diagram's source table.
