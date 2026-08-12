# Cortexa architecture

Status: Authoritative current-state architecture
Last updated: 2026-08-12

## Reading this document

This document distinguishes four states:

- **Current**: present in production source and covered by repository evidence.
- **Mocked**: present only as deterministic, non-authorizing test or UI behavior.
- **Planned**: approved direction without a shipping implementation.
- **Prohibited**: intentionally excluded by product or security policy.

`docs/product/ARCHITECTURE_BASELINE.md` preserves the target baseline. Accepted
decisions in `DECISIONS.md` govern when that target changes. Neither a plan nor a
diagram proves implementation.

## System context

```mermaid
flowchart LR
    Human["Human user"] --> UI["React WebView\nCurrent mock workspace"]
    UI -->|"narrow typed IPC"| Tauri["Tauri boundary"]
    Tauri --> Core["Trusted Rust core"]
    Core --> SQLite["SQLite bootstrap\nCurrent limited metadata"]
    Core -. "planned authenticated HTTPS" .-> Gateway["Product gateway\nPlanned"]
    Gateway -. "planned Responses API" .-> Model["Model provider\nUntrusted"]
    Core -. "future restricted adapters" .-> OS["macOS resources"]
    Model -. "proposal only" .-> Core
```

There is no direct model-to-device or WebView-to-device execution path.

## Trust boundaries

1. **Human authority**: supplies intent and explicit approval.
2. **React WebView**: untrusted presentation and volatile interaction state.
3. **Tauri IPC**: narrow serialization boundary; not authorization.
4. **Trusted Rust core**: validates, applies deterministic policy, manages exact
   approval state, and will eventually coordinate restricted execution.
5. **Local storage and OS resources**: protected resources available only
   through reviewed Rust boundaries.
6. **Product gateway**: planned authenticated service with no local authority.
7. **Model and third-party services**: untrusted external processors.

Files, websites, clipboard data, contacts, calendar data, model output, gateway
events, and tool results remain untrusted regardless of their source.

## Current application topology

```mermaid
flowchart TD
    App["App.tsx"] --> State["Application reducer/context"]
    App --> Pages["Conversations, Tasks, Memory, Activity, Integrations, Permissions, Settings"]
    App --> InfoClient["Typed get_app_info client"]
    App --> MenuClient["Typed assistant-menu-route listener"]
    InfoClient --> IPC["Tauri invoke boundary"]
    IPC --> Info["Rust app_info command"]
    Menu["macOS menu lifecycle"] --> MenuClient
    Startup["Rust startup"] --> Storage["SQLite bootstrap"]
    App --> Mock["Deterministic browser mock loop"]
```

The React mock loop and the transport-free Rust gateway turn are not wired to
each other.

## React presentation layer

**Current**:

- `src/App.tsx` composes the application shell and injects typed services.
- `src/application/` owns reducer-based volatile state, conversations, mock run
  lifecycle, provenance, mock results, and menu-route navigation.
- `src/features/` renders conversations, mock approval, Activity, Tasks,
  Permissions, Settings, and placeholders for Memory and Integrations.
- `src/infrastructure/tauri/` narrows the app-info response and menu-route event.
- Conversations, activity, approval state, tool results, and settings are not
  persisted by the WebView.

**Mocked**: assistant streaming, context provenance, tool activity, approval,
simulated tool results, final answers, Stop, Retry, and Activity are fixed local
behaviors. They perform no model request or operating-system action.

**Prohibited**: authorization, policy override, generic database access, raw
provider calls, arbitrary command selection, and operating-system execution in
the WebView.

## Tauri IPC boundary

**Current**:

- `get_app_info` is the only custom invoke command.
- `assistant-menu-route` is a closed native-to-WebView event for Open, New
  Request, and Tasks navigation.
- The main window has only `core:default` capability permission.
- The CSP and capability files contain no shell, filesystem, network, database,
  or privileged macOS plugin permission.

**Planned**: any future product command must be narrow, typed, locally
validated, capability-scoped, and separately approved. A generic
`execute_action`, SQL, shell, filesystem, provider, or tool-dispatch command is
prohibited.

## Trusted Rust core

`src-tauri/src/lib.rs` assembles current startup and Tauri behavior. The trusted
modules are intentionally transport-free where runtime coordination is absent.

### Initial gateway turn and protocol

**Current**:

- `agent::gateway_request` creates one bounded serialized initial request and
  binds a transport-free `InitialGatewayTurn`.
- `agent::gateway_protocol` validates closed normalized event frames, protocol
  identity, sequence, size, cancellation, text bounds, function-call bounds,
  and exactly one terminal outcome.
- `agent::function_call_validation` consumes a normalized function call and
  independently validates its local tool identity and arguments.
- The turn binds terminal function validation, deterministic policy, exact
  approval presentation, native resolution on macOS, and run-termination denial
  to one privately owned manager. Successful native and run-termination
  resolutions are validated and recorded by the turn's private typed in-memory
  approval-audit adapter before a closed resolution-plus-receipt value leaves
  the turn.

