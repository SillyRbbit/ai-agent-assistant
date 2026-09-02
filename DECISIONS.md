# Decision log

Record durable technical and workflow decisions here. Do not delete prior decisions; mark them superseded and add the replacement decision.

## D-001 — Use Tauri 2 with React, TypeScript, Vite, and Rust

Date: 2026-06-18
Status: Accepted

Decision: use Tauri 2 for the native desktop shell, React and strict TypeScript for the WebView, Vite for frontend development, and Rust for the trusted local core.

Rationale: this provides a native-feeling macOS application while preserving substantial reuse for Windows and Linux and keeping action authorization outside the WebView.

Consequences:

- IPC must remain narrow and typed.
- Platform-specific code must stay behind adapters.
- Tauri capabilities and CSP are security-critical configuration.

## D-002 — Support Node.js 26.3.0 and npm 11.16.0

Date: 2026-06-18
Status: Accepted

Decision: make Node.js 26.3.0 and npm 11.16.0 the preferred local JavaScript toolchain while also accepting supported Node.js 22 and 24 environments.

Rationale: the target workstation already uses Node.js 26 and npm 11. Strict engine checks should validate rather than block that environment.

Consequences:

- `engine-strict=true` remains enabled.
- Node.js 23 and 25 remain excluded.
- Dependency upgrades must continue to be tested against the declared engine range.

## D-003 — Pin Rust 1.90.0 through rust-toolchain.toml

Date: 2026-06-18
Status: Accepted

Decision: use Rust 1.90.0 with `clippy` and `rustfmt` components.

Rationale: a repository-pinned toolchain makes local and CI behavior reproducible.

Consequences:

- Developers using Homebrew `rustup` must ensure its `bin` directory is on `PATH`.
- Rust upgrades require a dedicated verified increment.

## D-004 — Keep the model outside the authorization boundary

Date: 2026-06-18
Status: Accepted

Decision: model output may propose typed calls but can never authorize or directly execute local actions.

Rationale: prompt instructions alone are not a security boundary.

Consequences:

- Tool validation, policy, approval, execution, and audit live in deterministic Rust code.
- Unknown tools and invalid arguments fail closed.
- No generic action or unrestricted shell tool will be registered.

## D-005 — Use repository files for durable project memory

Date: 2026-06-18
Status: Accepted

Decision: preserve project state across assistant sessions with `AGENTS.md`, `HANDOFF.md`, `PROJECT_STATUS.md`, `NEXT_STEPS.md`, `DECISIONS.md`, `CHANGELOG.md`, and `TROUBLESHOOTING_LOG.md`.

Rationale: chat history is not a reliable source of truth for long-running engineering work.

Consequences:

- Every meaningful session ends with a handoff update.
- Documentation drift is treated as an incomplete task.
- The exact resume prompt is stored in the repository.

## D-006 — Store reusable assistant skills under .agents/skills

Date: 2026-06-18
Status: Accepted

Decision: repository-scoped agent skills live under `.agents/skills/<skill-name>/SKILL.md` and include `name` and `description` metadata.

Rationale: this is the current repository-level skill discovery convention and keeps recurring workflows versioned with the codebase.

Consequences:

- Each skill remains focused on one repeatable job.
- Long-lived rules belong in `AGENTS.md`; detailed workflows belong in skills and workflow documents.
- Prompt files remain available for assistants that do not discover skills automatically.

## D-007 — Complete Phase 2 through narrow verified increments

Date: 2026-06-18
Status: Accepted

Decision: separate core interfaces, SQLite, menu-bar behavior, React shell, mocked streaming, and final integration into distinct increments.

Rationale: smaller changes make compile failures, security regressions, and platform-specific defects easier to diagnose.

Consequences:

- Work proceeds only on the first ready increment in `NEXT_STEPS.md` unless a blocker requires a smaller troubleshooting increment.
- Broader requests must be decomposed before implementation.

## D-008 — Keep Increment 2A interfaces inside the existing Tauri Rust crate

Date: 2026-07-09
Status: Accepted

Decision: implement the initial platform-neutral core interfaces as Rust modules under `src-tauri/src/` rather than splitting the repository into multiple Rust crates during Increment 2A.

Rationale: the increment is architecture-only and should prove contracts, deterministic mocks, and tests without introducing workspace movement, dependency changes, or additional build complexity.

Consequences:

- The current `get_app_info` IPC command and Tauri crate layout remain unchanged.
- Later increments may still split portable domain modules into workspace crates after the interfaces stabilize.
- Open decision O-002 remains partially answered for 2A but should be revisited before larger persistence or platform-adapter work.

## D-009 — Use rusqlite with bundled SQLCipher for the first storage implementation

Date: 2026-07-13
Status: Superseded by D-011

Decision: when SQLite is implemented in Increment 2B-1, use `rusqlite` with an exact version and the bundled SQLCipher feature set unless target-Mac verification exposes a blocker:

```toml
rusqlite = { version = "=0.40.1", features = ["bundled-sqlcipher-vendored-openssl"] }
```

Use `tempfile = "=3.23.0"` as a dev dependency for isolated temporary-database tests.

Rationale: `rusqlite` gives the trusted local Rust core explicit synchronous connection, transaction, prepared-statement, and PRAGMA control without adding an async runtime or compile-time database macro workflow. SQLCipher support should be present from the first database implementation to avoid migrating user data from plaintext storage later.

Consequences:

- Increment 2B-1 must verify the native SQLCipher build on the target Mac.
- Database keys must remain outside SQLite and outside repository configuration.
- Production Keychain integration remains a later increment behind a secret-store boundary.
- If SQLCipher blocks local builds, record a superseding decision before falling back to non-encrypted SQLite.
- Open decision O-001 is resolved by this decision.

## D-010 — Keep the SQLite connection private and migrations immutable

Date: 2026-07-13
Status: Accepted

Decision: `DatabaseConnection` owns the raw `rusqlite::Connection` privately and exposes only narrow connection-settings, migration, and applied-migration methods. Migration definitions are static source code with positive increasing versions, fixed names, fixed checksums, and one immediate transaction per pending migration. Migration 1 bootstraps `schema_migrations`; migration 2 creates `app_metadata`.

Rationale: a generic SQL execution surface would bypass future repository, policy, audit, and redaction boundaries. Immutable migration metadata detects accidental or malicious history drift, while per-migration transactions provide deterministic rollback behavior.

Consequences:

- Callers cannot submit arbitrary SQL through the public storage API.
- Unknown applied migration versions fail closed.
- A changed migration name or checksum fails closed.
- Released migrations must never be edited; corrections require a new version.
- SQLCipher support is compiled in, but key application remains deferred until a dedicated secret-store boundary exists.
- No product data may use a file-backed database before key management is implemented and reviewed.

## D-011 — Pin rusqlite 0.37.0 for Rust 1.90 compatibility

Date: 2026-07-13
Status: Accepted

Decision: retain bundled SQLCipher and vendored OpenSSL support while pinning the first storage implementation to:

```toml
rusqlite = { version = "=0.37.0", features = ["bundled-sqlcipher-vendored-openssl"] }
```

This resolves to `libsqlite3-sys 0.35.0` on the verified target-Mac lockfile.

Rationale: the originally selected `rusqlite 0.40.1` resolved to `libsqlite3-sys 0.38.1`, whose build script used a standard-library feature unavailable on the repository's pinned Rust 1.90.0 toolchain. The 0.37.0 dependency line preserves the required encryption build features and passed target-Mac formatting, Clippy, tests, and native launch.

Consequences:

- D-009's exact 0.40.1 version is superseded, but its choice of `rusqlite`, bundled SQLCipher, vendored OpenSSL, and external key management remains in force.
- Rust 1.90.0 remains pinned.
- `Cargo.lock` must retain `rusqlite 0.37.0` and `libsqlite3-sys 0.35.0` until a dedicated dependency/toolchain increment changes them.
- Dependency upgrades require target-Mac native verification.

## D-012 — Bootstrap storage at Tauri startup without persisting user data

Date: 2026-07-13
Status: Accepted

Decision: initialize the storage foundation from Tauri's setup hook and register one managed `Storage` value. Debug builds use an application-local file-backed database containing only migration history and `app_initialized=true`; release builds use an in-memory database until reviewed Keychain-backed key management exists.

Application metadata is a closed typed contract rather than arbitrary key/value strings. The only current key is `AppInitialized`, and the only current value type is Boolean.

Rationale: startup integration proves the database lifecycle, migration ordering, state ownership, and typed read/write boundary before product repositories are introduced. Keeping release storage ephemeral prevents accidental plaintext user-data persistence before an encryption-key boundary exists.

Consequences:

- The raw `rusqlite::Connection` remains private to the storage module.
- Tauri startup fails with a typed error if storage cannot initialize or managed state is already registered.
- Corrupt metadata values and negative timestamps fail closed.
- Startup logs contain no database path or metadata value.
- No conversation, task, memory, audit, approval, tool-call, credential, or personal data may be stored yet.
- The previously planned menu-bar increment is re-labeled Increment 2D; React shell, mocked streaming, and integration increments move to 2E, 2F, and 2G.

## D-013 — Use a fixed macOS menu-bar contract and hide only the main window

Date: 2026-07-13
Status: Accepted

Decision: enable Tauri's built-in `tray-icon` feature only for the macOS target and create one menu-bar entry with four fixed actions: open the main window, route a new request, route a tasks placeholder, and quit. New-request and tasks actions emit the closed-enum `assistant-menu-route` event only after the existing `main` window is shown and focused. Unknown menu identifiers are ignored.

Closing the `main` window prevents destruction and hides that window. Future non-main windows retain normal close behavior. A macOS `RunEvent::Reopen` restores the main window only when no application window is visible. The app keeps its regular activation policy and Dock icon in this increment.

Rationale: the menu bar needs a deterministic, least-privilege lifecycle contract before the React shell and global shortcut are introduced. Fixed IDs and closed route values prevent arbitrary model or content-driven actions. Keeping the Dock avoids an invisible-app failure mode while close-to-hide is first validated.

Consequences:

- `get_app_info` remains the only custom Tauri command.
- The backend may emit only `NewRequest` or `TasksPlaceholder` route values.
- React does not consume those route events until Increment 2E.
- Dedicated production tray artwork and accessory-only activation remain deferred.
- Increment 2D requires native macOS verification of tray, close, menu/Dock reopen, and quit behavior.
- Open decision O-005 is resolved for the MVP scaffold; artwork may be revisited later.

## D-014 — Use React reducer and context for application-shell state

Date: 2026-07-13
Status: Accepted

Decision: use React's built-in `useReducer` and context for the Phase 2 application shell. Keep navigation and composer state local and in memory. Represent application routes and native menu routes as closed TypeScript unions, and narrow native event payloads from `unknown` before dispatching them.

Rationale: the shell has a small deterministic state model and does not justify another runtime dependency. Reducer tests provide explicit transition coverage, split read/dispatch contexts preserve a narrow component boundary, and service injection keeps Tauri-dependent behavior testable without native or machine state.

Consequences:

- O-004 is resolved without adding a state-management or routing package.
- The shell state is intentionally volatile and resets when the application reloads.
- New Request clears only the in-memory composer draft and returns to Conversations.
- Tasks placeholder routes only to the closed Tasks destination.
- Unknown or extended native event payloads are ignored.
- Persisted application state remains prohibited until a later reviewed increment adds explicit repositories and privacy controls.
- Revisit the state architecture only if future run-event volume or cross-window synchronization produces a measured need.

## D-015 — Keep Increment 2F mock interaction entirely in frontend memory

Date: 2026-07-13
Status: Accepted

Decision: implement Increment 2F as a closed deterministic frontend run script managed by the existing React reducer and a timer-owning hook. Conversation messages, tool activity, approval previews, and decisions remain volatile. Approval controls are explicitly mock-only and never cross IPC or invoke the existing Rust approval or tool interfaces.

Rationale: Increment 2F proves user-visible streaming, cancellation, activity, and approval state transitions without creating a generic WebView executor, adding a Tauri command, or implying that WebView state is trusted authorization. Run identifiers bind scheduled events to the active run so late events fail closed after Stop or completion.

Consequences:

- Reloading the WebView clears every Increment 2F message, activity item, and decision.
- Approve records a mock outcome only; it does not create a task or call Rust.
- Reject records a mock rejection only.
- Edit returns a deterministic draft to the composer and executes nothing.
- Production provider events, trusted approval transactions, tool execution, audit persistence, and error recovery remain later reviewed increments.
- No dependency, IPC, Tauri capability, CSP, Rust source, SQLite schema, or permission change is introduced.

## D-016 — Model Phase 2 run failures and activity as closed volatile frontend events

Date: 2026-07-13
Status: Accepted

Decision: Increment 2G places the deterministic browser timer implementation behind a typed `MockRunDriver` with a closed chunk/completed/failed event union and an idempotent cancellation handle. Accepted lifecycle transitions append fixed-copy Activity events containing only an internal activity ID, a validated mock run ID, an event kind, and presentation metadata. Retry reuses the failed request in memory but does not append a duplicate user message.

Rationale: this hardens cancellation, bounded failures, Retry, and transparent lifecycle presentation without exposing a generic event channel, rendering raw errors, persisting sensitive content, or implying that WebView activity is a trusted audit record.

Consequences:

- Stop, completion, failure, Retry, and unmount all terminate the previous driver handle through React effect cleanup.
- Late chunk, completion, and failure events fail closed unless they match the active streaming run.
- Activity copy cannot contain request text, tool arguments, tool results, or provider error details.
- The Activity page is a volatile Phase 2 scaffold, not the trusted Rust audit log required in a later phase.
- Reloading clears messages, Retry state, and all Activity events.
- No Rust, Tauri, IPC, capability, CSP, SQLite, dependency, credential, network, or permission boundary changes.

## D-017 — Keep initial conversation history volatile and bind runs to conversation identity

Date: 2026-07-13
Status: Accepted

Decision: Increment 3A represents conversation history as readonly React application-state sessions with deterministic `conversation-N` identifiers, a title capped at 48 Unicode code points including any ellipsis, messages, and mock tool activity. At most one empty session is retained. Active and retryable mock runs carry the exact conversation ID they belong to. Conversation creation and selection are rejected while streaming or awaiting approval.

The native New Request route creates or selects the empty conversation only while idle. During an active run or approval it may focus Conversations but cannot create, switch, cancel, approve, or otherwise mutate the run.

Rationale: conversation identity is the smallest missing Phase 3 UI capability and is required before later context provenance work. Keeping it volatile proves creation, history, selection, and restoration without persisting personal content before reviewed encrypted repositories and privacy controls exist. Binding run events to both run and conversation IDs prevents asynchronous output or Retry from crossing transcript boundaries.

Consequences:

- Reloading the WebView clears every conversation title, message, tool activity, and Retry state.
- Conversation titles may contain normalized request text in the local UI but are never copied into Activity or logs.
- Activity events and monotonic run/activity ordinals remain application-session scoped rather than conversation scoped.
- Leaving a failed conversation clears its Retry eligibility.
- Conversation deletion, rename, search, export, synchronization, per-conversation drafts, attachments, voice, and context controls remain later increments.
- No Rust, IPC, Tauri, SQLite, dependency, capability, CSP, credential, network, packaging, or operating-system permission change is introduced.

## D-018 — Represent Phase 3B context provenance as fixed volatile mock data

Date: 2026-07-13
Status: Accepted

Decision: each deterministic frontend run appends one readonly `MockContextProvenance` record to its owning volatile conversation. The record is bound to validated `mock-run-N` and `conversation-N` identifiers and contains a closed, fixed-copy source list. The current request is marked used; earlier conversation messages, saved memory, device data, and external services are marked not used.

The provenance constructor accepts identifiers only and cannot receive request content. The conversation UI labels the record `Mock context used` and states that it is frontend-only, not trusted audit evidence.

Rationale: the product requires users to see what information the assistant used, and D-017 now provides the conversation ownership boundary. Fixed frontend provenance proves disclosure, attribution, Retry, and restoration behavior without collecting real context or falsely extending trust to the WebView.

Consequences:

- Each submission and Retry gets one fresh provenance record tied to its run and conversation.
- Request text, tool arguments, tool results, errors, paths, and personal content are excluded from provenance and Activity.
- A provenance-bearing conversation is not treated as empty.
- Reloading the WebView clears all provenance records.
- Displayed provenance is untrusted presentation and grants no permission, approval, or execution authority.
- Context selection, real data collection, trusted Rust provenance, audit persistence, and tool-result modeling remain later reviewed work.
- No Rust, IPC, Tauri, SQLite, dependency, capability, CSP, credential, network, packaging, or operating-system permission change is introduced.

## D-019 — Represent Phase 3C tool results as approve-only fixed simulations

Date: 2026-07-13
Status: Accepted

Decision: a valid `Approve mock` decision appends one readonly `MockToolResult` to the owning volatile conversation. The result is bound to validated `mock-run-N` and `conversation-N` identifiers, derives the exact `${runId}-tool` proposal ID, and supports only `create_local_task`, `status: simulated`, `executed: false`, and fixed no-change summary copy.

Reject, Edit, Stop, stale events, invalid decisions, missing proposals, and mismatched proposals produce no result. The result constructor accepts identifiers only and cannot receive request text, arguments, preview content, errors, paths, or arbitrary tool output.

Rationale: the Phase 3 center pane requires a distinct tool-result view, but trusted execution, provider continuation, arbitrary result schemas, and external content belong to later phases. A fixed approve-only simulation proves result attribution, restoration, privacy, and presentation while making the absence of execution explicit.

Consequences:

- `Approve mock` remains presentation-only and invokes no IPC or tool.
- Result creation fails closed unless the exact conversation proposal exists and matches the derived proposal ID.
- Duplicate decisions cannot append duplicate results because the awaiting-approval run is consumed once.
- Results remain volatile and restore only with their owning conversation.
- Result content is excluded from Activity and logs.
- Real executor output, provider continuation, arbitrary result payloads, persistence, and audit evidence remain later reviewed work.
- No Rust, IPC, Tauri, SQLite, dependency, capability, CSP, credential, network, packaging, or operating-system permission change is introduced.

## D-020 — Complete the Phase 3 mock loop with fixed final answers and closed limits

Date: 2026-07-13
Status: Accepted

Decision: Increment 3D defines one frozen frontend `MOCK_LOOP_LIMITS` contract: at most two consecutive mock model turns, one mock tool call per run, one retry attempt, 512 Unicode code points of assistant output per turn, and zero network requests, tool timeout milliseconds, file bytes, or search results.

An accepted `Approve mock` decision constructs one fixed `MockFinalAnswer` from validated run, conversation, and simulated-result IDs only. The result and final answer are appended atomically to the owning volatile conversation and render as an adjacent pair. The final answer is synchronous deterministic frontend copy, identified as mock output and model turn 2; no result is returned to a provider. Initial runs use retry attempt 0, the only Retry uses attempt 1, and failure of attempt 1 creates no further retry eligibility.

Rationale: Phase 3 requires a complete mocked loop and conservative limits, while production provider continuation and trusted tool execution begin in later phases. A fixed result-bound final answer and one closed limit contract prove post-result sequencing, attribution, bounded output, and bounded Retry without adding a provider, executor, network path, or native authority.

Consequences:

- Approve no longer appends the earlier generic approval outcome message; the distinct fixed final answer follows the simulated result instead.
- Reject and Edit retain their fixed assistant outcomes, while Stop and failures create no final answer.
- Output chunks exceeding the per-turn limit are rejected without mutating state.
- A run with an existing simulated tool result cannot append another result or final answer.
- Final answers remain volatile, contain no request or result payload, and restore only with their owning conversation.
- Zero-valued file, search, network, and timeout limits represent unavailable Phase 3 capabilities, not dormant integrations.
- Production provider continuation, real execution, configurable limits, arbitrary payloads, persistence, and trusted audit evidence remain later reviewed work.
- No Rust, IPC, Tauri, SQLite, dependency, capability, CSP, credential, network, packaging, or operating-system permission change is introduced.

## D-021 — Normalize the Responses boundary through a credential-owning gateway

Date: 2026-07-14
Status: Accepted; Increment 4A contract implemented and verified

Decision: production OpenAI credentials exist only in server-side secret storage owned by an authenticated product gateway. A future desktop gateway access token is audience-bound, has a maximum 15-minute lifetime, and is held in trusted Rust memory. Any refresh or session credential is read only through the platform secret-store abstraction and stored in macOS Keychain rather than the WebView or SQLite.

The desktop sends a closed product request containing a protocol version, opaque run/correlation identity, bounded user-selected content, a server-recognized tool-set identifier, and fixed limits. The gateway authenticates and authorizes the principal, enforces request/rate/model/tool-set policy, selects exact server-side strict function definitions, injects the OpenAI credential, forces foreground `store: false` streaming with `background: false` and no parallel tool calls, and normalizes recognized OpenAI Responses events. It never executes, approves, or authorizes a local tool.

The upstream adapter validates recognized event types, required fields, sequence, and bounds while ignoring additive fields on recognized events for documented API compatibility. Unknown event types and invalid required fields fail the run. The normalized gateway-to-Rust protocol is versioned and closed, rejects unknown fields, requires contiguous sequence numbers and one terminal event, and returns only bounded text, completed function-call proposals, terminal state, or closed redacted errors.

Function calls are untrusted at both boundaries. OpenAI strict mode must be enabled with all object properties required and `additionalProperties: false`, but trusted Rust still independently validates call identity, registered tool name, tool-contract version, duplicate-free JSON object arguments, the exact local per-tool schema, locally derived risk and permission, policy, approval, and execution eligibility. No provider or gateway field grants authority.

The initial contract retains two model turns, one non-parallel function call, one retry only when the gateway proves the request was not forwarded or returns an explicit pre-start rate limit, three gateway requests, bounded request/event/argument/output sizes, and explicit connection, idle, turn, and run deadlines. Cancellation first transitions Rust to a terminal local state, aborts foreground transport in both hops, and rejects late events; it expects no acknowledgment on the aborted stream and does not claim confirmed provider-side cancellation. Provider errors become closed redacted codes and opaque correlation IDs. Gateway operational telemetry and the local trusted audit are separate and exclude raw prompts, output, arguments, results, errors, and credentials. Gateway operational metadata has a default maximum seven-day retention.

`store: false` minimizes Responses application-state storage but does not remove default provider abuse-monitoring retention. The OpenAI project retention mode and user disclosure remain O-007 and must be resolved before live provider traffic.

Rationale: OpenAI requires server-side credential handling, Responses streaming is a typed but evolving external protocol, and the official cancel endpoint is limited to background responses. A product-owned normalized protocol prevents provider event drift, credentials, raw errors, or provider-selected tool metadata from crossing directly into the trusted desktop core while preserving local policy and execution authority.

Consequences:

- Increment 4A must establish and fixture-test the normalized Rust protocol before networking exists.
- The existing synchronous `AgentProvider` and placeholder `ToolSchema` are insufficient for live Responses integration and remain unchanged in Increment 4A.
- Increment 4A may classify a function call as untrusted protocol data but cannot convert it into an executable `ToolCallProposal`; exact per-tool schema validation is a later prerequisite.
- Live gateway transport, authentication, Keychain integration, model selection, provider continuation, and deployment require later approved increments.
- The gateway identity provider and deployment platform remain open decision O-006 and do not block the transport-free protocol contract.
- Provider retention configuration and user disclosure remain open decision O-007 and do not block the transport-free protocol contract.

## D-022 — Own exact tool input contracts and classification in trusted Rust

Date: 2026-07-14
Status: Accepted; Increment 4B implemented and verified

Decision: trusted Rust owns a closed local `ToolSchema` catalog. Its first and only variants are `get_current_datetime@1`, which accepts exactly an empty JSON object and is classified `InformationOnly` with no permission, and `create_local_task@1`, which accepts exactly one required canonical `title` string of 1-200 Unicode scalar values with no control characters and is classified `ReversibleLocalAction` with no permission.

Each `ToolDefinition` is constructed only from one schema variant, so name, description, version, risk, permission, schema document, and typed argument parser cannot be supplied independently. A normalized gateway function call remains untrusted until an ownership-consuming local validator finds its registered definition, independently matches the contract version, and parses its arguments through that exact schema. Successful output contains private typed arguments and locally derived metadata but grants no policy allowance, approval, audit status, dispatch eligibility, or execution authority.

Existing direct `serde` and `serde_json` dependencies implement the two closed contracts. A general JSON Schema engine or promoted `schemars` dependency is not justified at this scale. Exact schema-value assertions and accepted/rejected fixtures mitigate schema/parser drift; a reviewed dependency decision is required if materially different schemas make this approach incomplete or repetitive.

Rationale: provider strict mode and gateway object validation are defense in depth, not local authorization. Binding local identity, schema, risk, and permission in one closed catalog prevents model, gateway, or caller-supplied metadata from becoming trusted policy input. Consuming the normalized call and returning content-redacted non-serializable types prevents accidental raw-JSON retention or premature transport/persistence use.

Consequences:

- Raw argument JSON is discarded after successful typed parsing and omitted from all validation errors.
- Content-bearing validated types do not derive `Clone`, `Serialize`, or raw-value `Debug`; task titles remain available only through a read-only typed accessor.
- Unknown tools, contract mismatches, missing or additional properties, wrong types, malformed values, title bounds, surrounding whitespace, and control characters fail closed.
- `ToolCallProposal`, `AgentProviderResponse`, `ProposedAction`, policy, approval, audit, executor, runtime registration, transport, IPC, persistence, and UI remain unchanged.
- D-023 and Increment 4C resolve the legacy proposal gap by removing the raw constructors and defining the only canonical conversion into deterministic policy input.

## D-023 — Bind deterministic policy to one owned schema-validated call

Date: 2026-07-14
Status: Accepted; Increment 4C implemented and verified

Decision: remove `ToolCallProposal`, the unused `AgentProviderResponse::ToolCalls` path, `PolicyContext`, and `ProposedAction`. `PolicyInput` can be constructed only by consuming one `SchemaValidatedFunctionCall`, so exact call ID, local tool name, tool-contract version, closed typed arguments, risk class, and required permission remain one ownership-bound value through deterministic policy evaluation.

`PolicyDecision` owns the exact `PolicyInput` it evaluated. Its outcome is derived from one closed `PolicyReason`; neither input nor decision derives `Clone`, serialization, or raw-value debug output. `PolicyOutcome::Allow` is non-authorizing data and has no conversion to approval, audit, dispatch, or execution.

Caller-supplied `explicit_user_intent` and `permission_granted` booleans are removed rather than renamed as trusted state. Without exact call-bound intent, permission, resource-scope, provenance, and freshness evidence, policy is conservative: prohibited and external/high-impact classes deny first; any remaining permission-bearing call denies; read-only device access denies; reversible local and personal-data modifications require approval; only information-only calls with no required permission allow.

Rationale: a public boolean cannot prove that user intent, a permission grant, resource scope, and observation time belong to the exact validated call. Removing that bypass and retaining the consumed typed call prevents caller or model metadata from becoming policy truth, while conservative outcomes avoid inventing trusted evidence before its source and binding are designed.

Consequences:

- `get_current_datetime@1` evaluates to non-authorizing `Allow` with `InformationOnly`.
- `create_local_task@1` evaluates to `RequireApproval` with `ReversibleRequiresApproval`; no caller can bypass approval by setting an intent boolean.
- Permission-bearing and read-only tools cannot allow until a later approved type binds exact permission and scope evidence to the call.
- Approval requests, previews, canonical digests, run binding, expiry, one-time consumption, audit records, dispatch, and execution remain disconnected.
- Existing generic approval and audit scaffolds are not production-ready and remain unchanged.
- Existing Rust types are sufficient; no dependency, manifest, or lockfile change is introduced.

## D-024 — Bind approval state to one exact policy decision

Date: 2026-07-14
Status: Accepted; Increment 4D implemented and verified

Decision: retain validator-owned run and gateway-request IDs on normalized function calls and carry them through `SchemaValidatedFunctionCall`, `PolicyInput`, and `PolicyDecision`. Replace detached approval tool names, action hashes, and preview strings with an `ApprovalManager` that consumes one exact `PolicyOutcome::RequireApproval` decision. Derive the `create_local_task@1` preview as a borrowed typed projection of that retained decision; no caller can resupply identity, arguments, classification, preview, policy outcome, creation time, or deadline.

Approval state is volatile and bounded per manager instance: one request may be pending, at most 1,024 distinct subjects may enter one manager lifetime, and each request has a manager-owned monotonic 120-second lifetime from creation. Approve, reject, cancel, and expiry consume the subject once. Run/request/call tombstones are never evicted to admit more work. Future orchestration must own one authoritative manager and explicitly cancel pending approval when the run terminates or reaches its absolute deadline; the relative approval lifetime is not run-liveness evidence.

Remove caller-supplied `action_hash` and add no digest or dependency. Same-process ownership of the non-cloneable typed decision is the canonical binding. Approval IDs and any future digest are correlation data, not authentication or execution authority. Content-bearing gateway events, requests, previews, and resolutions are non-cloneable and use redacted or unavailable debug output for model text and arguments.

An `Approved` disposition proves only that the transport-free manager processed a closed Rust choice while the exact subject was pending and unexpired. It does not prove a user gesture, user presence, or local authentication. `ApprovalResolution` exposes no consuming conversion to policy input, audit, dispatch, or execution. A separately approved trusted interaction source, absolute run-state check, typed audit adapter, and executor capability remain prerequisites for any action.

Rationale: direct ownership makes preview/action substitution impossible inside the current process and is stronger than comparing caller-supplied strings or hashes. Fixed limits, monotonic expiry, cancellation, and non-evicting replay state fail closed without persistence or networking. Deferring user-interaction proof, audit, and execution avoids granting authority before their trust boundaries exist.

Consequences:

- Accepted calls retain exact run, gateway-request, and call identity through terminal approval state.
- Only `create_local_task@1` has a registered preview; information-only policy results and unsupported subjects cannot enter approval.
- Preview content is available through a borrowed typed accessor but omitted from debug output, errors, and the generic audit scaffold.
- One-time and replay guarantees apply to one manager instance; no production manager or trusted approval source is wired yet.
- The generic audit scaffold, Tauri, WebView, provider, storage, dispatch, and executor remain disconnected.
- No dependency, manifest, lockfile, capability, CSP, packaging, or permission change is introduced.

## D-025 — Use one Rust-owned native approval source with a scoped RustSec baseline exception

Date: 2026-07-14
Status: Accepted; Increment 4E implemented and verified

Decision: replace public raw approval choices with one manager-issued, owned, one-shot `ApprovalPresentation` and one sealed `TrustedApprovalSourceOutcome` constructed only by a Rust-owned macOS native message-dialog source. A private pointer-identical manager marker and exact approval/run/gateway-request/function-call identity bind the source outcome to the authoritative pending subject. The manager also rechecks one-shot issuance, cancellation, and monotonic expiry before terminal resolution.

Use exact macOS-target `rfd = "=0.17.2"` with default features disabled. The adapter exposes only the synchronous message-dialog path, passes no raw parent handle, wraps no file dialog, registers no Tauri plugin or WebView command, and leaves application `unsafe` forbidden. Reject is the first/default button. Approve and Reject map directly; Edit, native no-decision, source failure, run termination, and expiry are closed terminal outcomes. Every source outcome records `NotEvaluated` authentication and grants no actor-identity, user-presence, run-liveness, audit, dispatch, or execution authority.

Accept a scoped Increment 4E baseline exception for RUSTSEC-2026-0194 and RUSTSEC-2026-0195 in pre-existing `quick-xml 0.39.4`. The Increment 4E lockfile diff adds only `rfd`; `quick-xml` is already reached through `plist 1.9.0 -> Tauri`. Source review found that path uses plain `quick_xml::Reader`, not `NsReader`, and does not iterate attributes, so the cited duplicate-attribute and namespace-declaration APIs appear unreachable through the reviewed path. This exception does not declare the advisories fixed or `quick-xml` generally safe, and it adds no advisory ignore. Re-review is mandatory if the affected APIs become reachable, the dependency path changes, or dependency remediation is separately approved.

Rationale: a trusted native interaction source closes the raw-choice gap without granting authority to the WebView or adding production orchestration. Exact target scoping and a sealed ownership handoff minimize the new native dependency boundary. The two RustSec findings predate Increment 4E, are unrelated to the added `rfd` package, and were reviewed against the actual existing call path; blocking this bounded source increment would not remove the baseline exposure.

Consequences:

- Increment 4E may close even though the exact `cargo-audit 0.22.2` command exits nonzero; the command result remains recorded as failed with an approved reviewed baseline exception.
- No `cargo-audit` ignore, Tauri/plist/quick-xml upgrade, or dependency override is added.
- The native dialog may remain visibly stale after manager cancellation because `rfd` cannot programmatically close it; manager state still rejects every late outcome, and live orchestration remains excluded.
- The project-owner target-Mac gate confirms Approve, Reject, Edit, Return/default, fixed title/content order, terminal redaction, no action or persistence, and no permission prompt. Escape has no effect and no window-close control is available.
- The source remains disconnected from the shipping Tauri application, WebView, LocalAuthentication, audit, persistence, dispatch, executor, provider continuation, gateway networking, and credentials.
- Any production integration must still add exact run-state, typed audit, dispatch, executor, and, when policy requires it, exact-subject device-owner-authentication gates.

## D-026 - Use Cortexa as the display name while preserving compatibility identifiers

Date: 2026-07-14
Status: Accepted; Increment 4F implemented and verified

Decision: rename the assistant's former human-facing product name to `Cortexa`. Product-facing UI, window and native menu metadata, Settings application metadata, native approval copy, diagnostics, repository documentation, prompts, and local skill descriptions use `Cortexa`.

Retain the existing compatibility identifiers: repository and directory name `ai-agent-assistant`, npm and Cargo package name `ai-agent-assistant`, Rust library crate `ai_agent_assistant_lib`, executable target, bundle identifier `com.aiagentassistant.desktop`, tray ID `ai-agent-assistant-menu-bar`, `assistant-menu-route` event, `get_app_info` command, database and storage identifiers, paths, and code-domain uses of `assistant`. Tauri `productName` and window title are display metadata and therefore change to `Cortexa`; they do not authorize a package, executable, or bundle-ID rename.

Rationale: the project owner requested a product identity change while explicitly preserving installed-data, build, source, protocol, and repository compatibility. Separating display copy from technical identifiers avoids unnecessary migrations, broken imports, changed artifact contracts, or event/IPC regressions.

Consequences:

- Exact former product-name text must not remain in tracked product or documentation content.
- Historical documentation is normalized to the current product name, while literal compatibility commands and paths remain unchanged.
- Future identifier renames require separate explicit approval, migration analysis, and rollback planning.
- No dependency, permission, capability, storage, gateway, approval-authority, audit, execution, or security-boundary change is implied.

## D-027 - Defer the post-increment skill after the 4F owner closeout

Date: 2026-07-14
Status: Accepted; one-time Increment 4F closeout exception

Decision: the project owner explicitly confirms Increment 4F complete and directs commit, push, and merge before the referenced `$post-increment-gate` skill exists. The 4F automated, target-Mac manual, complete-diff, scope, compatibility, secret, code-review, security-review, and documentation gates passed. No post-increment skill was run and no PASS or PASS WITH ADVISORIES report is claimed.

This is a one-time sequencing exception for Increment 4F only. The project owner will create the missing skill on the next clean branch. The mandatory rule in `AGENTS.md` remains unchanged for later implementation increments.

Rationale: the product rename is fully implemented and verified, while adding a new repository skill would expand the approved rename scope and mix workflow infrastructure into the product-name commit. Closing 4F transparently and creating the skill from merged clean `main` keeps both changes independently reviewable.

Consequences:

- Increment 4F may be marked complete without claiming that the absent gate ran.
- The 4F increment record must preserve the exact skipped-gate evidence and project-owner direction.
- The next branch is limited to creating and validating the missing skill before another implementation increment starts.
- This exception cannot be reused for a later increment.

## D-028 - Use a trusted repository Stop hook with deterministic local evidence

Date: 2026-07-14
Status: Accepted; Workflow Increment 4G implemented and verified

Decision: every approved implementation increment records one ignored active state before edits and completes through one consolidated repository report. A repository-local Codex Stop hook invokes a Python standard-library validator. When state or evidence is missing, active, malformed, conflicting, suspicious, or stale, the hook requests exactly `Run $post-increment-gate for the active increment before ending the session.` A continuation turn with `stop_hook_active: true` emits no second request.

The validator accepts only a bounded report with a closed JSON manifest, exact changed-file inventory, required report sections, explicit verification/manual statuses, structured findings, and a computed `PASS` or `PASS WITH ADVISORIES` result. Required non-passing checks, pending required manual checks, and completion-blocking Critical or High findings fail. A completion marker is written atomically only after validation and is bound to the report hash and deterministic workspace-content fingerprint. Re-finalization of the same completed increment is allowed so a corrected report can replace stale evidence; the complete report and current workspace must validate again.

