# Architecture baseline

## Product shape

Cortexa is a standalone desktop executive assistant. The initial target is macOS, with portable domain logic intended for Windows, Linux, and iOS where platform capabilities permit.

## Layer model

### React presentation layer

The WebView renders conversations, navigation, settings, permission status, action previews, tool activity, and tasks. It receives typed application events and sends narrow commands. It is not trusted to authorize or execute local actions.

### Tauri IPC boundary

Only explicitly registered, typed commands cross into Rust. Capabilities are scoped per window. Generic shell, filesystem, database, tool-execution, and model-request commands are prohibited.

### Rust application core

The trusted core owns:

- Run lifecycle.
- Context provenance.
- Provider abstraction.
- Tool registry and schema validation.
- Deterministic policy.
- Approval transactions.
- Restricted dispatch.
- Persistence.
- Audit and redaction.
- Cancellation and limits.

### Provider boundary

The verified application still uses deterministic mocks. A production provider will be owned by trusted Rust and will call one authenticated HTTPS product gateway. The WebView cannot choose the gateway, provider, model, tool schemas, or credentials.

The product gateway owns the production OpenAI credential in server-side secret storage. A future audience-bound gateway access token has a maximum 15-minute lifetime and is held in desktop Rust memory; any refresh or session credential belongs to the platform secret store. Neither enters the WebView or SQLite. The gateway authenticates and authorizes the desktop principal, enforces request, rate, model, and tool-set limits, selects exact server-side tool contracts, injects the OpenAI credential, and adapts the OpenAI Responses stream into a versioned product protocol. The gateway has no local executor, approval, policy, or operating-system authority.

Initial production requests use foreground Responses streaming with `store: false`, `background: false`, strict function definitions, and parallel tool calls disabled. Hosted OpenAI tools, MCP tools, shell tools, arbitrary provider parameters, and caller-supplied tool schemas are not allowed. A live model name, gateway deployment, and identity provider require separate approval.

The OpenAI adapter validates recognized event types, required fields, sequence, and bounds. It may ignore additive fields on a recognized event for upstream compatibility, but unknown event types and invalid required fields terminate the run. It forwards only normalized product events. The gateway-to-Rust protocol is stricter: exact protocol version, closed fields and event variants, contiguous sequence numbers, bounded IDs/content/arguments, and exactly one terminal event.

A normalized function call is still untrusted. Trusted Rust independently validates the call ID, registered tool name, tool-contract version, complete JSON object, duplicate-key rejection, exact per-tool schema, risk and permission metadata derived from the local registry, deterministic policy, and approval state before any executor can run. Provider strict mode is not an authorization boundary.

Cancellation is cooperative and idempotent. Rust first transitions its validator to a terminal cancelled state, then aborts the desktop-to-gateway request; the gateway aborts the upstream foreground stream, and both reject late events. Because the Responses cancellation endpoint applies only to background responses, the desktop must neither expect an acknowledgment on the aborted stream nor represent a foreground transport abort as confirmed provider-side cancellation.

### Platform adapters

Operating-system access remains behind interfaces such as application, calendar, reminders, contacts, files, clipboard, notifications, system information, secret storage, and local authentication. Phase 2 does not enable privileged adapters.

## Trust boundaries

1. Human authority.
2. Untrusted WebView and rendered content.
3. Trusted Rust core and deterministic policy.
4. OS-protected resources and local encrypted storage.
5. Product gateway.
6. Third-party model and OAuth services.

The model, gateway, WebView, and untrusted content have no direct executor path.

## Phase 4 data flow