Every emitted value remains non-authorizing. There is no Tauri caller, live
transport, provider adapter, runtime coordinator, continuation loop, dispatcher,
or executor.

### Runtime adapter direction

**Current foundation; not wired to the application**: D-079's application-owned
runtime foundation now exists in Rust. `AgentRuntime` constructs one bounded
run from application-owned typed input. `RuntimeRun` exposes closed identity,
status, bounded untrusted-event acceptance, typed failure, and exact idempotent
cancellation. The repository still has no runtime coordinator, selector,
provider transport, live model, Hermes adapter, OpenClaw adapter, or Tauri/UI
consumer.

```mermaid
flowchart TD
    Application["Application services<br/>not wired"] --> Contract["AgentRuntime<br/>implemented typed foundation"]
    Contract --> Native["NativeAgentRuntime<br/>implemented; default/reference; test-only consumer"]
    Contract -. future blocked .-> Hermes["HermesAgentRuntime<br/>not implemented"]
```

`NativeAgentRuntime` is the sole/default implementation. It constructs and owns
exactly one unchanged `InitialGatewayTurn`; its concrete surface delegates the
turn's exact request bytes, normalized-frame validation, policy and approval
results, trusted macOS approval resolution, and audited run-termination cleanup.
The common shared-event lane privately translates only closed lifecycle, text,
and failure values through the unchanged gateway validator. Native does not
claim shared tool-proposal capability because the existing tool lane produces
governance-owned results. Concrete frames and shared runtime events cannot be
mixed within one run.

`RuntimeCapabilities` is a fixed closed representation. Capability declarations
grant no permission. Run/request/response/tool-call identities, selected text,
output text, and arguments use bounded application-owned types with redacted
Debug output. Generic cancellation closes either a nonterminal stream or a
run-owned pending approval through the existing typed audited termination path;
approval and audit values never enter the generic result.

`MockAgentRuntime` exists only as a private deterministic Rust contract fixture.
It uses fixed application-owned events, supports controlled unavailability,
start/event failure, invalid transitions, capability contradiction, terminal
cancellation, and late-event rejection, and has no clock, thread, filesystem,
network, process, model, provider, or Hermes prerequisite. The separate visible
React mock is unchanged.

“Fallback” remains an explicit future application-owned runtime choice; no
selector or automatic fallback code exists, and automatic model-provider
failover is not authorized.

`HermesAgentRuntime` would keep all Hermes types, configuration, events, and
errors inside one adapter and translate them to closed bounded Cortexa-owned
types. OpenClaw is only a possible later evaluation and is not a current or
selected adapter. No external runtime may own validation, policy, approval,
restricted execution, cancellation, audit, credentials, or direct device
access. Its output remains untrusted and must traverse the same deterministic
Rust gates as any other proposal.

This conceptual runtime seam is separate from model-provider transport. It does
not restore D-032's deleted synchronous arbitrary-string `AgentProvider` or
authorize networking, dependencies, credentials, provider selection, a live
model, dispatch, execution, or a Tauri capability. D-082 separately accepts the
multi-agent application-service target below; it does not add current behavior.

#### Native catalog, task, and bounded orchestration foundation

**Current Rust foundation; not wired to the application**: D-082 established
the native multi-agent ownership model and D-083 combined its former task and
first-flow phases. The Rust core now contains the closed catalog, bounded task
domain, and one deterministic application-owned orchestration service above the
unchanged one-run runtime seam.

```mermaid
flowchart TD
    User["User"] -. "future caller" .-> Personal["Personal Assistant<br/>current inert definition"]
    Personal --> Orchestrator["AgentOrchestrator<br/>implemented; unwired"]
    Orchestrator --> Research["Implemented generic route<br/>Personal Assistant to Research Agent"]
    Orchestrator --> Knowledge["Implemented document route<br/>Personal Assistant to Knowledge & Document"]
    Orchestrator -. staged .-> ResearchKnowledge["Blocked sequence<br/>Research Agent to Knowledge & Document"]
    Orchestrator -. staged .-> Engineering["Engineering<br/>Coding + QA + Security"]
    Orchestrator -. staged .-> Operations["Infrastructure & operations<br/>Cloud + Systems + QA + Security"]
    Orchestrator -. staged .-> Automation["Automation<br/>Workflow Automation"]
    Orchestrator --> Runtime["AgentRuntime<br/>implemented one-run boundary"]
    Runtime --> Native["NativeAgentRuntime<br/>sole/default; implemented and unwired"]
```

Agent-role arrows show logical assignment/delegation, not component authority.
The orchestrator invokes the runtime for each root or child run; definitions do
not call runtimes.