The hook executes no report content, makes no network request, reads no transcript or model content, and may run only fixed Git inspection commands. It does not modify product source, fix findings, commit, push, reorder the roadmap, grant approval, authorize execution, or create trusted audit evidence. Project-local hooks require normal `/hooks` trust review. Emergency use of `/hooks` disablement or `codex --disable hooks` must be recorded and cannot satisfy completion until the full gate is rerun.

Rationale: a local deterministic gate makes required verification, engineering review, documentation synchronization, and changed-file evidence harder to omit while preserving Codex's explicit project-trust model. Binding the marker to content rather than Git metadata keeps valid evidence stable across staging and commit while invalidating any subsequent workspace change.

Consequences:

- `$verified-increment` must begin gate state after approval and before editing; `$post-increment-gate` must run before completion.
- The complete report is tracked engineering evidence; ignored state contains no secret and is disposable local workflow state.
- A clean repository with no state may stop normally; a dirty repository with no state requests the gate so retroactive omission is visible.
- A user can disable or modify the hook because it is not a security boundary. Such a bypass prevents completion rather than creating an unbypassable enforcement claim.
- Increment 4G adds no dependency, product behavior, provider, gateway, IPC, Tauri capability, permission, persistence, approval, audit, or executor path.

## D-029 - Record terminal approval evidence through one closed in-memory adapter

Date: 2026-07-15
Status: Accepted; Increment 4H implemented and verified

Decision: add one dedicated `audit::approval` module whose bounded in-memory adapter borrows an exact manager-produced terminal `ApprovalResolution`. Before mutation it independently revalidates the current closed `create_local_task@1` tool/version, locally derived reversible-local risk and no-permission classification, `RequireApproval` outcome, `ReversibleRequiresApproval` reason, and the complete terminal disposition/interaction-evidence matrix.

The adapter privately constructs one non-cloneable, non-serializable record containing only an assigned sequence, approval ID, bounded opaque run/request/call identity, closed local tool and policy facts, terminal disposition, and optional closed interaction evidence. It never calls the title-bearing preview accessor and has no title, raw arguments, prompt, model output, native-dialog message, OS/provider/gateway error, arbitrary summary, arbitrary details, credential, user actor, or authentication-success field. Custom debug output redacts exact identity; typed errors expose only fixed variants and the fixed capacity limit.

One adapter instance retains at most 1,024 records, accepts one exact approval/run/request/call key once, performs checked contiguous sequence assignment, and never evicts. Failed validation, duplicate, capacity, or sequence checks do not mutate records or consume a sequence. These guarantees end with the adapter instance and make no durability, restart, transaction, or global-idempotency claim.

An `ApprovalAuditReceipt` contains only the assigned sequence. A record or receipt proves neither current run liveness, device-owner identity, successful authentication, a durable audit write, dispatch eligibility, execution, nor a tool result, and exposes no conversion to those capabilities. The existing generic arbitrary-string audit scaffold remains unchanged, disconnected, and non-production. A future authoritative coordinator and durable repository must be separately approved and must treat required audit failure as fail-closed before execution.

Rationale: terminal approval evidence already exists as one ownership-bound resolution, while routing it through arbitrary audit strings could duplicate personal content or fabricate authority. A closed projection preserves the evidence needed for later reviewed persistence without coupling approval to SQLite, IPC, UI, or an executor prematurely.

Consequences:

- Approved, rejected, Edit, native no-decision, each closed source failure, run termination, and expiry have exact accepted evidence shapes; missing, extra, or contradictory evidence fails before insertion.
- `MacOsNativeDialog` and a recognized button remain local source evidence only. `NotEvaluated` remains an explicit absence of device-owner-authentication evidence, and no record assigns a user actor.
- The adapter is transport-free and has no production caller. It does not make the generic audit logger suitable for production or satisfy durable local-audit requirements.
- No dependency, manifest, lockfile, Tauri command/event, frontend, SQLite, gateway, provider, native-dialog behavior, capability, CSP, packaging, entitlement, or operating-system permission changes.

## D-030 - Remove the unused arbitrary-string audit scaffold

Date: 2026-07-15
Status: Accepted; Increment 4I reconstructed and verified

Decision: delete the public `audit::logger` and `audit::types` modules, including caller-authored event type, summary, and details strings, clonable generic records, in-memory/no-op loggers, and token-pattern redaction. Preserve the separately verified `audit::approval` module unchanged and export only that typed audit boundary.

Future trusted audit event families for run lifecycle, proposals, policy, execution, cancellation, results, or final outcomes require separately approved closed typed inputs derived from authoritative values. Do not restore a generic arbitrary-string audit API as a convenience layer.

Rationale: repository search found no production or integration caller of the generic scaffold; its only executable references were its own four embedded unit tests. Its open strings and limited redactor could be mistaken for a production-safe structured audit boundary and create a bypass around the exact typed evidence introduced by D-029. Deletion is smaller and safer than designing unsupported persistence, retention, coordinator, execution, and result semantics prematurely.

Consequences:

- Current Rust audit source exposes only the closed typed approval-audit module.
- The four generic-scaffold unit tests are removed with the unused implementation; existing typed approval-audit unit, native-source, and integration coverage remains unchanged and passing.
- Historical decisions and completed plans that describe the scaffold at their original checkpoints remain intact.
- A future durable repository, authoritative coordinator, retention policy, Activity-history surface, or additional event family requires a separate approved increment.
- No dependency, manifest, lockfile, migration, SQLite, Tauri, frontend, IPC, provider, gateway, credential, approval authority, dispatch, executor, capability, CSP, packaging, entitlement, or operating-system permission changes.

## D-031 - Fingerprint existing repository content across deletion commits

Date: 2026-07-15
Status: Accepted; Repository Workflow Increment 4J implemented and verified

Decision: the post-increment workspace fingerprint includes only repository paths that exist in the current working-tree snapshot. For every existing path it retains the repository-relative path, executable bits, file type, regular-file content hash, or symlink-target hash. A path reported by `git ls-files --cached` but absent from the working tree contributes no fingerprint entry.

`changed_paths` remains the separate authoritative inventory of staged, unstaged, and untracked increment changes. A reviewed deletion must therefore appear in the report before finalization even though the absent path contributes no current content to the fingerprint. A file that exists at finalization is fingerprinted, so deleting it afterward still invalidates the completion marker.

No legacy-fingerprint compatibility fallback, state-schema migration, report-inventory relaxation, or Git-commit identity is added. Completion evidence created by the defective deletion fingerprint must be reconstructed and revalidated on the corrected workflow baseline. D-030 is reserved by the preserved, unmerged Increment 4I branch; 4J uses D-031 to avoid a later decision-number collision.

Rationale: before commit, Git's cached path list retains a staged deletion and the original algorithm hashed its path plus a `missing` token. After commit, that path leaves the cached list and contributes nothing, invalidating otherwise unchanged reviewed content. Fingerprinting the current existing-content snapshot makes pre-commit and post-commit deletion states equal while preserving stale-content detection.

Consequences:

- Positive and negative tracked-deletion regressions are mandatory: a reviewed deletion survives commit, while a deletion after finalization invalidates the marker.
- Metadata and content read failures for existing paths still fail closed; path safety, report hashing, exact changed-file validation, suspicious-path checks, and Stop-loop behavior are unchanged.
- Pre-fix deletion markers remain invalid. Increment 4I stays unpushed and unmerged until it is reconstructed from corrected `main` and passes a fresh gate.
- The hook remains a trusted, operator-controlled workflow guardrail and gains no security, authorization, audit, or execution authority.

## D-032 - Remove the legacy synchronous provider scaffold

Date: 2026-07-15
Status: Accepted; Increment 4K implemented and verified

Decision: delete the public `agent::provider` and `agent::types` modules,
including the synchronous `AgentProvider::complete` trait, caller-authored
`AgentRequest`, arbitrary-string `AgentProviderResponse`, arbitrary mock failure
strings, deterministic mock implementation, and their embedded tests. Export only
the verified `agent::gateway_protocol` and `agent::function_call_validation`
modules from the current `agent` namespace.

A future provider transport requires a separately approved closed, bounded
request contract and must produce only validated normalized gateway events under
D-021. It must preserve explicit stream cancellation, deadlines, retries, size
and event limits, closed redacted failures, credential isolation, and independent
local function-call validation. Do not restore the deleted synchronous
arbitrary-string interface as a convenience abstraction.

Rationale: repository search found no caller outside the legacy modules and their
three embedded tests. The interface predates the verified Phase 4 gateway
protocol and cannot represent its streaming sequence, cancellation, limits,
correlation, or redacted-error boundary. Adapting it now would prematurely couple
unapproved gateway request construction, authentication, transport, and runtime
orchestration. Deletion is the smallest change that removes the misleading
bypass-shaped surface while preserving every verified boundary.

Consequences:

- Current Rust agent source exposes only the normalized gateway protocol and
  exact local function-call validator.
- The three legacy provider unit tests are removed with the unused implementation;
  existing gateway, function-validation, and public gateway-to-policy coverage
  remains unchanged and passing.
- The cross-platform product requirement for a provider abstraction remains. Its
  exact transport contract must be designed later from D-021 rather than from the
  removed Phase 2 mock API.
- O-006 and O-007 remain unresolved and continue to block live gateway networking
  and provider traffic.
- No replacement provider, dependency, manifest, lockfile, Tauri command, WebView,
  SQLite, credential, Keychain, gateway deployment, coordinator, dispatch,
  executor, capability, CSP, packaging, entitlement, or operating-system
  permission is added.

## D-033 - Remove the legacy memory scaffold

Date: 2026-07-15
Status: Accepted; Increment 4L implemented and verified

Decision: delete the public Rust `memory` module, including its arbitrary-content
input, update, and record types; unbounded in-memory store; six-marker
secret-like substring check; and embedded tests. Do not restore that interface as
the basis for product memory.

Future session, working, or long-term memory requires a separately approved
bounded repository contract with explicit user opt-in, authoritative provenance,
creation time, optional expiration, visibility, edit, delete, export, retention,
sensitive-data classification, encryption, and reviewed key ownership. Schema
validity or marker-list filtering alone cannot authorize or establish safe
retention.

Rationale: repository search found no caller outside the legacy module and its
three embedded tests. The scaffold retains and clones unbounded arbitrary strings
without the control, lifecycle, provenance, and encryption properties required by
the product brief. Repairing it would prematurely decide Phase 8 storage, consent,
Keychain, migration, retention, export, UI, and context-selection boundaries.
Deletion is the smallest change that prevents accidental adoption while leaving
the independent verified SQLite bootstrap module unchanged.

Consequences:

- Current Rust source exposes no product memory repository or secret-detector
  boundary.
- The three legacy memory unit tests are removed with the unused implementation;
  the 13 storage unit tests and both public storage smoke tests remain unchanged
  and passing.
- The product requirement for user-controlled memory remains. Later work must
  start from this decision and the product brief rather than the deleted Phase 2
  mock API.
- No replacement type, repository, migration, persistence, encryption, Keychain,
  context selection, Tauri command, WebView path, dependency, capability,
  entitlement, or operating-system permission is added.

## D-034 - Remove the legacy generic platform scaffold

Date: 2026-07-15
Status: Accepted; Increment 4M implemented and verified

Decision: delete the public Rust `platform` module, including arbitrary-string
platform metadata, the generic capability enum and report, caller-authored
capability status, mock adapter, and embedded tests. Do not restore a generic
capability map as permission, support, availability, or execution evidence.

Future OS integration requires separately approved capability-specific adapters.
Any permission-bearing contract must derive authoritative, resource-scoped,
provenance-aware, fresh evidence from the relevant trusted operating-system
boundary and preserve user initiation, requestability, denial/restriction, and
capability-specific failure semantics where applicable. A public constructor or
generic `Available` value cannot establish permission or policy allowance.

Rationale: repository search found no caller outside the legacy module and its
three embedded tests. The mock predates live app-info and the fixed Permission
Center, makes no native query, and can fabricate broad capability status without
scope or observation evidence. Repairing it would prematurely decide Phase 6 and
Phase 7 native frameworks, permission flows, selected resources, Keychain,
LocalAuthentication, IPC, UI, and lifecycle ownership. Deletion is the smallest
change that removes misleading authority-shaped state while preserving the
product's platform-adapter requirements.

Consequences:

- Current Rust source exposes no generic platform capability or permission-status
  API.
- The three legacy platform unit tests are removed with the unused implementation;
  app-info, public metadata smoke, and Permission Center coverage remain unchanged
  and passing.
- The product requirement for capability-specific platform adapters and a
  user-controlled Permission Center remains. Later work starts from the product
  brief, architecture baseline, security policy, and this decision.
- No replacement adapter, native framework, OS query, permission request,
  Keychain, LocalAuthentication, Tauri command/event/plugin, frontend state,
  capability configuration, entitlement, dependency, or operating-system
  permission is added.

## D-035 - Construct one closed bounded initial gateway request

Date: 2026-07-15
Status: Accepted; Increment 4N implemented and verified

Decision: trusted Rust constructs the first desktop-to-gateway request as one
non-cloneable `InitialGatewayRequest`. Its only caller inputs are an opaque run
ID, an opaque gateway-request ID, and one owned user-selected text value. The
closed JSON representation fixes protocol version 1, request kind
`initial_user_turn`, model turn 1, retry attempt 0, tool set
`cortexa_desktop_mvp@1`, and every request, loop, event, function-call,
argument, output, retry, and deadline limit already defined by D-021 and the
verified gateway protocol.

The constructor reuses the normalized response protocol's opaque-ID predicate,
rejects blank or source-oversized content, and enforces the 64 KiB request limit
again after JSON escaping. Private wire structs own serialization. The public
value exposes only borrowed bytes, does not implement `Clone`, `Serialize`, or
`Deserialize`, and redacts the body from `Debug`. Closed errors expose fixed
reasons and numeric sizes only.

The desktop request does not contain a gateway URL, provider/model selection,
authorization value, credential, principal identity, OpenAI parameter, tool
definition, schema, hosted tool, MCP server, shell tool, local permission,
policy, approval, audit, dispatch, or execution field. The fixed tool-set value
is gateway contract input, not local authority. A future gateway remains
responsible for authenticating and authorizing the desktop principal, selecting
the exact strict server-side tool definitions, and forcing foreground Responses
streaming with `store: false`, `background: false`, and parallel calls disabled.

Rationale: D-021 requires a closed outbound product request before authenticated
transport, while the repository previously defined only the normalized inbound
event contract. A transport-free initial-turn envelope independently verifies
identity, privacy, schema closure, and byte limits without prematurely selecting
a gateway platform, identity provider, credential store, HTTP client, model,
continuation contract, or runtime coordinator.

Consequences:

- Selected user content is intentionally present in request bytes but absent
  from debug output, errors, logs, audit, persistence, and all current runtime
  paths.
- Request and normalized response identities use one shared predicate and cannot
  drift independently inside the agent module.
- The first request cannot be repurposed for continuation or retry because turn
  and attempt fields are fixed and no general wire constructor is public.
- The fixed tool-set ID requires exact agreement with a later deployed gateway
  before live use; it grants no policy, approval, dispatch, or execution
  authority.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, runtime
  coordinator, policy, approval, audit persistence, dispatch, executor,
  capability, entitlement, or operating-system permission is added.

## D-036 - Bind initial request construction to response validation

Date: 2026-07-15
Status: Accepted; Increment 4O implemented and verified

Decision: trusted Rust constructs the initial desktop-to-gateway exchange as one
non-cloneable `InitialGatewayTurn`. The turn privately owns both the D-035 closed
request bytes and one `GatewayStreamValidator` configured from the same opaque
run ID and gateway-request ID. It derives its allowed response function names
only from `ToolSchema::GetCurrentDatetimeV1` and
`ToolSchema::CreateLocalTaskV1`, requires their common version to equal the fixed
`cortexa_desktop_mvp@1` request tool-set version, and fails closed with a fixed
content-free error if internal validator configuration cannot be established.

The public initial-turn surface exposes only borrowed request bytes, stream
status, normalized frame acceptance, and local terminal cancellation. The raw
`InitialGatewayRequest` type, constructor, and byte accessor are private to
`gateway_request.rs`; no caller can separately construct initial request bytes.
The lower-level `GatewayStreamValidator::new` remains public for protocol
fixtures and independently verified downstream boundaries, but future initial
transport code must use `InitialGatewayTurn` and must not reconstruct a detached
validator.

Rationale: D-035 closed outbound serialization, but request and response
correlation plus allowed tool configuration still entered through two separate
public constructors. A future trusted transport could therefore pair a valid
request with a validator configured for different IDs, names, or version. Binding
the two values before transport closes that local configuration gap without
prematurely introducing networking, credentials, continuation, deadlines,
retries, or runtime orchestration.

Consequences:

- One initial-turn constructor is the source of both outbound correlation and
  inbound validation expectations.
- Selected text remains present only in borrowed request bytes and absent from
  debug output, errors, logs, audit, persistence, and all current runtime paths.
- Exact local tool names/version are derived from trusted Rust catalog values,
  not from the model, WebView, gateway event, or a future transport caller.
- Local cancellation only terminally closes validation state; no network abort
  or runtime scheduler exists.
- Making `InitialGatewayRequest` private narrows the Rust public API. The crate is
  not published and repository callers are migrated, but an unsupported external
  consumer would need to adopt `InitialGatewayTurn`.
- A normalized or schema-valid function call remains non-authorizing and must
  still pass local schema, policy, approval, audit, dispatch, and execution
  boundaries where applicable.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, continuation,
  runtime coordinator, policy, approval, audit persistence, dispatch, executor,
  capability, entitlement, or operating-system permission is added.

## D-037 - Bind initial gateway events to exact local schema validation

Date: 2026-07-15
Status: Accepted; Increment 4P implemented and verified

Decision: `InitialGatewayTurn` owns one private `InMemoryToolRegistry` populated
from the same exact fixed `ToolSchema` array that supplies the response
validator's allowed function names and common contract version. Registry
registration, version agreement, and validator construction share one fixed
content-free configuration failure. No initial-turn caller can provide, replace,
or mutate the registry.

The bound turn exhaustively converts each normalized `ValidatedGatewayEvent` into
one closed non-cloneable `InitialGatewayEvent`. Non-function events retain their
verified data contracts. A completed function call is immediately consumed by
`validate_function_call`; only a `SchemaValidatedFunctionCall` with locally
derived typed arguments, risk, and permission may leave the turn. Raw normalized
argument JSON and a caller-selected registry cannot cross this bound-turn
boundary.

Protocol errors and local function-validation errors remain distinct through one
typed `InitialGatewayTurnError`. Protocol rejection preserves the verified
transactional validator behavior. Local schema rejection sets private wrapper
terminal state, reports `GatewayStreamStatus::Failed`, rejects every later frame
as already terminal, and makes cancellation a no-op. This local state does not
claim a gateway/provider failure or transport abort. Event debug output redacts
assistant text and function arguments.

Rationale: Increment 4O bound request and response-protocol configuration, but a
future caller could still receive raw normalized arguments and independently
select or omit local schema validation before policy. Owning the exact registry
and typed conversion in the same turn closes that gap without introducing a
runtime coordinator or changing the lower-level independently verified protocol,
registry, function-validation, or policy APIs.

Consequences:

- Replacing the bound turn's `ValidatedGatewayEvent` and bare protocol-error
  surface with `InitialGatewayEvent` and `InitialGatewayTurnError` intentionally
  narrows the public Rust API. The crate is not published and the only repository
  caller is migrated; a theoretical unsupported external consumer would need to
  adopt the new closed types.
- `GatewayStreamValidator`, `ValidatedGatewayEvent`, `UntrustedFunctionCall`,
  `ToolRegistry`, and `validate_function_call` remain public for focused tests and
  independently verified downstream boundaries. Future initial transport code
  must use `InitialGatewayTurn` rather than reconstructing those steps.
- A schema-valid call remains non-authorizing. It must still pass deterministic
  policy, exact approval where required, trusted audit, dispatch, and execution
  boundaries.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, continuation,
  runtime coordinator, policy, approval, audit persistence, dispatch, executor,
  capability, entitlement, or operating-system permission is added.

## D-038 - Release an initial function call only after terminal completion

Date: 2026-07-15
Status: Accepted; Increment 4Q implemented and verified

Decision: `InitialGatewayTurn` privately retains the one bounded
`SchemaValidatedFunctionCall` accepted from a normalized non-terminal function
frame. `accept_frame` returns `Ok(None)` for that frame and may release the exact
owned call only after the same bound validator accepts terminal
`response_completed`. Terminal completion after text retains the existing closed
`ResponseCompleted` event.

An accepted `response_failed` event and successful local cancellation eagerly
discard any pending call before returning. Local schema failure never populates
the pending slot and retains Increment 4P's terminal wrapper behavior. A protocol
error remains transactional: it neither releases nor discards a previously
buffered call, so the correct contiguous terminal frame can still complete the
same response.

`None` means only that one schema-valid function call was accepted and withheld
pending terminal completion. Terminal release proves response completion plus
local schema validity; neither condition grants policy allowance, approval,
audit authority, dispatch eligibility, or execution authority.

Rationale: Increment 4P prevented raw normalized arguments and caller-selected
registries from leaving the bound turn, but it released a typed call while stream
status remained `Streaming`. A later gateway failure, local cancellation, or
missing terminal event could therefore follow after a future caller had already
started policy or approval. Private retention closes that ordering gap without a
runtime coordinator or changes to the independently verified protocol, schema,
registry, policy, approval, or audit APIs.

Consequences:

- `InitialGatewayTurn::accept_frame` now returns
  `InitialGatewayTurnResult<Option<InitialGatewayEvent>>`. The crate is not
  published and its only repository caller is migrated, but a theoretical
  unsupported external consumer must handle `None` and continue frame intake.
- Bounded typed arguments remain ephemeral for the additional interval between
  function-frame acceptance and terminal completion and remain absent from
  debug, errors, logs, audit, persistence, and IPC.
- Failure and cancellation cannot later release the discarded call because the
  owned validator is terminal and rejects late frames.
- Lower-level protocol and registry APIs remain public for focused fixtures and
  independently verified boundaries. Future initial transport code must own
  `InitialGatewayTurn` and must not reconstruct early-release behavior.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, continuation,
  runtime coordinator, policy, approval, audit persistence, dispatch, executor,
  capability, entitlement, or operating-system permission is added.

## D-039 - Bind terminal initial calls to one deterministic policy decision

Date: 2026-07-15
Status: Accepted; Increment 4R implemented and verified

Decision: after the bound validator accepts terminal `response_completed`,
`InitialGatewayTurn` consumes its exact private `SchemaValidatedFunctionCall`
through `PolicyInput::from_validated_call` and a locally selected
`DeterministicPolicyEngine::new()`. It returns only
`InitialGatewayEvent::PolicyEvaluated { decision }`; the previous terminal
standalone-call event is removed from this bound path. The decision retains the
exact owned call and exposes it only through borrowed accessors.

Policy evaluation cannot occur at the non-terminal function frame or after
gateway failure, local schema failure, or cancellation. The fixed existing rules
remain unchanged: `get_current_datetime@1` is `Allow` / `InformationOnly`, and
`create_local_task@1` is `RequireApproval` /
`ReversibleRequiresApproval`. Even `Allow` is non-authorizing data and grants no
approval, audit, dispatch, execution, permission, run-liveness, or user-intent
evidence.

Rationale: Increment 4Q closed terminal ordering but still allowed a future
initial transport caller to receive the schema-valid call and delay, replace, or
omit canonical policy evaluation. Keeping the fixed policy transition inside the
existing bound turn closes that choice without adding a coordinator, policy
injection point, approval path, or runtime.

Consequences:

- The request module has a bounded dependency on existing policy types while the
  policy decision retains an agent-owned call; the ownership graph remains
  non-recursive.
- The public initial event API narrows. The crate is unpublished and its only
  repository caller is migrated, but a theoretical unsupported external
  consumer must adopt the decision event.
- Lower-level validators and policy constructors remain public for focused
  fixtures. Future initial transport must own `InitialGatewayTurn` and must not
  reconstruct a detached validation-to-policy path.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, continuation,
  runtime coordinator, approval, audit persistence, dispatch, executor,
  capability, entitlement, or operating-system permission is added.

## D-040 - Bind terminal approval requirements to one exact presentation

Date: 2026-07-15
Status: Accepted; Increment 4S implemented and verified

Decision: after the bound validator accepts terminal `response_completed` and
the fixed deterministic policy engine derives `RequireApproval`,
`InitialGatewayTurn` must pass the exact owned `PolicyDecision` directly to its
private `InMemoryApprovalManager`, create one manager-owned request, and
immediately issue one owned `ApprovalPresentation`. The bound path returns only
`InitialGatewayEvent::ApprovalPresentationReady { presentation }` for that
outcome; it cannot release the approval-required decision for a caller-selected
transition. `Allow` and `Deny` continue to return non-authorizing
`PolicyEvaluated` events.

The presentation is proposal data only. It remains non-cloneable,
non-serializable, and non-comparable, and carries no approval disposition,
trusted interaction evidence, authentication evidence, audit receipt,
run-liveness proof, dispatch eligibility, permission grant, or execution
authority. Approval-manager failures cross the turn only through the existing
closed `ApprovalError` in typed `InitialGatewayTurnError::Approval`; no fallback
decision or presentation is emitted after terminal stream acceptance.

Rationale: Increment 4R prevented callers from omitting deterministic policy,
but still returned a terminal `RequireApproval` decision. A future caller could
therefore delay, replace, or omit the verified request and presentation
transition. Keeping the fixed manager transition inside the bound turn closes
that choice without adding native interaction, source resolution, audit,
dispatch, execution, or a new coordinator.

Consequences:

- `InitialGatewayTurn` owns one private approval manager whose 120-second TTL
  starts when terminal completion creates the request.
- The public initial event no longer implements `Eq` or `PartialEq` because it
  may own an intentionally non-comparable presentation. The crate is
  unpublished and its only repository caller is migrated.
- The private manager cannot yet receive a trusted source outcome after the
  presentation leaves the event. This deliberately incomplete path is
  non-executable and requires a separately approved orchestration increment.
- The request module's bounded dependency on approval and policy types remains
  at the trusted assembly boundary; any new coordinator requires separate
  architecture review.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, native
  interaction, audit write, dispatch, executor, capability, entitlement, or
  operating-system permission is added.

## D-041 - Return sealed approval outcomes only to the issuing turn manager

Date: 2026-07-15
Status: Accepted; Increment 4T implemented and verified

Decision: on macOS, `InitialGatewayTurn` consumes one
`TrustedApprovalSourceOutcome` and delegates it directly, without inspection or
transformation, to `ApprovalManager::resolve_source_outcome` on the same private
`InMemoryApprovalManager` that created and issued the presentation. The method
returns only the manager-owned `ApprovalResolution` or the existing typed
`InitialGatewayTurnError::Approval`.

The resolution is interaction evidence only and remains non-authorizing. Even
`ApprovalDisposition::Approved` grants no run-liveness, audit completion,
permission, persistence, dispatch, execution, provider continuation, or tool
result authority. Manager ownership, exact identity, source kind, presentation
issuance, deadline, and one-time consumption remain exclusively enforced by the
existing manager.

The API is gated to macOS because the sealed source outcome is currently owned
by the macOS native decision-source boundary. The existing synthetic dialog
result mapper is `pub(crate)` only under `cfg(test)` so crate unit tests can
exercise the sealed path without opening native UI; no production constructor,
feature flag, IPC route, or integration-test bypass is added.

Rationale: Increment 4S returned a manager-owned presentation while retaining
the manager privately, so a future caller could not return the resulting sealed
native outcome to that exact manager. A direct ownership-consuming delegation
closes that gap while preserving all existing manager checks and avoiding a new
coordinator or caller-authored approval metadata.

Consequences:

- `agent::gateway_request` gains a bounded macOS-gated dependency on the native
  sealed outcome and approval resolution at the trusted assembly boundary.
- A platform-neutral coordinator remains future work and requires separate
  architecture review before orchestration expands.
- Run-termination cancellation, proactive expiry, stale-dialog handling,
  active-run validation, audit binding, dispatch, and execution remain absent.
- O-006 and O-007 remain unresolved and continue to block authenticated gateway
  transport and live provider traffic.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, native UI
  invocation, capability, entitlement, or operating-system permission is added.

## D-042 - Retain and terminally cancel only the turn-owned approval subject

Date: 2026-07-15
Status: Accepted; Increment 4U implemented and verified

Decision: after `InitialGatewayTurn` successfully issues one manager-owned
`ApprovalPresentation`, it retains only that manager-assigned `ApprovalId` in a
private optional field. The public
`cancel_pending_approval_for_run_termination` operation accepts no approval ID,
choice, native result, interaction evidence, or run-liveness claim. With no
owned pending subject it returns `Ok(None)`; otherwise it delegates the exact
private ID to the existing manager's `cancel_for_run_termination` operation and
returns only the existing non-authorizing `ApprovalResolution`.

The turn clears its retained ID only after the manager successfully terminalizes
the subject. Manager expiry remains authoritative, so a call at or after the
deadline returns `Expired` rather than rewriting the result as run-terminated.
A successful sealed native resolution also clears the retained ID, while a
typed native-resolution error leaves it intact so the same manager can still
resolve or cancel the subject. Every late native outcome remains rejected after
run termination.

Rationale: Increment 4T bound native outcomes to the issuing private manager but
left no turn-owned handle for a trusted future orchestrator to close a pending
approval when its run terminates. Retaining only the manager-generated ID and
exposing a no-argument deny/close transition adds that lifecycle boundary
without allowing a caller to select a subject or fabricate approval evidence.

Consequences:

- Run termination can only deny or close the turn's exact pending subject; it
  cannot approve, authenticate, grant permission, dispatch, execute, or continue
  a provider response.
- No-pending and repeated calls are idempotent and return no resolution.
- The manager remains the source of truth for exact identity, deadline,
  disposition, one-time consumption, tombstones, and replay rejection.
- The run-termination resolution remains unaudited and non-authorizing.
- A native dialog may remain visibly stale after manager cancellation; 4U adds
  no native invocation, dismissal, timer, coordinator, or active-run registry.
- No dependency, manifest, lockfile, Tauri command/event/plugin, WebView path,
  SQLite path, credential, Keychain, provider SDK, network client, persistence,
  capability, entitlement, or operating-system permission is added.

## D-043 - Adopt the owner-supplied opaque raster as the brand authority

Date: 2026-07-15
Status: Accepted; Meta Increment 1 implementation approved

Decision: the owner-supplied 360 x 434 PNG with SHA-256
`ecdcc56f3c9193dd7caf092778ef096c3f9af7047355e417bce7815301426ea7`
is the authoritative Cortexa logo source. `logo-primary.png`,
`logo-light.png`, and `logo-dark.png` preserve its exact bytes. The light and
dark names are usage aliases, not recolored variants, because the source has a
fully opaque near-white protected field. The 64 x 64 favicon is a proportional
padded derivative, and the 512 x 512 app-icon source centers the unscaled source
on the same field color.

The logo must not be redrawn, traced, cropped, recolored, made transparent,
rotated, distorted, or reconstructed from a screenshot or presentation export.
The human-facing identity remains `Cortexa` under D-026. Repository, package,
crate, executable, bundle, database, GitHub, IPC, event, command, and storage
compatibility identifiers remain unchanged.

Production files under `src-tauri/icons/` remain unchanged in Meta Increment 1.
Their replacement requires separately approved Meta Increment 2, deterministic
generation from `assets/branding/app-icon-source.png`, complete format and
packaging verification, and target-Mac visual checks.

Rationale: preserving the complete owner image is the only lossless treatment
available from the supplied raster. Background extraction or separate dark
recoloring would require inventing edge pixels and a new design. A canonical
asset directory plus explicit usage rules prevents screenshots, stale decks,
and placeholder graphics from becoming competing sources of identity.

Consequences:

- The near-white field remains visible on both light and dark backgrounds.
- Raster use must respect native dimensions, aspect ratio, clear space, minimum
  size, and documented accessibility constraints.
- Favicon and app-icon-source padding is deterministic and introduces no logo
  crop or recolor.
- No font, icon, image-processing, runtime, or application dependency is added.
- External decks and diagrams are reference material until separately updated
  from current repository facts and the canonical assets.

## D-044 - Establish root engineering authorities and renumber the icon rollout

Date: 2026-07-15
Status: Accepted; Meta Increment 2 implementation approved

Decision: the root documents `ENGINEERING_GUIDE.md`, `ARCHITECTURE.md`,
`PRODUCT_REQUIREMENTS.md`, `ROADMAP.md`, `TESTING_GUIDE.md`,
`SECURITY_CHECKLIST.md`, and `RELEASE_CHECKLIST.md` are the authoritative
engineering operating references defined in the precedence order in
`ENGINEERING_GUIDE.md`. The inception product brief and target architecture
baseline remain preserved under `docs/product/`, while completed plans,
increment records, reviews, decisions, changelog entries, and troubleshooting
entries remain historical evidence. Current status comes from the root roadmap,
project status, next steps, and active plan rather than old checkpoint wording.

The project owner's explicit Meta Increment 2 engineering-operating-system
request supersedes the queue number assigned to the unimplemented application-
icon rollout by D-043. The icon rollout is renumbered Meta Increment 3 without
changing its exact 16-icon scope, source asset, risks, non-goals, or separate
approval requirement. D-043 remains authoritative for the logo and derivative
rules; only its follow-on increment number is superseded.

Rationale: the repository accumulated accurate but distributed guidance and
then published Meta Increment 1 after its closeout documents were finalized.
One explicit authority model prevents aspirational product text and historical
checkpoint state from being mistaken for current code. Renumbering preserves
the owner's selected Meta Increment 2 without discarding the already reviewed
icon plan.

Consequences:

- New implementation plans must label current, mocked, planned, and prohibited
  behavior and reconcile against `ARCHITECTURE.md`.
- `PRODUCT_REQUIREMENTS.md` normalizes current requirements without deleting or
  changing the inception brief's historical meaning.
- `NEXT_STEPS.md` remains the ordered immediate queue; `ROADMAP.md` remains the
  milestone view.
- Historical Meta Increment 1 review evidence may still say the icon plan was
  Meta Increment 2. That statement is a correct checkpoint and is explicitly
  superseded by this decision rather than rewritten.
- Meta Increment 3 may become Ready only after Meta Increment 2 is verified
  complete. It still requires separate project-owner approval and must not
  begin automatically.
- No application source, behavior, dependency, identifier, configuration,
  capability, permission, database, icon, or release system changes.

## D-045 - Modularize repository gates and renumber the icon rollout

Date: 2026-07-15
Status: Accepted; Meta Increment 3 implementation approved

Decision: preserve the supported trusted-project `.codex/hooks.json` Stop
definition and its existing `decision: block` continuation contract. Extract
its bounded Git, path, JSON, conflict, and suspicious-path primitives into a
standard-library-only shared module, and add a separate read-only session-end
inventory. Evidence-based architecture, security, code-health,
technical-debt, readiness, executive, release, quality, and post-increment
workflows live in focused repository skills, prompts, and templates. They may
report evidence and synchronize documentation, but they cannot run arbitrary
report content, use the network, silently fix advisories, modify product source
after verification, commit, push, merge, release, or begin later work.

The project owner's explicit Meta Increment 3 automation request supersedes the
queue number assigned to the still-unimplemented icon rollout by D-044. The
icon rollout becomes Meta Increment 4 without changing its 16-icon scope,
canonical source, risks, non-goals, target-Mac matrix, rollback, or separate
approval requirement.

Repository Git publication uses descriptive capability-based branches,
Conventional Commits, descriptive pull-request titles, and descriptions with
Purpose, Files changed, Testing performed, Breaking changes, and Next
increment. Publication still requires explicit project-owner direction; after
checks pass, the approved branch is pushed, reviewed through a pull request,
and squash-merged.

Rationale: the existing gate is verified and should be extended without
inventing a second Stop contract or duplicating policy across large prompts.
Small composable reviews make evidence easier to inspect while deterministic
scripts keep path and state validation closed. Renumbering preserves the
owner-selected automation work and the previously reviewed icon plan.

Consequences:

- `.codex/hooks.json` remains unchanged; repository hook trust and emergency
  disable procedures remain explicit operator boundaries.
- Hook scripts use only fixed Git argument arrays and Python standard-library
  operations. Their reports and ignored marker are workflow evidence, not
  authorization or product audit.
- Failed required checks, pending mandatory manual checks, conflicts, stale
  content, or blocking Critical/High findings cannot complete the gate.
- Meta Increment 4 may become Ready only after Meta Increment 3 is verified
  complete. It must not start automatically.
- Historical Meta Increment 1 and 2 evidence retains the icon plan numbers that
  were accurate at those checkpoints; D-044 and D-045 supersede only the live
  queue number.
- No product source, behavior, dependency, configuration, capability,
  permission, database, icon, identifier, or release artifact changes.

## D-046 - Use read-only GitHub automation and exact visible advisory baselines

Date: 2026-07-16
Status: Accepted; Meta Increment 5 implemented and verified

