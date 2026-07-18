# Cortexa product requirements

Status: Authoritative normalized product requirements
Last updated: 2026-07-15

## Purpose and authority

This document translates the owner-supplied inception brief into testable
current requirements while preserving accepted architecture and security
decisions. `docs/product/PRODUCT_BRIEF.md` remains the historical source for
product intent. `DECISIONS.md`, `SECURITY.md`, and this document govern current
implementation when the brief is aspirational or ambiguous.

Requirement status does not prove implementation. Current capability evidence
is in `ARCHITECTURE.md` and `PROJECT_STATUS.md`.

## Product mission

Cortexa is a standalone, native-feeling, local-first executive assistant that
helps a user organize work, understand context, propose controlled actions, and
eventually perform narrowly registered actions only after deterministic policy
and any required human approval.

The product must reduce workflow fragmentation without transferring device or
account authority to a language model.

## Target users

### Primary users

- Executives who need a controlled workspace for requests, follow-up, tasks,
  context, and approvals across work systems.
- Employees and operators who need repeatable assistance without surrendering
  accountability for consequential actions.

### Stakeholders

- IT leaders responsible for deployment, identity, support, and integration.
- Security and privacy reviewers responsible for permissions, data movement,
  credentials, audit, and supply-chain risk.
- Business owners responsible for workflow outcomes and adoption.

The first platform is macOS. Portable domain behavior should remain suitable
for later Windows, Linux, and iOS work where platform capabilities permit.

## Core use cases

### Executive use cases

- Ask for a concise answer using explicitly disclosed context.
- Organize a request into a bounded task or follow-up.
- Review the exact effect of a proposed personal-data change before deciding.
- Stop an active run and trust that late proposals or approvals cannot proceed.
- Review what context, policy, approval, and result contributed to an outcome.
- Manage integrations, permissions, memory, and tool availability.

### Employee and operator use cases

- Maintain separate conversations and deterministic task context.
- Retrieve permitted information from connected systems.
- Create or update a narrow local item after policy and approval.
- Inspect activity and failures without exposing secrets or unnecessary content.
- Retry safe pre-acceptance failures without duplicating mutations.

No use case permits unattended consequential external action in the MVP.

## Functional requirements

### Workspace and interaction

- **FR-001**: The macOS application shall provide a main Cortexa window and
  native menu/status-item access.
- **FR-002**: The workspace shall provide conversations, tasks, activity,
  memory, integrations, permissions, and settings destinations.
- **FR-003**: A user shall be able to submit text, observe bounded progress,
  stop an active run, and receive explicit failure or completion state.
- **FR-004**: Conversation and activity presentation shall distinguish actual,
  simulated, pending, approved, rejected, cancelled, and failed states.
- **FR-005**: The product shall disclose the context categories used and not
  used for a run.

### Agent and gateway

- **FR-010**: The WebView shall send only a narrow typed user request to trusted
  Rust; it shall not select provider credentials, model parameters, gateway
  origin, or tool schemas.
- **FR-011**: Trusted Rust shall create a closed versioned, size-bounded gateway
  request with an exact supported tool-set identifier.
- **FR-012**: A production gateway shall authenticate and authorize the desktop
  principal, own the production provider credential, force approved Responses
  parameters, and normalize upstream events.
- **FR-013**: Trusted Rust shall reject unknown, malformed, out-of-sequence,
  oversized, late, or multiply terminal gateway events.
- **FR-014**: Runs shall enforce independently reviewed turn, request, retry,
  event, content, argument, idle, provider, and total deadlines.
- **FR-015**: Cancellation shall be terminal and idempotent and shall reject
  late events and late approval outcomes.

FR-012 and live transport portions of FR-010 through FR-015 are planned; the
transport-free request and validation contracts are current.

### Tools, policy, and approval

- **FR-020**: Every tool shall have a fixed local identity, version, strict
  schema, risk class, permission requirement, and registered implementation.