The current `AgentDefinition` is immutable, application-owned identity,
purpose, versioned instructions, and a non-authorizing activation disposition.
The catalog documents the closed functional groups, including cross-cutting
membership, without adding a routing field. The current `AgentRegistry`
performs validated deterministic lookup/listing of all nine definitions and
their closed `Initial`/`Deferred` catalog state. Discovery is non-authorizing;
operational selection and task creation fail closed for deferred definitions.
Neither registration, grouping, nor activation grants tools, routing, policy,
memory, provider, or device authority.
The catalog marks Personal Assistant, Research Agent, and Knowledge & Document
Agent `Initial`; all nine definitions remain unwired and none is a shipping
assistant. Knowledge eligibility applies only to D-085's separate approved-
document route and grants no generic delegation edge. The embedded
instruction sources are closed application-owned Rust assets rather than
runtime-loaded files.

`AgentTask` now owns one bounded objective, exact agent/root/parent/depth
lineage, the closed `Pending`/`Running`/`WaitingForChild`/terminal state
machine, and at most one typed terminal outcome. `AgentExecutionContext` is
derived from live task and runtime-run state and binds the agent, task, root,
optional parent, runtime, depth, run, request, and exact versioned policy- and
memory-profile identities. Both profiles are captured from the sealed built-in
definition identity rather than supplied independently by a caller.

`AgentOrchestrator<R: AgentRuntime>` is a separate application service. One
instance owns at most one root workflow, two tasks, three sequential runtime
runs, one non-replenishing child, one active child, depth one, and 32 runtime
and application events. It owns task assignment, exact live-context checks,
the generic Personal Assistant-to-Research route, D-085's separate approved-
document Personal Assistant-to-Knowledge route, bounded output accumulation,
result attribution, synthesis resumption, and child-first cancellation. It is
not a runtime, provider, policy engine, tool registry, approval manager, or
executor. It directly owns one workflow-local `MemoryStore` and
`ApprovedDocumentReader` without transferring their authority to an agent or
runtime, and composes the non-executing governance service below.

A root may complete directly in one runtime run. Delegation is accepted only
before root output begins; the orchestrator then terminally cancels that first
run without cancelling the root task, starts one Research child run, treats its
bounded result/failure/cancellation as untrusted attributed data, and starts a
fresh Personal Assistant synthesis run. Runtime events must match the exact
active task, run, request, and sequence before any limit or content processing.
Tool proposals fail this text-only boundary closed and are never forwarded.
Process-local workflow namespaces prevent events or contexts from one live
orchestrator instance from binding to another.

Authoritative application boundaries remain `AgentOrchestrator`,
`AgentRuntime`, `NativeAgentRuntime`, `AgentRegistry`, `ToolRegistry`,
`PolicyEngine`, `ApprovalManager`, `AuditLogger`, `MemoryStore`, and
`PlatformAdapter`, with several still planned rather than implemented. Security
& Risk is not `PolicyEngine`, QA & Validation is not `ApprovalManager`, and
Workflow Automation is not `AgentOrchestrator`. Agents may emit bounded
recommendations or typed requests; application code owns lifecycle,
authorization, approval, execution, and audit decisions.

The implemented orchestration boundary uses depth one, one child task total per
root, and one active child. Completion or cancellation does not replenish the
budget. Only the orchestrator may create a child task. Generic delegation is a
typed application-service call and remains exactly Personal Assistant to
Research Agent. D-085 adds a separate trusted application document-task call
from Personal Assistant to Knowledge & Document; it is not generic delegation,
a runtime control event, shell command, filesystem tool, or `agent.delegate`
host tool.

The initial root is exactly Personal Assistant. Research and Knowledge cannot
delegate; self, reverse, unknown, and all other generic routes fail before task
creation. Research-to-Knowledge remains Blocked. These allowlists are owned by
the application/orchestrator. Registry membership, activation, or a memory
profile grants no route. Later research/knowledge, engineering-quality,
infrastructure/operations, and automation workflows must add exact closed
routes and finite task caps under separate plans. Their arrows mean
orchestrator-controlled sequencing at depth one, never specialist spawning.

#### Per-agent governance foundation

**Current Rust foundation; not wired to the application**: D-084 adds nine
closed versioned policy profiles and captures the exact profile in each built-in
definition, task, live execution context, delegation request, governed tool
request, approval lifecycle, and governance record. `AgentDefinition` remains
the sole AgentId-to-profile mapping. Only an orchestrator-private proof minted
after exact live task/run validation can derive `AgentAttribution`; stale or
foreign contexts fail before governance or audit mutation.

`AgentGovernanceService` composes, but does not replace, the existing
application-owned `ToolRegistry`, `DeterministicPolicyEngine`, and
`ApprovalManager`, plus a new closed volatile governance audit. Its synthetic
tool-proposal entry point is separate from runtime events and accepts only the
two already registered local schemas. Personal Assistant alone is eligible:
date/time is `Allow`, local-task creation is `RequireApproval`, and every other
profile is `Deny`. All outcomes carry `ExecutionDisposition::NotAttempted`;
there is no executor or dispatch. Runtime tool proposals remain rejected by the
text-only orchestrator boundary.