Decision: GitHub quality automation receives only top-level `contents: read`,
uses no repository secrets, disables persisted checkout credentials, excludes
`pull_request_target`, and may only inspect, format-check, lint, test, build, and
retrieve public advisory data. Official checkout and Node setup actions are
pinned to immutable commit digests. Dependabot may create review branches for
npm, Cargo, and GitHub Action proposals, but no repository workflow rebases,
commits, pushes, merges, publishes, deploys, signs, notarizes, or auto-merges.

Local repository-health scripts use only the Python standard library, fixed Git
inspection arguments, bounded text reads, redacted finding output, and explicit
exit codes. Rust advisory automation runs exact `cargo-audit 0.22.2` and accepts
only the following unchanged lockfile identities:

- D-025's vulnerabilities `RUSTSEC-2026-0194` and `RUSTSEC-2026-0195` in
  `quick-xml 0.39.4`.
- Warning `RUSTSEC-2024-0370` in `proc-macro-error 1.0.4`.
- Warnings `RUSTSEC-2024-0411` through `RUSTSEC-2024-0420`, excluding numbers
  not published in that range, in the exact GTK3-family packages and versions
  encoded by `scripts/cargo_audit_gate.py`, plus `RUSTSEC-2024-0429` in
  `glib 0.18.5`.
- Warnings `RUSTSEC-2025-0075`, `RUSTSEC-2025-0080`, `RUSTSEC-2025-0081`,
  `RUSTSEC-2025-0098`, and `RUSTSEC-2025-0100` in their exact `unic` 0.9.0
  packages encoded by the gate.
- Unsoundness warning `RUSTSEC-2026-0190` in `anyhow 1.0.102`.

Any new or changed identity/version, partial baseline drift while findings
remain, malformed report, inconsistent audit exit status, or audit tool failure
fails the workflow. A clean report is accepted. Accepted findings stay visible
as unresolved advisories and do not become an ignore list or a claim that the
dependencies are fixed or generally safe.

Rationale: read-only, reproducible checks provide useful pull-request evidence
without granting untrusted code or dependency proposals write authority. The
unchanged lockfile has pre-existing RustSec findings, including the D-025
vulnerabilities, unmaintained transitive packages, and one `anyhow` unsoundness
warning. An exact parser prevents silent baseline expansion while allowing the
repository to detect new findings and schedule dependency remediation as its
own reviewed increment.

Consequences:

- `anyhow 1.0.102` is active transitively through Tauri and Tauri utilities.
  Project source has no direct `downcast_mut` or `.context(...)` reference, but
  no complete transitive reachability claim is made; remediation remains debt.
- The GTK3 and `unic` warning families remain tracked even when target-specific
  dependencies are not active in the current macOS build.
- Hosted workflow success, CODEOWNERS, badges, labels, and milestones are
  coordination evidence, not branch-protection proof or security approval.
- Advisory retrieval is the only planned workflow network use and receives no
  repository secret context.
- Meta Increment 5 changes no dependency, manifest, lockfile, application
  source, Tauri boundary, permission, CSP, capability, or SQLite schema.

## D-047 - Record that no repository license is selected

Date: 2026-07-16
Status: Accepted; Meta Increment 5 implemented and verified

Decision: do not add a root `LICENSE` file or infer open-source,
source-available, commercial, or contribution terms. Record the current
no-license-selected state in `docs/github/LICENSING.md`. Repository visibility
alone is not represented as permission to use, copy, modify, distribute,
sublicense, or sell the software, and external code contributions require prior
maintainer coordination.

Rationale: selecting legal terms is a project-owner decision, not a mechanical
repository-hygiene choice. An explicit decision record is more accurate than an
invented license and allows health checks to distinguish an intentional legal
boundary from a missing file.

Consequences:

- Public release and an open external contribution program remain blocked on a
  separately approved license decision and legal review where appropriate.
- The repository health check passes only because the explicit licensing record
  exists; it does not describe the project as open source.
- Future license adoption must add the authoritative text, reconcile README and
  contribution guidance, and receive separate review.

## D-048 - Select Meta Increment 5 and renumber the unchanged icon rollout

Date: 2026-07-16
Status: Accepted; project-owner queue direction

Decision: reconcile Meta Increment 3 as squash-merged at `ad9042c` and select
repository health and GitHub hygiene as Meta Increment 5. The project-owner
Meta Increment 4 executive-document request was stopped before mandatory gate
state, plan creation, or repository edits; it is neither completed work nor a
Ready increment. Renumber the unchanged verified application-icon rollout from
Meta Increment 4 to Meta Increment 6, preserving its exact 16-icon source scope,
canonical asset, risks, non-goals, target-Mac matrix, rollback, and separate
approval requirement.

Rationale: repository history must not invent completion evidence for a stopped
request, and the owner's explicit Meta Increment 5 scope must coexist with the
previously reviewed but unimplemented icon plan. A recorded gap is more accurate
than silently reusing Meta Increment 4 or rewriting historical checkpoints.

Consequences:

- Historical Meta Increment 1 through 3 records keep the icon-plan numbers that
  were accurate at those checkpoints; D-044, D-045, and D-048 define the live
  number progression.
- Meta Increment 6 becomes Ready only after Meta Increment 5 is verified and
  published, and still requires separate project-owner approval.
- Increment 4V remains Proposed and is not selected by this meta queue change.
- No executive document, icon, product source, runtime behavior, dependency,
  configuration, capability, permission, database, or compatibility identifier
  changes through this decision.

## D-049 - Assign Meta Increment 6 to the product readiness audit

Date: 2026-07-16
Status: Accepted; project-owner queue direction

Decision: the project owner's latest explicit direction assigns Meta Increment 6
to the documentation-only Product Readiness Audit. The audit evaluates current
source and verification evidence, records a `NOT READY` result, and creates no
application behavior, dependency, configuration, permission, schema, icon, or
release change.

The previously Ready application-icon rollout remains unimplemented and
preserves its exact reviewed 16-file source scope in
`docs/plans/meta-06-verified-application-icon-rollout.md`. That filename and its
historical content are retained as evidence during this audit. The icon work is
deferred and must receive a new live increment number, reconciled plan links,
separate approval, and its own gate before any icon edit begins.

Rationale: the latest owner-assigned increment purpose supersedes the live queue
number from D-048, but a readiness audit must not silently implement or rewrite
the displaced remediation. Preserving the old plan while explicitly deferring
it keeps the audit documentation-only and prevents either body of work from
being represented as complete without evidence.

Consequences:

- Meta Increment 5 is reconciled as verified, published, and squash-merged at
  `6b149fa`; its completion marker was complete and valid on that clean baseline.
- Meta Increment 6 records an evidence-based product result of `NOT READY` with
  a composite maturity score of 57/100.
- Increment 4V terminal approval audit is the smallest recommended remediation,
  but remains Proposed until separately selected and approved.
- The application-icon rollout is not Ready under a current live number. It may
  be renumbered only through a later planning decision; its existing source
  scope and verification requirements remain unchanged.
- No later increment, gate state, source edit, commit, push, merge, or release
  starts through this decision.

## D-050 - Assign the verified application-icon rollout to Meta Increment 7

Date: 2026-07-16
Status: Accepted; project-owner queue direction

Decision: after publishing the Meta Increment 6 Product Readiness Audit at
`5281fac`, assign the unchanged verified application-icon rollout the new live
number Meta Increment 7. Rename its plan to
`docs/plans/meta-07-verified-application-icon-rollout.md`, reconcile current
queue and milestone references, and mark the plan Ready for separate
implementation approval.

The rollout retains the exact 16 existing `src-tauri/icons/` paths, canonical
`assets/branding/app-icon-source.png`, brand constraints, risks, non-goals,
package and target-Mac verification matrix, and rollback from its previously
reviewed plan. This planning decision does not begin `meta-07` gate state,
generate or replace an icon, change application behavior, or select Increment
4V.

Rationale: D-049 correctly displaced the icon plan when the owner assigned Meta
Increment 6 to the readiness audit. That audit is now published and complete,
while the bounded icon work remains unimplemented. A new live number and
reconciled links remove the naming collision without rewriting historical plans,
increment records, decisions, or dated reviews that accurately describe earlier
checkpoints.

Consequences:

- Meta Increment 7 is the first Ready item in `NEXT_STEPS.md` and requires
  explicit project-owner implementation approval before any gate or icon edit.
- D-043 through D-049 remain historical authority for the brand source,
  constraints, prior numbering, Meta 6 audit assignment, and deferred state at
  those checkpoints.
- Increment 4V remains Proposed as the audit's smallest recommended product
  remediation and is not selected by this meta planning change.
- No application source, icon, source asset, dependency, manifest, lockfile,
  Tauri configuration, capability, permission, identifier, database, commit,
  push, merge, or release changes through this decision.

## D-051 - Keep raw Tauri development icon behavior as a documented exception

Date: 2026-07-16
Status: Accepted; project-owner-approved Meta Increment 7 baseline exception

Decision: generate the exact existing 16 Tauri icon outputs from
`assets/branding/app-icon-source.png` with the repository's installed Tauri CLI
and require official Cortexa identity for debug-bundled and release-bundled
`.app` artifacts. Do not expand Meta Increment 7 into runtime source,
configuration, dependency, capability, or permission changes solely to replace
the generic icon macOS assigns to the raw unbundled `npm run tauri -- dev`
executable.

Rationale: target-Mac AppKit inspection proves both debug and release app
bundles register the generated Cortexa icon, preserve the configured Cortexa
name, and present the same official mark under Aqua and Dark Aqua. The raw
development command runs a bare executable rather than an application bundle,
and macOS registers that process with its generic `exec` icon. Correcting that
development-only presentation would require work outside the frozen 16-icon
scope and is not evidence that the production bundle is wrong.

Consequences:

- Packaged icon acceptance requires D-052's exact/semantic generated-output
  checks, embedded ICNS byte equality, successful debug and release `.app`
  bundles, and target-Mac system icon inspection.
- The raw unbundled development icon remains a visible non-blocking advisory and
  must not be represented as Cortexa in verification evidence.
- Default DMG creation is not waived as a release gate. Its Finder AppleScript
  failure remains a separate release-readiness advisory even though the required
  debug and release app bundles pass.
- No runtime source, Tauri configuration, identifier, dependency, capability,
  permission, or application behavior changes through this decision.

## D-052 - Verify regenerated ICNS by decoded representations

Date: 2026-07-16
Status: Accepted; Meta Increment 7 verification rule

Decision: compare the 15 non-ICNS Tauri icon outputs byte-for-byte with a fresh
generation. Validate `icon.icns` by its complete representation inventory and
decoded RGBA pixels because repeated runs of the pinned Tauri CLI can serialize
equivalent ICNS content to different container bytes. Continue to require the
reviewed repository `icon.icns` and each debug/release bundled resource to match
byte-for-byte.

Rationale: two fresh generations from the unchanged canonical source produced
three distinct ICNS SHA-256 values when compared with the reviewed repository
file, while all ten 1x/2x representations from 16 through 1024 pixels decoded
pixel-identically. A raw regeneration comparison would therefore be flaky and
would not measure visual identity. Decoded representation equality fails closed
on any actual image change, while exact repository-to-bundle equality still
proves packaging used the reviewed artifact.

Consequences:

- Future icon regeneration checks must parse ICNS structure and compare every
  decoded representation, not accept a size list or visual sample alone.
- Any missing representation, dimension mismatch, decode failure, or pixel
  difference fails verification.
- Debug and release app resources must remain byte-identical to the committed
  repository ICNS.
- This rule changes no canonical asset, icon output, source, configuration,
  dependency, identifier, capability, permission, or product behavior.

## D-053 - Bind terminal approval success to one turn-owned typed audit record

Date: 2026-07-16
Status: Accepted; verified implementation squash-merged through PR #23 at
`6e6f91d`

Decision: `InitialGatewayTurn` owns one private
`InMemoryApprovalAuditAdapter`. After its private approval manager successfully
terminalizes either a sealed native-source outcome or the turn-owned
run-termination cancellation, the turn passes that exact owned
`ApprovalResolution` by reference to the adapter. It does not reconstruct an
identity, disposition, preview, interaction-evidence value, or policy fact.

The public success value is one closed, non-cloneable
`AuditedApprovalResolution` that owns the exact resolution and its
`ApprovalAuditReceipt`. Callers receive only a shared resolution reference and a
copy of the existing sequence-only receipt. No constructor, mutable accessor,
serialization path, or conversion to dispatch or execution authority exists.

Manager terminalization is authoritative and occurs before audit recording. The
turn clears its retained pending approval ID immediately after manager success,
including when the subsequent audit write returns a typed error. Audit failure
returns no resolution or receipt and cannot roll back the consumed manager
subject. Manager failure still leaves pending turn ownership intact.

Rationale: Increment 4H provided a closed validator and record projection, while
4T and 4U exposed two manager-owned terminal resolution paths. Keeping the exact
resolution inside the trusted turn until record validation succeeds removes the
future caller bypass without inventing persistence, a transaction, or execution
authority.

Consequences:

- Native and run-termination success cannot return a standalone
  `ApprovalResolution` from `InitialGatewayTurn`.
- The adapter is process-local and turn-local. Sequence one is expected for the
  current one-subject initial turn and is not a durable or global ordering claim.
- A record or receipt proves neither user authentication, current run liveness,
  durable storage, dispatch eligibility, execution, nor provider continuation.
- Audit failure after manager success is fail-closed for the caller but is not a
  transaction; a future durable coordinator must define stronger atomicity
  before execution.
- No lower-level manager, native source, audit adapter, policy, schema, gateway
  protocol, dependency, manifest, lockfile, Tauri command, WebView, SQLite,
  capability, entitlement, permission, network, credential, dispatch, or
  executor contract changes.

## D-054 - Route trusted repository checks to a dedicated self-hosted runner

Date: 2026-07-17
Status: Accepted; verified implementation squash-merged through PR #24 at
`eaf6c9f`; original active routing superseded by D-057, with its trust controls
reused and extended by D-058

Decision: assign the repository-specific `cortexa-ci` custom label to the
registered Linux x64 runner and require the exact
`[self-hosted, Linux, X64, cortexa-ci]` selector in CI, Documentation, and
Security. Preserve top-level `contents: read`, no secret context, immutable
actions, non-persistent checkout credentials, and the prohibition on workflow
writes or publication.

Self-hosted workflows must not subscribe to `pull_request`. Eligible repository
pushes are limited to `main`, `codex/**`, `feature/**`, `fix/**`, `refactor/**`,
`meta/**`, and `phase*/**`; scheduled security checks and explicit dispatch
remain eligible. Fork and dependency-bot PRs must be reviewed and reproduced on
a maintainer-controlled allowlisted branch. The runner host is dedicated,
unprivileged, and provisioned outside workflow execution; workflows perform
fail-fast prerequisite checks and do not use `sudo`.

D-053 was published later through the separately reviewed Increment 4V /
ARB-001 PR #23. D-054 was published first to prevent a decision-number
collision.

Rationale: the existing workflows name GitHub-hosted images and cannot match
the registered runner. GitHub-hosted jobs are currently not starting because of
an account billing or spending-limit restriction. Exact custom-label routing
restores a usable verification path while the trigger allowlist, no-secret
posture, and dedicated host limit persistent-runner exposure. An in-workflow
PR-author condition is explicitly insufficient because the PR can modify its
own workflow definition.

Consequences:

- The persistent runner remains a trust risk and is not equivalent to an
  ephemeral clean virtual machine. A trusted writer can alter workflow code;
  repository access and workflow review remain security controls.
- Fork and dependency-bot pull requests receive no self-hosted checks and must
  be reproduced on a maintainer-controlled allowlisted branch before merge.
- Linux verification does not replace target-Mac native, signing, notarization,
  installer, or release evidence.
- Replacing or reregistering the runner requires reapplying `cortexa-ci`.
- The separately approved portability correction target-gates only private
  native decision-source support in `approvals::manager` and
  `approvals::types`; public contracts and target-Mac behavior are unchanged.
- No runtime behavior, dependency, Tauri boundary, capability, permission, CSP,
  identifier, or SQLite schema changes.

## D-055 - Organize copy-paste prompts by responsibility

Date: 2026-07-17
Status: Accepted

Decision: organize the repository prompt library under `prompts/increments/`,
`prompts/reviews/`, `prompts/workflows/`, and `prompts/templates/`. Keep
`.agents/skills/` as named Codex procedures, `docs/workflows/` as human-readable
runbooks, and `docs/templates/` as repository artifact templates. The prompt
library is a copy-and-paste fallback and does not override repository authority,
grant approval, or prove verification.

Every executable prompt and prompt-authoring template uses one human-readable
Markdown metadata block containing title, category, purpose, use and non-use
conditions, required inputs, expected outputs, related skills, related prompts,
and last-reviewed date. No custom metadata parser or dependency is introduced.
Shared project rules remain in `AGENTS.md`, `ENGINEERING_GUIDE.md`, `SECURITY.md`,
and accepted decisions; prompts cross-reference those authorities and retain
only task-specific restrictions.

The former start and resume prompts are one start-session workflow with an
explicit mode. The former end-session and post-increment prompts are one
end-session workflow that still invokes the quality and post-increment skills.
The troubleshooting prompt becomes the bounded bug-fix increment prompt. All
other useful prompts move to their responsibility category. Historical plans,
reviews, backups, and dated handoff evidence retain old paths; active references
must use `prompts/README.md` and the categorized destinations.

Rationale: the flat library mixed implementation authorization, non-mutating
reviews, operating procedures, and authoring skeletons. Categorization and
consistent metadata improve discovery and reduce duplicated instructions while
preserving the authority and operational detail of existing skills and
runbooks.

Consequences:

- Prompt additions must represent a distinct responsibility and use lowercase
  kebab-case names.
- Long shared rule blocks are replaced by references to authoritative files.
- Old active prompt paths are invalid after migration; dated historical
  references remain intentional evidence.
- Prompt templates do not replace the document templates under `docs/templates/`.
- No skill, hook, application source, product behavior, dependency, Tauri
  boundary, persistence, capability, permission, CSP, or credential changes.

## D-056 - Use risk-based validation and one stable completion gate

Date: 2026-07-18
Status: Accepted

Decision: use the smallest affected verification during implementation, batch
related edits before expensive checks, and avoid rerunning successful checks
unless relevant content changed or policy requires a rerun. After the final
relevant edit, run one completion-gate sequence selected by change class.

Documentation-only work uses Git status, diff, Markdown, internal-link,
referenced-path, and protected-scope validation without unrelated frontend or
Rust tests and application builds. Isolated frontend and Rust changes use their
respective checks. IPC, storage, SQLite, policy, approval, security, dependency,
Tauri-configuration, release, and other cross-boundary changes require the
complete `npm run verify` suite plus applicable manual evidence. An approved
plan or stricter security or release policy may add checks.

Rationale: repeatedly running the complete suite after unrelated intermediate
edits adds latency without improving evidence. Focused feedback during
implementation catches local regressions earlier, while one stable final gate
preserves complete evidence where the change's risk and blast radius require
it.

Consequences:

- Completion reports must classify every applicable check accurately and give
  the risk-based reason for checks recorded as Not run.
- A partial suite cannot be called complete when the change class or approved
  plan requires `npm run verify`.
- Relevant edits after a successful completion check invalidate that check and
  require the affected verification to run again.
- The mandatory post-increment gate, trust boundaries, strict compiler and
  linter settings, and manual target-platform requirements remain unchanged.
- The preserved stash containing an earlier draft is not applied because it
  also contains stale Meta Increment 8 state.

## D-057 - Use risk-based GitHub-hosted validation for untrusted changes

Date: 2026-07-18
Status: Accepted historical design; active routing superseded by D-058 after
GitHub rejected both PR #30 jobs before allocation because the account Actions
minute or spending limit was exhausted

Decision: replace the three D-054 persistent-runner workflows with two
read-only workflows on ephemeral GitHub-hosted `ubuntu-latest` runners.
Documentation validates Markdown, prompts, project memory, links, paths, YAML,
and repository governance without application builds. Application CI uses a
standard-library classifier over fixed Git SHAs to select frontend, Rust, and
dependency-audit jobs. Unknown non-documentation paths fail closed to both
application jobs. Security-sensitive paths also select the audit job.

Consolidate the former weekly Security workflow into CI's dependency-audit job.
Preserve D-025's exact two-vulnerability RustSec baseline and D-046's exact
18-warning baseline, immutable official action SHAs, `contents: read`, disabled
checkout credential persistence, no repository secrets, concurrency
cancellation, bounded timeouts, and the prohibition on repository writes,
publication, deployment, signing, and notarization.

The workflows run for pull requests targeting `main`, pushes to `main`, and
explicit dispatch; CI retains the weekly audit schedule. Event-level path
filters prevent documentation-only changes from starting Application CI.
Classifier rules, workflow paths, focused fixtures, and the testing matrix must
change together when repository structure or boundary ownership changes.

Rationale: D-054 restored checks during a GitHub-hosted runner availability
problem, but a persistent machine is an inappropriate default for untrusted
pull-request code and forced the full suite for unrelated changes. Ephemeral
hosted runners restore pull-request validation while D-056's risk classes avoid
unnecessary compilation and preserve fail-closed behavior for ambiguous paths.

Consequences:

- D-054 remains historical and rollback evidence. The registered self-hosted
  runner is not selected by active workflows and requires a separate security
  decision before reuse.
- GitHub Actions does not replace the local final increment gate or target-Mac
  native, signing, notarization, installer, and release evidence.
- Path-filtered workflows are conditional and cannot be represented as
  universal branch-protection checks because a skipped required workflow may
  remain pending. Require each applicable job during review and use manual
  dispatch plus the local gate for ambiguous scope.
- Remote rulesets, branch protection, billing, runner availability, and hosted
  execution are not proven by local YAML or command validation. Hosted results
  remain pending until publication and actual GitHub runs.
- The official checkout and Node setup actions are pinned by full commit SHA.
  No application source, product behavior, dependency version, lockfile, Tauri
  boundary, capability, permission, CSP, identifier, or SQLite schema changes.

## D-058 - Route risk-based validation across dedicated Linux and macOS runners

Date: 2026-07-18
Status: Accepted; implementation and closeout squash-merged through PR #30 at
`1780d7f`; branch and post-merge verification passed

Decision: preserve D-057's two-workflow risk classification and consolidated
dependency audit, but route eligible jobs to the two registered
repository-specific self-hosted runners. The exact Linux selector is
`[self-hosted, Linux, X64, cortexa-ci]`; the exact target-Mac selector is
`[self-hosted, macOS, X64, cortexa-ci]`. Linux owns classification, repository
policy, documentation, frontend, Linux Rust, and dependency-audit jobs.
Rust-classified changes also run strict Clippy and all Rust targets on macOS so
target-gated native code is compiled and tested.

The workflows must not subscribe to `pull_request` or `pull_request_target`.
Eligible pushes are limited to `main`, `codex/**`, `feature/**`, `fix/**`,
`refactor/**`, `meta/**`, and `phase*/**`; CI retains its weekly dependency
audit schedule and both workflows retain explicit dispatch. Fork,
external-contributor, and dependency-bot changes must be reviewed and
reproduced on a maintainer-controlled allowlisted branch before either
persistent runner executes them.

Preserve top-level `contents: read`, no secret context, immutable official
action SHAs, disabled checkout credentials, fixed Git commands and validated
SHAs/paths, concurrency cancellation, bounded timeouts, no `sudo`, and the
prohibition on repository writes, publication, deployment, signing, or
notarization. Host packages and runner services are provisioned outside
workflow execution under dedicated unprivileged accounts.

Rationale: the D-057 workflows were locally valid, but GitHub rejected both
PR #30 hosted jobs before runner allocation because the account Actions minute
or spending limit was exhausted. Reusing the registered runners restores
validation without weakening D-056's risk-based selection. Adding the macOS
job preserves Linux portability checks while compiling target-gated native Rust
on the product platform.

Consequences:

- Push-triggered CI run `29670565671` and Documentation run `29670565657`
  passed for `9a2c75d`. Linux runner 21 executed classification,
  documentation, frontend, Linux Rust, and dependency audit; macOS runner 22
  executed target-Mac Rust. The run listing for that commit contains only the
  two push-triggered workflows.
- Documentation closeout commit `da08573` passed Documentation run
  `29671289962` before PR #30 was squash-merged at `1780d7f`. Post-merge CI run
  `29672575232` and Documentation run `29672575254` passed with the same exact
  Linux and macOS assignments.
- Persistent runners are not ephemeral security boundaries. Trusted writer
  access, workflow review, host isolation, patching, workspace cleanup, and
  incident response remain mandatory controls.
- Pull-request checks are produced only after a reviewed commit is pushed to an
  allowlisted repository branch. Missing checks on an untrusted PR are never
  approval to merge.
- Linux and target-Mac Rust checks complement each other. Neither proves native
  UI behavior, signing, notarization, installer behavior, or release readiness.
- D-054 and D-057 remain dated historical and rollback evidence. Returning to
  hosted runners after billing or minute availability is restored requires a
  separate reviewed routing change.
- No application source, product behavior, dependency version, lockfile, Tauri
  boundary, capability, permission, CSP, identifier, or SQLite schema changes.

## D-059 - Use trigger-bound dispositions for unreachable High findings

Date: 2026-07-19
Status: Accepted

Decision: assess High-severity findings against currently reachable behavior
and active trust boundaries before selecting remediation. A High finding may
remain deferred without a severity reduction only when the affected capability
is absent or unreachable, no current boundary is weakened, the current safe
increment remains verifiable, and an explicit future trigger and acceptance
criteria remain recorded. Missing Executor, complete Workflow, durable product
data, and enterprise controls are future product capabilities, not defects to
implement inside this remediation.

Use reversible secure defaults while owner decisions remain open. O-006 and
O-007 remain decision-required and block all live model networking. The
temporary repository posture is proprietary and all rights reserved without an
open-source license grant. The provisional tested platform baseline is macOS
14+ on Apple Silicon; Intel and older macOS support remain unclaimed. Signing
and notarization block trusted public distribution, not unsigned local
development. No vendor, identity provider, token issuer, legal license,
credential owner, release authority, or signing owner is selected by this
decision.

Rationale: implementing absent capabilities merely to reduce an advisory count
would expand scope and create unreviewed authority. Trigger-bound dispositions
preserve visible High risk, prohibit premature networking and release claims,
and identify the exact point at which each decision or capability becomes
mandatory.

Consequences:

- ARB-001 remains `RESOLVED` with its published Increment 4V evidence.
- ARB-002 is `DECISION REQUIRED`; O-006 and O-007 must be approved before live
  gateway or provider traffic.
- ARB-003, ARB-004, ARB-005, and ARB-008 are
  `BLOCKED - FUTURE CAPABILITY`; they are not implemented by this remediation.
- ARB-006 and ARB-007 are `DEFERRED - NON-BLOCKING` for current private local
  development and retain High severity with explicit distribution and release
  triggers.
- ARB-044 remains `SUPERSEDED` by its canonical split findings.
- No finding is recorded as accepted temporary risk, and no unresolved finding
  is represented as resolved.
- This decision changes documentation only and grants no network, execution,
  persistence, enterprise, distribution, signing, or release authority.

## D-060 - Separate identity, hosting, and AI-provider boundaries

Date: 2026-07-19
Status: Accepted; exact identity and AI-provider configurations remain approval-bound

Decision: Cortexa AI will use an Azure-first, provider-neutral architecture.
The following identity-provider, cloud-hosting, and AI model-provider boundaries
are separate. Selecting or approving one does not select or approve another.

**Current state:** no gateway is deployed, no networking is enabled, no
identity provider is integrated, no cloud deployment exists, and no AI
model-provider networking exists. Only synthetic test data may be considered by
a separately approved future transport test.

**Identity providers:** the application boundary will be pluggable OAuth 2.0
and OpenID Connect. Phase 1 may support Microsoft, Google, and Apple consumer
sign-in through a system browser using Authorization Code Flow with PKCE. These
are candidates, not enabled configurations. Phase 2 may support Microsoft Entra
ID workforce SSO and other separately approved enterprise OIDC or SAML identity
providers. AWS accounts and Google Cloud accounts are not treated as consumer
identity systems; future support concerns their associated standards-based
identity services or an organization's approved identity provider.

The configured identity provider determines the token issuer. The Cortexa
gateway must validate every configured trusted issuer, audience, signature,
expiration, tenant context when applicable, and authorization context against a
closed server-owned configuration. A user, WebView, model, desktop content, or
arbitrary runtime configuration may not select an issuer, authorization
endpoint, token endpoint, audience, client identity, tenant rule, or gateway
origin. Exact providers, issuers, redirect URIs, scopes, account-linking rules,
and tenant policies require separate approval before implementation.

A gateway access token must be audience-bound to the Cortexa AI gateway, have a
maximum 15-minute lifetime, and exist only in trusted Rust process memory. A
refresh or session credential belongs only in platform-secure credential
storage: macOS Keychain for the macOS application and an equivalent separately
reviewed facility on any future supported platform. Neither credential may
enter the WebView, SQLite, application logs, or ordinary CI.

**Cloud hosting:** Microsoft Azure Container Apps in Central US is the primary
planned hosting platform and region. The reserved production origin is
`https://api.cortexaai.io`. No Azure resource or gateway currently exists, and
the origin must not be described as active or approved until DNS, TLS,
deployment, authentication, authorization, logging restrictions, and security
verification all pass. Initial production will use one primary cloud.

The gateway should remain containerized and portable enough for future AWS or
Google Cloud deployment. Those deployments are deferred until customer,
data-residency, resilience, or commercial requirements justify them. This
decision does not define active-active multicloud, require three-cloud
deployment, promise cloud failover, or approve a second cloud.

**AI model providers:** the future gateway and a future trusted
`AgentProvider` abstraction may support multiple separately approved AI model
providers. No `AgentProvider` implementation currently exists; Increment 4K
deleted the legacy generic provider scaffold. Provider selection must occur in
the trusted gateway or trusted core according to closed policy. The desktop
must never receive an AI provider credential. Each provider requires separate
approval for retention, ZDR, data use, logging, region, and security under
D-061 before it may receive real user content.

AI provider credentials are owned by Henry Dang, Founder & Principal Engineer,
Cortexa AI. For the initial Azure target they belong only in Azure-managed
gateway secret storage; any future cloud requires an equivalent separately
reviewed managed secret facility. Credentials are never distributed to the
desktop application or stored in the repository, SQLite, application logs, or
ordinary CI. Henry Dang is also the operations and incident-response owner.
HTTPS is the only permitted future gateway transport.

Phase 1 targets individual consumers, consultants, IT professionals,
small-business owners, and other professional power users through individual
accounts, simple onboarding, and personal workspaces. Phase 2 may add
organization accounts, team workspaces, centralized billing and administration,
role-based access control, organization policy and audit, workforce SSO,
tenant-aware authorization, and group-based controls. Phase 1 has no enterprise
tenant administration, SCIM, enterprise policy administration, or
organization-wide deployment control.

Rationale: identity federation, deployment topology, and AI provider routing
have different trust, privacy, availability, and ownership boundaries. Keeping
them separate prevents an Azure-first deployment from becoming Entra-only,
prevents identity support from implying cloud support, and prevents one AI
provider's approval from being inherited by another.

Consequences:

- Azure Container Apps, Central US, the Cortexa-operated gateway, and the
  reserved origin are selected initial targets, not deployed facts.
- O-006 remains decision-required for exact Phase 1 identity configurations,
  any later cloud expansion, and every AI model-provider approval.
- Entra workforce identity is a Phase 2 enterprise target, not the exclusive
  Phase 1 consumer identity mechanism.
- Container portability is a design constraint, not active-active multicloud or
  a three-cloud release requirement.
- No networking, cloud resource, DNS, credential, OAuth/OIDC flow, PKCE,
  Keychain adapter, token validation, identity federation, `AgentProvider`, AI
  provider integration, team workspace, enterprise administration, SAML, or
  SCIM implementation is authorized.
- D-021's gateway, credential, token-lifetime, and trust-boundary rules remain
  authoritative and compatible with this staged selection.

## D-061 - Require verified ZDR and bounded external-processing disclosure

Date: 2026-07-19
Status: Accepted; provider ZDR verification remains a live-traffic gate

Decision: no external model processing is currently permitted. For each AI
model provider, provider-approved Zero Data Retention must be verified for the
exact production organization, project, endpoint, model, and region
configuration. Until that provider-specific evidence exists, only synthetic
test data may be considered for a separately approved future transport test;
real user content remains prohibited. After ZDR is verified, the initial
permitted real-user data class is limited to explicitly submitted,
non-sensitive text.

Credentials and secrets, attachments, regulated data, financial or healthcare
data, and sensitive personal data remain prohibited in both the consumer and
enterprise phases. Gateway logging is limited to operational metadata with a
maximum seven-day retention. Content logging is prohibited. External-processing
disclosure is required before the first external transmission and must remain
visible in Settings.

Henry Dang, Founder & Principal Engineer, Cortexa AI, is the privacy owner and
security owner. ZDR must not be claimed from `store: false` or intended
configuration alone; provider approval and exact account/project evidence are
required. A later implementation must also verify endpoint and model
eligibility, deletion procedures, allowed metadata fields, access controls, and
the disclosure presentation before any external transmission.

Rationale: provider retention controls and product logging are separate
boundaries. Requiring verified ZDR, a narrow initial data class, explicit
disclosure, and content-free operational logs prevents an implementation plan
from treating configuration intent as privacy evidence.

Consequences:

- O-007's product policy and accountable owners are decided, but no current or
  future live traffic is authorized by this record and no provider inherits
  another provider's approval.
- Synthetic-only testing remains the maximum pre-ZDR data boundary and still
  requires a separately approved transport plan and disclosure gate.
- Real user content remains blocked until provider-approved ZDR is verified;
  verification is an operational prerequisite, not a documentation claim.
- ARB-002 remains High and unresolved because O-006 identity and AI-provider
  configurations, deployment, threat modeling, implementation, and operational
  verification remain incomplete.
- No provider connectivity, account configuration, disclosure UI, logging,
  retention job, deletion process, credential, or product behavior is added.

## D-062 - Select Microsoft personal identity for Phase 1

Date: 2026-07-19
Status: Accepted; configuration evidence and implementation remain approval-bound

Decision: Microsoft personal identity is the sole Phase 1 identity provider.
Phase 1 accepts personal Microsoft accounts only through the `/consumers`
authority; work, school, guest, and arbitrary Entra tenants remain outside the
initial boundary. Google is deferred until demonstrated demand after Microsoft
verification. Sign in with Apple is deferred until Mac App Store planning or
demonstrated demand. These deferrals do not remove D-060's provider-neutral
application boundary or approve a later provider implicitly.

The planned desktop flow uses the system browser and OAuth 2.0 Authorization
Code Flow with PKCE S256, one-time `state`, and OIDC `nonce`. It uses separate
registrations for the public desktop client and the Cortexa gateway API
resource. The gateway validates the exact personal-account issuer from the
approved OIDC discovery metadata, tenant, signature, expiration, audience,
delegated scope, and authorization context. A closed loopback callback targets
`127.0.0.1` on an ephemeral port; the exact registered representation, client
identifier, gateway application identifier, issuer, audience, redirect, and
scope values remain pre-implementation evidence and must not be invented or
stored as secrets in the repository.

Initial scopes are `openid`, `email`, and one exact delegated Cortexa gateway
scope. `profile`, Microsoft Graph, directory, group, mail, calendar, file, and
contact scopes are excluded. `offline_access` is excluded until a separate
persistent-session decision. No refresh or session credential is authorized by
this record.

The canonical external account key is provider ID plus normalized issuer plus
subject. Email is optional contact data, not an identity key, and matching
email addresses never automatically link accounts. A future explicit
cross-provider linking flow requires a separate threat model, reauthentication
of both identities, collision handling, user confirmation, and redacted audit
design.

Rationale: Microsoft personal identity supports the accepted system-browser
OIDC and PKCE boundary, can issue an access token for a separately registered
Cortexa gateway API resource, and preserves a bounded path to a separately
approved Phase 2 Entra workforce design. Personal-account-only authority avoids
premature multitenant issuer and enterprise-policy complexity. Minimal scopes,
stable subject-based identity, and no automatic email linking reduce privacy
and account-takeover risk.

Consequences:

- O-006's Phase 1 provider choice is decided, but exact registration,
  issuer/audience, redirect, account-lifecycle, and threat-model evidence remains
  required before an identity implementation can be planned.
- D-060's identity, hosting, and AI-provider boundaries remain independent.
  Microsoft identity selection does not approve Azure deployment, gateway
  networking, or an AI model provider.
- D-061 still blocks external transmission until the selected AI provider's
  exact ZDR, retention, logging, region, deletion, disclosure, and security
  evidence passes.
- ARB-002 remains High, unresolved, and not Ready. No account, registration,
  OAuth/OIDC, redirect, token, Keychain, gateway, networking, or provider path
  is implemented or authorized.