1. The WebView submits a local request through a future narrow typed command; it never supplies provider credentials or raw provider parameters.
2. Trusted Rust selects local context, the exact supported tool-set identifier, limits, and an opaque run/correlation ID.
3. The Rust gateway client reads a future gateway token from platform secret storage and sends one bounded request to the configured gateway origin.
4. The gateway authenticates the principal, authorizes the contract version, selects the matching server-side tool definitions, forces safe Responses parameters, and calls OpenAI with the gateway-owned credential.
5. The gateway validates and normalizes the provider stream. Raw OpenAI events and errors never cross into the desktop application.
6. Trusted Rust validates the normalized event sequence and accumulates bounded text. A function call remains a proposal until local registry and schema validation succeed.
7. Deterministic policy, trusted approval, restricted execution, and local audit remain entirely inside the desktop trust boundary.
8. A later continuation may return a redacted verified tool result to the provider through the same bounded gateway contract; neither the WebView nor gateway can manufacture execution authority.

## Phase 4 conservative limits

The first protocol contract freezes these upper bounds until a reviewed decision changes them:

- Two consecutive model turns per run.
- One function call per run and no parallel function calls.
- One retry, permitted only before an accepted response event or for an explicitly safe rate-limit response.
- Three gateway requests per run, covering two model turns and at most one retry.
- 64 KiB serialized gateway request, 16 KiB normalized event frame, 8 KiB completed function arguments, 8,192 Unicode code points of assistant text per turn, and 256 normalized events per turn.
- 10-second connection timeout, 20-second stream-idle timeout, 60-second provider-turn deadline, and 120-second total run deadline.

Exceeding any limit cancels the local run and produces a closed redacted limit error. Limits are enforced independently by Rust and the gateway where applicable.

## Error and audit boundaries

Gateway errors expose only a closed code, retryability, an optional bounded retry delay, and opaque gateway/provider correlation IDs. Raw provider messages, bodies, headers, URLs, stack traces, prompts, output, and function arguments are never returned to the WebView or local audit.

The gateway operational record may contain an opaque principal, correlation IDs, contract and model versions, timestamps, status, latency, rate-limit metadata, and aggregate token usage. Its default maximum retention is seven days unless a later compliance decision approves another period. The local trusted audit owns run lifecycle, normalized proposal identity, locally derived tool/risk/permission metadata, policy, approval, execution, cancellation, and final outcome. Neither record stores production credentials or duplicates raw personal content.

`store: false` minimizes Responses application-state storage but does not remove the provider's default abuse-monitoring retention. Before live traffic, O-007 must select and verify the OpenAI project retention mode and the product must disclose the external processing and applicable retention to the user.

## Action risk classes

- **Class 0:** information only; no approval.
- **Class 1:** read-only device access within existing permission and scope.
- **Class 2:** reversible local action; direct only for narrow explicit intent, otherwise confirmation.
- **Class 3:** personal-data modification; trusted preview and approval every time.
- **Class 4:** external or high-impact action; not registered in the MVP.
- **Class 5:** prohibited autonomy; always denied.

A tool's target can escalate its effective risk. Shared calendars, attendees, executable files, unsupported URL schemes, or out-of-scope resources must be blocked.

## Data and secrets

- SQLite stores local product data through versioned migrations.
- Database encryption and Keychain-held key material are required before sensitive persistence.
- OAuth tokens and production credentials never enter SQLite.
- Raw sensitive tool output remains ephemeral where possible.
- Audit records retain normalized and redacted evidence, not duplicate personal content.

## Agent loop

1. Receive a user request.
2. Select only approved context.
3. Ask the provider for text or a typed function proposal.
4. Validate the tool and arguments.
5. Apply deterministic policy.
6. Execute an allowed read or request approval.
7. Execute the exact approved proposal.
8. Record a redacted audit event.
9. Return a verified tool result to the provider.
10. Continue until final text or a conservative limit is reached.

## Phase boundaries

- Phase 2: repository and application shell, interfaces, mocks, local UI scaffolding.
- Phase 3: complete conversation UI and mocked loop.
- Phase 4: versioned gateway protocol, authenticated gateway transport, and Responses integration in independently verified increments.
- Phase 5: policy, approval, and audit enforcement.
- Phase 6: basic macOS tools.
- Phase 7: permissions and onboarding.
- Phase 8: memory and tasks.
- Phase 9: adversarial security testing.
- Phase 10: signing, notarization, packaging, and release.

The full product brief is in `PRODUCT_BRIEF.md`.