Approval origin is a closed `LegacyGateway` or exact `Agent` attribution. The
pending request, presentation, trusted target-Mac result, cancellation, expiry,
and terminal resolution preserve that origin without exposing attribution in
Debug. Pending approval blocks further runtime events and delegation for its
task. Task cancellation reconciles and audits approval first, with child-first
ordering for root cancellation. A manager failure leaves approval/task/run
state live and unchanged; a later runtime-cancellation failure leaves the
approval terminally Cancelled while the task/run remain retryable.

The governance audit is a typed, redacted, process-local family capped at 32
subjects. It reserves one slot before policy/approval/control mutation, updates
that slot through terminal disposition without new capacity, uses deterministic
logical ticks, prevents exact-subject replay, retains no arguments or content,
and grants no authority. Delegation remains outside `ToolRegistry`; the audit
records the exact Personal-to-Research matrix result independently from later
control denial or child-creation outcome.

Before memory access, a sealed built-in definition now supplies the exact
memory-profile identity carried through task, live execution context, and
attribution. Missing, unknown, stale, foreign, or mismatched live identity fails
before content clone or mutation and never defaults to Personal Assistant.
Persistence, data-bearing privileged actions, and device effects remain
separately gated.

The current baseline contains no multi-agent Tauri IPC, multi-agent React state,
provider, live model, tool executor, platform adapter, durable audit, durable
memory, or device action. The catalog, task, orchestrator, governance, volatile
memory, and approved-document foundations are Rust-only and unwired. The Tasks
and Memory screens remain placeholders.

#### Volatile memory and approved-document Knowledge boundary

**Current verified Rust foundation; unwired**: D-085 adds one application-owned
`MemoryStore` and one
`ApprovedDocumentReader` directly owned by each one-root `AgentOrchestrator`.
They are process-local, non-global, non-injectable application services with no
thread, database, network, provider, clock, background index, or persistence.
Dropping the orchestrator/store clears retained content; nothing crosses an
orchestrator workflow or survives process exit.

The closed memory namespaces are approved shared, agent-private,
task-temporary, and proposed shared. Personal Assistant may read approved
shared and its own private/task records. Research and Knowledge may use only
their own private/task records and create inert proposals; the other six roles
are memory-disabled. Approved-shared promotion requires trusted application
review against an exact proposal version. Reads name exact record IDs, context
selection is bounded to eight records and 8,192 bytes, and no history,
document, namespace, sibling result, or private record is copied implicitly.
Terminal task cleanup removes only that task's temporary records. Disable
atomically clears all volatile memory and re-enable starts empty.

Agent memory access requires a non-cloneable grant derived from exact live
agent, policy profile, memory profile, task/root/parent, runtime, depth, run, and
request attribution. Application review/delete/disable and document
registration/revocation use separate application-control proof types. These
proofs have no public constructor and cannot be supplied by a model, runtime,
WebView, document, or caller-authored identity.

The document reader retains exact paths privately behind opaque workflow-bound
IDs. Trusted application code may register one user-selected file, task
attachment, generated artifact, or exact approved-root member. There is no
directory enumeration. Reads accept only nonempty lowercase `.txt` or `.md`
UTF-8 content up to 16,384 bytes. Relative paths, component counts, roots, and
references are bounded; traversal, noncanonical input, symlinks, hard-link
aliases, non-regular files, unsupported formats, replacement, detected in-read
mutation, replay, revocation, and cross-workflow binding fail closed. Supported
Unix targets compare registered, opened-handle, and final-path identity before
and after the bounded read. Pure-`std` opening retains a narrow documented
TOCTOU advisory; non-Unix targets report this boundary unavailable.

One reference is linearly reserved, then aborted or consumed. After the root
run is terminally cancelled, a consumed reference feeds exactly one direct
Personal Assistant-to-Knowledge child request. The document and an optional
explicit approved-shared selection are labeled untrusted and bounded to 26,624
raw UTF-8 bytes including framing. The result is one attributed
`DocumentTaskResult`, followed by a fresh Personal Assistant synthesis run.
Knowledge cannot choose paths, crawl roots, use tools, write artifacts,
delegate, or publish approved shared memory. The generic delegation matrix is
unchanged and Research-to-Knowledge remains Blocked.

Focused evidence passes 6 memory units, 9 document units, 10 public memory/
document contracts, and the 7 registry, 10 governance, 22 orchestration, 20
runtime, and 10 gateway regression contracts. Both storage smoke contracts
pass. The all-target Rust suite passes 269 tests with one intentionally ignored
opt-in Hermes probe; formatting, all-target/all-feature check, strict Clippy,
complete repository verification, and independent architecture/security/code
review pass. The quality result is `PASS WITH ADVISORIES` for the accepted
pure-`std` document-open TOCTOU residual. No Tauri command, React consumer,
provider, live model, SQLite product data, dependency, permission, executor, or
device effect was added. See
[`AGENT_MEMORY_DOCUMENT_PRIVACY.md`](docs/security/AGENT_MEMORY_DOCUMENT_PRIVACY.md).

