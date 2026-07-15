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

## Open decisions

| ID    | Topic                                                            | Required before                     |
| ----- | ---------------------------------------------------------------- | ----------------------------------- |
| O-002 | Workspace split between one Tauri crate and multiple Rust crates | Revisit before later modularization |
| O-003 | macOS minimum deployment target confirmation on target Mac       | Native release preparation          |
| O-006 | Gateway identity provider and deployment platform                | Before live gateway networking      |
| O-007 | Provider retention mode and user disclosure                      | Before live provider traffic        |