- Google requires a later demand and gateway-session decision. Apple requires a
  later native-versus-web identity decision and must be revisited before any Mac
  App Store plan that triggers its login-service requirements.

## D-063 - Select Azure OpenAI as the Phase 1 AI-provider candidate

Date: 2026-07-19
Status: Accepted for synthetic evaluation only; deployment and real-content approval remain blocked

Decision: Azure OpenAI in Microsoft Foundry is the sole Phase 1 AI
model-provider candidate. The planned Cortexa gateway may call one
Standard/Regional Azure OpenAI deployment in Central US through the Responses
API. The initial synthetic-evaluation model candidate is `gpt-5.1`, version
`2025-11-13`; that model and version are not production commitments and must be
revalidated when an exact resource and deployment are approved.

The Azure Container Apps gateway must authenticate to Azure OpenAI with a
managed identity and least-privilege Azure RBAC. API-key authentication is not
approved. Provider credentials or tokens must never reach the desktop,
WebView, SQLite, repository, logs, ordinary CI, or user-controlled
configuration.

The first provider contract is foreground Responses streaming with
`store: false`, `background: false`, strict custom functions, and parallel tool
calls disabled. Files, retrieval, Assistants, Agents, Batch, stored
completions, hosted tools, web search, MCP, code execution, and response
retrieval are excluded. Provider errors and events must be normalized by the
gateway before they cross into trusted Rust. No automatic fallback to direct
OpenAI or another provider is permitted.

This selection does not satisfy D-061. Real user content remains prohibited
until the exact Azure subscription, resource, endpoint, deployment, model,
version, region, and commercial agreement have Microsoft-approved retention
evidence; the resource reports `ContentLogging=false`; the exact stateless
Responses configuration has documented no application-state retention; and
logging, deletion, access, disclosure, security, and operational evidence all
pass. Until then, only synthetic test data may be used under a separately
approved future transport plan.

Rationale: Azure OpenAI aligns with the accepted Azure-first gateway target and
supports managed identity, Azure RBAC, regional deployment, and resource-level
abuse-monitoring evidence. A provider-neutral gateway contract preserves a
later direct OpenAI or other approved provider adapter without distributing
credentials or provider-specific authority to the desktop.

Consequences:

- O-006's Phase 1 AI-provider choice is decided, but no Azure resource,
  deployment, endpoint, credential, network path, or `AgentProvider` exists.
- D-061 remains an operational live-traffic gate; policy intent, `store:
false`, or this decision record is not ZDR evidence.
- Direct OpenAI and other providers are deferred until separately justified
  and independently approved under D-061. There is no silent or automatic
  cross-provider fallback.
- ARB-002 remains High, unresolved, and not Ready. Exact identity evidence,
  deployment and threat-model evidence, disclosure, implementation, and
  operational verification remain incomplete.
- This record changes documentation only and grants no cloud, networking,
  identity, credential, Keychain, provider, or runtime authority.

## D-064 - Stage gateway evidence and close the Phase 1 configuration

Date: 2026-07-19
Status: Accepted design boundary; provisioning, transport, and real-content activation remain separately approval-bound

Decision: resolve the circular dependency between pre-implementation design
and post-provisioning evidence by separating ARB-002 into four explicit stages:

1. **Stage A - design:** the documentation-only ARB-002A threat model and
   closed configuration specification.
2. **Stage B - no-traffic provisioning:** separately approved creation of the
   intended Microsoft registrations and Azure resources with traffic disabled,
   followed by sanitized configuration and ownership evidence.
3. **Stage C - synthetic-only transport:** separately approved implementation
   and security verification using only owner-approved synthetic text after
   external-processing disclosure and acknowledgement.
4. **Stage D - real-content activation:** separately approved activation only
   after exact provider-approved ZDR, `ContentLogging=false`, stateless
   Responses, retention, deletion, disclosure, operational, and security
   evidence passes.

No stage grants authority for or automatically starts the next stage. Missing,
expired, failed, or mismatched evidence keeps dependent stages blocked.

The closed Phase 1 identity configuration uses Microsoft personal identity,
separate public desktop and gateway API registrations, an API Application ID
URI in the form `api://<gateway-api-client-id>`, one delegated scope named
`gateway.access`, and a loopback callback on `127.0.0.1`, an
operating-system-assigned ephemeral port, and `/oauth/callback`. The planned
flow retains D-062's system-browser Authorization Code Flow, PKCE S256,
`state`, `nonce`, minimal scopes, maximum 15-minute gateway token, no
`offline_access`, and provider-plus-issuer-plus-subject account key. Actual
registration identifiers and observed token claims are Stage B and Stage C
evidence, not repository constants. Any difference from the closed audience or
callback fails closed and requires an additive decision.

Microsoft currently documents a default access-token lifetime of approximately
60 to 90 minutes and manifest-specific handling for HTTP `127.0.0.1` redirects.
Neither is represented as satisfying Cortexa's accepted 15-minute gateway-token
maximum or ephemeral callback until Stage B and Stage C evidence passes. If the
accepted values cannot be enforced for Microsoft personal accounts, a later
additive architecture decision is required; there is no silent lifetime or
redirect fallback.

The closed Azure configuration uses one Cortexa-operated Azure Container Apps
gateway in Central US, the reserved inactive
`https://api.cortexaai.io` origin, one dedicated non-shared user-assigned
managed identity, `Cognitive Services OpenAI User` at the exact Azure OpenAI
resource scope, a private Azure OpenAI endpoint, and disabled Azure OpenAI
public network access before any provider traffic. API-key fallback,
subscription- or resource-group-wide runtime roles, caller-selected endpoints,
unrestricted egress, alternate provider origins, secondary-cloud failover, and
automatic provider fallback are prohibited.

External-processing disclosure requires an explicit versioned acknowledgement
before the first external transmission, including synthetic transmission, and
after a material provider, retention, data-classification, or disclosure
change. Exact user-facing copy and acknowledgement storage remain Stage C
evidence. Operational identifiers and evidence belong in a restricted
owner-approved evidence store; the repository contains only sanitized
references and no credentials or tokens.

The authoritative detail is in
`docs/security/phase4-gateway-configuration-spec.md` and
`docs/security/phase4-gateway-threat-model.md`. D-060 through D-063 remain
authoritative and are not superseded.

Rationale: identity registration, resource provisioning, transport proof, and
provider retention evidence cannot all exist before implementation. Explicit
stages let the project approve secure defaults without fabricating operational
proof, while requiring exact evidence before each newly reachable boundary.

Consequences:

- ARB-002A closes only the documentation design stage. It creates no current
  authentication, cloud, network, provider, disclosure, or data path.
- ARB-002 remains High and unresolved. Stage B, Stage C, and Stage D each need
  a bounded plan, project-owner approval, verification, and rollback.
- Stage C permits only synthetic data and still requires disclosure before its
  first transmission. Stage D remains blocked by D-061 evidence for the exact
  intended production configuration.
- An actual registration ID, subscription, resource, endpoint, deployment,
  model/version, DNS record, certificate, or evidence-store location must not
  be invented in repository documentation.
- Exact disclosure copy, registration/resource evidence, deployment-time model
  availability, provider approval, and operational verification remain future
  prerequisites rather than accepted facts.
- The maximum 15-minute gateway-token lifetime and approved IP-literal
  ephemeral callback remain hard Stage C blockers until compatible Microsoft
  configuration and target-Mac behavior are evidenced.
- No identity client, OAuth/OIDC flow, loopback listener, token, Keychain
  integration, Azure resource, DNS, credential, networking, `AgentProvider`,
  provider call, disclosure UI, runtime behavior, or ARB-002 implementation is
  authorized.

## D-065 - Establish the Codex instruction hierarchy

Date: 2026-07-20
Status: Accepted repository-governance policy

Decision: use root `AGENTS.md` as the concise automatically loaded repository
entry point and `docs/governance/MASTER_PROMPT.md` as the detailed,
maintainable repository-wide Codex instruction layer.

Instruction precedence is platform and system instructions, root `AGENTS.md`,
the master prompt, current project-state records, the approved task-specific
skill/prompt/plan, then future directory-level `AGENTS.md` files. This
instruction ordering does not replace the documentation-authority ordering in
`ENGINEERING_GUIDE.md`.

All active reusable prompts must begin by requiring root `AGENTS.md` and the
master prompt, then preserve their task-specific goals, scope, validation, and
stop conditions. The existing `prompts/README.md` remains the prompt index;
no prompt is deleted, renamed, or relocated by this decision.

The master prompt defines global scope, validation, security, Git, handoff,
future multi-agent, and final-response guidance. It requires an advisory
model-and-effort recommendation based on the next proposed work. GPT-5.6 Terra
is preferred for documentation and planning and GPT-5.6 Sol for implementation
when available; otherwise Codex must name the closest available fallback. The
recommendation does not switch models and must not recommend Ultra while
multi-agent/subagent execution remains outside approved scope.

Future multi-agent concepts may be documented only. This decision authorizes no
product orchestration, subagent execution, agent communication, workflow engine,
background worker, database, provider integration, cloud service, IPC, UI,
dependency, hook, or model-routing automation.

Consequences:

- Root instructions remain concise while essential safety, destructive-Git, and
  validation restrictions stay directly available.
- Detailed global rules are maintained once in the master prompt and reference
  authoritative architecture, security, testing, and project-state documents.
- The approved product roadmap is preserved. The hierarchy does not authorize
  Stage B, Stage C, Stage D, ARB-002 runtime work, or another product increment.
- A missing or unavailable preferred model requires a transparent fallback
  recommendation, not an unsupported claim about the active model.

## D-066 - Select OpenAI for a synthetic-only demo

Date: 2026-07-20
Status: Accepted documentation-only provider decision

Decision: supersede D-063's unpublished Azure provider direction with OpenAI as
the sole candidate for a future synthetic-data-only demo. A future trusted
gateway owns any credential; the desktop and WebView never receive it. No API
call, account, key, external transmission, production data, tool execution,
automatic fallback, or multi-provider routing is authorized by this decision.

Consequences: the Azure Stage B plan is superseded before publication. A later
implementation requires exact OpenAI data-control evidence, owner-approved
synthetic corpus, disclosure, fixed limits, redacted errors, security tests,
and a separate project-owner approval.

## D-067 - Select Cloudflare Workers for the internal synthetic-demo gateway

Date: 2026-07-20
Status: Accepted documentation-only deployment decision

Decision: select Cloudflare Workers Free as the sole remote gateway candidate
for the internal, owner-only OpenAI synthetic demo. A future OpenAI API key may
exist only as a Cloudflare Worker secret. This does not select client-to-Worker
authentication or authorize a Worker, DNS route, secret, deployment, provider
request, Keychain credential, or runtime code.

Consequences: Cloudflare is a demo-only boundary and does not replace the
historical planned Azure production architecture. Client authentication,
Worker route/domain, deployment, logging, rotation, rollback, and manual
security evidence require a separate approved plan. Automatic provider
fallback and prompt/response logging remain prohibited.

## D-068 - Permit one Cloudflare Access service token for the internal demo

Date: 2026-07-20
Status: Accepted documentation-only security exception

Decision: permit one Cloudflare Access service token, with a maximum 30-day
duration, to authenticate trusted Rust to one future internal-demo Access
application. Its Client Secret may exist only in macOS Keychain and may be read
only by trusted Rust. The future Worker must validate the Access JWT signature,
issuer, and exact audience. Immediate token revocation and route disablement are
mandatory rollback controls.

Consequences: this is a narrow owner-only, fake-data, non-production exception.
It does not change D-064's maximum 15-minute production gateway access-token
requirement or authorize a token, Keychain item, Access application, Worker,
route, DNS record, deployment, or provider traffic. There is no bypass, retry,
alternate credential, or provider fallback.

## D-069 - Accept the fake-only macOS Keychain proof and keep real ingestion blocked

Date: 2026-07-28
Status: Accepted local security-boundary evidence; real credential ingestion remains blocked

Decision: accept one local, fake-value-only macOS Keychain proof implemented in
trusted Rust. It may read only service
`io.cortexa.demo.cloudflare-access` with the fixed `client-id` and
`client-secret` accounts through exactly pinned macOS-only
`security-framework 3.7.0` and `security-framework-sys 2.17.0`. The public
boundary returns only `Available` or closed redacted errors. It exposes no raw
credential, write, update, delete, enumeration, arbitrary-label lookup, Tauri
command, WebView path, SQLite storage, startup integration, network client, or
runtime credential consumer.

Owner-operated target-Mac evidence used only two fake generic-password items.
It observed missing, cancelled/denied-as-cancelled, and available outcomes, then
removed both items and re-observed missing. Multiple login-keychain
authorization prompts were required. This does not prove stable app-specific
access for the unsigned development executable.

Consequences:

- The fake proof may complete with an advisory because it achieved its
  evidence goal and left no credential behind.
- Real service-token creation or ingestion remains blocked until a separately
  approved increment defines a stable signed identity or another narrowly
  reviewed app-specific access-control design, production-grade secret-memory
  handling, one-time owner transfer, rotation, revocation, and rollback.
- The pinned wrapper is MIT OR Apache-2.0, requires Rust 1.85 or newer, and has
  no build script. Its upstream maintenance metadata says
  `looking-for-maintainer`, so dependency health must be reassessed before real
  credential use.
- D-068's 30-day maximum remains demo-only. D-064's production 15-minute
  access-token maximum is unchanged.
- No real credential, Keychain item, Access application, Worker, route, DNS
  record, secret, deployment, provider request, traffic, or runtime behavior is
  authorized.

## D-070 - Require stable macOS credential controls before real demo ingestion

Date: 2026-07-28
Status: Accepted documentation-only readiness gate

Decision: a future real Cloudflare Access demo service-token ingestion proposal
requires a separately approved design and target-Mac evidence for stable signed
application identity or a narrowly reviewed app-specific Keychain ACL,
production-grade secret-memory lifecycle, direct owner-only transfer, rotation,
revocation, rollback, and dependency-health reassessment. Repeated prompts from
an unsigned development executable are not stable-access evidence.

Consequences: D-068's one-token, 30-day demo exception and D-064's 15-minute
production maximum are unchanged. This decision creates no credential,
Keychain item, application identity, entitlement, Cloudflare resource, provider
request, traffic, runtime behavior, or implementation authority.

## D-071 - Do not select macOS credential controls by documentation alone

Date: 2026-07-28
Status: Accepted documentation-only decision boundary

Decision: a future real-demo credential boundary must use either a stable,
owner-controlled signed application identity with least-privilege Keychain
access or a narrowly reviewed alternative app-specific ACL. A separate
owner-approved decision must select one model and define its scope, lifecycle,
revocation, update/reinstall behavior, and target-Mac evidence before any
implementation may begin. The secret-memory model must separately define
bounded ownership, one-time use, redaction, practical zeroization limits, and
failure closure.

Consequences: repeated prompts from an unsigned executable are not a selected
control and no unsigned fallback is permitted. This decision creates no signing
asset, certificate, entitlement, profile, Keychain item, credential, Cloudflare
resource, provider request, traffic, runtime behavior, or implementation
authority. D-064 and D-068 remain unchanged.

## D-072 - Select signed macOS identity for the future demo credential boundary

Date: 2026-07-28
Status: Accepted documentation-only owner decision

Decision: select one stable, owner-controlled signed macOS application identity
as the future credential control model. The narrow alternative Keychain ACL is
not selected. A later implementation must define least-privilege Keychain scope,
bounded secret-memory ownership, one-time consumption, lifecycle, private
target-Mac evidence, and signing provenance before handling a real credential.

Consequences: this decision does not create or authorize a certificate, signing
identity, profile, entitlement, notarization, Keychain item, credential,
Cloudflare resource, provider request, traffic, runtime behavior, or
implementation. Unsigned behavior remains prohibited as a fallback. D-064 and
D-068 remain unchanged.

## D-073 - Constrain the signed-identity secret proof to three existing Rust paths

Date: 2026-07-28
Status: Accepted documentation-only implementation boundary

Decision: a future fake-only signed-identity and bounded secret-memory proof may
change only `cloudflare_access.rs`, its public integration test, and its
status-only example. It must stop for new approval before adding any dependency,
manifest, lockfile, entitlement, profile, Tauri configuration, script, IPC,
startup, WebView, network, or runtime-consumer path.

Consequences: this boundary authorizes no implementation, signing, Keychain
action, credential, Cloudflare resource, provider request, traffic, or runtime
behavior. D-064 and D-068 remain unchanged.

## D-074 - Defer Apple Developer enrollment and conditionally prefer individual membership

Date: 2026-07-30
Status: Accepted documentation-only owner recommendation

Decision: Cortexa should not enroll in the Apple Developer Program now. Current
documentation, mock, and fake-only planning work requires no membership. If a
later separately approved owner-only fake-demo signing increment requires
membership while Cortexa remains personally owned by one individual or sole
proprietor, recommend individual enrollment. The individual enrollee is the
Account Holder and can control the relevant signing assets, subject to Apple's
then-current membership and certificate rules.

Re-evaluate before enrollment and prefer organization enrollment instead when a
legal entity should own the Apple agreement, seller identity, certificates, or
future team access. Do not assume a later individual-to-organization transition
is low-risk, automatic, or sufficient for company ownership; it requires a
separate ownership, distribution, certificate-control, and migration review.

Comparison:

| Factor                     | Individual                                                                                          | Organization                                                                                                                | Recommendation for current scope                                                        |
| -------------------------- | --------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------- |
| Ownership                  | One person is Account Holder and controls the membership relationship.                              | A legal entity owns the relationship through its authorized Account Holder.                                                 | Individual only while the owner-only demo remains personally owned.                     |
| Seller-name visibility     | Apps are listed under the person's name.                                                            | Apps are listed under the legal entity name.                                                                                | Defer enrollment now; select organization before a company seller identity is needed.   |
| Future team access         | Not the intended model for company-managed team ownership.                                          | Supports organization membership and delegated team access.                                                                 | Organization when contributors or business continuity require shared control.           |
| Certificate control        | Individual Account Holder may manage permitted signing assets.                                      | Entity governance can separate company ownership from an individual employee.                                               | Individual for a sole owner; organization for company-controlled assets.                |
| Cost                       | Apple currently lists the same Program membership price for individual and organization enrollment. | Apple currently lists the same Program membership price; organization eligibility has additional legal-entity requirements. | Cost does not decide the current choice.                                                |
| Migration risk             | A later business transition could require ownership and distribution changes.                       | Avoids an avoidable personal-to-company ownership gap when a legal entity already exists.                                   | Do not enroll individually for short-term convenience if company ownership is imminent. |
| Owner-only fake-demo scope | Can fit a sole owner's future limited proof.                                                        | Adds organizational prerequisites not needed for a personal proof.                                                          | Conditional individual preference only after separate approval.                         |