See
[`NATIVE_MULTI_AGENT_ASSESSMENT.md`](docs/architecture/NATIVE_MULTI_AGENT_ASSESSMENT.md),
[`ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md`](docs/adr/ADR-NATIVE-MULTI-AGENT-ARCHITECTURE.md),
the authoritative [`ROADMAP.md`](ROADMAP.md), and its subordinate
[`NATIVE_MULTI_AGENT_ROADMAP.md`](docs/roadmap/NATIVE_MULTI_AGENT_ROADMAP.md).

#### Hermes transport evaluation

**No selected transport; three pinned-release mechanisms rejected**:
D-080 records raw TUI-gateway stdio as rejected for production at Hermes Agent
package/application version `0.20.0`, release tag `v2026.8.3`, source commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb`. At decision time it conditionally
selected a Rust-supervised managed local `hermes serve` child plus a closed
projection of the documented TUI-gateway JSON-RPC/WebSocket surface for a
contained spike after the native boundary. That spike later returned NO-GO.
ACP was then evaluated separately and rejected under D-081.

No Hermes executable, dependency, process, socket, token, runtime home,
provider, or adapter exists in the repository. The owner supplied an external
pinned candidate and authorized Milestone 0; source/tag/commit, critical hashes,
installed metadata, and isolated module discovery passed, but no Hermes server
was started. The spike's transport verdict is FAIL / NO-GO because the supplied provenance did not
content-manifest the complete virtual environment and its external Python
runtime, while target-Mac review found no sufficient containment mechanism.

Pinned source inspection also found unavoidable update-prefetch,
dotenv/managed-secret loading, credential keepalive, skill synchronization,
plugin discovery, and privileged default-tool initialization with no supported
complete disable mode. Deprecated `sandbox-exec` alone cannot prove exact
port-zero listener restriction, package-manager execution denial, blanket
Unix-socket denial, or membership and cleanup of detached descendants.

Any renewed spike remains Blocked on exact target-Mac whole-process containment,
isolated state and environment, complete immutable whole-distribution
provenance, importable `[web]`/POSIX `[pty]` extras, supported no-update/
no-credential/no-plugin/zero-tool controls, and authenticated
`ws://127.0.0.1:<port>/api/ws?token=<per-launch-token>` startup. It must suppress
the pinned lazy-install and update-check paths, keep the candidate read-only,
deny every pinned dotenv/managed-secret source without reading secret contents,
restrict Hermes egress to the exact deterministic local fake-provider endpoint,
deny every other network/Unix-socket destination, enforce closed protocol/event
limits, and clean up containment membership including detached descendants.
Configuration and upstream allowlists are defense in depth; they do not replace
the OS boundary. A failure to prove any of these controls is NO-GO. That
WebSocket/containment basis is rejected and authorizes no adapter.

D-081 separately rejects ACP for the same pinned release. ACP has a supported
public newline-delimited JSON-RPC stdio launcher, reported protocol and
implementation version, sessions, structured updates, cancellation, and
stdout/stderr separation. Every session nevertheless hardcodes the broad
`hermes-acp` toolset inside Hermes, including terminal/process, filesystem
mutation, browser, memory, skills, code execution, and delegation. Tool
progress and selected permission callbacks do not move pre-execution
validation, policy, exact approval, restricted execution, and audit into
Cortexa-owned Rust.

The supplied candidate also lacks a complete immutable runtime/interpreter
manifest and its pinned `agent-client-protocol==0.9.0` dependency. Five
deterministic fixture tests prove only bounded host-side framing and
direct-child mechanics; no Hermes executable was run. Raw TUI-gateway stdio,
managed `hermes serve` WebSocket, and ACP are rejected for the exact pinned
release under their evaluated conditions. A different path, patched
distribution, or later Hermes release requires a separate owner-approved
ADR/plan amendment and fresh contained spike.

### Agent provider

**Planned**: trusted Rust will call one configured authenticated product gateway
using a closed request and normalized response protocol. Identity-provider,
cloud-hosting, and AI model-provider support are separate boundaries under
D-060; approval of one does not approve another.

**Current absence**: the legacy generic provider scaffold was deleted in
Increment 4K. No HTTP client, provider SDK, gateway origin, model name,
authentication, credential loader, or live Responses request exists.

**Identity-provider boundary**: D-062 selects Microsoft personal identity as
the sole Phase 1 provider for consumer and prosumer individual accounts. The
planned flow uses the system browser, OAuth 2.0 Authorization Code Flow, PKCE
S256, `state`, and OIDC `nonce` behind the pluggable OAuth/OIDC application
boundary. The authority is personal-account-only; work, school, guest, and
arbitrary Entra tenants remain outside Phase 1. No provider registration,
identity client, redirect handler, token exchange, or account path exists.
Google is deferred until demonstrated demand after Microsoft verification.
Apple is deferred until Mac App Store planning or demonstrated demand. Phase 2
may separately add Entra workforce SSO and other approved enterprise OIDC or
SAML identity providers.

