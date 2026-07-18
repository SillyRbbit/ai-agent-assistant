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
`eaf6c9f`; active routing superseded by D-057

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
Status: Accepted; implementation locally verified, hosted execution pending

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

## Open decisions

| ID    | Topic                                                            | Required before                     |
| ----- | ---------------------------------------------------------------- | ----------------------------------- |
| O-002 | Workspace split between one Tauri crate and multiple Rust crates | Revisit before later modularization |
| O-003 | macOS minimum deployment target confirmation on target Mac       | Native release preparation          |
| O-006 | Gateway identity provider and deployment platform                | Before live gateway networking      |
| O-007 | Provider retention mode and user disclosure                      | Before live provider traffic        |