Evidence basis: Apple's current enrollment guidance permits individual or
organization enrollment, identifies the individual enrollee's legal-name seller
visibility, and lists a 99 USD annual membership price that may vary by region.
Apple's membership comparison identifies Developer ID and notarization as
Program benefits, while its certificate overview says an individual enrollee is
the Account Holder and that only an Account Holder or Admin can create
distribution certificates. See [Become a member](https://developer.apple.com/programs/enroll/),
[Choosing a Membership](https://developer.apple.com/support/compare-memberships/),
and [Certificates overview](https://developer.apple.com/help/account/certificates/certificates-overview/).

Consequences: this recommendation creates no membership, payment, agreement,
account access, certificate, signing identity, key, profile, entitlement,
notarization, Keychain item, credential, Cloudflare resource, provider setting,
provider request, traffic, deployment, runtime behavior, or implementation
authority. A free Xcode Personal Team is not accepted as the project's stable
signed-identity boundary. D-064's 15-minute production requirement and D-068's
30-day demo-only exception remain unchanged.

## D-075 - Select Developer ID Application for the future signed macOS identity proof

Date: 2026-07-31
Status: Accepted documentation-only identity-planning decision

Decision: if a later separately approved owner-operated signing increment is
needed for D-072's stable macOS application identity proof, select exactly one
owner-controlled **Developer ID Application** certificate. Do not select a
Developer ID Installer certificate, a Mac App Store distribution certificate,
or an Apple Development certificate for that future proof. The proof's future
purpose is a stable signed macOS application identity outside Mac App Store
distribution; it does not include an installer, App Store submission, advanced
capabilities, or product distribution.

Apple identifies Developer ID Application as the certificate that signs a Mac
app distributed outside the Mac App Store, and Developer ID Installer as the
separate certificate for installer packages. Apple also states that Developer ID
certificate creation requires the Account Holder role and a CSR, certificate
download, and local Keychain installation. These are future, sensitive actions,
not authorization for this decision. See [Developer ID certificates](https://developer.apple.com/help/account/certificates/create-developer-id-certificates/)
and [Certificates overview](https://developer.apple.com/help/account/certificates/certificates-overview/).

Consequences: a separate owner-approved operational increment must still define
the target application identifier, private-key generation and non-export policy,
private target-Mac evidence, certificate lifecycle, compromise response, and
the exact fake-only signing procedure. No Apple account access, certificate,
CSR, key, profile, App ID, entitlement, download, signing, notarization,
Keychain activity, credential, Cloudflare action, provider request, deployment,
traffic, code, dependency, or runtime behavior is created or authorized. D-064's
15-minute production maximum and D-068's 30-day demo-only exception are
unchanged.

## D-076 - Defer the signed macOS identity path after TS-017

Date: 2026-08-01
Status: Accepted documentation-only owner decision

Decision: defer the future signed macOS identity path selected by D-072 and
D-075. TS-017's owner-operated read-only observations confirmed only that the
configured user/default Keychain state was observable, zero valid code-signing
identities were present, no authorization prompt appeared, and no state change
was observed. They did not identify why Certificate Assistant could not create
the CSR. The cause remains `not determined`.

The owner considered and declined to initiate either of these future paths now:

1. An owner-operated Apple Support assistance path. It would require its own
   separately approved plan that limits account and diagnostic disclosure,
   private evidence, external coordination, stop conditions, and follow-up.
2. An alternate CSR workflow. It would require a separate design and owner
   approval proving that the private key is generated only on the target Mac,
   remains non-exported and owner-controlled, and preserves D-072's stable
   signed-identity boundary without bypassing macOS or Apple controls.

Consequences: do not repeat diagnostics, retry CSR creation, contact Apple
Support, repair or alter Keychain state, or select an alternate CSR mechanism.
The prior no-asset baseline remains: no CSR file, certificate, or new named
private key exists. No Apple Developer access, certificate, key, signing,
notarization, profile, App ID, entitlement, credential, Cloudflare resource,
provider request, deployment, traffic, code, dependency, or runtime behavior is
created or authorized. D-064's 15-minute production maximum and D-068's
30-day demo-only exception are unchanged.

## D-077 - Conditionally reopen Apple Support contact consideration after TS-017

Date: 2026-08-02
Status: Accepted documentation-only owner decision

Decision: retain D-076's deferral of the signed macOS identity path, but
conditionally reopen consideration of exactly one future owner-operated Apple
Support contact under
[`apple-support-ts-017-assistance-plan.md`](docs/plans/apple-support-ts-017-assistance-plan.md).
This narrow change supersedes only D-076's present decision not to consider an
Apple Support path. It does not authorize Apple Support contact, Apple Developer
access, a CSR retry, an alternate CSR workflow, or any signing-related action.

Before any contact, the owner must separately approve an operational increment
that names the existing assistance plan and limits the scope to one owner-only
contact for diagnostic guidance. The minimum sanitized summary, optional broad
macOS-version/architecture disclosure, no-screen-share/no-upload/no-device-access
boundary, no-execution rule, closed outcome categories, and no-state-change
rollback in that plan remain mandatory. No case number, transcript, screenshot,
account detail, Keychain detail, signing-material identifier, or repository
content may enter the repository or chat.

The prior filesystem CSR/private-key pair was owner-attested deleted and never
satisfied D-072. Ordinary deletion does not establish cryptographic erasure or
signing readiness. TS-017's cause remains `not determined`; D-072's stable,
owner-controlled signed-identity requirement remains unsatisfied. If the owner
does not separately approve contact, or if a contact triggers any prohibited
disclosure or state-changing recommendation, stop or return to D-076's full
deferral without acting on advice.

Consequences: no Apple Developer or Apple Support access, diagnostics, CSR or
key creation, Keychain action, certificate, signing, credential, Cloudflare,
provider, deployment, traffic, code, dependency, or runtime behavior is created
or authorized. D-064's 15-minute production maximum and D-068's 30-day demo-only
exception remain unchanged.

## D-078 - Adopt personal-project scope and conceptual runtime-adapter direction

Date: 2026-08-11
Status: Accepted owner project-direction and documentation-only architecture
decision

Decision: Cortexa is currently a private, personally owned, local-first side
project for one technical user, the repository owner. Its present uses are
personal productivity, experimentation, learning, development, and
demonstrations. It does not currently require SaaS infrastructure,
multi-tenancy, billing, enterprise IAM, public deployment infrastructure, or
production-scale distributed systems.

This present scope does not cancel D-060 or the accepted Phase 1 and Phase 2
future product targets. Cortexa may later be published or publicly distributed.
Clean application-owned and framework-neutral boundaries should preserve that
option, while every publication, commercial, cloud, enterprise, and distributed
capability remains deferred until an explicit owner-selected increment satisfies
its evidence, decision, security, and authorization gates.

Adopt this guiding principle:

> Build it with clean architecture. Scope it like a personal project. Preserve
> the path to a future product.

Preserve the verified native Rust boundaries, deterministic frontend mocks,
closed contracts, tests, documentation, and accepted decisions unless a later
task explicitly authorizes evidence-backed removal. This does not revive the
unused generic scaffolds removed in Increments 4I through 4M, including D-032's
legacy synchronous arbitrary-string `AgentProvider`.

For future planning only, an application-owned `AgentRuntime` seam may expose a
`NativeAgentRuntime` over the current typed native path and an optional
experimental `HermesAgentRuntime` adapter. The native path should remain the
default, an explicit fallback, the reference and deterministic test path, and a
possible standalone runtime. Hermes-specific types must stay inside its adapter.
OpenClaw may be evaluated later but is not selected or planned for
implementation. None of these runtime types currently exists.

Every runtime output remains untrusted. Deterministic Rust retains validation,
policy, exact approval, restricted execution, cancellation, and audit ownership.
The conceptual runtime seam is distinct from provider transport and grants no
network, dependency, credential, model, coordination, dispatch, execution,
multi-agent, permission, or device authority.

Consequences: `docs/PROJECT_DIRECTION.md` is the durable present-scope and
planned-direction guide below accepted decisions and current-state evidence.
Future external-runtime work requires its own bounded plan and explicit owner
approval. No production source, dependency, application behavior, current
capability, or readiness classification changes through this decision.
Previously accepted consumer, cloud, provider, enterprise, signing, and release
targets are neither canceled nor implemented.

## D-079 - Accept the application-owned multi-runtime agent architecture

Date: 2026-08-11
Status: Accepted owner architecture decision

Decision: Accept the small framework-neutral runtime target documented in
`docs/adr/ADR-MULTI-RUNTIME-AGENT-ARCHITECTURE.md`:

```text
AgentRuntime
├── NativeAgentRuntime
└── HermesAgentRuntime
```

`AgentRuntime` owns only a closed, bounded runtime descriptor, current-needed
capabilities, one run start boundary, untrusted events, terminal state, typed
errors, and exact idempotent cancellation. It does not own provider transport,
tools, policy, approval, restricted execution, audit, memory, storage, secrets,
platform access, Tauri IPC, or the WebView. Runtime output remains untrusted and
must traverse the existing deterministic Rust gates.

`NativeAgentRuntime` is implemented first by composing, not rewriting, the
verified `InitialGatewayTurn` and native typed boundaries. Native remains the
default, reference implementation, deterministic contract-test path, explicit
fallback, and possible standalone runtime. Fallback means a separately started,
explicitly selected run; it never means automatic cross-runtime/provider
failover or replay after an ambiguous result.

The native plan may use a deterministic test-only `MockAgentRuntime` to prove
the application-owned contract, failures, cancellation, and capability
discovery without network, Hermes, a model, or external I/O. The mock is test
infrastructure, not a production runtime or replacement for the visible
frontend deterministic mock.

`HermesAgentRuntime` remains optional, experimental, and separately blocked.
All Hermes process, protocol, configuration, version, event, and error types
must remain inside its adapter and translate to Cortexa-owned closed types. The
adapter may not revive D-032's deleted arbitrary-string `AgentProvider` or the
generic audit, memory, and platform scaffolds removed under D-030, D-033, and
D-034.

Consequences: the native runtime boundary plan is the sole next Ready
implementation plan, but a plan alone grants no editing authority and this
documentation increment adds no runtime code. Provider, process, UI, tool,
execution, persistence, credential, and external-runtime phases remain
independent. No later phase starts automatically. D-060 through D-064, D-065,
and D-078 remain intact.

## D-080 - Conditionally select managed local Hermes serve WebSocket transport

Date: 2026-08-11
Status: Accepted conditional evaluation decision; implementation Blocked

Decision: Preserve the completed raw TUI-gateway stdio NO-GO and conditionally
select a Rust-supervised managed local `hermes serve` child plus a closed
projection of the documented TUI-gateway JSON-RPC/WebSocket protocol for a
contained spike after the native runtime boundary is verified complete.

The canonical evaluated upstream is Hermes Agent package/application version
`0.20.0`, calendar release tag `v2026.8.3`, source commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`, released 2026-08-03. The package
version and tag are separate identifiers for the same tagged source artifact.
Mutable `main` or `latest` is not the compatibility target.

The spike must use only an explicit validated complete Hermes distribution,
`127.0.0.1`, an OS-assigned port, and a high-entropy per-launch backend token
sent only through `HERMES_DASHBOARD_SESSION_TOKEN` and the private
`/api/ws?token=...` query. It must use an isolated owner-only Hermes home and
working directory, a sanitized environment, a closed method/event projection,
strict byte/event/time limits, redaction, and whole-process plus descendant
containment. The containment must restrict Hermes egress to the exact local
fake-provider endpoint, deny every other network/Unix-socket destination, and
track and terminate descendants that detach into another session or process
group. It must prove readiness, liveness, version, one synthetic text-only
session, streaming, timeout reconciliation, cooperative interrupt plus bounded
status reconciliation through the pinned human-oriented
`Agent Running: Yes|No` line, mandatory `session.info`/`message.start` events,
crash handling, and clean/forced shutdown without a live credential or cloud
provider.

The spike may not install or update Hermes or enable tools, approvals, secrets,
MCP, memory, skills, plugins, subagents, schedules, messaging, shell,
filesystem, Git, clipboard, browser, cloud, or device actions. It must preflight
the exact `[web]` and POSIX `[pty]` extras, keep the distribution read-only, and
prove that the pinned lazy-dependency and update-check paths cannot execute a
package manager, mutate the artifact, or reach a network. Configuration and
allowlists are defense in depth, not containment. The spike must also enumerate
and deny the pinned distribution-root, isolated-home, machine-managed, and
configured external dotenv/managed-secret sources without reading their
contents. A forbidden method/event, host access, unapproved endpoint, Unix
socket, process escape, install/update or secret-source access attempt,
unbounded output, ambiguous terminal result, inability to prove tool absence,
or missing whole-distribution provenance makes the result NO-GO.

Raw TUI-gateway stdio remains rejected for the evaluated release. ACP remains a
documented deferred fallback and is neither selected nor implemented. The
contained spike is Blocked until D-079's native phase passes. The
`HermesAgentRuntime` adapter remains Draft/Blocked until both phases pass,
supported-version and packaging assumptions are recorded, containment is
demonstrated, and tool execution remains disabled.

Consequences: this decision selects an evaluation direction, not a production
adapter or current capability. It adds no source, dependency, executable,
process, socket, credential, provider request, UI, or behavior. A failed spike
preserves native-only operation and requires an additive ADR revision before
another transport can be selected.

Subsequent evidence: the owner-selected spike returned FAIL / NO-GO at
Milestone 0. Complete immutable runtime provenance, supported suppression of
update/credential/plugin/skill/privileged-tool initialization, and exact
target-Mac containment could not be established. The associated WebSocket ADR
is rejected as an implementation basis for the pinned release. No Hermes
server, socket, session, provider, credential, tool, or adapter was started.

## D-081 - Reject Hermes ACP for the pinned release and retain native-only operation

Date: 2026-08-11
Status: Accepted owner evaluation outcome

Decision: Reject Hermes ACP as the transport for an experimental
`HermesAgentRuntime` at Hermes Agent package/application version `0.20.0`, tag
`v2026.8.3`, commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`.

ACP is materially better than the rejected raw TUI-gateway stdio mechanism as
a wire contract: it has a supported public launcher, newline-delimited JSON-RPC
stdio, protocol/version initialization, structured session methods and update
events, cancellation, and stdout/stderr separation. It also avoids the
listener, token, and dynamic-port concerns of the rejected `hermes serve`
WebSocket path.

Those advantages do not satisfy Cortexa's authority boundary. Pinned ACP
session construction hardcodes Hermes's broad `hermes-acp` toolset, including
terminal/process, filesystem mutation, browser, memory, skills, code execution,
and delegation. Tool progress and selected terminal/edit permission callbacks
do not route every effect through Cortexa's registered schemas, deterministic
policy, exact approval, restricted executor, and audit before execution. No
supported conversation-only or true zero-tool ACP mode exists at this release.

The operator-supplied candidate is also ineligible for real execution: the
complete virtual environment and external Python runtime are not covered by an
immutable content manifest, and the pinned optional
`agent-client-protocol==0.9.0` package is absent. No install, repair, update, or
candidate execution is authorized.

Consequences: the ACP ADR is Rejected. D-080 remains the historical accepted
evaluation decision whose selected path also failed; raw TUI-gateway stdio,
managed `hermes serve` WebSocket, and ACP are all rejected for the exact pinned
release under their evaluated conditions. Native remains sole/default.
`HermesAgentRuntime` remains Draft/Blocked with no selected transport. Do not
automatically evaluate OpenAI-compatible HTTP, another protocol, patched
Hermes, or another release. A future proposal requires a separate owner-
approved decision, a fully content-manifested artifact, an upstream/enforced
conversation-only mode, exact target-platform containment, and a fresh spike.
No application source, production dependency, provider, credential, process,
socket, UI, or behavior changes through this decision.

## D-082 - Accept application-owned native multi-agent architecture

Date: 2026-08-11
Status: Accepted owner architecture decision

Decision: Adopt application-owned native multi-agent orchestration as the
primary agent direction above D-079's implemented single-run runtime seam:

The diagram is a logical task/delegation topology, not a component-call or
authority graph. The orchestrator invokes `AgentRuntime` for root and child
runs; definitions do not call a runtime.

```text
User -> Personal Assistant -> AgentOrchestrator
                              |
                              +-- Research and knowledge
                              |   +-- Research Agent
                              |   `-- Knowledge & Document Agent
                              +-- Software engineering
                              |   +-- Coding Agent
                              |   +-- QA & Validation Agent
                              |   `-- Security & Risk Agent
                              +-- Infrastructure and operations
                              |   +-- Cloud Infrastructure Agent
                              |   +-- Systems Operations Agent
                              |   +-- QA & Validation Agent
                              |   `-- Security & Risk Agent
                              `-- Automation
                                  `-- Workflow Automation Agent

AgentOrchestrator -> AgentRuntime -> NativeAgentRuntime (sole/default)
```

The first planned catalog contains exactly nine immutable application-owned
definitions: Personal Assistant; Research Agent; Coding Agent; Cloud
Infrastructure Agent; Systems Operations Agent; Knowledge & Document Agent; QA
& Validation Agent; Security & Risk Agent; and Workflow Automation Agent. The
functional groups are exactly Core orchestration (Personal Assistant), Research
and knowledge (Research and Knowledge & Document), Software engineering
(Coding, QA & Validation, and Security & Risk), Infrastructure and operations
(Cloud Infrastructure, Systems Operations, QA & Validation, and Security &
Risk), and Automation (Workflow Automation). QA & Validation and Security &
Risk are cross-cutting participants in software, infrastructure, operations,
document, and automation workflows.

The first definition/registry increment is planned to introduce all nine
definitions, but this creates no operational agent. Personal Assistant and
Research Agent are only catalog-eligible for the later deterministic initial
flow. Knowledge & Document is gated on the knowledge/document and memory phase;
Coding on engineering; QA & Validation on engineering quality; Security & Risk
on engineering security; Cloud Infrastructure on infrastructure; Systems
Operations on infrastructure/operations; and Workflow Automation on typed
workflow governance. Unknown or gated targets fail closed, and registry
membership, grouping, or activation posture never grants a route or privilege.

The role boundaries are durable. Personal Assistant owns user-facing
classification, controlled delegation requests, progress, synthesis, and
approval explanation without unrestricted privileged tools. Research is
read-only by default and may use internal or external research only through
governed tools. Knowledge & Document is limited to approved files or roots and
cannot crawl unrestricted paths or silently write durable shared memory.
Coding cannot autonomously commit, push, install dependencies, or run
destructive commands. Cloud Infrastructure cannot autonomously apply, mutate,
delete, change IAM, or use credentials. Systems Operations cannot autonomously
restart, shut down, delete, change configuration/accounts, or use privileged
shell execution. QA & Validation may use only approved safe validation tools,
cannot approve its own privileged action, and is not `ApprovalManager`.
Security & Risk is advisory, is not `PolicyEngine`, and cannot authorize or
execute remediation. Workflow Automation may propose only typed bounded
workflows and cannot execute arbitrary commands, bypass `AgentOrchestrator`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, or `AuditLogger`, self-modify,
recursively expand, or become `AgentOrchestrator`.

Agents are resolved through a validated deterministic `AgentRegistry`.
`AgentOrchestrator` is the sole application service allowed to create and
schedule agent tasks; it owns assignment, bounded delegation, task lifecycle,
workflow sequencing, result collection, attribution, and cancellation
propagation. It calls `AgentRuntime`; it does not become a runtime, provider,
tool registry, policy engine, approval manager, audit logger, memory store, or
unrestricted device-effect executor.

`AgentRuntime` remains the closed one-run execution boundary accepted by D-079.
`NativeAgentRuntime` remains sole/default, reference implementation,
deterministic contract-test path, explicit future fallback, and possible
standalone/publishable runtime. No selector or automatic fallback is added.

`AgentOrchestrator`, `AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`,
`ToolRegistry`, `PolicyEngine`, `ApprovalManager`, the future application-owned
`AuditLogger`, the future application-owned `MemoryStore`, and a future bounded
application-owned `PlatformAdapter` retain separate authoritative boundaries.
Naming a planned component is not implementation evidence and does not revive
the deleted generic scaffolds. Agents may recommend or emit typed requests
only. Application code validates lifecycle and routes, makes policy and
approval decisions, records audit, and dispatches any separately approved
restricted effect through registered tools and a bounded platform adapter.

The initial Personal-to-Research milestone uses depth one, at most one child
task total per root, and at most one active child; completion or cancellation
does not replenish that budget. Only the orchestrator may create a child task
through an explicit typed application-service call. A runtime event or
`agent.delegate` host tool does not create children. A future untrusted
proposal may be validated into a delegation request, but it grants no
authority.

The initial root is exactly Personal Assistant and the sole allowed child edge
is Personal Assistant to Research Agent. Research Agent cannot delegate, and
self, reverse, unknown, or out-of-route requests fail before task creation.
This route policy belongs to the application/orchestrator; registry membership
or definition metadata never authorizes delegation.

Four later workflow families are accepted only as staged direction:

- Research: Personal Assistant -> Research Agent -> Knowledge & Document Agent
  -> Personal Assistant synthesis.
- Engineering: Personal Assistant -> Coding Agent -> QA & Validation Agent ->
  Security & Risk Agent -> Personal Assistant synthesis -> approval before a
  consequential application action.
- Infrastructure and operations: Personal Assistant -> exactly one of Cloud
  Infrastructure Agent or Systems Operations Agent -> QA & Validation Agent ->
  Security & Risk Agent -> Personal Assistant synthesis -> approval before a
  consequential application action.
- Automation: Personal Assistant -> Workflow Automation Agent -> application
  validation of its typed proposal -> QA & Validation Agent -> Security & Risk
  Agent -> owner approval where required -> orchestrator coordination of only
  the validated approved workflow.

Those arrows are sequential orchestration and bounded result flow, never
specialist-to-specialist spawning. Every specialist remains a direct child of
the Personal Assistant root, so maximum depth remains one. Active-child
concurrency remains one until a separately approved bounded-parallelism phase.
The initial one-total-child budget applies only to the first
Personal-to-Research milestone; each later workflow plan must explicitly raise
the finite total-child budget to its allowlisted sequence (two for Research and
three for Engineering, Infrastructure/Operations, or Automation). Specialists
never spawn; Workflow Automation only proposes; over-budget, recursive,
unavailable, or unallowlisted requests fail before task creation. Every task is
independently attributable and cancellable.

Before privileged behavior is introduced, every agent action must bind exact
agent, task, optional parent task, runtime, policy profile, and memory namespace
identity. The application derives those values from trusted registry and
orchestrator state. Missing, unknown, duplicate, stale, or mismatched identity
fails closed and never defaults to the Personal Assistant. Agent names and
runtime capabilities grant no permission.

Tools, schemas, deterministic policy, exact approval, restricted execution,
audit, credentials, persistence, memory, and platform access remain
application-owned and separately gated. No external agent framework is
required for the initial implementation. Agent names and activation posture do
not grant capabilities.

Hermes integration is Deferred — evaluated transport and containment
requirements not met. D-080 remains the historical accepted WebSocket
evaluation decision whose mechanism failed, and D-081 remains the accepted ACP
rejection for Hermes Agent `0.20.0` / tag `v2026.8.3` / commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`. `HermesAgentRuntime` remains
Draft/Blocked with no selected transport. This is not permanent abandonment;
any future release or mechanism requires a separate accepted decision and
evidence. External runtimes may still implement `AgentRuntime` without owning
orchestration or governance.

Consequences: the AgentDefinition/AgentRegistry plan is the sole next Ready
multi-agent implementation plan, but this documentation increment authorizes no
code. That plan covers all nine inert definitions with staged catalog
eligibility. Tasks, orchestration, delegation, per-agent governance,
knowledge/document boundaries, four staged workflows, memory, parallelism,
desktop UI, providers, tools, demos, and final architecture/security review
remain Blocked or Future under separate plans.
D-079 remains intact. D-082 supersedes only D-065's then-current
documentation-only limitation by accepting this bounded architecture while
retaining D-065's no-implicit-authority, implementation, and trust-boundary
rules. D-078's personal-project scope remains intact. No production source,
test, dependency, provider, process, IPC, UI, or behavior changes through this
decision.

## D-083 - Combine the task/orchestration foundation and first deterministic delegation proof

Date: 2026-08-12
Status: Accepted owner architecture and sequencing amendment

Decision: Combine the former native multi-agent roadmap Phases 2 and 3 into one
bounded implementation increment. The increment may implement the closed task
lifecycle, trusted execution context, application-owned `AgentOrchestrator`,
typed delegation, result return, cancellation, direct Personal Assistant
response, and exactly one deterministic Personal Assistant -> Research Agent
-> Personal Assistant synthesis proof.

This decision supersedes only D-082's requirement to verify those two phases as
separate increments. Every D-082 ownership, trust-boundary, catalog, route,
limit, and non-authority rule remains in force:

- `AgentOrchestrator` is the only component that may create a child task;
- the root is Personal Assistant and the only child target is Research Agent;
- depth, total-child budget, and active-child concurrency are one, and the
  total budget is not replenished after a terminal child;
- the other seven catalog roles remain `Deferred` and non-operational;
- delegation is an explicit typed application-service operation, never an
  `agent.delegate` host tool or runtime control event;
- `AgentRuntime` remains a one-run port and `NativeAgentRuntime` remains the
  sole/default runtime;
- policy, approval, registered tools, restricted execution, audit, memory,
  provider, IPC, UI, persistence, parallelism, and external frameworks remain
  outside this increment; and
- Hermes remains Deferred/Blocked.

The combined deterministic delegated lifecycle may use at most three
sequential runtime runs: an initial Personal Assistant run, one Research Agent
child run, and a fresh Personal Assistant synthesis run. Accepting delegation
terminates only the initial root runtime run and moves the root task to
`WaitingForChild`; it does not cancel the root task. A terminal child outcome
is returned as bounded, attributed, untrusted application data before the
orchestrator starts the synthesis run. A direct response uses only the initial
root run.

The orchestrator derives agent, task, root, parent, runtime, depth, and active
run identities from trusted application state. Callers and model text may not
supply or override those identities. Policy-profile and memory-namespace IDs
are omitted until their enforcing phases rather than represented by inert
placeholders. Objectives, delegated context, expected deliverables, results,
events, and errors remain closed, bounded, and redacted. Runtime tool proposals
fail closed because this increment is text-only.

Consequences: the combined plan may become Ready after its exact lifecycle,
interfaces, files, tests, validation, and rollback are reviewed. Successful
completion proves deterministic application orchestration only; it does not
prove a provider, model, tool, memory store, UI, or shipping assistant flow.
Later governance, knowledge/document, memory, parallelism, specialist, UI, and
end-to-end demonstration plans remain separately gated.

## D-084 - Add a non-executing per-agent governance foundation

Date: 2026-08-12
Status: Accepted owner security and architecture decision

Decision: Authorize one bounded security-sensitive increment that assigns an
exact versioned policy profile to every built-in agent, carries application-
derived live agent/task/runtime attribution through a synthetic tool-policy-
approval-audit contract, and strengthens the exact initial delegation matrix.
The increment is non-executing: `Allow`, approval, rejection, cancellation, and
expiration are evidence only, and every execution disposition is
`NotAttempted`.

This decision preserves D-082 and D-083 ownership and route boundaries while
resolving their deliberately deferred policy-profile question:

- the nine built-in agents receive nine exact versioned profile identities;
- profiles are immutable deterministic policy inputs, never role-derived
  authority, generic permission booleans, or execution grants;
- the application derives profile identity from the immutable definition when
  creating a task and revalidates it against the exact live task and runtime
  run before governance;
- Personal Assistant alone may present the two already registered local tool
  schemas to policy evaluation; every other profile has an empty current tool
  allowlist;
- the existing deterministic `PolicyEngine` remains singular and authoritative
  for registered schema risk and permission classification;
- the existing approval boundary remains the only pending user-approval
  mechanism, and approval cannot dispatch because no executor is authorized;
- one closed bounded volatile audit family records typed attribution,
  policy/approval disposition, injected-clock evidence, and execution
  `NotAttempted` without prompts, parameters, output, secrets, or reasoning;
- delegation remains an explicit `AgentOrchestrator` service governed by the
  exact Personal Assistant -> Research matrix, never a host tool or runtime
  control event; and
- QA & Validation and Security & Risk remain advisory, while Workflow
  Automation remains proposal-only. None may approve, authorize, create child
  tasks, or execute.

Tool proposals accepted by a model/runtime remain disabled. The shared
`AgentRuntime` and `NativeAgentRuntime` contracts do not change, and
`AgentOrchestrator` continues to reject runtime `ToolProposal`. The new
governance path is an application-owned synthetic contract used to prove exact
identity, profile, policy, approval, denial, cancellation, audit, and
redaction behavior. It does not claim shipping tool capability.

Delegation does not pass through `ToolRegistry`. Only the orchestrator may
create a child, after exact live-context, registry, activation, matrix, depth,
and budget validation. Missing, unknown, stale, duplicate, deferred,
mismatched, forged, unsupported, or unauthorized identity and action data fail
closed before downstream authority or state mutation.

The verified legacy concrete gateway turn remains compatible and explicitly
separate. Its existing `SchemaValidatedFunctionCall`, `PolicyInput`, and public
policy entry point remain legacy-only. The new path uses a distinct sealed
agent request and a dedicated profile-aware method on the same deterministic
policy engine; there is no public conversion into legacy `PolicyInput` and no
optional identity or `None` fallback. Approval retains a closed explicit
legacy-versus-agent origin. This increment does not claim that the legacy
gateway is a multi-agent consumer or that its existing audit is the new
governance audit.

`AgentGovernanceService` is the non-executing composition boundary. One
instance is composed inside the one-root orchestrator so every governance
entry can revalidate the exact live task/run before sequencing the separate
profile registry, tool registry, policy engine, approval manager, and closed
governance audit. A pending approval blocks task runtime events and delegation;
task cancellation consumes and audits it before terminalizing the runtime and
task. This coordination does not move policy, approval, execution, or audit
authority into an agent or runtime.

Governance audit uses one preallocated bounded lifecycle record per exact tool
or delegation subject. Capacity and replay checks precede policy, approval,
runtime cancellation, or child allocation; later terminal updates are
infallible and use a process-local logical tick. Tool records cover validation,
policy, and approval states with execution always `NotAttempted`. Delegation
records cover matrix denial, allow, child creation, or typed failure while
remaining outside `ToolRegistry`.

D-082's memory-namespace requirement remains mandatory before any future
memory access, persistence, data-bearing privileged action, or device effect.
No such behavior is introduced here, so an unenforced memory-namespace
placeholder is forbidden and the memory decision remains separately gated.

Consequences: the approved governance ExecPlan may proceed after a fresh
Ready/Ready-with-advisories review. It may modify only the declared agent,
policy, approval, audit, test, and documentation boundaries. No executor,
provider, memory store, platform adapter, IPC, UI, permission, dependency,
Hermes integration, specialist activation, or visible behavior is authorized.
Later knowledge/document, memory, specialist workflow, parallelism, UI, and
end-to-end work remains Blocked.

## D-085 - Add volatile agent memory and an approved-document Knowledge boundary

Date: 2026-08-12
Status: Accepted owner security and architecture decision

Decision: Authorize one bounded implementation increment that replaces the
absence left by D-033 with a new namespace-aware, application-owned, volatile
`MemoryStore`; adds a narrow read-only approved-document boundary; and makes the
Knowledge & Document Agent eligible only for an explicit Personal Assistant to
Knowledge document task. The former deleted arbitrary-content memory scaffold
must not be restored, and the existing SQLite bootstrap, React conversation
state, runtime, native runtime, tools, policy, approval, governance audit, and
storage migrations remain unchanged except for the exact agent identity fields
needed to enforce memory access.

The memory model is process-local, workflow-local, and bounded. One one-root
`AgentOrchestrator` owns one store; records never cross orchestrator workflows
and are cleared when that owner drops. It distinguishes approved shared memory,
agent-private memory, task-temporary memory, and proposed shared memory.
Personal Assistant may read approved shared memory, use its own private and
task-temporary memory, and propose shared content. Research and Knowledge may
use only their own private and task-temporary memory and may propose shared
content; shared context reaches them only through explicit application-owned
selection. Other agents remain memory-disabled. No agent may directly create an
approved shared record. A closed application review decision may edit, approve,
or reject a proposal only against the exact reviewed version; stale review,
withdrawal, or deletion fails atomically. Promotion is atomic and approval is
not tool or execution authority.

`AgentDefinition` remains the sole mapping from agent identity to a closed
memory-access profile. That profile is captured into `AgentTask`,
`AgentExecutionContext`, and live `AgentAttribution`; missing, stale, forged, or
mismatched memory identity fails closed. All store access is sequenced by the
application after exact live-context validation and requires a non-forgeable,
non-cloneable live access grant; review and configuration use a distinct sealed
application-control authority. Reads require exact record
selection; there is no broad implicit read of all shared memory, conversation
history, documents, sibling results, or another agent's private/task data.
Task-temporary records are removed only after the corresponding task reaches a
terminal state; failed cancellation leaves live task memory intact for a safe
retry. Disabling the volatile store clears its records and makes later access
fail closed until explicitly re-enabled.

The document boundary is a dedicated `ApprovedDocumentReader`, not a generic
filesystem tool or platform adapter. Trusted application code may register one
user-selected file, task attachment, approved-root member, or application-owned
generated artifact and receives an opaque task-bound reference. Agents and
runtime input never receive or select a path. Approved-root members use closed
relative paths; absolute paths, traversal, schemes, directory enumeration,
non-regular files, and symlink roots/components/targets are rejected. Every
read uses a linear reserve/abort/commit lifecycle, revalidates authorization and
file identity, opens once, and reads through a fixed byte limit. The exact
encoded document request, including selected memory and framing, is bounded by
the existing runtime-request limit. Errors, Debug, audit, and events contain no
path or document content.

Initial format support on the repository's supported Unix executable-test
targets is exactly UTF-8 plain text in lowercase `.txt`
and Markdown in lowercase `.md`. Invalid UTF-8, NUL-bearing/binary-looking
content, empty or oversized files, PDFs, office documents, HTML, RTF, archives,
images, OCR, and external extraction services are unsupported. Non-Unix targets
compile but fail closed as unavailable without adding unsafe code or a
dependency. The application
may request only the closed initial operations `Read`, `Summarize`,
`ExtractFacts`, `IdentifySections`, `Classify`, and `ProposeOutline`; these are
instructions for a bounded agent task and do not claim a parser, provider,
artifact writer, or successful semantic transformation.

Knowledge & Document changes from deferred catalog metadata to initial
eligibility only after the memory and document contracts pass. Generic
delegation remains governed by the exact Personal Assistant to Research matrix.
A separate document-task entry validates a live Personal Assistant root,
approved document reference, Knowledge eligibility, depth and one-child limits,
then starts one Knowledge child with only the selected operation, bounded
document content, and content-free provenance label. Research to Knowledge is
not implemented, specialists still cannot spawn, and no other deferred agent is
activated. `NativeAgentRuntime` remains sole/default and runtime tool proposals
remain rejected.

This increment intentionally does not resolve ARB-005. No memory or document
content enters SQLite, the development database, logs, audit, Tauri IPC, React,
provider traffic, backup, export, synchronization, or external storage. The
volatile store has no restart recovery or durable-retention claim. Durable
memory still requires a separate accepted encryption/key ownership, schema,
migration, transaction, retention, deletion/export, recovery, corruption, and
rollback decision. No vector database, embedding service, semantic index,
filesystem plugin, OCR service, dependency, permission, credential, network,
process, Hermes integration, or unrestricted file access is authorized.

Consequences: the combined memory/document ExecPlan may proceed after it fixes
exact types, limits, ownership, lifecycle, files, tests, validation, and
rollback and passes a fresh readiness review. Completion proves only an unwired
Rust application boundary and deterministic mock-runtime Knowledge task. It
does not prove a shipping UI, native file picker, provider, model quality,
durable memory, full Research-to-Knowledge workflow, or general document
processing.

## D-086 - Add one structured Research-to-Knowledge workflow

Date: 2026-08-12
Status: Accepted owner architecture and implementation decision

Decision: Authorize one deterministic, Rust-only Personal Assistant ->
Research Agent -> Knowledge & Document Agent -> Personal Assistant synthesis
workflow above the existing `AgentRuntime` port. This is a fixed application
service, not a general workflow engine. Both specialists are sequential sibling
children of the same Personal Assistant root at depth one. Only
`AgentOrchestrator` creates either child; Research never delegates to or spawns
Knowledge.

D-086 preserves D-079 and D-082 through D-085 except for this workflow's exact
finite sequencing limits. Generic delegation remains Personal Assistant ->
Research and retains its original one-child semantics. D-085's separate
approved-document Personal Assistant -> Knowledge route remains unchanged. The
sealed workflow alone may own three tasks, two non-replenishing children, one
active child, and four sequential runtime runs: initial Personal, Research,
Knowledge, and final Personal synthesis. Automatic retries are zero.

The workflow accepts only a bounded immutable catalog of application-supplied
deterministic fixtures. It does not authorize search, browser, network,
provider, filesystem discovery, or live research. Runtime output is untrusted
strict structured data. Research may reference only opaque source IDs issued by
the application catalog; Knowledge may preserve only references already
validated from the Research result. Unknown or remapped IDs fail closed,
missing references produce explicit partial status, and the application never
invents a citation. Final synthesis receives validated attributed results plus
an explicit fixture-only disclosure and stage status, never raw invalid output
or chain-of-thought.

Research and Knowledge may use the D-085 private and task-temporary memory
namespaces only through their exact live contexts. Sibling private/task memory
is not transferred. A reusable-knowledge value remains proposal data or an
entry in `ProposedShared`; it is never automatically approved, selected,
persisted, or treated as fact. Durable memory and ARB-005 remain unresolved.

The application exposes a bounded content-free workflow event family for
Research start/completion, Knowledge organization start/completion, synthesis
start, partial failure, cancellation, and completion. A separate closed
volatile workflow attribution record binds stage, task/root/agent/runtime/run,
predecessor where applicable, and typed outcome without source content,
findings, summaries, paths, URLs, prompts, output, or reasoning. It is not a
generic or durable `AuditLogger` and grants no authority.

Research failure, cancellation, or invalid structured output skips Knowledge
and permits a bounded truthful Personal fallback. Knowledge failure,
cancellation, or invalid output permits Personal synthesis from the validated
Research result plus an explicit unavailable status. Root cancellation cancels
pending governance and the active child before the root and never starts a
later stage. Final synthesis failure fails the root. Late, duplicate, foreign,
cross-stage, wrong-run, and wrong-sequence events fail before mutation.

Knowledge instructions advance to a versioned V2 source because the role may
now consume validated Research evidence in addition to D-085 approved
documents. This changes no runtime, native-runtime, policy profile, tool,
approval, executor, document-reader, provider, permission, IPC, or UI
authority.

Consequences: the exact Research/Knowledge ExecPlan may proceed after fresh
readiness, architecture, and security review. Completion proves only bounded
fixture-based orchestration, structured specialist and final-output
attribution, partial-failure handling,
and Native-compatible contracts. It does not prove live research, provider or
model quality, factual accuracy, user-visible capability, a general workflow
engine, parallelism, or activation of Coding, QA, Security, Cloud, Systems
Operations, or Workflow Automation.

Implementation outcome (2026-08-12): the bounded D-086 implementation is
verified complete with advisories under gate
`agent-research-knowledge-workflow`. Strict Research, Knowledge, and final
synthesis contracts accept only catalog-issued fixture references; the sealed
orchestrator sequence preserves depth-one sibling lineage, one active child,
zero retries, truthful partial failure, child-first cancellation, and
task-local memory cleanup. Native remains sole/default and unchanged. Generic
Research-to-Knowledge delegation, live retrieval, providers, persistence, IPC,
UI, parallelism, and every other specialist workflow remain Blocked.

## D-087 - Add one fixture-only engineering quality workflow

Date: 2026-08-12
Status: Accepted owner security and implementation decision

Decision: Authorize one deterministic, Rust-only, proposal-only Personal
Assistant -> Coding Agent -> QA & Validation Agent -> Security & Risk Agent ->
Personal Assistant synthesis workflow above the existing `AgentRuntime` port.
This is a fixed application service, not a general workflow engine. The three
specialists are sequential sibling children of the same Personal Assistant
root at depth one. Only `AgentOrchestrator` creates them; specialists never
delegate to or spawn one another.

The initial mode accepts only bounded immutable synthetic repository fixtures
supplied by the application. It may produce strict structured code analysis,
architecture explanation, diff review, implementation planning, patch
proposals, proposed validation, and advisory security review. It does not
inspect the live repository or invoke a filesystem, code-search process, test,
formatter, shell, package manager, network, credential source, or Git command.
Fixture path labels and opaque file IDs grant no filesystem authority.
An application-owned validation-evidence catalog may contain only deterministic
`ObservedFixture` facts or `NotRun` proposed checks; it cannot represent a real
test pass or external observation. QA must cover each application-issued
acceptance criterion exactly once as demonstrated or not demonstrated, with
exact evidence provenance. Missing, duplicate, or unknown coverage is invalid,
and any not-demonstrated criterion makes the report incomplete or blocked.

The sealed workflow may own exactly four tasks, three non-replenishing
children, one active child, and five sequential runtime-run attempts: initial
Personal, Coding, QA, Security, and final Personal synthesis. Delegation depth
remains one and automatic retries remain zero. The workflow has exact bounded
runtime, generic, workflow-event, descriptive-audit, input, result, list, and
field limits defined in its ExecPlan. Capacity and successor state are prepared
before terminal runtime acceptance, cancellation is child-first, late or
mismatched events fail closed, and continuation-start failure cannot reverse an
accepted terminal event or replenish a budget.
The engineering selector is a sealed application-only operation, mutually
exclusive in both directions with generic delegation, D-085 document work, and
D-086. All workflow/task/profile/runtime/predecessor identity is derived from
private live application state and remains redacted; model output supplies no
trusted identity. Native request size, not raw selected-text length alone,
remains bounded: the ExecPlan must choose and adversarially prove a
conservative selected-text limit against the existing 65,536-byte encoded
gateway boundary. Any unexpected Native serialization rejection inside that
accepted limit is a typed stage-start failure and never an authority fallback.

Coding returns a versioned `ChangeProposal` that references only known fixture
file IDs and describes proposed patches as inert data. A closed application
classifier represents fixture analysis and planning as proposal-only and
represents actual file writes or deletion, path escape, dependency installation,
package-manager/test/formatter execution, Git commit or push, branch deletion,
destructive shell, credential access, and network access as denied. It never
dispatches an operation or registers a tool.

QA returns a versioned `ValidationReport` bound to the exact proposal. It must
account for every supplied acceptance criterion, preserve proposed tests as
`not_run`, identify missing validation or regression gaps, and remain advisory.
QA cannot approve, become `ApprovalManager`, modify source, suppress a failing
test, or fabricate executed evidence.

Security returns a versioned `RiskAssessment` bound to the exact proposal and
QA result or explicit QA-unavailable status. Findings must be evidence-bound or
marked as hypotheses; absent dependency evidence is stated rather than
invented. Security remains advisory, is not `PolicyEngine`, cannot authorize or
remediate, cannot supply trusted risk or permission metadata, and cannot access
or expose secret values.

Final Personal synthesis preserves the validated proposal, QA, Security,
fixture, and partial-failure attribution. It must state that inputs are
fixture-based, the result is proposal-only, no changes or tests were executed,
and carry an application-derived approval requirement: `NotApplicable` for
analysis with no mutation proposal or `RequiredBeforeMutation` when a patch is
proposed. No approval request is created in this increment because there is no
actionable execution subject. The derived value is a deterministic result
invariant, not approval by an agent, model, runtime, or orchestrator.

Coding failure skips QA and Security and permits truthful Personal fallback.
QA failure preserves the proposal and may be followed by Security with an
explicit unavailable status; QA `incomplete` or `blocked` forces partial
synthesis. Security failure preserves validated Coding and QA results and
permits partial synthesis. Final synthesis failure fails the root. Invalid raw
output never reaches a later stage.

Coding, QA & Validation, and Security & Risk may move from `Deferred` to
`Initial` only after their new exact versioned instructions, strict contracts,
sealed workflow, and regression tests pass. `Initial` means non-authorizing
eligibility for this one unwired fixture workflow; it does not mean a live
assistant, repository tool, or operational capability. Their policy profiles
remain tool-ineligible, their memory profile remains `MemoryDisabledV1`, and
runtime tool proposals remain rejected.

This decision does not add or change `ToolRegistry`, `PolicyEngine`,
`ApprovalManager`, an executor, durable `AuditLogger`, memory, documents,
`AgentRuntime`, `NativeAgentRuntime`, Tauri, React, dependencies, permissions,
IPC, persistence, external frameworks, providers, or device behavior. The two
existing tools remain unrelated and available only to Personal Assistant under
the existing non-executing governance proof. Every execution disposition
remains `NotAttempted`. Native remains sole/default. Codex, Hermes, and OpenClaw
are not integrated.

Consequences: the exact engineering-quality ExecPlan is Ready but not Active
after recording the selected fixture-only contracts, limits, files, tests,
validation, failure strategy, and rollback. Fresh readiness, architecture, and
security review must confirm it against the actual workspace before its gate
begins. Any actual repository inspection, scoped write, test or
formatter execution, package/dependency operation, Git operation, or approval-
to-execution path requires a separate accepted decision and plan defining exact
registered schemas, repository-root and path containment, command allowlists,
executor ownership, approval subjects, audit, rollback, and target-platform
evidence. Cloud Infrastructure, Systems Operations, Workflow Automation,
parallelism, provider/runtime wiring, IPC, UI, and all consequential actions
remain separately Blocked.

## D-088 - Add two fixture-only infrastructure and systems operations workflows

Date: 2026-08-12
Status: Accepted owner security and implementation decision

Decision: Authorize one bounded implementation increment containing two
separate deterministic, Rust-only, fixture-only, proposal-only workflows above
the existing `AgentRuntime` port:

```text
Personal Assistant -> Cloud Infrastructure Agent -> QA & Validation Agent
  -> Security & Risk Agent -> Personal Assistant synthesis

Personal Assistant -> Systems Operations Agent -> QA & Validation Agent
  -> Security & Risk Agent -> Personal Assistant synthesis
```

These are two closed application-service selectors, not one polymorphic route
chosen by model output and not a general workflow engine. Trusted application
code selects exactly one workflow from a live Personal Assistant root before
root output begins. The two selectors are mutually exclusive with each other
and with generic delegation, the approved-document route, D-086, and D-087.
All three specialists in either workflow are sequential depth-one siblings of
the Personal Assistant root. Only `AgentOrchestrator` creates them; specialists
never delegate, spawn, or invoke one another.

The public surfaces remain distinct:
`CloudInfrastructureWorkflowRequest`/
`CloudInfrastructureWorkflowAcceptance`/
`CloudInfrastructureWorkflowResult` and
`SystemsOperationsWorkflowRequest`/
`SystemsOperationsWorkflowAcceptance`/
`SystemsOperationsWorkflowResult`, with distinct result and event getters. One
owned private `InfrastructureOperationsWorkflowState::{Cloud, Systems}` stores
exactly one selected state. Private shared lifecycle helpers may prepare common
QA, Security, synthesis, failure, and cancellation mechanics, but no generic
public start path or generalized workflow engine exists.

Each selected workflow may own exactly four tasks, three non-replenishing
children, one active child, and five sequential runtime-run attempts: initial
Personal, the selected Cloud or Systems specialist, QA, Security, and final
Personal synthesis. Delegation depth remains one, active-child concurrency
remains one, and automatic retries remain zero. The existing global runtime and
orchestration event limits remain 32. The new workflow family has exact
per-run, structured-event, attribution-record, input, result, list, and field
limits fixed by its ExecPlan. Capacity and the complete successor input are
prepared before a terminal event mutates state. A successor start failure is a
typed partial outcome and never reverses an accepted terminal event or
replenishes a task, run, event, child, or retry budget. Cancellation remains
child-first and prevents every later specialist stage.

The workflows accept only bounded immutable synthetic fixtures from one
built-in application-owned catalog. A caller may select one closed typed Cloud
or Systems scenario ID but cannot provide or alter trusted fixture kind,
content, fixture/evidence/criterion IDs, or workflow identity. Public request
fields are private, have no deserializer or general raw constructor, retain a
catalog-issued sealed proof, and are revalidated against the canonical catalog
snapshot before mutation. Cross-kind selection or any internal/test alteration
fails preflight with zero mutation. The Cloud workflow accepts synthetic
Terraform, Azure, AWS, architecture, and inventory-snapshot evidence. The
Systems workflow accepts synthetic and sanitized operating-system, service,
process, log, configuration, resource, VMware, backup, and recovery evidence.
Fixture labels and opaque IDs grant no filesystem, account, provider, host,
network, process, service, or virtualization authority.

Runtime output is untrusted strict structured data and may reference only
application-issued IDs. Unknown, duplicate, remapped, malformed, oversized,
identity-supplying, authority-claiming, or reasoning-bearing content fails
closed. Credential defense is deterministic defense in depth, not semantic
secret detection: exact secret-named JSON fields are forbidden, and normalized
ASCII lines are checked only for the ExecPlan's enumerated fake sentinel,
token/bearer, PEM/private-key, AWS, Azure/ARM, and VMware credential prefixes.
Debug, error, event, and attribution surfaces remain content-free and redacted.

The Cloud workflow may produce only a versioned `InfrastructureAssessment`, an
inert `ChangePlan`, exact fixture findings, limitations, unresolved questions,
and proposed validation. It may describe static Terraform review, but it must
record `terraform fmt`, `terraform validate`, provider inventory, and every
external check as not run unless a future separately approved application tool
supplies real evidence. `terraform apply`, plan/apply dispatch, resource
creation/update/deletion, IAM or firewall changes, production access, cloud
shell, credential use, secret rotation, Terraform state mutation, and backend
reconfiguration are closed denied capability proposals and are never
dispatched.

The Systems workflow may produce only a versioned `OperationalAssessment`,
evidence-bound or explicitly hypothetical `DiagnosticFinding` values, an inert
diagnostic plan, remediation proposal, rollback considerations, limitations,
and proposed validation. It may analyze only supplied fixtures. Live log,
service, process, configuration, disk, memory, VMware, backup, or host
inspection remains unavailable. Service restart/stop, reboot/shutdown, process
termination, configuration mutation, package installation, patching,
account/permission change, deletion, privileged shell, VMware mutation, backup
mutation, and credential access are closed denied capability proposals and are
never dispatched.

QA & Validation and Security & Risk are reused as cross-cutting advisory roles
through new exact instruction versions and infrastructure/operations-specific
structured results. QA must reconcile every application-issued acceptance
criterion exactly once against `ObservedFixture` or `NotRun` evidence, preserve
provenance, and mark every unexecuted check as not run. It cannot approve,
modify inputs, suppress a gap, or fabricate validation. Security findings must
be evidence-bound or explicit hypotheses and must distinguish absent
credential, dependency, provider, platform, audit, and rollback evidence. It
cannot authorize, provide trusted policy or permission metadata, access a
secret, or execute remediation. Neither role becomes `ApprovalManager` or
`PolicyEngine`.

Final Personal synthesis preserves the validated specialist, QA, Security,
fixture, and partial-failure attribution. It must state that all operational
inputs are synthetic fixtures, no Terraform/cloud/host/VMware command or live
inventory ran, no credential was loaded, and no action occurred. The
application derives a closed approval requirement as `NotApplicable` when no
closed denied-effect capability is present or
`RequiredBeforeConsequentialAction` when at least one is present. Free text,
risk prose, or an agent flag cannot affect this derivation. No approval or
governance subject is created, their audits do not change, and every execution
disposition remains `NotAttempted` because this increment has no executable
subject, executor, or effect. Approval status is not authorization and cannot
be supplied by an agent or runtime.

Cloud or Systems failure skips QA and Security and permits a truthful bounded
Personal fallback. QA failure preserves the validated first-stage assessment
and is followed by Security with an explicit QA-unavailable status; an
incomplete or blocked QA conclusion forces partial synthesis. Security failure
preserves the validated assessment and QA outcome and permits partial
synthesis. Invalid raw output never reaches another stage. Root cancellation
cancels any active-child governance subject, then the active child run/task,
then any root governance subject, and finally the root; it starts no
later stage, and produces no synthesis result. Cancellation first preflights
governance resolution and identity/capacity without mutation. If governance
terminalization succeeds and a later runtime cancellation fails, that approval
remains terminally cancelled while the affected task/run stays live and safely
retryable; no false workflow `Cancelled` event is emitted. Child cancellation
precedes root cancellation. A child cancellation failure leaves child and root
live; a later root cancellation failure cannot reverse an already cancelled
child. No successor starts in a failure branch. Final Personal synthesis
failure fails the root.

Cloud Infrastructure and Systems Operations may move from `Deferred` to
`Initial` only after their exact versioned instructions, strict contracts,
sealed selectors, regression tests, and completion gate pass. `Initial` means
non-authorizing eligibility only for these two unwired fixture workflows. QA
and Security remain `Initial` only for the sealed workflows explicitly
implemented for them. All four specialist policy profiles retain empty tool
allowlists and `MemoryDisabledV1`; runtime tool proposals remain rejected and
every execution disposition remains `NotAttempted`.

This decision adds no schema to `ToolRegistry` and changes no `PolicyEngine`,
`ApprovalManager`, executor, durable `AuditLogger`, `PlatformAdapter`, memory,
document, credential, provider, process, shell, PowerShell, Terraform, Azure,
AWS, VMware, service, logging, filesystem, network, IPC, React, Tauri
capability, permission, dependency, persistence, or external-runtime boundary.
Developer-machine availability of a binary or CLI is not product capability
and supplies no workflow evidence. Native remains sole/default; Codex, Hermes,
and OpenClaw are not integrated.

Consequences: the exact infrastructure/systems-operations ExecPlan is Ready but
not Active after recording its contracts, limits, files, tests, atomicity,
cancellation, validation, rollback, and stop conditions. Implementation still
requires the mandatory gate and fresh architecture and security review. Any
live inventory, Terraform command, filesystem or platform diagnostic,
credential, provider API/CLI, shell, process/service control, VMware operation,
approval-to-execution path, parallelism, IPC, UI, or device effect requires a
separate accepted decision and plan with exact registered schemas, containment,
policy, approval subjects, restricted executor, audit, rollback, and
target-platform evidence. Workflow Automation and every consequential action
remain Blocked.

## D-089 - Decompose private workflow internals before Workflow Automation

Date: 2026-08-12
Status: Accepted owner implementation prerequisite

Decision: satisfy the D-088 post-increment technical-debt gate through one
behavior-preserving source-organization increment before Workflow Automation
implementation begins. The increment extracts the private D-088 workflow
lifecycle from the `AgentOrchestrator` facade and separates the immutable
fixture, transfer-framing, and strict validation/parser implementation from the
public infrastructure/operations contracts.

This is a mechanical ownership change only. `AgentOrchestrator` remains the
sole task, run, runtime-event, child-creation, cancellation, and selector
authority. All existing public paths, serialized fields, fixture bytes, bounds,
error variants, event and audit ordering, cancellation semantics, activation
states, profiles, tool eligibility, memory behavior, and execution
dispositions remain unchanged. Child modules are private; the narrow methods
called by the facade may be no wider than `pub(super)`.

The exact new private files are
`agent/orchestrator/infrastructure_operations_workflow.rs` and the
`agent/infrastructure_operations/{catalog,framing,validation}.rs` modules.
Research/Knowledge and Engineering lifecycle code is not refactored in this
increment; their public contracts and complete regression suites remain
mandatory evidence. The extraction must not introduce a workflow trait,
generic state machine, DAG executor, DSL, dynamic registry, scheduler, event
bus, macro-generated engine, or reusable execution authority.

This decision adds no Workflow Automation schema, proposal, validation,
activation, task, run, tool, policy, approval, audit authority, execution,
persistence, configuration, IPC, UI, provider, external runtime, parallelism,
background work, or device effect. Native remains sole/default. D-079 and D-082
through D-088 remain authoritative.

Consequences: the owner-authorized Workflow Automation request remains the
selected next product objective, but its implementation gate cannot begin
until `agent-workflow-internals-decomposition` completes with a valid marker
and a review that explicitly clears D-088's `blocks_next_increment` finding.
Completion of this prerequisite grants only a fresh Workflow Automation
readiness review; it does not activate the agent or authorize tool execution.
Actual tool dispatch remains separately blocked because the current registry
contains definitions only, approval does not dispatch, and every execution
disposition remains `NotAttempted`.

Stop the prerequisite if any public behavior or expected test result must
change, any internal API must widen beyond `pub(super)`, a generalized workflow
abstraction becomes necessary, or a tool, policy, approval, audit, runtime,
memory, document, dependency, permission, IPC, UI, persistence, provider, or
effect boundary would change.

## D-090 - Narrow Workflow Automation to typed proposals and manual sealed dispatch

Date: 2026-08-13
Status: Accepted owner implementation decision

Decision: activate Workflow Automation only as an application-owned,
proposal-only planner for five immutable typed templates. The application
strictly validates every proposal. A complete validated proposal for the four
already implemented fixture-only/no-I/O workflow families may produce one
process-local, non-clone, expiring dispatch token. A trusted manual
application-service call may consume that token once in a fresh
`AgentOrchestrator` and invoke only the corresponding existing sealed selector.
Proposal completion never dispatches automatically.

For this phase, D-090 narrows and supersedes D-082's provisional Automation
topology that placed QA and Security after the Workflow Automation proposal.
The implemented proposal route is exactly Personal -> Workflow Automation ->
Personal synthesis. QA and Security participation in proposal review is
deferred; neither is silently invoked or represented as having reviewed D-090
output.

The exact manually eligible families are Research -> Knowledge -> Personal
synthesis; Coding -> QA -> Security -> Personal synthesis; Cloud -> QA ->
Security -> Personal synthesis; and Systems -> QA -> Security -> Personal
synthesis. Knowledge -> Workflow Automation proposal -> Personal synthesis is
cataloged as proposal-only and produces no dispatch token. This increment does
not implement a generic workflow engine or reinterpret the existing four
state machines.

The proposal schema recognizes only agent-task, synthesis, governed-tool, and
approval-checkpoint step kinds. Every agent and tool reference is checked
against application-owned registries; one shared crate-private built-in tool
registry supplies the same two read-only definitions to governance and the
validator without evaluating policy or creating an approval/audit subject.
Dependencies, cycles, exact canonical
shape, bounds, and activation are validated deterministically. Because no tool
executor, approval-to-dispatch path, or general durable `AuditLogger` exists,
every tool or approval step is non-executable and prevents token issuance. No
approval request is created. Arbitrary shell/code/script fields, unknown step
types, recursive/nested execution, self-modification, dynamic templates, and
unbounded inputs fail closed.

The proposal lifecycle is one Personal root, one depth-one Workflow Automation
child, and Personal synthesis, with one active child, zero retries, and finite
task/run/event/audit limits. The maximum duration is exactly 120 seconds and is
enforced at cooperative monotonic checkpoints. The deadline cannot be extended and is
carried into the one-time token. It does not promise hard preemption of a
synchronous runtime call already executing. A successful A-D dispatch carries
the original deadline into the destination orchestrator; its next trusted
central event, successor, cancellation, or manual-result call after expiry
attempts child-first cancellation and starts no successor. Tests use an
injected monotonic clock without sleeps. No timer, hard preemption, or
asynchronous expiry is claimed.

Workflow Automation may be `Initial` only for this exact unwired proposal
selector. It remains
`WorkflowProposalOnlyV1`, `MemoryDisabledV1`, tool-ineligible, absent from
generic delegation, unable to spawn, and non-authorizing. Native remains
sole/default. Workflow-local content-free attribution is not a durable or
general audit logger.

This decision adds no tool schema or execution, policy permission, approval
dispatch, executor, repository/filesystem/network/platform operation,
credential access, memory/document access, persistence, scheduler, trigger,
parallelism, provider, external runtime, dependency, configuration, IPC/UI,
Tauri permission, deployment, or device effect. Workflow creation or proposal
acceptance never authorizes current or future consequential action.

Consequences: the exact
[`workflow automation ExecPlan`](docs/plans/2026-08-11-workflow-automation.md)
is verified complete with advisories under a complete, valid
`agent-workflow-automation-proposals` gate. It must stop if a general runner,
template-E dispatch, tool/approval execution, persistent workflow state, hard
preemption, or any new I/O/effect boundary becomes necessary. Executable tool
steps, approval dispatch, scheduling, recurring execution, and broader
automation each require separate accepted decisions and prerequisites.

Implementation evidence on 2026-08-13 passes the focused Workflow Automation
tests 12/12, public contract 18/18, strict Clippy, 393 all-target Rust tests with
one intentional ignored probe, and complete verification with 124 frontend and
208 Rust library tests plus release builds. Independent architecture, security,
and code review is `PASS WITH ADVISORIES`. Final documentation, repository,
security, diff, session-end, and marker checks pass; no next owner-selected
Ready plan exists.

## D-091 - Bound parallel specialists to one sealed same-thread selector

Date: 2026-08-13
Status: Accepted owner implementation decision

Decision: add one application-owned `BoundedParallel` selector to
`AgentOrchestrator` for three immutable fixture-only/no-I/O scenarios. The
selector may retain multiple independent depth-one specialist `RuntimeRun`
values and accept their later events through one same-thread, task/run-addressed
multiplexing boundary. It does not create an OS thread, async executor,
provider-concurrency path, scheduler, worker, distributed service, or general
workflow engine. `AgentRuntime` and the sole/default `NativeAgentRuntime`
remain unchanged.

The exact scenarios are: Research plus Knowledge independently under
`ContinuePartial`, then Personal synthesis; Coding plus Security independently,
then dependent QA under `CancelDependentOnly`, then Personal synthesis; and
Cloud plus Systems independently, then dependent Security under specialist-lane
`FailFast`, then truthful Personal complete or partial synthesis. The
application owns the scenario graphs, synthetic fixtures, stable work-item
ordinals, dependency release, failure policy, and final ordered projection.
Completion timing never selects result order. These scenarios do not modify or
compose D-086 through D-090, and Workflow Automation cannot create a live task.

Delegation depth remains one. The production default is two active specialist
children; a trusted pre-start application choice may select one through the
hard maximum of three. Total specialist children are three, total tasks
including Personal are four, runtime-run attempts are five, retries are zero,
accepted events are eight per run, and the global runtime, generic
orchestration, workflow-event, and workflow-attribution journals are each
bounded at 32 as applicable. The root lease is 120 seconds and each admitted
child lease is 60 seconds capped by the root deadline. Deadlines use an
injected monotonic clock and cooperative before/after checks; they cannot hard-
preempt a synchronous runtime call already executing.

Every cataloged slot enters the bounded root-local pending set in ordinal order
and emits `Queued` exactly once, including immediately admitted, dependency-
blocked, and capacity-waiting slots. It is not an `AgentTask` until `Started`.
Hard-active, total, depth, duplicate, dependency, task/run, retry, and event
violations fail with typed behavior before the set is created. Active-capacity
release may admit an already-counted slot but never replenishes a budget.

Every admitted child has a unique task, exact definition and policy-profile
attribution, distinct `AgentExecutionContext` and `RuntimeRunIdentity`, bounded
output/event state, child deadline, application cancellation handle, and
separate task-memory identity. Research uses `ResearchWorkingMemoryV1` and
Knowledge uses `KnowledgeWorkingMemoryV1`; scenario A may use only their
existing live-grant-governed agent/task namespaces. Sibling reads fail,
task-temporary data is cleaned at terminal state, and memory is never copied
into results/synthesis, auto-written, proposed, or promoted. Coding, QA,
Security, Cloud, and Systems remain `MemoryDisabledV1` and writes are denied.
D-091 changes no production `MemoryStore`. No provider session registry exists;
distinct run identity is the complete supported session boundary.

The public ordered parallel status is exactly `Succeeded`, `Failed`,
`Cancelled`, `TimedOut`, or `Skipped`. An admitted timeout uses the existing
generic failed task/outcome with one new exact
`AgentTaskFailureCode::DeadlineExceeded`; it projects as `TimedOut` without
adding another generic task state or outcome variant. A skipped dependent was
never admitted and therefore has no synthetic task, run, context, cancellation
handle, or generic task outcome. Final Personal synthesis must identify every
expected source agent and ordered application-derived status and must disclose
failures, cancellations, timeouts, skipped dependencies, and unresolved issues.

Root cancellation propagates child-first in stable ordinal order and leaves no
live run, task memory, or pending task object only after every cancellation
succeeds. A failure pauses the ordinal sweep: prior successful cancels remain
terminal, the failing/later child or root/synthesis runs and root remain live in
closed-cancelling state, no queued/dependent task, timeout, synthesis, or root
terminal starts, and the next trusted ingress retries that ordinal. Unadmitted
slots are `Skipped(RootCancelled)`, never cancelled tasks. Individual
cancellation leaves independent siblings active under `ContinuePartial` and
`CancelDependentOnly`; under `FailFast`, it cancels the remaining specialist
lane. Retries remain zero except retrying an incomplete cancellation transition
does not retry model/runtime work.

Event deadline linearization validates exact identity/sequence and preflights
the prepared terminal/result/transfer/successor state, then samples the
monotonic clock immediately before `accept_event`; an accepted terminal event
wins with no post-accept timeout. A late-returning start or expired nonterminal
projects `TimedOut` only after cancellation succeeds. Cancel failure retains
the live task/run and starts nothing. Root expiry uses the same resumable sweep;
after success admitted active slots are `TimedOut(RootDeadlineExceeded)`,
unadmitted slots are `Skipped(RootDeadlineExpired)`, the generic root is
`Failed(DeadlineExceeded)`, and workflow state is `Failed` without synthesis.
Every `runtime.start` failure is typed/nonretrying and follows the exact
scenario policy, including dependent/synthesis starts, without an orphan.

Events and matching workflow-local attribution records are closed, content-
free, sequence-checked, and bounded. `Skipped` carries work-item ID, ordinal,
expected agent, and closed reason. Queued/skipped planned slots carry sealed
root/scenario/ordinal/expected-agent/profile attribution with no task/run/
runtime identity. `Started` exists only after a successful start and validated
returned run identity. Start error terminalizes the admitted task as
`Failed(RuntimeStartFailed)` and emits one `StartAttemptFailed` event/audit
outcome with root/scenario/subject/task/agent/profiles/runtime attempted
attribution and explicitly no `RuntimeRunIdentity`; it replaces Started plus
terminal. Live records use exact run attribution. Work-item `Progress` and the
separate content-free Personal-root `SynthesisProgress` each have a matching
audit outcome and coalesce to one applicable variant per run. The exact worst
success bound remains 19 paired records against 32; capacity is preflighted and
runtime events remain eight per run.

All untrusted content has exact scalar, byte, list, and aggregate caps: the
objective is 1,024 scalars/4,096 bytes, the fixture catalog 16,384 serialized
bytes, specialist raw output 6,144 scalars/12,288 bytes, three findings plus one
unresolved issue per child with 256 scalars/1,024 bytes each and 4,096 aggregate
text bytes, each child transfer 6,144 bytes, the three-entry ordered transfer
18,432 bytes, synthesis framing 2,048 bytes and total selected text 24,576
bytes, and final synthesis 4,096 scalars/8,192 bytes with a three-entry/2,048-
byte status table. Boundary, aggregate, escaping, and multibyte tests must prove
the encoded Native request remains at or below 65,536 bytes.

This decision adds no specialist spawning, recursion, nested workflow,
replenishing fan-out, scheduling, persistence, tool, policy permission,
approval dispatch, executor, provider, process, thread, dependency,
configuration, credential, filesystem/network/platform I/O, IPC/UI, external
runtime, distributed infrastructure, or device effect. Prior selectors retain
their existing one-active-child behavior. Consequential capabilities remain
separately gated.

Consequences: the exact
[`bounded parallelism ExecPlan`](docs/plans/2026-08-11-bounded-agent-parallelism.md)
is Ready and owner-selected at clean synchronized baseline `1f85264`, with
D-090's published completion marker complete and valid. The plan is not Active
until `agent-bounded-parallelism` begins. Implementation must stop if it needs
a runtime-trait or Native implementation change, thread/async/provider
concurrency, a reusable graph/queue/scheduler abstraction, a fourth specialist,
depth above one, a retry, a new authority or I/O boundary, or any file outside
the declared scope.

**Additive completion evidence (2026-08-13):** gate
`agent-bounded-parallelism` began and D-091 implementation verification now
passes. Focused library and public contracts are 41/41 each, strict Clippy and
formatting pass, all-target Rust passes 481 tests with one intentionally
ignored probe, and `npm run verify` passes. Independent code, architecture,
security, and technical-debt review is `PASS WITH ADVISORIES` with no D-091
completion blocker. Final post-documentation gates pass; deterministic
finalization completed and status reports `complete`, `valid: true`, and `PASS
WITH ADVISORIES`. No successor plan is owner-selected or Ready.

## D-092 - Keep the Command Center a deterministic frontend projection

Date: 2026-08-20
Status: Accepted owner implementation decision; source implemented, validation pending

Decision: add one lazy, reversible Command Center route driven only by the
closed frontend-owned `command-center-demo-v1` projection. It presents one
distinct `AgentOrchestrator`, all nine exact agent roles, five view-only
groups, seven bounded scenarios, a non-editable topology, synchronized grouped
structured view and relationship table, contextual inspector, bounded
activity, and persistent `DEMO MODE · SIMULATED AGENT DATA` disclosure.

Search, filters, selection, viewport controls, inspector, and activity remain
feature-local presentation state. No UI value is a trusted Rust identity and no
control invokes agent/task/workflow IPC, a provider/model/runtime/tool, policy,
approval, audit, persistence, network, filesystem, clipboard, permission, or
device effect. QA remains advisory and not `ApprovalManager`; Security
remains advisory and not `PolicyEngine`; Workflow Automation is not
`AgentOrchestrator`.

Add exact `@xyflow/react@12.11.3` (MIT) only inside one topology adapter and
exact `lucide-react@1.33.0` (ISC) through static named imports. The reviewed
lockfile consequence is 19 transitives. Production audit is zero
vulnerabilities; five pre-existing development-only advisories remain
unchanged. The route is lazy: initial JS+CSS increases by 1,118 gzip bytes and
the lazy Command Center JS+CSS is 84,889 gzip bytes, within the approved
budgets. No Rust/Tauri dependency, capability, CSP, command, or permission
changes.

Consequences: automated source evidence and independent review have no source
blocker. The ExecPlan remains Active because the mandatory real-browser/Tauri
viewport, input, focus, overflow, contrast, reduced-motion, and resize matrix
is Not run. This decision grants no live integration or later-milestone
authority and has no completion marker.

Additive owner clarification, 2026-08-20: the Command Center route may override
the shared chat-oriented page maximum and use the complete available main
column. This clarification is route-scoped; unrelated routes keep their shared
readable-width constraint. Graph framing must derive from the measured canvas
and visible deterministic topology, preserve manual viewport intent, and make
Fit View and Reset recompute current framing. Structured topology behavior and
all native, authority, dependency, and fixture boundaries remain unchanged.

Additive completion evidence, 2026-08-25: mandatory M5 rendered validation is
complete on the approved real-browser and native Tauri matrix. Browser Control
and Computer Use evidence covered the required viewport, resize, input, focus,
overflow, light/dark, reduced-motion, scroll, and screenshot states. One
contrast defect was corrected and revalidated. Manual browser zoom reached
125% (`devicePixelRatio` 1.25) and was reset. Gate
`native-multi-agent-command-center-prototype` is complete with a valid
fingerprint and `PASS WITH ADVISORIES`. These results validate only the
deterministic frontend projection and do not grant backend IPC, provider,
runtime, tool, policy, approval, audit, persistence, or live-data authority.

## D-093 - Accept separate safe branches for the deterministic Workflow Automation demonstration

Date: 2026-08-25
Status: Accepted owner clarification

Decision: for the native multi-agent end-to-end demonstration increment,
validate Workflow Automation's checkpoint-denial branch and the
application-owned manual safe-dispatch branch separately. Treat the absent
checkpoint-to-approval-to-manual-execution connection as a documented advisory,
not a completion blocker. Do not add or imply an approval-to-dispatch bridge.

This clarification aligns the acceptance scope with D-090's existing
`MAX_WORKFLOW_EXECUTABLE_TOOL_STEPS == 0` boundary. Checkpoint/tool proposals
remain non-executable and cannot issue a dispatch token. A complete A-D fixture
proposal may still yield one process-local take-once token that application code
maps only to an existing sealed selector. The two paths are not causally linked,
and neither path grants Workflow Automation task creation, approval, policy,
tool, execution, scheduling, persistence, provider, IPC, or device authority.

Any future combined approval-to-manual-dispatch capability requires a separate
owner-approved architecture decision, threat review, ExecPlan, and validation
increment. This decision changes acceptance evidence only and authorizes no
production implementation.

## D-094 - Define the first usable Personal Assistant v0 as text-only and empty-tool

Date: 2026-08-28
Status: Accepted owner product and architecture planning decision; no implementation authority

Decision: define the first genuinely usable Cortexa capability as one
foreground, explicitly user-initiated Personal Assistant text request at a
time. It may stream bounded text and produce one bounded final answer. Explicit
cancellation is terminal and idempotent; failures are closed and redacted.

Trusted Rust owns run, gateway-request, presentation, local support-correlation,
agent, instruction, provider/model, tool-set, limit, deadline, cancellation,
cleanup, and late-event policy. It validates and keeps gateway/provider
identities private. The synthetic WebView contract supplies no text; a future
real-content-v2 contract may eventually supply only one versioned bounded text
request after separate admission. Either contract may return only a Rust-issued
opaque presentation handle for polling or cancellation. Neither can select a
trusted agent, task, run, request, response, profile, runtime, workflow,
instruction, provider, model, tool set, endpoint, credential, data class,
limit, outcome, retry, or fallback.

Milestone 1 uses only `personal-assistant`, immutable synthetic instruction
profile `personal-assistant-text-v0@1`, the already selected synthetic-only
OpenAI-through-Cloudflare direction with `gpt-5.6-luna`, and `empty@1` with zero
tools and zero function calls. A future real-content profile must be version 2
and remains unselected. Every v0 lane has one model turn, one gateway request,
no automatic retry or fallback, and no files, attachments, persistence,
memory, scheduling, background autonomy, specialist delegation,
policy/approval dispatch, durable audit claim, filesystem access, process
launch, permission request, or device effect.

Implementation must preserve three separate milestones:

1. a live synthetic-text proof using only an application-owned fixed fixture;
2. the first usable private assistant, enabled only after exact real-content
   identity, ZDR, disclosure, logging, deletion, operations, and target-Mac
   evidence; and
3. any later action-taking or production product under new decisions.

The synthetic proof is not a usable personal assistant. The private assistant
is not an action-taking agent or production release. D-066/D-067 remain
synthetic-demo-only selections; this decision does not extend OpenAI or
Cloudflare to real prompts. D-068's long-lived demo service-token exception
also remains synthetic-only. D-061, explicit real provider/hosting authority,
and an approved non-demo owner-authentication decision remain real-content
gates; `store: false` is not ZDR. D-062/D-064 remain planned production
direction unless a later owner decision supersedes them.

D-061/D-064 disclosure applies before **every first external boundary**, not
only before model content. The V0-9 zero-body Access test therefore requires a
distinct trusted-Rust, one-use
`personal-assistant-access-auth-probe@1` acknowledgment after displaying the
exact authentication-only network disclosure; no probe socket may open first.
That acknowledgment cannot start model transport. The later synthetic provider
request separately requires V0-11's
`personal-assistant-synthetic-external-processing@1` visible WebView
acknowledgment and Rust admission. Neither acknowledgment is durable or
interchangeable.

For the synthetic lane, D-061's maximum seven-day gateway-log limit covers
Access authentication-request logs and Worker/runtime operational metadata.
Cloudflare's mandatory account/admin audit trail is a separate control-plane
record of operator/configuration actions, not runtime gateway request logging.
The current documented 18-month retention is accepted for the synthetic lane
only if the pre-traffic gate verifies that its fields contain no prompt,
output, request body, authorization value, credential secret, or provider
content and the disclosure continues to state Cloudflare operational
retention. Any field/class/retention change stops Stage C. This classification
does not authorize real prompts; V0-14 requires a fresh explicit outcome.

The synthetic model disclosure also freezes the current documented maximums:
OpenAI abuse-monitoring content up to 30 days, encrypted prompt-cache state up
to 24 hours, Cloudflare Free-plan Access authentication metadata for 24 hours,
and Cloudflare admin-action audit records for 18 months. It states that
`store=false` is not ZDR. V0-12/V0-13 must reverify the exact account/project,
endpoint/model eligibility, fields, and periods; any drift requires a new
disclosure version and owner approval before traffic.

Live configuration ownership is single and staged: V0-5 owns the exact
issuer/AUD/JWKS and fixed desktop origin/path, V0-8 owns the expected service-
token Client ID binding, V0-12 owns the provider secret, and only the exact
server binding `PA_V0_TRAFFIC_ENABLED=true` admits a new request. Missing,
malformed, or `false` denies. Setting it false or removing a route blocks new
admission but is not represented as aborting an active request; owned
desktop/Worker cancellation and deadline paths must tear down the original
request.

The current `InitialGatewayTurn` continues to advertise two fixed tool schemas
for its existing verified path and must not be silently repurposed. The v0
requires a distinct exact empty-tool turn. `NativeAgentRuntime` remains the
sole/default runtime and `AgentRuntime::start` remains the sole runtime-start
authority. A minimal application-owned Personal Assistant host may issue
process-local identity and invoke that existing start boundary without
creating an orchestrator task, delegation, memory path, inherent Native start,
or second runtime API. Provider transport remains outside `AgentRuntime`, and
the current `AgentOrchestrator`, agent memory, specialist workflows, Command
Center projection, Conversations mock, sealed Research/Knowledge lifecycle,
and acceptance workflows remain separate and unchanged.

The exact dependency-ordered program is
[`2026-08-28-personal-assistant-v0-program.md`](docs/plans/2026-08-28-personal-assistant-v0-program.md).
Its fourteen independently approval-bound plans separate local profile/lifecycle,
signed identity, local admission validation, no-traffic provisioning, HTTPS
dependency/transport, credential transfer, authentication rehearsal, fake
provider mapping, disclosure-bound Tauri presentation, provider provisioning,
live synthetic traffic, and real-content admission. The validated planning
closeout marks only V0-1 Ready for separate owner approval; every later plan
remains Blocked.

This decision adds no source, account, credential, signing state, Keychain
item, Cloudflare/OpenAI resource, provider request, network path, Tauri IPC,
disclosure UI, persistence, log, tool, device effect, release, or publication.
Each increment still requires exact owner approval and a fresh gate.

## Open decisions

| ID    | Topic                                                                                       | Required before                                      |
| ----- | ------------------------------------------------------------------------------------------- | ---------------------------------------------------- |
| O-002 | Workspace split between one Tauri crate and multiple Rust crates                            | Revisit before later modularization                  |
| O-003 | macOS target and hardware support beyond the provisional baseline                           | Any Intel, older-macOS, or production support claim  |
| O-006 | Exact Microsoft identity evidence, Azure deployment evidence, and future provider expansion | Before authentication or live gateway networking     |
| O-008 | Repository and distribution licensing                                                       | Before public distribution or external contributions |
| O-009 | Signing, notarization, credential ownership, and release authority                          | Before trusted public macOS distribution             |

O-007's policy decision is accepted in D-061. Provider-approved ZDR evidence,
the required disclosure, and all deployment and security gates still block
external transmission.

## D-095 - Define an Xcode-managed Developer ID recovery candidate without reopening V0-3

Date: 2026-08-28
Status: Accepted owner documentation-planning decision; no external action authority

## Context

The private owner does not intend to publish Cortexa to the Mac App Store. That
does not remove V0-3's need for a stable identity: its future fake-only
Keychain proof must distinguish the owner-controlled application from an
unsigned or unauthorized copy. D-075 already selected Developer ID Application
for outside-App-Store use, but TS-017 left the manual Certificate Assistant CSR
route unresolved and D-076 deferred the lane.

## Decision

Retain D-072 and D-075's stable Developer ID Application identity. Define one
future, owner-operated **Xcode-managed Developer ID Application** recovery path
as the candidate alternative to the failed manual Certificate Assistant CSR
flow. The detailed boundary is
[2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md](docs/plans/2026-08-28-personal-assistant-v0-xcode-developer-id-recovery.md).

The candidate is for a private target-Mac proof only. It does not select App
Store publication, distribution, notarization, a provisioning profile,
entitlement, self-signed identity, app-specific ACL, cloud-managed certificate,
or a different certificate class.

## Rationale

Apple documents Developer ID Application for Mac apps used independently of the
Mac App Store and documents Xcode as a supported creation path. An ad-hoc
signature has no signing identity, while a self-signed identity has no
Apple-backed revocation and would supersede the selected D-072/D-075 model.

## Consequences

- D-076 remains a deferral. TS-017 remains not determined; neither is resolved
  by this documentation decision.
- A later operational increment must receive separate explicit owner approval
  before opening Xcode, accessing Apple services, creating a certificate,
  changing Keychain state, signing a build, or recording private target-Mac
  evidence.
- The separately approved recovery execution may prove only identity
  availability, a non-exported owner-controlled private key, the fixed bundle
  identifier, and one locally signed build using that identity.
- The later V0-3 increment owns the fake-Keychain-item and
  signed-versus-unsigned-or-unauthorized-copy proof. A separate manual gate
  before V0-3 owns update/reinstall stability and
  renewal/revocation/compromise/removal handling. Every increment records only
  redacted evidence.
- Stop and return for new approval if any entitlement, provisioning profile,
  Tauri configuration, dependency, script, source change, filesystem key,
  key export, or real credential is needed.

## Alternatives considered

- **Skip signing for personal use**: rejected. Personal use changes
  distribution scope, not the future Keychain trust-boundary requirement.
- **Ad-hoc signature**: rejected. It has no signing identity.
- **Self-signed identity or app-specific ACL**: not selected. Either would
  supersede D-072/D-075 and needs a separately approved security design.
- **Repeat the failed Certificate Assistant CSR flow**: rejected by D-076 and
  TS-017.
- **Apple Support contact**: remains conditionally available under D-077 but is
  not selected by this planning decision.

## Supersedes or is superseded by

This supplements D-072, D-075, and D-076. It does not supersede their identity
selection or deferral, and it does not authorize V0-3.

## Later execution evidence

A separately approved execution on 2026-08-28 caused Xcode to create and list
one Developer ID Application certificate record. Sanitized CLI checks found no
usable code-signing identity. On 2026-08-29, the owner categorically confirmed
that Keychain Access shows the certificate with a private key beneath it. That
is owner-attested evidence that Keychain Access displayed local pairing, but it
does not by itself verify certificate identity/validity, code-signing usability,
non-exported owner control, or signing. After separate exact residual-risk
acceptance and approval, one bounded sanitized default-user-Keychain query
returned exactly one valid code-signing identity with the fixed Developer ID
Application label prefix. That establishes current scoped identity visibility,
not provenance, non-exportability, custody, a signed build, or TS-017's
historical cause. The owner separately reported no authorization prompt and no
visible state change. This evidence does not amend D-095, reopen D-076, resolve
TS-017, or authorize V0-3, another query, certificate, Keychain change,
import/export/revocation/removal, or signing.

### 2026-08-29 evidence-standard planning note

Later security review found that D-095's phrase “a non-exported
owner-controlled private key” is not retrospectively provable from the selected
pairing, identity-list, and signature evidence. Apple documents that some
Keychain certificates and keys can be exported. Present pairing and signing can
prove current owner-operated use, but not historical absence of export,
exclusive custody, or absence of a prior copy; the separate current-item
extractability attribute was not queried and remains `not_proven`.

At drafting time, the proposed
[Developer ID present-use and local signing proof](docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md)
did not itself amend this accepted decision. D-096 now additively governs
prospective evidence with closed owner-attestation, workflow-private-key-no-
export, present-use, and `not_proven` categories. D-072's stable signed-identity
selection, D-075's Developer ID Application class, D-076's deferral, the
historical privacy failure, and every separate operational approval remain
unchanged.

## D-096 - Use bounded present-use evidence without claiming historical private-key custody

Date: 2026-08-29
Status: Accepted owner documentation-only evidence decision; no operational authority

## Context

D-095 selected an Xcode-managed Developer ID Application recovery candidate
and described a later proof of a “non-exported owner-controlled private key.”
The subsequent recovery execution established local Keychain pairing and one
currently valid scoped code-signing identity, but those observations cannot
prove that the private key was never exported, copied, backed up, synchronized,
or compromised. They also cannot prove exclusive custody. The separate current-
item extractability attribute was not queried and remains `not_proven`; even a
current non-extractable attribute would not disprove a prior copy.

## Decision

For prospective Developer ID evidence, replace only the unattainable proof
interpretation with these closed categories:

- `owner_attested_known_private_key_export=none_known | known | declined`;
- `approved_workflow_private_key_export=not_performed`;
- `technical_nonextractability=not_proven`;
- `historical_absence_of_export=not_proven`;
- `exclusive_custody=not_proven`; and
- present-session use remains `not_run` until a separately approved bounded
  signing proof actually succeeds.

The owner accepts this evidence standard for future planning. This decision
does not supply the owner attestation, establish present use, authorize private-
key use, or convert any historical result to Passed. D-095's original text and
the recovery record remain intact as historical evidence. The screenshot/chat
privacy requirement remains Failed; the later-discovered
`getpwuid`/`opendirectoryd` boundary remains Pending; the signed build remains
Not run; and the active recovery result remains `FAIL`.

## Rationale

Present pairing, identity enumeration, and one eventual signature can establish
current visibility or use, but not a negative fact about all prior copies or
custody. Explicit `not_proven` categories and a bounded owner attestation are
truthful, reviewable, and do not invite private-key export or broader Keychain
inspection as substitute evidence.

## Consequences

- The evidence-standard documentation milestone in
  [2026-08-29-v0-developer-id-present-use-local-signing-proof.md](docs/plans/2026-08-29-v0-developer-id-present-use-local-signing-proof.md)
  is accepted.
- At D-096 acceptance time, the future local signing milestone remained Blocked
  by the unresolved terminal-failed-gate disposition, Pending Open Directory boundary, executable-build-
  script containment, exact sanitizer and static review, expected-team and
  signer binding, bounded cleanup, all manual gates, and a fresh explicit one-
  attempt owner approval.
- No Keychain, Apple, Xcode, build, signing, private-key, prompt, cleanup,
  provider, network, source, dependency, configuration, hook, or external
  operation is authorized by this decision.
- No new increment or second post-increment gate began under D-096. At that
  checkpoint, the active recovery gate remained `active` because then-current
  tooling could not encode its truthful terminal `FAIL` result. D-097 later
  resolves only that representation gap.

## Supersedes or is superseded by

This additively governs only prospective interpretation of D-095's historical
“non-exported owner-controlled” evidence phrase. It does not rewrite or
supersede D-072's stable signed identity, D-075's Developer ID Application
class, D-076's deferral, D-095's recovery candidate, TS-017, the historical
privacy failure, or any separate operational-approval requirement.

## D-097 - Record truthful terminal gate failure without completion authority

Date: 2026-08-29
Status: Accepted owner same-active-gate governance implementation decision

## Context

The Xcode Developer ID recovery report truthfully computes `FAIL` because it
contains immutable Failed evidence plus required Not run and Manual verification
pending results. The v1 gate could encode only `active` or a passing `complete`
state. Leaving the increment active forever caused a Stop-hook loop, while a
passing marker would falsify evidence. The owner explicitly authorized one
bounded recovery inside the existing active increment and prohibited a second
increment.

## Decision

Add an exact state-schema-v2 `failed` variant and a `close-failed` command.
`close-failed` accepts only a structurally valid report whose declared and
computed result is exactly `FAIL`. It records the original baseline, current
HEAD, report path and digest, workspace fingerprint, and exact next-increment
readiness. It never writes `POST_INCREMENT_GATE_COMPLETE` or any other
completion marker.

The state machine is closed:

- `active` plus a passing report may use `finalize` and become `complete`;
- `active` plus an exact failing report may use `close-failed` and become
  `failed`;
- `failed` can be reclosed only for the same increment, baseline, report path,
  and unchanged HEAD after complete revalidation of the revised report and
  workspace;
- `failed` can never use `finalize`, restart the same increment, or become
  `complete`; and
- `complete` can never use `close-failed`.

A valid failed state lets the Stop hook end. Unreclosed report or workspace
drift, conflicts, suspicious paths, or malformed state make it invalid and
restore the Stop block. Any finding with `blocks_next_increment: true` forces
readiness `Blocked`. In the checkout retaining the ignored state, a different
increment can begin only from valid non-Blocked failed evidence, only after the
workspace is clean, and only with separate owner approval. The current recovery
report remains `Blocked`, so this decision does not make a successor Ready;
repository policy and owner authority prohibit bypassing it from a fresh clone.

Report schema remains v1. State schema becomes v2 while legacy v1 active and
complete states remain readable. Existing passing refinalization behavior is
preserved; the new prohibition is specifically failed-to-complete promotion.
The hook now enforces the template-required `Scope and boundaries` section.

## Security and operational limits

The ignored state is checkout-local workflow evidence, not authentication,
authorization, or durable audit. Its digests detect ordinary unreclosed drift,
but a same-user process that can rewrite the state and repository can forge
them, and a fresh clone does not inherit the ignored state. Readiness is
admission evidence only and never substitutes for owner approval.

This decision adds no product source, dependency, workflow, capability, CSP,
permission, provider, credential, Keychain, Apple, signing, filesystem product
effect, network, persistence, release, publication, or new increment authority.
The historical privacy failure, pending Open Directory disposition, and every
future signing blocker remain unchanged.

## Consequences

- The current increment can end truthfully as valid `failed`/`FAIL`/`Blocked`
  without a completion marker.
- Failed evidence remains visible and cannot be laundered into completion.
- Reclosure after HEAD changes is deliberately rejected because the current
  report inventory is relative to the working tree; any future post-commit
  supersession needs a separate cumulative-evidence design.
- A failed record remains valid after committing identical reviewed contents,
  but its Blocked readiness still denies another gate.
- Focused tests must cover exact state keys, PASS rejection, failed-to-complete
  denial, Stop behavior, drift, conflicts, suspicious paths, readiness
  consistency, clean successor admission, same-HEAD reclosure, post-commit
  reclosure denial, legacy-state compatibility, and report sections.

## Alternatives considered

- Fabricate or downgrade failing evidence: rejected as false.
- Leave the gate active indefinitely: rejected because it cannot represent the
  observed terminal outcome.
- Delete or bypass ignored state: rejected because it discards workflow
  evidence and weakens the fail-closed process.
- Start a second recovery increment: rejected by the owner's exact scope.
- Add a general abandonment, override, or cumulative post-commit supersession
  mechanism: deferred as broader than the smallest truthful recovery.

## Supersedes or is superseded by

This resolves only D-096's recorded terminal-failed-gate tooling blocker. It
does not supersede D-072, D-075, D-076, D-095, D-096's evidence standard,
TS-017, the recovery report's `FAIL`, or any product or operational gate.

## D-098 - Permit one additive exact-target disposition of the published D-097 Blocked failure

Date: 2026-08-29
Status: Accepted owner exceptional same-terminal-record recovery implementation decision

## Context

D-097 truthfully closed the Xcode recovery record as valid
`failed` / `FAIL` / `Blocked`. PR #84 later published the identical reviewed
contents at `a417e5f1c1c602b917ca27c65af71480e3db6a45`. The original Failed,
Pending, and Not-run evidence cannot be rewritten, and its Blocked readiness
correctly denies another `begin`. D-097 intentionally deferred post-commit
cumulative-evidence supersession, so a new planning gate cannot lawfully design
its own admission path.

The owner therefore authorizes one bounded source recovery against the same
terminal record. It must not start a second increment, delete or replace the
ignored state, alter the predecessor report, fabricate completion, or use a
fresh checkout to bypass the recorded disposition.

## Decision

Add state schema v3 while retaining v1/v2 compatibility. Change only the
top-level `schema_version` from 2 to 3, preserve every other v2 failed-state
evidence field unchanged, and add one exact `successor_disposition` object with
these closed keys:

- `disposition_id`;
- `predecessor_state_sha256`;
- `report_path` and `report_sha256`;
- `baseline_commit`;
- `workspace_fingerprint`;
- `successor_increment_id`;
- `quality_gate`;
- `next_increment_readiness`; and
- `allowed_paths`.

The hook exposes one argument-free `record-failed-disposition` command. Source
constants bind it to the D-097 failed increment and report, baseline
`a417e5f1c1c602b917ca27c65af71480e3db6a45`, recovery ID and report, exact
22-path recovery inventory, and sole successor
`personal-assistant-v0-signing-security-prerequisite-planning`. No caller may
choose any identity, report, path, readiness, or successor.

The two path contracts are distinct. The source-bound
`FAILED_DISPOSITION_RECOVERY_ALLOWED_PATHS` is the exact 22-path change set
required to record D-098. The `allowed_paths` stored inside
`successor_disposition` is the separate exact 15-path ceiling for the later
documentation successor; it is not the recovery inventory and cannot authorize
changes during D-098.

The command requires the predecessor structure and report digest to remain
unchanged, the recovery report to compute `PASS` or
`PASS WITH ADVISORIES`, every required automated and manual result to Pass,
recovery readiness to be non-Blocked, no next-blocking finding, the complete
diff to match the source-bound 22-path recovery inventory, and no conflict or
suspicious path. It writes no
completion marker and does not change the predecessor's `FAIL` or `Blocked`
readiness. Exact replay may be idempotent; altered replay fails closed.

If the disposition remains valid and the workspace is clean, a later `begin`
may admit only the exact successor and must carry the validated lineage as
`predecessor_disposition`. It does not start that successor automatically, and
readiness never substitutes for separate owner approval.

## Rationale

An additive, exact-target lineage lets the repository acknowledge new passing
governance evidence without laundering historical failure. An argument-free,
source-allowlisted transition is smaller and safer than a generic waiver,
abandonment, override, or caller-selected supersession interface. Carrying the
lineage into the active successor prevents admission from silently discarding
the failed predecessor.

## Consequences

- The recovery itself is not a new increment and must not call `begin`,
  `finalize`, or `close-failed`.
- A successful recovery report and schema-v3 state create no completion marker
  and do not make the failed Xcode recovery complete.
- Only the exact documentation-only signing-security prerequisite planning
  successor can become admissible, and it remains a separate owner-controlled
  action.
- The historical screenshot/privacy failure, Pending Open Directory boundary,
  Not-run signed build, D-076, TS-017, build-process containment, and every
  Apple, Keychain, signing, credential, provider, product, and external gate
  remain unchanged.
- The ignored state remains same-user writable, checkout-local workflow
  evidence rather than authentication, authorization, or durable audit.

## Alternatives considered

- Rewrite the original report or readiness: rejected as evidence laundering.
- Mark the failed increment complete: rejected as false.
- Delete the ignored state or use a fresh clone: rejected as bypass.
- Add a generic failed-state override: rejected as excessive authority.
- Begin the documentation successor directly: rejected because D-097 readiness
  is Blocked.
- Leave the queue permanently stranded: rejected because bounded additive
  evidence can preserve the failure while admitting one exact planning task.

## Supersedes or is superseded by

D-098 additively implements only the cumulative-evidence design deferred by
D-097. It does not supersede D-072, D-075, D-076, D-095, D-096, D-097's
terminal failure, TS-017, or any operational prerequisite.

## D-099 - Limit the admitted signing-security successor to prerequisite documentation

Date: 2026-09-01
Status: Accepted owner-authorized documentation-only implementation decision

## Context

The valid D-098 schema-v3 disposition carries the published D-097 terminal
failure into exactly one admitted successor:
`personal-assistant-v0-signing-security-prerequisite-planning`. Its fifteen-path
allowlist admits a narrow documentation closeout but does not clear the Failed
screenshot/privacy finding, the Pending `getpwuid`/`opendirectoryd` boundary,
the Not-run signed build, or any operational signing blocker.

## Decision

Use the admitted successor only to document four separately approvable future
prerequisites: P1 privacy-safe categorical evidence, P2 an account-directory
boundary disposition or contained resolver, P3 fail-closed executable
build-child containment/observation, and P4 an application-owned immutable
signer contract with a no-argument closed-output sanitizer. Each is Proposed or
Blocked until its own exact plan, review, manual gates, and owner approval.

The current increment may alter only D-098's exact fifteen documentation paths.
It must retain `predecessor_disposition` lineage, preserve every predecessor
status and digest claim, and state that a documentation completion marker, if
earned, applies only to this plan. It never constitutes Keychain custody,
private-key safety, signing, build containment, Apple-system, product, or
external-system evidence.

## Consequences

- No Apple, Xcode, Keychain, certificate, private-key, signing, build,
  credential, provider, network, product, Git publication, or external action
  is authorized.
- The completed planning record may make no operational successor Ready.
- Future operational work stops on a prompt, selector ambiguity, raw-evidence
  leak, unaccepted directory boundary, unproven child containment, or missing
  immutable signer binding; it cannot retry or downgrade the historical record.

## Alternatives considered

- Treating current identity visibility as signing readiness: rejected; it is
  not historical custody, signer binding, or an operational proof.
- Using this documentation increment to rerun or resolve prior evidence:
  rejected; the original query approval is consumed and the failures are
  immutable.
- Broadening the allowlist to implement a sanitizer or product capability:
  rejected; that would exceed D-098 and needs a separate approved increment.

## Supersedes or is superseded by

D-099 adds bounded planning guidance only. It does not supersede D-072, D-075,
D-076, D-095, D-096, D-097, D-098, TS-017, or any operational prerequisite.

## D-100 - Require closed source-minimized evidence for future security operations

Date: 2026-09-01
Status: Accepted owner-authorized documentation-only governance decision

## Context

D-099 identifies P1 privacy-safe categorical evidence as the first separate
prerequisite before any later security or signing operation. The historical
D-097 screenshot/privacy failure demonstrates that asking for a private capture
and redacting or summarizing it later is not an adequate boundary. Free-text
owner descriptions, copied command output, and arbitrary report fields can also
carry identifiers or sensitive content even when no secret was intended.

The repository needs a closed vocabulary before another plan can propose
operational evidence. This decision defines that vocabulary only. It neither
collects evidence nor implements a sanitizer or external operation.

## Decision

Define `evidence_privacy_v1` as a three-field documentation contract:

- `protocol_version` is exactly `evidence_privacy_v1`;
- `check_id` is one exact enum member owned by the separately approved future
  plan; and
- `outcome` is one exact enum member from that check's predeclared allowlist.

There is no extension map, optional free-text note, arbitrary key, caller-
selected identifier, timestamp, count, path, diagnostic, excerpt, or raw error.
A future plan may use applicable closed outcomes such as `observed`,
`not_observed`, `not_run`, `pending`, `unavailable`, and `boundary_failed`, or a
narrower check-specific set. Every operational check's allowlist must include
`boundary_failed`; the check and all other outcomes must be fixed and reviewed
before any operation begins.

`check_id` and `outcome` are repository-reviewed static lowercase ASCII tokens
matching `[a-z][a-z0-9_]*` and bounded to 64 and 32 bytes respectively. Their
spellings and allowlists are not target-derived; `check_id` is fixed before
inspection, while only selection of one already allowed outcome may depend on
the bounded predicate. A future serialized form is exactly one compact ASCII
JSON record, at most 256 bytes, with exactly the three unique keys in declared
order, string values only, and no leading/trailing whitespace or bytes.
Comparison is exact, with no trimming, case folding, Unicode normalization,
confusable mapping, or aliases.

Each check represents one directly observable or deterministically established
bounded predicate. Check/outcome names and semantics cannot claim approval,
authorization, safety, readiness, broad verification, exclusivity, historical
absence, or permission to proceed. The record has no standalone provenance,
freshness, authentication, authorization, audit, or readiness meaning. A future
trusted consumer must privately bind it to one separately approved plan, check,
and attempt, accept it once within that attempt's fixed window, and reject
duplicate, replayed, cross-plan, pre-admission, or late records.

Evidence must be minimized at its source. Raw bytes may not cross the approved
local inspection boundary and then be redacted for chat, Git, reports, logs,
tests, screenshots, recordings, attachments, clipboard transfers, or ordinary
CI. An automated boundary is compliant only when separately approved trusted
local code discards raw data before returning the closed record. A manual check
uses one prewritten question and one closed answer; it accepts no attachment,
copied output, screenshot, recording, or explanatory free text. If either
method cannot satisfy this rule, the check remains Not run.

Unknown versions, checks, outcomes, keys, prompts, malformed records,
ambiguity, unexpected output, or unapproved side effects fail closed. If
prohibited evidence appears, stop without retry; do not copy, quote,
retransmit, attach, or summarize it. Record only the applicable
`boundary_failed` category, preserve historical truth without claiming that an
immutable record was erased, and require a separate incident/disposition plan
before related operational work resumes.

## Consequences

- Screenshots, recordings, transcripts, raw stdout/stderr, logs, traces,
  diagnostic/exception text, raw/sensitive/dynamic/target-derived identifiers
  or certificate/account metadata, fingerprints, serials, target-derived
  labels, personal/private paths, credentials, secrets, and sensitive content
  are prohibited evidence. Static repository-owned protocol literals and
  necessary preexisting governance metadata not derived from the target remain
  permitted documentation.
- These prohibitions govern the payload/result of a separately approved
  security or signing check. Necessary non-sensitive repository validation and
  governance metadata that are not derived from the inspected security target
  remain separate from the protocol record.
- A passing P1 documentation closeout establishes only the protocol. No
  sanitizer, operation, runtime, IPC, filesystem, Apple, Keychain, signing,
  build, provider, product, or external-system boundary exists because of it.
- D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker; its
  original report and digests remain unchanged, and its historical privacy
  finding remains Failed. The Open Directory boundary remains Manual
  verification pending, and signing remains Not run.
- P2 account-directory, P3 executable-build-child containment, P4 immutable
  signer binding, and every operational successor remain Proposed/Blocked.
- Every future operational plan must bind its own exact check/outcome tables,
  non-authorizing predicate semantics, private attempt/freshness context, threat
  model, local minimization boundary, manual gates, stop conditions, review, and
  separate owner approval.

## Alternatives considered

- Capture then redact: rejected because sensitive data already crossed the
  intended boundary.
- Owner-supplied screenshots or copied output: rejected because they recreate
  the historical privacy failure and allow uncontrolled metadata.
- Free-text attestations: rejected because their content and bounds are open.
- A generic evidence object with optional fields: rejected because it creates
  an extensible data channel.
- Implement a sanitizer in this increment: rejected as executable and
  operational work outside the approved documentation scope.

## Supersedes or is superseded by

D-100 implements only the P1 documentation protocol identified by D-099. It
does not supersede D-072, D-075, D-076, D-095, D-096, D-097, D-098, D-099,
TS-017, any historical evidence, or any operational prerequisite.

## D-101 - Prohibit explicit account-directory resolution in future signing-security checks

Date: 2026-09-01
Status: Accepted owner-authorized documentation-only governance decision

## Context

The consumed `keychain_identity_v1` wrapper called `pwd.getpwuid()` to derive an
account home before constructing a default-Keychain path. Post-execution review
found that the lookup may have invoked `opendirectoryd`, materialized a full
account record, consulted configured local or remote directory systems, and
used OS-owned cache/socket/log state. The wrapper emitted no account field and
remote traffic is not proven, but the boundary was not separately disclosed or
accepted. It remains Manual verification pending, and the consumed query must
not run again.

D-099 permits P2 to choose either a contained future design or a separately
approved exact residual-risk acceptance. D-100 now supplies the required closed
evidence policy. The smallest security-first P2 decision is to prohibit
explicit resolution at the application boundary and separately require
platform-effect evidence. The application policy alone does not establish
operational account-directory containment or control over operating-system
internals.

## Decision

Adopt the conceptual documentation invariant
`ExplicitAccountResolutionPolicyV1::Prohibited` for any future signing-security
checker. It is an application-resolution policy, not a claim of OS-level
containment, implemented type, runtime DTO, platform API selection, or
operational authorization.

Future trusted application code:

- takes no account, user, home, path, Keychain, profile, runtime, task, run,
  workflow, caller, WebView, model, environment, or command-output identity;
- does not call or wrap account/passwd databases, Open Directory, directory-
  service clients, numeric UID/eUID-to-account derivation, login/session/
  console-user resolution, home/standard/current/temporary/configuration-
  directory derivation, search-list resolution, shell expansion, account
  environment values, or constructed account paths;
- does not materialize an account-home or Keychain path in argv, environment,
  logs, errors, events, evidence, tests, or ordinary CI;
- uses no subprocess, search-list enumeration, generic filesystem authority,
  path fallback, or retry; and
- may proceed only through one in-process application-owned, unexported,
  non-serializable, attempt-bound, operation-specific opaque no-input wrapper
  over an adapter-private platform-issued reference whose supported static
  contracts are reviewed before implementation; the wrapper cannot cross IPC,
  persistence, logs, or evidence, grants no generic enumerate/read/write/
  delete/sign authority, and cannot be reused across attempts. Cleanup is
  attempted on every terminal path. Success destroys the native reference;
  failure retains the wrapper and reference in private adapter-owned quarantine
  until process exit and blocks replacement, retry, reuse, exposure, or early
  ownership loss.

Trusted application code owns the fixed policy and operation-specific wrapper;
the target-gated platform adapter privately owns and destroys any native
reference. Authoritative static evidence must establish three independent
contracts: the selected API requires no application-supplied account identity
or home/path resolution; exact scope provenance is one fixed application
credential domain rather than ambient current-user, login-session, default-
Keychain, default-search-list, current-directory, or environment authority; and
the selected operation excludes account-record acquisition plus account/passwd,
Open Directory, or directory-service resolution.

The exact cache, log, socket, trust-service, process-metadata, and possible-
network effects each require their own one-predicate D-100 disposition from the
P2 plan. An absent, ambiguous, unsupported, or deprecated-without-a-supported-
replacement contract produces only `contract_unproven`; private policy state,
not that non-authorizing evidence token, keeps the successor Blocked. Runtime
observation, tracing, packet capture, log inspection, or absence of visible
effects cannot prove that undocumented OS internals never use those resources.

The explicit application-resolution prohibition and the no-application-input
and exact-scope-provenance contracts are non-waivable under D-101. A later
separate exact decision and owner approval may disposition only specifically
disclosed OS-internal effect uncertainty; it cannot authorize caller,
environment, account, home, or path input, explicit resolution, ambient/default
scope, fallback, retry, or reuse. Under the containment path, operational P2
remains Blocked unless every input, provenance, directory, cache, log, socket,
trust-service, process-metadata, and network predicate is satisfied.

There is no automatic fallback to owner acceptance. Any future proposal to
accept residual OS-internal effects requires a separate documentation decision,
exact disclosure, and separate owner approval. It cannot alter the historical
finding or authorize a rerun.

Future review evidence uses only the D-100 `evidence_privacy_v1` tables recorded
in the P2 plan. Every check remains non-authorizing, contains
`boundary_failed`, and is privately bound to one plan/check/attempt. Source
review additionally requires a fixed commit and complete predeclared adapter,
reachable-helper, wrapper, FFI, and dependency-source inventory. No P2 evidence
is collected in this increment.

## Consequences

- The P2 documentation policy can pass without pretending that it closes the
  operational P2 boundary, that a suitable API exists, or that OS-internal
  effects are absent.
- Future implementation remains Blocked pending every exact input, provenance,
  platform-effect, and source predicate, an exact adapter plan, focused tests,
  security review, and owner approval.
- The historical `getpwuid`/`opendirectoryd` finding remains Manual verification
  pending; D-097 remains `failed` / `FAIL` / `Blocked` without a completion
  marker, and its report/digests/findings remain unchanged.
- No Apple, Xcode, Keychain, Security.framework, certificate, signing, build,
  credential, provider, network, product, or external-system action is
  authorized.
- P3 build-child containment, P4 immutable signer binding, V0-3, and every
  operational successor remain Proposed/Blocked.

## Alternatives considered

- Reuse `getpwuid` with better disclosure: rejected for this containment path;
  it still materializes account-directory state.
- Read `HOME`, username, shell state, or a caller-supplied path: rejected as
  mutable or caller/environment-selected authority.
- Construct a conventional account/Keychain path: rejected because it requires
  account identity and creates filesystem/path authority.
- Use a subprocess or default search-list enumeration: rejected because it
  broadens process, path, and metadata boundaries.
- Treat no visible runtime effect as proof: rejected because observation cannot
  establish universal absence of undocumented OS behavior.
- Accept residual effects in this decision: rejected because the owner approved
  planning only, not residual-risk acceptance or operation.

## Supersedes or is superseded by

D-101 implements only the application-resolution prohibition and independent
input, exact scope-provenance, and effect-suite proof gates identified by
D-099's P2 documentation work. It does not supersede D-072, D-075, D-076,
D-095, D-096, D-097, D-098, D-099, D-100, TS-017, any historical evidence, or
any operational prerequisite.

## D-102 - Require pre-effect build-child containment before a signing proof

Date: 2026-09-01

Status: Accepted — owner-authorized documentation-only governance decision

## Context

D-099 requires a fail-closed P3 executable-build-child prerequisite before a
signing proof. The ordinary build graph reaches npm lifecycle hooks, Cargo build
scripts, Vite/Tauri descendants, compilers, and linkers. Routing outputs or
caches under a disposable root, clean Git status, post-hoc scans, and process-
group shutdown do not stop an outside-root write, undeclared connection,
ambient host-data read, or a descendant that escapes/reparents. Historical
`sandbox-exec` research does not supply complete membership, shutdown/reaping,
or effect control as a sole primitive. D-100 requires source-minimized closed
evidence, and D-101's independent account-directory boundary remains Blocked.

## Decision

Adopt the conceptual documentation invariant
`BuildChildContainmentPolicyV1::Required`. It is not a type, API, primitive,
controller, command, configuration, or operational authorization. A future
private `ContainedBuildAttemptV1` may be created only by trusted
application-owned code in a separately approved plan. It is no-input, opaque,
non-serializable, non-cloneable, attempt-bound, and cannot cross IPC,
persistence, logs, tests, or evidence.

The future boundary must freeze one reviewed transitive graph to its commit,
lockfiles, and pinned toolchains; bind shell-free fixed absolute executables and
literal arguments; close inherited environment and file descriptors; and admit
no caller/model/WebView/environment-selected executable, task, run, profile,
runtime, workflow, working directory, graph, retry, network policy, or cleanup
target. It must constrain ambient host-data reads and filesystem writes. Before
an effect, it must deny outside-root writes and undeclared network effects.
Post-hoc observation never upgrades a failed or unproven boundary.

Membership must cover every owned descendant across fork, exec, reparenting,
`setsid`, and `setpgid`; a process group is only a shutdown adjunct. Fixed
deadlines and cancellation are terminal. Every terminal path must terminate the
owned graph, reap each direct child, close pipes, and await race-free quiescence;
late effect/event acceptance fails closed. Cleanup must use descriptor-bound,
non-following ownership. Cleanup failure preserves private quarantine until
process exit and blocks retry/replacement without widening cleanup authority.

Future evidence uses only the 32 predeclared `evidence_privacy_v1` source and
contract categories in the P3 plan. It contains only a fixed ID, allowed closed
outcome, and fixed predicate; raw output, command lines, paths, process/account/
host identifiers, traces, packets, secrets, and free text are prohibited. An
incomplete source inventory is `boundary_failed`; an absent, ambiguous, or
unsupported contract is `contract_unproven`; neither grants authority.

## Consequences

- P3 remains Blocked until a later supported no-new-dependency target-Mac
  primitive, controller, synthetic proof, and disposable no-sign build proof
  each have separately approved plans and passing evidence.
- P4 immutable signer binding, signing, V0-3, and every operational successor
  remain Blocked. This documentation result cannot substitute for containment.
- D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker; its
  Failed privacy finding, Pending Open Directory boundary, and Not-run signing
  state are untouched.
- No build, process, network, filesystem, Apple, Xcode, Keychain, certificate,
  signing, credential, provider, product, dependency, or external action is
  selected or authorized.

## Alternatives considered

- Treat output/caches/logs under a disposable root as containment: rejected;
  they do not control reads, outside-root writes, connections, or descendants.
- Use process groups as the authoritative child graph: rejected; descendants
  can escape sessions/groups or reparent.
- Use post-hoc scanning as proof: rejected; it observes after effect and may
  miss effects.
- Use `sandbox-exec` as the sole target-Mac primitive: rejected; it does not
  establish all required membership, reaping, and effect contracts.
- Add a dependency or perform an exploratory probe now: rejected; no primitive
  selection or operational work was approved.

## Supersedes or is superseded by

D-102 implements only D-099's P3 documentation prerequisite. It does not
supersede D-072, D-075, D-076, D-095, D-096, D-097, D-098, D-099, D-100, D-101,
TS-017, any historical evidence, or any operational prerequisite.

## D-103 - Select no build-child containment primitive from the frozen P3-2 candidate set

Date: 2026-09-02

Status: Accepted — owner-authorized documentation-only negative selection

## Context

D-102 requires one supported target-Mac primitive to deny unsafe filesystem and
network effects before they occur while retaining authoritative ownership of
the complete build descendant graph through detachment and reparenting. The
owner approved a static authoritative-source review of one frozen candidate
set. The review ran no build, primitive, child, probe, target-Mac inspection,
Apple operation, or signing action. Approved read-only public documentation
access was the sole external contact; no authenticated or state-changing
external-system operation ran.

The only plausible deep-review candidate combined an application-owned App
Sandbox build helper with closed public supervision primitives: `posix_spawn`,
`waitpid`, `kqueue` `EVFILT_PROC`, `setpgid`, and process-group signaling. Apple
documents App Sandbox as entitlement-configured and documents helper
inheritance through entitlement-bearing app/helper code signatures; Developer
ID distribution additionally uses Developer ID signing. The review does not
equate development/ad hoc signing with P4's later Developer ID signer-binding
proof. Any prerequisite signing state or new entitlement nevertheless fails the
approved independent P3-2 eligibility rule. The reviewed direct-child wait/
reap, known-PID event observation, and mutable process-group contracts also do
not establish complete application-owned descendant membership, containment-
wide termination, and race-free quiescence across detachment or reparenting.

The fixed negative controls remain insufficient: deprecated/private
`sandbox-exec` profiles do not supply the full lifecycle or exact network
contract; process groups neither deny effects nor establish durable graph
membership; and post-hoc output-root scans occur after effects. Privileged
system/Endpoint/Network Extension and virtual-machine/container classes require
entitlements, signing, privilege, user/global state, or guest resources in the
reviewed extension/VM routes. No exact supported OS-shipped macOS 14+ container
candidate contract was identified, so that branch remains
`contract_unproven`.

## Decision

Select **no eligible candidate in the reviewed set**. This is a bounded
negative decision, not a claim that macOS has no possible containment
mechanism. It does not admit a candidate outside the frozen set, weaken a D-102
predicate, accept residual risk, or authorize P3-3.

The frozen set is:

- deep review:
  `app_sandbox_build_helper_plus_libsystem_supervision_v1`;
- negative controls: `sandbox_exec_supervisor_v1`,
  `posix_process_group_supervisor_v1`, and
  `output_root_posthoc_scan_v1`; and
- scope eligibility only: `privileged_system_extension_v1` and
  `virtual_machine_or_container_v1`.

Every D-102 eligibility predicate is conjunctive. Missing, ambiguous,
deprecated, conflicting, inferred, or merely empirical evidence remains
`contract_unproven`. The ten P3-3 implementation-source review checks remain
`not_run` because no controller source exists. Static public citations and
closed dispositions carry no runtime, product, signing, or successor authority.
Only the App Sandbox composition entered the D-100 contract review. The
negative controls retain D-102's prior exclusions and the scope-only classes
failed the independent eligibility screen; neither is represented as a D-100
candidate attempt. The one static attempt privately binds candidate, check,
and attempt identity. Unexpected source, provenance loss, target-derived data,
or a malformed record is `boundary_failed` and stops without retry.

## Consequences

- P3-2 closes truthfully as a negative documentation selection; it does not
  establish operational containment.
- P3-3, P3-4, P3-5, P4, signing, V0-3, and every product or external successor
  remain Blocked. No successor is Ready.
- A future proposal must first obtain separate owner approval to change an
  eligibility constraint or introduce a new candidate and architecture. It may
  not silently add an entitlement, signing dependency, privilege, dependency,
  VM/container, system extension, external resource, or residual-risk waiver.
- D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker. Its
  report and digests, historical Failed privacy finding, Pending Open Directory
  boundary, and Not-run signing results remain unchanged.
- No source, dependency, lockfile, configuration, capability, CSP, permission,
  entitlement, workflow, hook, build, process, filesystem, network, Apple,
  Keychain, signing, credential, provider, product, or external state changes.

## Alternatives considered

- Accept App Sandbox helper signing as harmless setup: rejected because it is
  the circular authority that P3 is meant to protect before signing.
- Combine App Sandbox with wait, kqueue, or process groups and infer full graph
  ownership: rejected because no reviewed public contract establishes it.
- Use Endpoint Security, Network Extension, or another system extension:
  rejected because it changes privilege, entitlement, signing, user-approval,
  and persistent system-state boundaries.
- Use a VM or container: rejected because the reviewed VM route adds an
  entitlement, guest image, and storage, while no exact qualifying container
  contract was identified. Neither branch satisfies this frozen review.
- Relax one mandatory contract or accept residual risk: rejected; outside this
  approved increment and incompatible with D-102.

## Supersedes or is superseded by

D-103 applies D-102 to only the frozen P3-2 candidate set. It does not
supersede D-072, D-075, D-076, D-095, D-096, D-097, D-098, D-099, D-100, D-101,
D-102, TS-017, any historical evidence, or any operational prerequisite.

## D-104 - Separate identity-free sandbox activation from later Developer ID signer binding

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only bootstrap-trust decision

## Context

D-103 correctly rejected the only deep-review App Sandbox helper composition
under its then-independent no-entitlement/no-prerequisite-signing eligibility
rule. Its result remains correct for that frozen candidate set. The decision
also records independent unresolved contracts: App Sandbox plus direct-child
waiting, process events, and process groups does not establish D-102's complete
application-owned descendant membership, containment-wide termination, or
race-free quiescence after detachment or reparenting.

Apple documents that an ad-hoc code signature is sealed without a signing
identity, while App Sandbox relies on entitlements carried in a code signature.
Apple's helper guidance also describes local ad-hoc signing during helper
preparation before an embedded product replaces that identity. These sources
support a narrow distinction between the mechanics of a disposable sandbox
activation seal and P4's later Developer ID identity proof. They do not prove
that the planned helper is a qualifying containment primitive or that its first
build is safely bootstrapped.

## Decision

Define two conceptual, non-runtime, non-authorizing classes:

- `sandbox_activation_adhoc_v1` is a future disposable local ad-hoc code seal
  used solely to carry one exact reviewed App Sandbox helper entitlement set. It
  contains no Developer ID, Apple-issued certificate, private key, Team ID,
  Keychain, provisioning profile, Apple-account, authentication, distribution,
  or product-identity authority.
- `product_signer_binding_v1` is P4's later fixed Developer ID team, leaf, and
  fingerprint binding with controlled private-key use. It cannot bootstrap P3,
  and the bootstrap class is categorically inadmissible as P4 identity, custody,
  provenance, signing-success, or release evidence.

For a future separately owner-approved **static** candidate re-review only,
the presence of the first class is no longer by itself equivalent to a
Developer ID signing dependency or P4 circularity. This changes only that one
eligibility classification. It does not select a candidate, authorize a
`codesign` operation, or relax any D-102 effect-control, graph-ownership,
terminal-cleanup, evidence, no-root, no-global-state, or no-new-dependency
requirement.

The future re-review must still fail closed unless authoritative current public
contracts establish all remaining predicates. In particular, it must separately
resolve the fixed helper bootstrap provenance without running an uncontained
package/build-script graph or accepting an unreviewed prebuilt binary or new
dependency. It must define an exact entitlement allowlist that excludes network
client/server, arbitrary or user-selected file access, temporary exceptions,
automation, Keychain groups, Mach lookup, device access, and every other
authority not indispensable to the reviewed candidate. Absent, ambiguous,
archived-only, or inferred support remains `contract_unproven`.

## Consequences

- D-103 remains an immutable bounded no-selection result for its frozen set.
- P3-3, P3-4, P3-5, P4, signing, V0-3, and all operational successors remain
  Blocked. No successor is Ready.
- A later candidate re-review requires its own exact plan, public-source
  register, architecture/security review, owner approval, and documentation
  gate. It may conclude negatively.
- No current source, dependency, configuration, capability, CSP, permission,
  entitlement, build, process, target-Mac, Apple, Xcode, Keychain, certificate,
  private-key, signing, credential, provider, product, or external state changes.
- D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker. Its
  report/digests, Failed privacy finding, Pending Open Directory boundary, and
  Not-run signing evidence remain unchanged.

## Alternatives considered

- Treat any code signature as P4 signing: rejected because official public
  documentation distinguishes an ad-hoc seal from a signing identity.
- Treat the narrow distinction as a qualifying App Sandbox design: rejected;
  D-102 lifecycle, effect, and bootstrap requirements remain independently
  unproved.
- Permit a generic entitlement or signing exception: rejected as broader
  authority and incompatible with the closed future-review boundary.
- Use P4 Developer ID state to bootstrap P3: rejected as circular.
- Add a helper, entitlement, dependency, build, or target-Mac experiment now:
  rejected as operational work outside this documentation increment.

## Official public source register

- [Apple: `kSecCodeSignatureAdhoc`](https://developer.apple.com/documentation/security/seccodesignatureflags/adhoc)
  — an ad-hoc signature is created without a signing identity.
- [Apple: Configuring the macOS App Sandbox](https://developer.apple.com/documentation/xcode/configuring-the-macos-app-sandbox)
  — App Sandbox is kernel-enforced and entitlement-configured.
- [Apple: Embedding a command-line tool in a sandboxed app](https://developer.apple.com/documentation/xcode/embedding-a-helper-tool-in-a-sandboxed-app)
  — the bounded helper-signature and inheritance guidance.
- [Apple: Understanding the Code Signature](https://developer.apple.com/library/archive/documentation/Security/Conceptual/CodeSigningGuide/AboutCS/AboutCS.html)
  — entitlements are sealed by code signatures and App Sandbox evaluates them.

## Supersedes or is superseded by

D-104 additively refines only the future P3 eligibility classification for the
defined identity-free bootstrap class. It does not supersede D-072, D-075,
D-076, D-095, D-096, D-097, D-098, D-099, D-100, D-101, D-102, D-103, TS-017,
any historical evidence, or an operational prerequisite.

## D-105 - Select no App Sandbox containment candidate after the D-104 re-review

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only bounded negative decision

## Context

D-104 permits one future static review to separate an identity-free
`sandbox_activation_adhoc_v1` seal from P4's later
`product_signer_binding_v1` proof. It changes no other D-102 predicate and does
not revise D-103's frozen `app_sandbox_build_helper_plus_libsystem_supervision_v1`
result.

The separately approved re-review froze exactly one additive candidate,
`app_sandbox_build_helper_plus_libsystem_supervision_v2`: a conceptual
application-owned host with App Sandbox enabled and no inherit entitlement; a
directly spawned helper with exactly App Sandbox plus inherit; the D-104
identity-free conceptual seal; and `posix_spawn`, direct-child `waitpid`, known-
PID `kqueue` `EVFILT_PROC`, `setpgid`, and process-group signaling as narrow
supervision adjuncts. Every other entitlement and authority is absent.

Current first-party Apple documentation supports narrow App Sandbox, helper,
signature, direct-child, file-handle, process-event, and process-group facts.
It does not establish the complete conjunction required by D-102. Archived
manuals may support only narrow historical semantics and cannot establish
current macOS 14+ availability. Silence, inference, post-hoc observation, and
one contract's evidence cannot pass another contract.

## Decision

The closed result is:

```text
no_eligible_candidate_after_d104_rereview
```

Every D-102 contract remains independently unproved:

| Contract ID                          | Disposition         | Reason                                                                                      |
| ------------------------------------ | ------------------- | ------------------------------------------------------------------------------------------- |
| `containment_primitive_contract`     | `contract_unproven` | The public App Sandbox contract does not establish the complete frozen-candidate boundary.  |
| `fixed_build_graph_contract`         | `contract_unproven` | The helper recipe presupposes build/sign/embed work and does not contain its first graph.   |
| `executable_identity_contract`       | `contract_unproven` | Every transitive executable byte identity and literal argument is not fixed by contract.    |
| `working_directory_binding_contract` | `contract_unproven` | One exact application-owned working directory is not bound for the complete graph.          |
| `descriptor_inheritance_contract`    | `contract_unproven` | Closure of every undeclared inherited descriptor before launch is not established.          |
| `filesystem_read_scope_contract`     | `contract_unproven` | Every ambient host-data read by the build and toolchain graph is not denied before effect.  |
| `filesystem_root_contract`           | `contract_unproven` | Exact repository, toolchain, dependency, cache, and output roots are not established.       |
| `outside_root_write_contract`        | `contract_unproven` | Container access does not establish exact-root pre-effect outside-write denial.             |
| `declared_network_scope_contract`    | `contract_unproven` | TCP/UDP entitlement facts do not close inherited descriptors or total permitted scope.      |
| `undeclared_network_contract`        | `contract_unproven` | IPv4, IPv6, DNS, Unix-socket, and relevant Mach-service denial is not jointly established.  |
| `descendant_membership_contract`     | `contract_unproven` | Direct-child/PID/group APIs do not own every detached or reparented descendant.             |
| `group_shutdown_contract`            | `contract_unproven` | Mutable group membership and signaling do not guarantee complete graph termination.         |
| `direct_child_reap_contract`         | `contract_unproven` | Narrow direct-child wait semantics do not establish the candidate's complete reap contract. |
| `terminal_quiescence_contract`       | `contract_unproven` | Race-free graph quiescence and rejection of all late effects are not established.           |
| `pipe_closure_contract`              | `contract_unproven` | Closing one handle does not establish closure of every copied/inherited descriptor.         |
| `process_metadata_effect_contract`   | `contract_unproven` | Process metadata content, visibility, logging, caching, and lifetime are not bounded.       |
| `cleanup_binding_contract`           | `contract_unproven` | No complete descriptor-rooted, non-following recursive cleanup contract exists.             |
| `cleanup_failure_contract`           | `contract_unproven` | Private quarantine and retry blocking require a controller that does not exist.             |
| `platform_cache_effect_contract`     | `contract_unproven` | OS and toolchain cache destinations, contents, and lifetime are not bounded.                |
| `platform_log_effect_contract`       | `contract_unproven` | Sandbox and unified-log effects are possible and not completely bounded.                    |
| `platform_socket_effect_contract`    | `contract_unproven` | Inherited, resolver, Unix, Mach/XPC, and other socket effects are not completely bounded.   |
| `platform_network_effect_contract`   | `contract_unproven` | OS-managed resolver, trust, signature, container, and network traffic is not bounded.       |

The repository contains no P3-3 controller source. Therefore
`caller_selected_process_source_review`, `shell_launch_source_review`,
`inherited_environment_source_review`, `unreviewed_spawn_source_review`,
`retry_fallback_source_review`, `raw_child_output_source_review`,
`deadline_enforcement_source_review`, `cancellation_terminal_source_review`,
`late_effect_rejection_source_review`, and `cleanup_ownership_source_review`
all remain `not_run`.

This is a valid fail-closed documentation result, not an operational failure
and not a universal impossibility claim. It selects no primitive and admits no
successor. P3-3, P3-4, P3-5, P4, signing, V0-3, and every product or external
operation remain Blocked. A future candidate would require a new bounded plan,
new owner approval, and authoritative evidence for every D-102 predicate; it
cannot be substituted into this frozen attempt.

## Consequences

- D-104's identity distinction remains correct but insufficient for candidate
  eligibility.
- No source, dependency, configuration, capability, CSP, permission,
  entitlement, helper, build, process, target-Mac, Apple, Xcode, Keychain,
  certificate, private-key, signing, credential, provider, product, or
  state-changing external action is authorized or evidenced.
- D-097 remains `failed` / `FAIL` / `Blocked` with its original report and
  digests and no completion marker. The historical screenshot/privacy finding
  remains Failed, the Open Directory boundary remains Pending, and signing
  evidence remains Not run.
- D-098's schema-v3 successor disposition and D-099 through D-104 remain valid
  and unchanged.

## Alternatives considered

- Infer full containment from App Sandbox enforcement: rejected because the
  specific bootstrap, graph, terminal, cleanup, and platform-effect contracts
  remain unproved.
- Treat absent network entitlements as proof of no network effect: rejected;
  the documented TCP/UDP initiation boundary does not close inherited sockets,
  DNS, Unix/Mach IPC, or OS-managed traffic.
- Treat direct-child wait, known-PID events, or process groups as complete
  graph ownership: rejected because detachment, reparenting, mutable membership,
  quiescence, and late effects remain unresolved.
- Run a build, probe, trace, packet capture, signature operation, or target-Mac
  experiment: rejected as outside the documentation-only scope and insufficient
  to prove universal pre-effect guarantees.
- Add a broader entitlement, privileged mechanism, dependency, VM, service, or
  residual-risk waiver: rejected as outside the approved boundary.

## Official public source register

The frozen source register and permitted narrow claims are recorded in
[`docs/plans/2026-09-02-personal-assistant-v0-app-sandbox-containment-rereview.md`](docs/plans/2026-09-02-personal-assistant-v0-app-sandbox-containment-rereview.md).
It includes current Apple App Sandbox, helper, file-access, network-entitlement,
App Groups, Foundation/Dispatch/System API, logging, and cache documentation,
plus explicitly bounded archived POSIX/manual references. No target-derived or
private evidence entered the review.

## Supersedes or is superseded by

D-105 additively applies D-102 and D-104 to only the frozen v2 candidate. It
does not supersede D-072, D-075, D-076, D-095, D-096, D-097, D-098, D-099,
D-100, D-101, D-102, D-103, D-104, TS-017, any historical evidence, or any
operational prerequisite.

## D-106 - Do not admit the frozen codeless signing fixture from the current contract record

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only bounded classification

## Context

D-096 permits a later separately approved signature to establish only
present-session identity use. Technical nonextractability, historical absence
of export, and exclusive custody remain `not_proven`. Its existing operational
plan still targets an executable Cortexa application bundle with product
identifier, designated-requirement, and hardened-runtime evidence.

D-102 requires fail-closed containment before a signing proof. D-103 and D-105
selected no candidate from their respective frozen build-bearing candidate
sets without claiming universal impossibility. The owner separately approved a
static review of one materially different conceptual candidate:
`repository_owned_codeless_bundle_signing_fixture_v1`. It proposes fixed
repository data with one `Contents/Info.plist` and at most one inert resource,
no executable, and no fixture-generation, build, download, installation,
entitlement, launch, or distribution path. It removes a proposed build graph
rather than containing one.

The frozen current Apple source register documents that a codeless bundle has
no executable code and can hold a signature, and that a bundle without Mach-O
stores signature material under `_CodeSignature` with hash-sealed resources.
It does not directly establish the complete exact candidate shape and source
workflow, Developer ID Application signing for this exact codeless class,
attempt-fresh present-private-key-use semantics, exact codeless verification,
non-product identifier/designated-requirement semantics, total absence of a
fixture-preparation graph, or a D-102 applicability split.

## Decision

The closed factual D-100 record is:

```text
{"protocol_version":"evidence_privacy_v1","check_id":"codeless_signing_fixture_classification","outcome":"contract_unproven"}
```

The closed governance result is:

```text
not_eligible_or_unproven
```

The factual record is privately bound once to this exact increment, candidate,
three-source register, and classification attempt. An altered, duplicate,
cross-plan, pre-admission, or late record is `boundary_failed` and stops without
retry.

The thirteen contract dispositions are:

| Contract ID                                  | Disposition         | Reason                                                                                                                                                       |
| -------------------------------------------- | ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `candidate_identity_contract`                | `documented`        | The codeless class is materially distinct from D-103/D-105's build-bearing app/helper candidates.                                                            |
| `codeless_bundle_shape_contract`             | `contract_unproven` | Standard locations and the codeless class do not establish every exact plist, mode, nested-item, link, resource-fork, and extended-attribute constraint.     |
| `repository_owned_construction_contract`     | `contract_unproven` | No exact fixture bytes or reviewed source workflow establish repository-only provenance without fixture-generation, build, download, or install executables. |
| `codeless_signature_storage_contract`        | `documented`        | TN3126 directly places a no-Mach-O bundle signature under `_CodeSignature` and documents resource hash sealing.                                              |
| `developer_id_codeless_sign_contract`        | `contract_unproven` | Separate codeless and Developer ID guidance does not directly establish Developer ID Application signing for the exact codeless class.                       |
| `private_key_use_semantics_contract`         | `contract_unproven` | No fixed source defines the attempt-fresh, replay-resistant present-private-key-use claim for this exact signature.                                          |
| `codeless_verification_semantics_contract`   | `contract_unproven` | The complete verification procedure and closed valid claims for a Developer-ID-signed codeless bundle are absent.                                            |
| `identifier_binding_semantics_contract`      | `contract_unproven` | General bundle-ID and designated-requirement guidance does not establish exact codeless Developer ID behavior or non-product equivalence.                    |
| `hardened_runtime_nonclaim_contract`         | `documented`        | A codeless bundle has no executable, while hardened-runtime signing guidance applies to a main executable; no Cortexa runtime proof can result.              |
| `no_build_graph_contract`                    | `contract_unproven` | The concept has no frozen bytes or reviewed source workflow proving that fixture generation/build/download/install are absent.                               |
| `d102_applicability_split_contract`          | `contract_unproven` | D-102 has no general waiver or `not_applicable` result, and the later signing executable/effect boundary remains.                                            |
| `operational_boundary_preservation_contract` | `documented`        | D-096 and D-100 through D-105 retain all signer, account, evidence, process, effect, cleanup, and operational blockers.                                      |
| `claim_ceiling_contract`                     | `documented`        | The source and decision record prohibits product-signing, hardened-runtime, distribution, release, custody, and V0-3 claims.                                 |

Totals are `documented=5`, `contract_unproven=8`, `not_run=0`, and
`boundary_failed=0`. The sources were available and all rows were reviewed, so
the last two dispositions do not apply. Because every row had to be
`documented` for a positive result, the candidate is not admitted. There is no
score, compensating control, inference, alternate source, candidate
substitution, retry, residual-risk acceptance, or automatic successor.

This is a bounded source result, not a claim that codeless Developer ID signing
is impossible. Candidate distinctness alone is insufficient. Removing a build
graph would be scope reduction, not containment and not a waiver. No D-102 row
becomes Passed or Not applicable; D-102 remains unchanged for every
executable-generating, generated-artifact, product-build, or otherwise build-
bearing path.

The future `codesign`, verifier, sanitizer, Keychain/Security framework,
filesystem-copy, process, output, deadline, cancellation, reaping, quiescence,
late-result, platform-effect, cleanup, and quarantine boundaries remain
unresolved. D-100 and D-101 remain mandatory.

## Consequences

- The only external contacts were approved read-only Git remote
  synchronization/checks and reads of the three frozen first-party Apple
  public-documentation pages. No fixture, source, dependency, configuration,
  entitlement, build, or product/signing/target-Mac operational process or
  state change exists or is authorized.
- D-096 present-session use and signing remain `not_run`; its executable
  product-bundle proof is not replaced or narrowed.
- The candidate cannot prove a signed Cortexa application, hardened runtime,
  product identity, Gatekeeper, notarization, distribution, release,
  historical non-export, technical nonextractability, exclusive custody, or
  V0-3 readiness.
- D-103 and D-105 remain immutable bounded negative decisions for their frozen
  candidates. All 22 D-105 contracts remain `contract_unproven`, and all ten
  controller-source checks remain `not_run`.
- D-097 remains `failed` / `FAIL` / `Blocked` with its original report,
  digests, Failed privacy finding, Pending Open Directory boundary, Not-run
  signing evidence, and absent completion marker. D-098 remains a valid,
  immutable, non-reusable schema-v3 disposition that never converts D-097 to
  complete.
- P3-3 through P3-5, P4, signing, V0-3, and every operational successor remain
  Blocked. No successor is Ready.

## Alternatives considered

- Combine the codeless-signing and Developer ID pages into an unstated exact
  guarantee: rejected as inference.
- Use another Apple source after the source register was frozen: rejected as
  candidate/source drift; it requires a new bounded plan and approval.
- Treat the conceptual shape or repository ownership as implementation proof:
  rejected because no fixture bytes or source workflow exists.
- Treat absence of a product build as a D-102 waiver: rejected because D-102
  remains controlling and later signing still executes a child with account,
  filesystem, process, effect, and cleanup boundaries.
- Create or sign a fixture to answer the static question: rejected as
  operational work outside the approved scope and insufficient to repair the
  missing public contracts.

## Official public source register

- [Apple: Placing content in a bundle](https://developer.apple.com/documentation/bundleresources/placing-content-in-a-bundle)
- [Apple TN3126: Inside Code Signing: Hashes](https://developer.apple.com/documentation/technotes/tn3126-inside-code-signing-hashes)
- [Apple: Creating distribution-signed code for macOS](https://developer.apple.com/documentation/xcode/creating-distribution-signed-code-for-the-mac/)

The source corpus is closed. No target-derived or private evidence entered the
classification.

## Supersedes or is superseded by

D-106 additively classifies only
`repository_owned_codeless_bundle_signing_fixture_v1` against the frozen
three-source register. It does not supersede D-072, D-075, D-076, D-095,
D-096, D-097, D-098, D-099, D-100, D-101, D-102, D-103, D-104, D-105,
TS-017, historical evidence, or any operational prerequisite.

## D-107 - Do not admit the frozen in-process key-use candidate from the current contract record

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only source classification

## Context

D-096 permits only a future bounded present-session private-key-use claim.
D-100 requires closed source-minimized evidence, D-101 prohibits explicit
account/home/path and ambient/default Keychain authority, and D-102 requires
pre-effect containment for every build-bearing path. D-103 and D-105 are
bounded negative reviews of frozen build-child candidates. D-106 rejects a
codeless bundle candidate that still depended on a filesystem artifact and a
future `codesign` process.

The owner selected one materially different containment-by-elimination
candidate for a static source review. The question is whether an in-process,
childless, fileless cryptographic liveness proof can be classified separately
from product code signing without weakening any existing prerequisite.

## Decision

Freeze exactly:

```text
in_process_security_framework_ephemeral_challenge_proof_v1
```

The conceptual candidate is one foreground, explicit-owner-action,
single-attempt, no-input trusted-Rust operation. It may use only one separately
proven, application-owned, attempt-bound opaque `SecIdentity` reference. It
would create a fixed-domain 32-byte fresh challenge, perform exactly one
signature with one fixed algorithm, verify exactly once against the retained
identity's paired public key, and expose only one closed D-100 record.

It has no build, helper, child, subprocess, bundle, artifact, filesystem write,
timestamp request, network request, persistent reference, generic signing
interface, caller-selected value, fallback, or retry. Identity, key,
certificate, challenge, signature, attribute, and native-error values remain
adapter-private. This is a conceptual boundary, not implemented behavior or
operational authority.

The exact factual D-100 record is:

```text
{"protocol_version":"evidence_privacy_v1","check_id":"in_process_key_use_candidate_classification","outcome":"contract_unproven"}
```

The closed source record is:

| Contract                                  | Disposition         | Bounded reason                                                                                                                                                                                                                          |
| ----------------------------------------- | ------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `candidate_identity_contract`             | `documented`        | The no-child, no-artifact data-signature class is distinct from D-103/D-105 build-bearing candidates, D-106's bundle/`codesign` candidate, and product code signing.                                                                    |
| `pinned_dependency_provenance_contract`   | `documented`        | `security-framework` 3.7.0 and `security-framework-sys` 2.17.0 are pinned with registry checksums at the reviewed baseline.                                                                                                             |
| `safe_wrapper_surface_contract`           | `documented`        | Safe public methods exist for identity-to-certificate/private-key, CSPRNG fill, one signature, certificate-to-public-key, and verification; this does not establish algorithm preflight, interaction denial, lifecycle, or effects.     |
| `opaque_prebound_identity_contract`       | `contract_unproven` | `SecIdentity` is opaque, but no current component issues the required no-input, application-domain, attempt-bound reference without lookup, enumeration, or fallback.                                                                   |
| `exact_signer_binding_contract`           | `contract_unproven` | Identity pairing proves certificate/key correspondence only; no immutable expected Developer ID Application class and certificate/public-key binding exists.                                                                            |
| `account_keychain_scope_contract`         | `contract_unproven` | File-based queries use ambient search-list/default behavior, while access-group scope is not proved for the existing Developer ID identity or current disabled data-protection-keychain feature path.                                   |
| `private_key_nonexport_contract`          | `contract_unproven` | The intended path need not export bytes, but Apple and the safe crate expose external representation and no implementation inventory proves export is unreachable.                                                                      |
| `fresh_challenge_contract`                | `documented`        | Apple defines `SecRandomCopyBytes` as cryptographically secure with mandatory status checking; the candidate freezes a 32-byte, domain-separated, one-attempt shape without claiming runtime freshness or guaranteed zeroization today. |
| `fixed_algorithm_contract`                | `contract_unproven` | The exact Developer ID key type and permitted algorithm are not frozen, and the safe crate lacks a safe wrapper for algorithm-support preflight.                                                                                        |
| `single_use_sign_contract`                | `documented`        | Apple defines one signature over supplied data and the pinned safe wrapper performs one underlying signing call without internal retry; later trusted Rust must still enforce one invocation.                                           |
| `paired_public_key_verification_contract` | `documented`        | Apple defines an identity as a certificate/private-key pair, exposes the certificate public key, and defines same-data/signature/algorithm verification; no trust, revocation, or product-signing claim follows.                        |
| `interaction_denial_contract`             | `contract_unproven` | Lookup-time skip/context controls do not establish that private-key retrieval and signing on an already-held file-based identity cannot display UI.                                                                                     |
| `hard_deadline_cancellation_contract`     | `contract_unproven` | Keychain lookup is documented as blocking, and the synchronous identity/sign/verify APIs expose no timeout or cancellation parameter.                                                                                                   |
| `late_result_rejection_contract`          | `contract_unproven` | No attempt host exists, and rejecting a late application result would not prove that late private-key use stopped.                                                                                                                      |
| `cleanup_quarantine_contract`             | `contract_unproven` | Reference release, failure quarantine, process-wide retry denial, bounded cleanup, and buffer lifecycle are not implemented or fully sourced.                                                                                           |
| `evidence_minimization_contract`          | `documented`        | D-100 supplies the exact fixed three-field, non-authorizing, source-minimized record and failure behavior for this classification only.                                                                                                 |
| `platform_effect_contract`                | `contract_unproven` | Keychain database, `securityd`, trust/revocation, cache, log, IPC, process-metadata, and possible OS-managed network effects remain undispositioned.                                                                                    |
| `d102_applicability_split_contract`       | `contract_unproven` | The candidate removes its own child and artifact, but D-102 contains no accepted waiver or `not_applicable` result for this proof class.                                                                                                |
| `claim_ceiling_history_contract`          | `documented`        | D-096 fixes the present-attempt ceiling, data signing remains distinct from code signing, and D-097 through D-106 remain unchanged.                                                                                                     |

Totals are `documented=8`, `contract_unproven=11`, `not_run=0`, and
`boundary_failed=0`. All sources were available and all rows were reviewed, so
the last two dispositions do not apply. Because every row is conjunctive, the
governance result is exactly `not_eligible_or_unproven`. The candidate is not
admitted and no successor is Ready.

The available sources establish narrower facts. Apple documents secure random
bytes, opaque identity/private-key references, data-signature creation, and
public-key verification. The pinned safe Rust crate exposes corresponding
operations. Apple also documents Keychain lookup as blocking and distinguishes
lookup-time interaction controls. These facts do not establish the missing
full boundary, and source silence is not upgraded by inference.

This is a bounded negative classification, not a claim that an in-process
design is impossible. Eliminating a child and artifact is scope reduction, not
D-102 containment. D-102 remains unchanged for every product, executable-
generating, artifact-generating, or otherwise build-bearing path.

## Consequences

- D-096 present-session use and operational signing remain `not_run`.
- D-101 remains Blocked; no identity source, residual-effect acceptance, or
  platform-operation authorization exists.
- D-102 remains fully binding for product signing and every build-bearing path.
- A future successful challenge could establish only that one already-bound
  opaque key reference signed one application-owned challenge during one
  approved attempt and that its paired public key verified the signature.
- It cannot establish a signed Cortexa application, stable client identity,
  Developer ID code-signature validity, trust/revocation, hardened runtime,
  Gatekeeper, notarization, distribution, technical nonextractability,
  historical non-export, exclusive custody, absence of compromise, or V0-3.
- D-097 remains `failed` / `FAIL` / `Blocked` with its original report and
  digests, Failed privacy finding, Pending Open Directory boundary, Not-run
  signing, and absent completion marker. D-098 remains valid and non-reusable.
- D-103, D-105, and D-106 remain immutable bounded negative decisions.
- P3-3 through P3-5, P4, signing, V0-3, and every operational successor remain
  Blocked. No successor is Ready.

## Alternatives considered

- Query the default/login Keychain and filter by label or fingerprint: rejected
  because ambient scope and target-derived selection violate D-101.
- Treat `kSecUseAuthenticationUISkip` as proof that signing cannot prompt:
  rejected because the documented control applies to item lookup, not the
  complete private-key-use operation.
- Put a blocking call on a worker and discard a late result: rejected because
  result rejection is not cancellation or quiescence of private-key use.
- Use raw `security-framework-sys` calls to fill wrapper gaps: rejected because
  application Rust forbids unsafe code and no lint relaxation is authorized.
- Treat no direct child/file/network request as absence of OS-managed effects:
  rejected because direct call shape does not prove operating-system internals.
- Substitute the challenge for app signing: rejected because data signing and
  code signing prove different claims.

## Frozen source register

Repository sources are the baseline `src-tauri/Cargo.toml`,
`src-tauri/Cargo.lock`, and checksum-resolved `security-framework` 3.7.0
`Cargo.toml`, `identity.rs`, `item.rs`, `key.rs`, `random.rs`, and
`os/macos/certificate.rs` plus `security-framework-sys` 2.17.0 `certificate.rs`
and `key.rs`.

Current public sources are limited to:

- [Apple: Code Signing Services](https://developer.apple.com/documentation/security/code-signing-services)
- [Apple: Randomization Services](https://developer.apple.com/documentation/security/randomization-services)
- [Apple: SecRandomCopyBytes](https://developer.apple.com/documentation/security/secrandomcopybytes%28_%3A_%3A_%3A%29)
- [Apple: Identities](https://developer.apple.com/documentation/security/identities)
- [Apple: kSecClassIdentity](https://developer.apple.com/documentation/security/ksecclassidentity)
- [Apple: Parsing an Identity](https://developer.apple.com/documentation/security/parsing-an-identity)
- [Apple: SecItemCopyMatching](https://developer.apple.com/documentation/security/secitemcopymatching%28_%3A_%3A%29)
- [Apple: kSecAttrAccessGroup](https://developer.apple.com/documentation/security/ksecattraccessgroup)
- [Apple: kSecMatchSearchList](https://developer.apple.com/documentation/security/ksecmatchsearchlist)
- [Apple: kSecUseAuthenticationUI](https://developer.apple.com/documentation/security/ksecuseauthenticationui)
- [Apple: kSecUseAuthenticationUISkip](https://developer.apple.com/documentation/security/ksecuseauthenticationuiskip)
- [Apple: kSecUseAuthenticationContext](https://developer.apple.com/documentation/security/ksecuseauthenticationcontext)
- [Apple: LAContext interactionNotAllowed](https://developer.apple.com/documentation/localauthentication/lacontext/interactionnotallowed)
- [Apple: SecKeyIsAlgorithmSupported](https://developer.apple.com/documentation/security/seckeyisalgorithmsupported%28_%3A_%3A_%3A%29)
- [Apple: SecKeyCreateSignature](https://developer.apple.com/documentation/security/seckeycreatesignature%28_%3A_%3A_%3A_%3A%29)
- [Apple: SecCertificateCopyKey](https://developer.apple.com/documentation/security/seccertificatecopykey%28_%3A%29)
- [Apple: SecKeyVerifySignature](https://developer.apple.com/documentation/security/seckeyverifysignature%28_%3A_%3A_%3A_%3A_%3A%29)
- [Apple: SecKeyCopyExternalRepresentation](https://developer.apple.com/documentation/security/seckeycopyexternalrepresentation%28_%3A_%3A%29)
- [Apple TN3137: On Mac keychain APIs and implementations](https://developer.apple.com/documentation/technotes/tn3137-on-mac-keychains)

No target-derived or private evidence entered the classification.

## Supersedes or is superseded by

D-107 additively classifies only
`in_process_security_framework_ephemeral_challenge_proof_v1` against the frozen
source register. It does not supersede D-072, D-075, D-076, D-095, D-096,
D-097, D-098, D-099, D-100, D-101, D-102, D-103, D-104, D-105, D-106,
TS-017, historical evidence, or any operational prerequisite.

## D-108 - Split D-102 only for the frozen non-build proof class

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only security-governance decision

## Context

D-102 defines a fail-closed future policy for a build-bearing child-process
graph. D-107 froze a materially distinct conceptual in-process challenge with
no build, helper, child, subprocess, executable, bundle, artifact, filesystem
write, `codesign`, product-signing operation, or direct application-owned
network request. D-107 correctly kept
`d102_applicability_split_contract` unproved because no accepted decision
defined whether that exact non-build class was outside D-102's subject.

The owner authorized reconsideration of exactly that constraint. This decision
must prevent both category error and waiver laundering: a build-child policy
need not be presented as implemented for a class with no build or child, but
removing those elements cannot weaken D-102 wherever any of them exists.

## Decision

Define `D102ApplicabilityPolicyV1` as a documentation policy for exactly:

```text
in_process_security_framework_ephemeral_challenge_proof_v1
```

It has only these human-readable governance dispositions:

| Condition                                                                                             | Disposition                                   |
| ----------------------------------------------------------------------------------------------------- | --------------------------------------------- |
| The exact frozen candidate retains every exclusion and only D-102's build-child subject is classified | `split_documented_for_frozen_non_build_class` |
| The bounded policy question is answered negatively without candidate drift                            | `split_not_accepted`                          |
| A required fact is missing, ambiguous, contradictory, or the candidate/policy boundary drifts         | `boundary_failed`                             |

Select exactly `split_documented_for_frozen_non_build_class`.

This is not a D-100 factual evidence record. The selected governance token
exceeds D-100's 32-byte outcome bound and must not be serialized into
`evidence_privacy_v1`. D-107's exact factual record remains unchanged with
outcome `contract_unproven`.

The split is limited to D-102's build-child subject and exists only while all
of these exclusions remain exact:

- no product build, package, compiler, linker, lifecycle script, or build graph;
- no helper, child, subprocess, shell, external executable, or detached
  descendant;
- no executable, bundle, staged file, generated file, signature artifact, or
  other artifact generation;
- no application- or Rust-dependency-authored, selected, or requested
  filesystem/path API access, read, write, mutation, mapping, or file-loading
  operation; OS-managed access internal to the fixed system-framework
  operations remains separately unproved;
- no `codesign`, code signing, product signing, code-signature or
  product-signature verification, notarization, distribution, or release
  operation;
- no application- or Rust-dependency-authored, selected, or requested network,
  socket, or IPC API operation; OS-managed effects internal to the frozen
  Security framework/RNG operations remain separately unproved;
- no application- or Rust-dependency-selected or requested dynamic-loader call,
  module, plug-in, JIT-generated code, or external code path; OS loader behavior
  for the fixed linked system frameworks remains separately unproved;
- no direct application operation beyond the frozen sequence of one fixed-
  domain fresh 32-byte challenge generation, one data-signature creation, and
  one paired-public-key verification;
- no caller-, model-, WebView-, environment-, current-directory-, account-,
  home-, path-, runtime-, profile-, task-, run-, workflow-, or agent-selected
  shape or identity; and
- no ambiguity, unmodeled effect, fallback, retry, substitution, scope
  expansion, or implementation drift.

If a definitive excluded feature is present, the candidate is outside the
frozen non-build class and D-102 is fully mandatory without a split
disposition. If any required fact is missing, ambiguous, contradictory, or
drifted, the disposition is `boundary_failed` and D-102 is also fully
mandatory. There is no generic `not_applicable` state, caller-selected
classification, score, compensating control, inferred acceptance,
residual-risk acceptance, or fallback.

D-102 is neither waived, satisfied, replaced, nor weakened. It remains fully
binding for every product-signing, executable-generating, artifact-generating,
build-bearing, helper, child, subprocess, or `codesign` path.

The split does not disposition operating-system effects. Keychain database,
`securityd`, directory, cache, log, IPC, trust, revocation, process-metadata,
and possible OS-managed network effects remain independently
`contract_unproven` under `platform_effect_contract`. No-child and no-direct-
network scope are not no-effect proof.

Historical D-107 remains byte-for-byte unchanged with eight `documented` and
eleven `contract_unproven` rows. D-108 permits only this additive current
interpretation:

| Contract                            | Current additive disposition | Reason                                                                                                                                         |
| ----------------------------------- | ---------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `d102_applicability_split_contract` | `documented`                 | D-102's subject is build-child containment; the exact frozen non-build class is separately classified with automatic fail-closed reattachment. |

The current prospective totals are therefore `documented=9` and
`contract_unproven=10`. The ten unproved rows are
`opaque_prebound_identity_contract`, `exact_signer_binding_contract`,
`account_keychain_scope_contract`, `private_key_nonexport_contract`,
`fixed_algorithm_contract`, `interaction_denial_contract`,
`hard_deadline_cancellation_contract`, `late_result_rejection_contract`,
`cleanup_quarantine_contract`, and `platform_effect_contract`.

Every D-107 row remains conjunctive. The governance result remains exactly
`not_eligible_or_unproven`; the candidate is not admitted and no successor is
Ready.

## Consequences

- D-102 remains mandatory for P3, P4 product signing, every product build,
  every executable-, artifact-, helper-, child-, subprocess-, or `codesign`-
  bearing path, and every application- or Rust-dependency-requested filesystem
  path operation in a proof candidate. OS-managed downstream effects remain
  governed separately by `platform_effect_contract`.
- D-108 creates no runtime classifier, containment primitive, identity source,
  signing interface, architecture edge, evidence parser, or operational
  authority.
- D-096 present-session use and operational signing remain `not_run`.
- D-101 and all ten listed contracts remain Blocked and unproved.
- A later design change cannot inherit this split; it must first pass the exact
  exclusion test, and ambiguity restores full D-102 applicability.
- D-097 remains `failed` / `FAIL` / `Blocked` with its original report,
  digests, Failed privacy finding, Pending Open Directory boundary, Not-run
  signing, and absent completion marker. D-098 remains valid and non-reusable.
- D-103 through D-107 remain immutable bounded records. P3-3 through P3-5, P4,
  signing, V0-3, and every operational successor remain Blocked.

## Alternatives considered

- Keep the split unresolved: valid and fail-closed, but rejected because it
  conflates D-102's build-child subject with an exactly non-build conceptual
  class without improving any remaining boundary.
- Mark D-102 generally `not_applicable`: rejected as an unsafe waiver and an
  unbounded bypass route.
- Treat no child as containment proof: rejected because scope reduction does
  not satisfy D-102 for any build-bearing path.
- Treat no direct application network call as no platform effect: rejected
  because OS-managed Keychain, security, trust, cache, log, IPC, and network
  effects remain unresolved.
- Shorten and emit the selected disposition as D-100 evidence: rejected because
  this is an internal governance decision, not one bounded external or target
  predicate; the existing factual record remains the truthful output.

## Supersedes or is superseded by

D-108 additively dispositions only D-107's
`d102_applicability_split_contract` for the exact frozen conceptual candidate.
It does not supersede or weaken D-072, D-075, D-076, D-095, D-096, D-097,
D-098, D-099, D-100, D-101, D-102, D-103, D-104, D-105, D-106, D-107,
TS-017, historical evidence, or any operational prerequisite.

## D-109 - Do not accept opaque identity-reference issuance from the current repository record

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only governance decision

## Context

D-107 freezes exactly
`in_process_security_framework_ephemeral_challenge_proof_v1` and records
`opaque_prebound_identity_contract` as `contract_unproven`. The required
future shape is stricter than a native opaque object: trusted Rust would need
one application-owned, no-input, operation-specific, attempt-bound opaque
identity reference that is neither selected by a caller nor obtained by lookup,
enumeration, default/search-list, account, home, path, environment, profile,
runtime, task, run, workflow, agent, fallback, or retry.

D-101 separately prohibits explicit account-directory resolution and ambient
identity authority. D-108 classifies only D-102's build-child subject for the
exact non-build candidate; it provides no identity source, signer binding, or
platform-effect proof. The current Rust
`src-tauri/src/credentials/cloudflare_access.rs` reader has two fixed
generic-password labels for a separate Cloudflare credential proof. It does not
issue, bind, retain, or destroy a signing identity, and must not be repurposed
as identity provenance evidence.

This repository-only review asks one narrow governance question: can the
current accepted repository record establish the required opaque identity-
reference issuer without operational evidence or a new implementation?

## Decision

Define `OpaquePreboundIdentityReferencePolicyV1` as a documentation policy
for exactly the frozen D-107 candidate. It has only these governance
dispositions:

| Condition                                                                                                                          | Disposition                       |
| ---------------------------------------------------------------------------------------------------------------------------------- | --------------------------------- |
| Current accepted source establishes the entire fixed no-input issuance and ownership boundary without a prohibited selection route | `reference_issuance_documented`   |
| Current accepted source establishes that no such issuer exists in the reviewed repository state                                    | `reference_issuance_not_accepted` |
| A required fact is missing, ambiguous, contradictory, or the candidate/contract boundary drifts                                    | `boundary_failed`                 |

Select exactly `reference_issuance_not_accepted`.

No current repository component issues an application-domain, attempt-bound
opaque signing-identity reference without lookup, enumeration, selection,
fallback, or ambient authority. A native object being opaque does not identify
who selected it, what credential domain applies, whether an ambient default or
search list was consulted, or whether the reference is bound to one attempt.
The fixed-label Cloudflare credential reader establishes none of those facts.

The long human-readable disposition is governance documentation, not a D-100
`evidence_privacy_v1` outcome and not a runtime, Tauri, or WebView result.
D-107's canonical factual record stays exactly `contract_unproven`.

## Consequences

- Historical D-107 remains byte-for-byte unchanged with eight
  `documented` and eleven `contract_unproven` rows. D-108's additive
  prospective interpretation remains nine documented and ten unproved rows.
- The current `opaque_prebound_identity_contract` row remains unproved.
  `reference_issuance_not_accepted` does not add a documented row, lower the
  blocker count, admit the candidate, or make a successor Ready.
- No future design may substitute a fixed label, certificate filter,
  fingerprint, default Keychain, search list, account, home, path,
  environment, caller, model, WebView, runtime, profile, agent, task, run,
  workflow, generic selector, fallback, or retry for an application-owned
  issuer.
- A future positive design needs separate owner authorization and independently
  reviewed contracts for trusted-Rust ownership, adapter-private issuance,
  reference non-export, attempt binding, exact signer binding, account/Keychain
  scope, private-key non-export, fixed algorithm, interaction denial,
  cancellation, late-result rejection, cleanup/quarantine, and platform
  effects.
- This decision creates no source implementation, credential, Keychain access,
  certificate/private-key operation, signing operation, Tauri interface,
  persistence, build, provider, target-Mac action, or external-system effect.
- D-097 remains `failed` / `FAIL` / `Blocked` with its original report,
  digests, Failed privacy finding, Pending Open Directory boundary, Not-run
  signing, and absent completion marker. D-098 remains valid and non-reusable.

## Alternatives considered

- Treat `SecIdentity` opacity as prebinding: rejected because opacity is not
  provenance, scope, issuer ownership, or attempt binding.
- Treat fixed credential labels as a signing-identity selector: rejected
  because the existing reader is a separate Cloudflare credential proof and
  would still not establish signer provenance or no-ambient authority.
- Add a certificate label, fingerprint, default Keychain, or search-list
  filter: rejected because each is selection/ambient authority, not an
  application-owned issuer.
- Implement an issuer or inspect Keychain state: rejected as product/system
  work outside this documentation-only authorization.
- Claim universal impossibility: rejected because the result is limited to the
  current reviewed repository state.

## Supersedes or is superseded by

D-109 additively selects a negative governance disposition for only
`opaque_prebound_identity_contract` in the frozen D-107 candidate. It does
not supersede D-072, D-075, D-076, D-095, D-096, D-097, D-098, D-099, D-100,
D-101, D-102, D-103, D-104, D-105, D-106, D-107, D-108, TS-017, historical
evidence, or any operational prerequisite.

## D-110 - Do not accept exact signer binding from the current repository record

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only governance decision

## Context

D-107 records `exact_signer_binding_contract` as `contract_unproven`:
an identity can establish certificate/private-key correspondence, but the
repository has no immutable expected Developer ID Application class and
certificate/public-key binding. D-109 separately records that the repository
has no application-owned issuer for the prerequisite opaque identity reference.
D-101 prohibits ambient/default account, home, path, Keychain, and search-list
authority; D-108 grants no signer authority.

This review is limited to whether existing repository evidence establishes the
missing expected-signer contract without reading a certificate, using Keychain,
or adding a configuration or runtime selection path.

## Decision

Define `ExactSignerBindingPolicyV1` for exactly the frozen D-107 candidate:

| Condition                                                                                                                                                              | Disposition                   |
| ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------- |
| Repository evidence establishes one immutable, application-owned expected Developer ID Application signer/certificate/public-key binding without a prohibited selector | `signer_binding_documented`   |
| Repository evidence establishes no complete binding in the reviewed state                                                                                              | `signer_binding_not_accepted` |
| A required fact is missing, ambiguous, contradictory, non-immutable, or drifted                                                                                        | `boundary_failed`             |

Select exactly `signer_binding_not_accepted`.

No current repository component owns an immutable expected Developer ID
Application signer class or certificate/public-key binding. Native identity
pairing, fixed labels, certificate subject/issuer metadata, fingerprints,
default Keychain state, search lists, filters, and caller input prove neither
expected-signer provenance nor resistance to substitution. This decision is
governance documentation, not D-100 evidence or a runtime/Tauri interface.

## Consequences

- D-107's immutable factual 8/11 record, D-108's additive 9/10 interpretation,
  and D-109's negative issuance result remain unchanged. This decision does not
  lower the ten blocker count, admit a candidate, or make a successor Ready.
- A future positive signer design requires separately approved trusted-Rust
  policy ownership, an adapter-private comparison over fixed application-owned
  material, no selector/lookup/default/fallback route, and independent proof of
  every remaining D-107 contract.
- No source, certificate, public key, fingerprint, serial, team/account value,
  Keychain operation, signing, build, provider, target-Mac, or external effect
  is created or disclosed.
- D-097 and D-098 remain unchanged and controlling.

## Alternatives considered

- Treat certificate/private-key correspondence as expected signer binding:
  rejected; it establishes correspondence, not expected identity.
- Persist or configure a certificate label/fingerprint: rejected; it creates a
  selector/identifier channel and does not establish issuer or scope authority.
- Inspect target certificate/Keychain state: rejected as operational work
  outside the approved documentation-only scope.
- Claim universal impossibility: rejected; this negative result covers only the
  reviewed repository state.

## Supersedes or is superseded by

D-110 additively selects a negative governance disposition for only
`exact_signer_binding_contract`. It does not supersede D-072, D-075, D-076,
D-095 through D-109, TS-017, historical evidence, or any operational
prerequisite.

## D-111 - Do not accept account and Keychain scope from the current repository record

Date: 2026-09-02
Status: Accepted owner-authorized documentation-only governance decision

## Decision

D-107 records `account_keychain_scope_contract` as `contract_unproven`:
file-based queries use ambient default/search-list behavior, while access-group
scope is unproved and the related data-protection-Keychain feature is disabled.
D-101 prohibits account/home/path resolution; D-109 and D-110 provide no
identity issuer or signer binding.

Define `AccountKeychainScopePolicyV1` with only
`scope_contract_documented`, `scope_contract_not_accepted`, and
`boundary_failed`. Select exactly `scope_contract_not_accepted`. Current
repository source has no application-owned identity scope that excludes account,
home, path, default/search-list, environment, profile, caller, or fallback
authority. Fixed labels, access-group names, disabled features, opaque
references, and absence of a visible prompt are not scope proof.

This is governance documentation, not D-100 evidence or an implemented
interface. It preserves D-097, D-107's 8/11 factual record, D-108's 9/10
interpretation, D-109, and D-110. It does not reduce the ten blockers, admit a
candidate, or make a successor Ready. No Keychain/account/directory/certificate/
private-key/signing/Apple/Xcode/build/target-Mac/provider/product/external
operation is authorized or performed.