The configured identity provider determines the token issuer. The gateway must
validate each trusted issuer, audience, signature, expiration, applicable
tenant context, and authorization context against closed server-owned
configuration. The planned Microsoft boundary uses separate public desktop and
gateway API resource registrations, one exact delegated gateway scope, and the
issuer returned by the approved personal-account OIDC discovery metadata. The
canonical external identity key is provider ID plus normalized issuer plus
subject. Email is optional contact data, never an identity key, and automatic
email-based linking is prohibited. Provider neutrality is not permission for
the WebView, model, user, or arbitrary configuration to select an issuer,
endpoint, audience, client identity, tenant policy, or gateway origin.

A future audience-bound gateway token has a maximum 15-minute lifetime and
belongs only to trusted Rust process memory. Phase 1 requests only `openid`,
`email`, and one exact delegated Cortexa gateway scope. `profile`, Microsoft
Graph, directory, group, mail, calendar, file, and contact scopes are excluded.
`offline_access` is excluded until a separate persistent-session decision. If
later approved, a refresh or session credential belongs only to
platform-secure credential storage: macOS Keychain on macOS and an equivalent
separately reviewed facility on any future supported platform. No credential
enters the WebView, SQLite, application logs, or ordinary CI.

Microsoft currently documents a longer default access-token lifetime. Stage B
must prove a Microsoft personal-account configuration that enforces Cortexa's
maximum 15-minute boundary, or an additive decision must define another
short-lived gateway-session exchange. Stage C stays blocked until then. The
manifest-based `127.0.0.1` ephemeral callback also requires exact Stage B and
target-Mac Stage C evidence; no redirect or lifetime fallback is implicit.

**Cloud-hosting boundary**: Cortexa AI is the planned operator. Azure Container
Apps in Central US is the initial planned platform and region, and
`https://api.cortexaai.io` is the reserved production origin. None is deployed,
active, or approved for traffic. Initial production targets one primary cloud.
The containerized gateway should remain portable enough for future AWS or
Google Cloud deployment, but those targets require separate customer,
data-residency, resilience, or commercial justification. There is no
active-active multicloud architecture, three-cloud release requirement, or
approved secondary-cloud failover.

**AI model-provider boundary**: D-066 selects OpenAI as the sole synthetic-demo
candidate behind a future trusted gateway; D-063's Azure design is superseded
before publication. No provider transport is planned or implemented by this
decision. Any later OpenAI implementation must establish its exact model,
version, data controls, limits, and server-side secret handling as separate
evidence; desktop clients never receive provider credentials. The Azure-specific
configuration below remains historical D-064 design evidence only, not the
current demo-provider direction.

D-067 selects Cloudflare Workers Free as the sole remote gateway candidate for
the internal synthetic demo. A future Worker owns the OpenAI key as a Worker
secret, accepts only authenticated bounded synthetic-text requests, emits only
closed redacted results, and gains no local tool authority. No Worker, route,
secret, authentication mechanism, or network path currently exists. This
demo-only choice does not replace the planned production hosting boundary.

D-068 permits one future demo-only Cloudflare Access service token. Its secret
remains in macOS Keychain, trusted Rust sends it only to the fixed gateway
origin, Cloudflare Access restricts it to one application, and the Worker
independently validates the Access JWT signature, issuer, and audience. This
30-day maximum machine credential is not a production pattern and does not
change D-064's 15-minute production access-token maximum.

**Current fake-only Keychain proof**: `credentials::cloudflare_access` uses
macOS-only Security.framework bindings to read exactly the fixed
`io.cortexa.demo.cloudflare-access` service with separate `client-id` and
`client-secret` accounts. It validates bounded visible-ASCII fake values and
returns only `Available` or a closed redacted error. It exposes no raw-value
accessor and has no Tauri command, WebView, SQLite, startup, network, or runtime
consumer. Target-Mac evidence passed fake-item availability and cleanup, but
the unsigned development executable required repeated authorization prompts.
That proof does not establish stable app-specific access, so D-069 keeps real
credential ingestion blocked.

D-070 additionally requires a separately approved stable signed identity or
narrow app-specific Keychain ACL, production secret-memory lifecycle, direct
owner transfer, rotation, revocation, rollback, dependency reassessment, and
target-Mac evidence before an implementation proposal can be considered. This
adds no runtime path or external authority.

D-071 keeps the future macOS access-control choice and secret-memory model out
of the current runtime. A later owner-approved decision must select a stable
signed identity or narrow app-specific ACL and define bounded one-time secret
ownership before implementation; unsigned prompts are not a fallback.

D-072 selects signed macOS application identity as that future control model.
It remains documentation-only: no signed artifact, entitlement, Keychain access,
secret consumer, or runtime path exists.

D-073 limits a future fake-only signed-identity and bounded secret-memory proof
to the existing credential module, public integration test, and status-only
example. No manifest, entitlement, Tauri, IPC, startup, WebView, network, or
runtime-consumer path is authorized.

**Phase 2 target**: organization accounts and team workspaces may add
centralized administration, role-based access control, organization policy and
audit, Microsoft Entra ID workforce SSO, tenant-aware token validation, and
group-based authorization. SAML, SCIM, and other enterprise identity providers
remain demand-driven future decisions. No enterprise identity or administration
capability currently exists.