- **FR-021**: Trusted Rust shall independently validate every normalized
  function call. Provider strict mode shall not be treated as authorization.
- **FR-022**: Deterministic policy shall derive its decision only from trusted
  local metadata and evidence.
- **FR-023**: Personal-data modifications shall require an exact trusted preview
  and explicit one-time approval. Reversible actions require approval unless a
  later accepted policy defines narrow direct intent.
- **FR-024**: Approval shall bind the exact run, request, call, tool, canonical
  arguments, preview, deadline, and manager-issued identity.
- **FR-025**: Rejection, cancellation, expiry, or consumption shall prevent
  replay and late execution.
- **FR-026**: Execution shall occur only through the exact registered restricted
  implementation after every required transition succeeds.
- **FR-027**: An approval resolution, policy decision, or audit receipt alone
  shall never authorize execution.

FR-020 through FR-025 have current transport-free primitives. FR-026 has no
shipping dispatcher or executor.

### Data, memory, tasks, and audit

- **FR-030**: Local product data shall use versioned SQLite migrations and typed
  repository interfaces.
- **FR-031**: Sensitive persisted data shall require encryption with key
  material held outside SQLite.
- **FR-032**: Credentials and authentication codes shall use an approved
  platform secret store and shall never enter SQLite.
- **FR-033**: Users shall be able to review and delete product memory and disable
  future memory use.
- **FR-034**: Task state shall be user-visible and locally controlled before any
  external synchronization.
- **FR-035**: Local audit shall record normalized lifecycle, proposal identity,
  policy, approval, execution, cancellation, and outcome evidence while
  minimizing content.
- **FR-036**: Gateway operational logs and local audit shall remain separate and
  follow approved retention and redaction rules.

Only bootstrap metadata storage and a turn-bound volatile in-memory
approval-audit adapter are current. Product persistence, memory, task
repositories, and durable audit are planned.

### Permissions and integrations

- **FR-040**: Permissions shall be requested only from a user-initiated feature
  flow at the time the capability is needed.
- **FR-041**: Permission Center shall explain capability purpose and current
  status without triggering prompts merely by viewing the page.
- **FR-042**: Users shall be able to connect and revoke each integration
  independently.
- **FR-043**: OAuth access and refresh tokens shall remain outside the WebView
  and SQLite.
- **FR-044**: Operating-system integrations shall use narrow platform adapters
  and validate scope, identity, and target at execution time.

The current Permission Center is status-only and no integration, OAuth, or
privileged adapter is enabled.

## Security and privacy requirements

- **SR-001**: Treat the model, WebView, gateway, files, websites, clipboard,
  connected-service content, and tool results as untrusted.
- **SR-002**: No model-to-device, gateway-to-device, or generic WebView-to-device
  execution path may exist.
- **SR-003**: Production provider credentials shall remain in gateway server-side
  secret storage.
- **SR-004**: External processing shall be disclosed, minimized, and blocked
  until identity, retention, and data-use decisions are accepted.
- **SR-005**: Errors and logs shall exclude secrets, raw provider messages,
  prompts, arguments, results, arbitrary paths, headers, and stack traces unless
  an accepted requirement explicitly needs a bounded field.
- **SR-006**: Tauri commands, capabilities, CSP, and operating-system permissions
  shall remain least-privilege and reviewable.
- **SR-007**: Unknown tools, fields, identities, states, or evidence shall fail
  closed.
- **SR-008**: Security-sensitive state transitions shall have positive, negative,
  replay, expiry, cancellation, and boundary tests.
- **SR-009**: Dependency and release artifacts shall undergo supply-chain,
  secret, signing, and provenance review before production release.
- **SR-010**: Raw personal content shall not be duplicated into audit or
  operational logs.

`SECURITY.md` and `SECURITY_CHECKLIST.md` provide the normative guardrails.

## Non-functional requirements