D-061 permits no current external processing. Each AI model provider requires
separate approval for retention, ZDR, data use, logging, region, and security.
Before provider-approved ZDR is verified for that provider's exact production
configuration, only synthetic test data may be considered by a separately
approved future transport test. Real user content then remains limited
initially to explicitly submitted, non-sensitive text; credentials,
attachments, regulated data, financial or healthcare data, and sensitive
personal data are prohibited. Gateway logs contain operational metadata only
for at most seven days, content logging is prohibited, and the
external-processing disclosure must precede the first transmission and remain
visible in Settings.

**Closed Phase 1 gateway configuration**: D-064 separates future work into
design, no-traffic provisioning, synthetic-only transport, and real-content
activation. The authoritative design is
[`docs/security/phase4-gateway-configuration-spec.md`](docs/security/phase4-gateway-configuration-spec.md),
and its trust boundaries, abuse cases, controls, and security-test matrix are
in
[`docs/security/phase4-gateway-threat-model.md`](docs/security/phase4-gateway-threat-model.md).

The planned identity registration uses one public Microsoft personal desktop
client and one separate gateway API resource. The closed API Application ID URI
format is `api://<gateway-api-client-id>`, its sole delegated scope is
`gateway.access`, and the callback is `127.0.0.1` on an ephemeral port at
`/oauth/callback`. Registration identifiers and observed token claims remain
future evidence. A mismatch fails closed rather than falling back to another
issuer, audience, scope, redirect, or account type.

The former Azure-design gateway uses one dedicated non-shared user-assigned managed
identity and exact-resource `Cognitive Services OpenAI User` RBAC. Azure OpenAI
must be reachable only through a private endpoint with public network access
disabled before any synthetic or real provider traffic. The public gateway
origin remains authenticated HTTPS; no Container Apps hostname, alternate
origin, public provider endpoint, API key, or unrestricted egress is a fallback.

This closed design is not a deployed architecture. D-064 grants no Stage B,
Stage C, or Stage D authority, and ARB-002 remains High and unresolved.

### Tool registry and schemas

**Current**: `tools::registry` and `tools::schema` define a closed catalog:

- `get_current_datetime@1`: information only, exact empty object.
- `create_local_task@1`: reversible local action with a canonical bounded title.

Registry validation derives tool identity, contract version, risk, and required
permission locally. It rejects unknown tools, versions, fields, malformed JSON,
duplicate keys, and invalid arguments. No tool implementation executes either
proposal.

### Policy engine

**Current**: `policy::engine` evaluates only trusted typed inputs. Information-
only proposals produce a non-authorizing Allow decision. Reversible and
personal-data modifications require approval. Missing read or permission scope,
high-impact actions, and prohibited autonomy are denied.

Policy does not execute, grant permission, authenticate a user, or replace an
approval transition.

### Approval manager

**Current**: `approvals::manager` provides an in-memory exact-subject state
machine with one pending subject, bounded lifetime capacity, manager-issued
identity, 120-second expiry, one-time presentation and resolution, replay
rejection, and terminal run-cancellation denial.

The macOS `rfd` decision source returns a sealed outcome bound to its exact
presentation and manager. It is not invoked by the shipping UI or a production
coordinator. A stale visible native dialog may outlive run termination, but its
late outcome is rejected.

Approval presentations and resolutions are non-authorizing. No execution token
or dispatch authority exists.

### Audit logger

**Current**: `audit::approval` is a typed, bounded, in-memory adapter that derives
a redacted approval record and returns a sequence-only receipt. The initial turn
owns one private adapter and exposes successful terminal approval resolutions
only through a closed non-cloneable resolution-plus-receipt value. Manager
terminalization precedes recording; a typed audit failure returns no resolution
and cannot restore manager state.

**Current absence**: the generic audit scaffold was deleted in Increment 4I.
There is no run, execution, durable, cross-run, IPC, UI, or product audit logger.
The turn-owned adapter is volatile and sequence-local; neither its record nor its
receipt authorizes dispatch, execution, or provider continuation.

### Memory store

**Mocked**: the React Memory page is a placeholder and conversation state is
volatile.

**Current, unwired and volatile**: D-085 adds a new bounded namespace-aware
`MemoryStore` owned by one `AgentOrchestrator`, with explicit selected-record
context, versioned application review, exact deletion, task cleanup, and atomic
disable. It is not the deleted legacy scaffold and does not persist, cross a
workflow, or satisfy user-facing product-memory requirements.

**Current absence**: there is no durable product memory repository, SQLite
product-data schema, encryption/key boundary, restart recovery, export, UI,
cross-session selector, vector database, embedding service, or semantic index.

### Platform adapters

**Current**: macOS-specific code is limited to menu/window lifecycle and the
disconnected native approval source. Non-macOS menu lifecycle adapters are
no-ops where required for compilation.

**Current absence**: the legacy generic platform scaffold was deleted in
Increment 4M. There are no calendar, reminders, contacts, file, clipboard,
notification, secret-store, local-authentication, Accessibility, screen-capture,
Apple Events, or microphone adapters.