- **NFR-001 Portability**: keep platform-neutral behavior in Rust and React
  modules that do not depend on macOS APIs.
- **NFR-002 Reliability**: deterministic boundaries shall produce stable typed
  outcomes and fail closed under malformed or late input.
- **NFR-003 Performance**: interactive UI state shall remain responsive; bounded
  requests, events, arguments, and timeouts shall prevent unbounded work.
- **NFR-004 Accessibility**: controls, dialogs, status, navigation, and content
  shall support semantic names, keyboard operation, focus, and readable contrast.
- **NFR-005 Maintainability**: strict TypeScript, strict Clippy, formatting, exact
  dependency versions, focused modules, and verified documentation are required.
- **NFR-006 Testability**: core domain behavior shall be deterministic and
  testable without network, native prompts, or external accounts.
- **NFR-007 Privacy**: local data and external disclosure shall be minimized by
  default and controlled by the user.
- **NFR-008 Observability**: failures shall be diagnosable with closed codes and
  opaque correlation identifiers, not sensitive payloads.
- **NFR-009 Release integrity**: release artifacts shall be reproducible from a
  reviewed commit and pass signing, notarization, installer, launch, and rollback
  gates when those systems are introduced.

## Current verified baseline

The repository currently provides:

- a runnable Cortexa Tauri application with macOS menu and window lifecycle;
- a React workspace with deterministic in-memory assistant interactions;
- narrow app-info IPC and closed native menu-route events;
- SQLite migration and bootstrap metadata infrastructure;
- transport-free gateway request, event, function-schema, policy, approval,
  cancellation, and in-memory approval-audit primitives;
- no live model, gateway, tool execution, product persistence, integration,
  privileged permission, or durable audit path.

This baseline is an engineering proof, not the complete user-ready MVP.

## MVP target scope

The first usable controlled macOS MVP should include:

- native launch, menu access, main workspace, and text interaction;
- live bounded assistant streaming through an authenticated product gateway;
- conversation and task persistence with user controls;
- strict local tool registry, deterministic policy, exact approvals, restricted
  execution, cancellation, and redacted audit;
- Permission Center and independently revocable integrations;
- a small approved set of low-risk information and reversible local tools;
- clear activity, context provenance, failure, and completion presentation;
- signed, notarized, installable, upgradeable, and recoverable macOS artifacts.

Every item remains subject to the incremental security gates in `ROADMAP.md`.

## Explicit non-goals

- A browser-only product, custom GPT, or ChatGPT Project.
- Generic shell, arbitrary script, unrestricted filesystem, raw SQL, or generic
  action execution.
- Direct model or gateway control of device APIs.
- Autonomous email, messages, purchases, bookings, uploads, public posting,
  deletion, or account-setting changes in the MVP.
- Hosted provider tools, arbitrary MCP servers, caller-supplied tool schemas, or
  parallel function calls in the initial production loop.
- Broad Accessibility, screen capture, Apple Events, or microphone permission
  during the current controlled MVP work.
- Storing production credentials, OAuth tokens, passwords, authentication codes,
  or database keys in the application bundle, WebView, SQLite, logs, or audit.
- Claiming enterprise synchronization, mobile support, or cross-platform parity
  before those capabilities are implemented and verified.

## Success criteria

Product success requires evidence in four dimensions:

- **Control**: no consequential action bypasses local validation, policy, exact
  approval, restricted execution, and audit.
- **Utility**: target users complete selected workflows with less manual
  switching, rework, and follow-up effort.
- **Trust**: users can understand context use, proposed effects, permissions,
  outcomes, failures, memory, and audit history.
- **Operability**: IT can install, update, diagnose, revoke, and roll back the
  application with documented security and release evidence.

Pilot measures should include time to information, action cycle time, follow-up
completion, context-switch avoidance, rework, exception rate, policy traceability,
and user trust. Numeric targets require product-owner approval and real pilot
evidence; none are invented here.