### SQLite storage

**Current**:

- `storage` owns a private `rusqlite` connection with typed errors.
- Every connection enables foreign keys and a busy timeout; file databases use
  WAL.
- Embedded ordered migrations create `schema_migrations` and `app_metadata` and
  verify immutable checksums.
- Multi-step migration writes use immediate transactions.
- Startup persists only typed `app_initialized` bootstrap metadata in the debug
  app-local database. Release startup currently uses in-memory storage.
- No generic SQL or database IPC exists.

**Current absence**: no conversation, task, memory, credential, token, approval,
or audit product persistence exists. Database encryption and Keychain-held key
material are required before sensitive persistence.

## macOS menu bar and window lifecycle

**Current**:

- A tray/status item provides Open Cortexa, New Request, Tasks (Coming Soon),
  and Quit Cortexa.
- Menu actions map to closed domain actions before native dispatch.
- Closing the main window hides it; Dock reopen restores it when no application
  window is visible.
- The configured window is titled Cortexa and uses stable minimum dimensions.

Debug and release Tauri app bundles use the official Cortexa icon family from
the canonical app-icon source. The raw unbundled `tauri dev` executable retains
macOS's generic `exec` icon; D-051 records that approved development-only
baseline exception. D-052 requires ICNS verification by decoded representation
pixels because repeated Tauri CLI generation can produce byte-distinct but
visually equivalent ICNS containers; bundled resources must still match the
reviewed repository ICNS byte-for-byte.

## Current and future capability matrix

| Capability                                    | State                          | Evidence or gate                                               |
| --------------------------------------------- | ------------------------------ | -------------------------------------------------------------- |
| React workspace and navigation                | Current                        | Frontend tests and application source                          |
| Assistant interaction                         | Mocked                         | Deterministic in-memory driver only                            |
| App info and menu routing                     | Current                        | Narrow Tauri command/event                                     |
| SQLite bootstrap metadata                     | Current                        | Storage tests and startup integration                          |
| Gateway request/protocol validation           | Current, transport-free        | Phase 4A and 4N-4P                                             |
| Function schema and policy binding            | Current, non-authorizing       | Phase 4B-4C and 4Q-4R                                          |
| Approval presentation/resolution/cancellation | Current, disconnected          | Phase 4D-4E and 4S-4U                                          |
| Approval audit adapter                        | Current, bound and volatile    | Phase 4H and 4V                                                |
| Workflow-local volatile agent memory          | Current, unwired               | D-085 verified contracts; process-local only                   |
| Selected UTF-8 text/Markdown document reading | Current, unwired and read-only | D-085 verified contracts; no IPC or provider                   |
| Live gateway and model-provider transport     | Planned                        | Blocked by O-006, per-provider O-007 evidence, and future plan |
| Restricted tool execution                     | Planned                        | No dispatcher or executor exists                               |
| Product memory and task persistence           | Planned                        | Phase 8 direction only                                         |
| Privileged macOS integrations                 | Planned or prohibited for MVP  | Separate permission and threat-model gates                     |
| Generic shell or model-to-device execution    | Prohibited                     | `SECURITY.md`                                                  |
| Signing, notarization, and production release | Planned                        | Phase 10 and `RELEASE_CHECKLIST.md`                            |

## Approved future data flow

```mermaid
sequenceDiagram
    participant U as Human
    participant W as WebView
    participant R as Trusted Rust
    participant G as Product gateway
    participant M as Model provider
    participant O as Restricted OS adapter

    U->>W: Submit request
    W->>R: Narrow typed command
    R->>R: Select bounded context and tool-set version
    R->>G: Authenticated closed request
    G->>M: Forced safe Responses request
    M-->>G: Untrusted provider stream
    G-->>R: Normalized closed events
    R->>R: Validate schema and deterministic policy
    R->>U: Exact approval when required
    U-->>R: Trusted decision
    R->>O: Execute exact registered action
    O-->>R: Verified bounded result
    R->>R: Write redacted local audit
    R-->>W: Present bounded outcome
```

This flow is a target, not current end-to-end behavior. Every dotted or future
boundary requires its own approved plan and verification.

## Prohibited execution paths

- Model or gateway directly invoking device APIs.
- WebView directly invoking a generic executor, shell, filesystem, SQL, or
  provider command.
- Caller-supplied tool schemas, model parameters, approval evidence, risk, or
  permission metadata becoming trusted.
- Approval or audit receipts being treated as execution authority.
- Production credentials in the desktop bundle, WebView, SQLite, logs, crash
  reports, or audit records.
- Autonomous email, messages, purchases, bookings, uploads, public posting,
  deletion, or account-setting changes in the MVP.

## References

- `PRODUCT_REQUIREMENTS.md`
- `SECURITY.md`
- `DECISIONS.md`
- `PROJECT_STATUS.md`
- `docs/product/ARCHITECTURE_BASELINE.md`
- `docs/branding/PRESENTATION_GUIDELINES.md`
