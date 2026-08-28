# Cortexa product requirements

Status: Authoritative normalized product requirements
Last updated: 2026-08-28

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

### Phase 1 primary users

- Individual consumers and professional power users who need a controlled
  personal workspace.
- Consultants and IT professionals who coordinate work across changing
  contexts without enterprise administration.
- Small-business owners who need bounded assistance without organization-wide
  deployment controls.
- Executives, employees, and operators using Cortexa as an individual account.

Phase 1 uses individual accounts, simple onboarding, and personal workspaces.
D-062 selects Microsoft personal identity as its sole initial identity
provider. Work, school, guest, and arbitrary Entra tenants remain outside the
Phase 1 boundary. Google and Apple are deferred under their approved triggers.
Phase 1 does not include enterprise tenant administration, SCIM, enterprise
policy administration, or organization-wide deployment controls.

### Phase 2 users and stakeholders

- Organizations that require team workspaces, centralized billing and
  administration, role-based access control, organization policy, and audit.
- IT leaders responsible for workforce identity, deployment, support, and
  integration.
- Security and privacy reviewers responsible for permissions, data movement,
  credentials, audit, and supply-chain risk.
- Business owners responsible for workflow outcomes and adoption.

Phase 2 may add Microsoft Entra ID workforce SSO, tenant-aware authorization,
and group-based policy. SAML, SCIM, and other enterprise identity providers are
future, demand-driven scope rather than Phase 1 requirements.

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
  principal, own all AI model-provider credentials, select only separately
  approved providers under trusted policy, force approved provider parameters,
  and normalize upstream events. Desktop clients shall never receive provider
  credentials.
- **FR-013**: Trusted Rust shall reject unknown, malformed, out-of-sequence,
  oversized, late, or multiply terminal gateway events.
- **FR-014**: Runs shall enforce independently reviewed turn, request, retry,
  event, content, argument, idle, provider, and total deadlines.
- **FR-015**: Cancellation shall be terminal and idempotent and shall reject
  late events and late approval outcomes.
- **FR-016**: Phase 1 authentication shall use Microsoft personal identity
  through a system browser and OAuth 2.0 Authorization Code Flow with PKCE S256,
  `state`, and OIDC `nonce` behind a provider-neutral OAuth/OIDC application
  boundary.
- **FR-017**: Phase 1 shall accept only the approved personal-account authority,
  issuer, tenant, desktop client, gateway audience, redirect, and delegated
  scope configuration. The gateway shall validate signature, expiration,
  authorization context, and every closed identity field. The WebView, model,
  user content, and arbitrary runtime configuration shall not select identity
  or gateway endpoints.
- **FR-018**: Gateway access tokens shall be audience-bound, limited to a
  maximum 15-minute lifetime, and held only in trusted Rust memory. Initial
  identity scopes shall be `openid`, `email`, and one Cortexa gateway delegated
  scope. `offline_access` and persistent sessions require a separate decision.
- **FR-019**: Identity-provider support, cloud hosting, and AI model-provider
  support shall remain separate approval boundaries. Phase 2 enterprise
  identity and administration shall remain optional capabilities separated
  from Phase 1 individual-account behavior. Initial production targets one
  primary Azure deployment; future AWS or Google Cloud portability does not
  constitute active-active multicloud or a three-cloud release requirement.
  External accounts shall use provider ID plus normalized issuer plus subject;
  email shall not identify or automatically link accounts.
- **FR-019A**: Synthetic demo evaluation shall target OpenAI only through a
  future trusted gateway. Credentials remain server-owned; exact data-control
  evidence, disclosure, limits, and D-061 evidence remain mandatory before any
  real user content. D-067 selects Cloudflare Workers Free only for the internal
  synthetic-demo gateway; it does not alter production hosting. Automatic
  provider fallback is prohibited.
- **FR-020A**: The internal fake-data demo may use one D-068 Cloudflare Access
  service token for at most 30 days, stored only in macOS Keychain and accessed
  only by trusted Rust. This exception is non-production and does not change
  the 15-minute production gateway access-token maximum.

Phase 1 gateway activation shall use D-064's four independent stages. An
approved threat model and closed configuration do not authorize resource
creation; no-traffic provisioning does not authorize authentication or
provider traffic; synthetic-only verification does not authorize real user
content; and real-content activation requires exact D-061 evidence and a
separate project-owner decision. The closed registration uses separate desktop
and gateway API applications, delegated scope `gateway.access`, and a loopback
callback on `127.0.0.1` at `/oauth/callback`. D-064's Azure path remains
historical configuration evidence only; D-066 supersedes it for a future
synthetic demo and grants no provisioning or transport authority.

FR-012 and live transport portions of FR-010 through FR-019A are planned; the
transport-free request and validation contracts are current. No account,
identity-provider, OAuth/OIDC, PKCE, token, gateway, or external-processing path
is implemented. D-064 is a documentation contract, not product capability or
operational evidence.

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
- **FR-037**: The sealed fixture-only Research and Knowledge workflow shall be
  selected only by trusted application code from an exact live Personal
  Assistant root. It shall use exactly two sequential depth-one specialist
  siblings, at most one active child, three total tasks, four sequential run
  attempts, two non-replenishing child attempts, and zero automatic retries.
  Generic Personal-to-Research and the separate approved-document route shall
  retain their existing limits; generic or direct Research-to-Knowledge shall
  remain denied.
- **FR-038**: Research and Knowledge results shall be strict, bounded,
  versioned structured data. Research may reference only application-issued
  IDs from one to eight deterministic fixtures; Knowledge may preserve only
  IDs already validated from the exact predecessor Research task/version.
  A valid Research result with missing references shall remain partial and skip
  Knowledge; a valid incomplete Knowledge result shall remain partial. Unknown,
  duplicate, remapped, malformed, oversized, or reasoning-bearing output shall
  fail closed. Final synthesis shall be a strict bounded V1 envelope whose
  source-ID set exactly matches the set preserved by the validated Research
  outcome, whose fixture disclosure is true, and whose complete/partial status
  matches the validated stage outcomes. Its answer shall remain within 2,048
  scalar values/8,192 bytes and explicitly disclose fixture evidence and
  partial status when applicable. Missing, invented, duplicate, or unknown
  references, false disclosure, wrong status, URLs, live-research claims,
  reasoning, and unknown fields shall fail closed; final synthesis shall never
  receive raw invalid specialist output.
- **FR-039**: Workflow partial failure, continuation-start failure,
  cancellation, memory, event, and audit handling shall remain bounded and
  truthful. Terminal transitions shall preflight required capacity and prepare
  parsing and continuation input before accepting the terminal runtime event;
  an accepted terminal event shall not be reversed by a later continuation
  start failure. Root cancellation shall cancel pending governance and the
  active child before the root and start no later stage. Specialist task memory
  shall be isolated and cleaned at terminal state, reusable Knowledge content
  shall remain pending review, and the 16-record workflow journal and matching
  non-authoritative attribution audit shall exclude content and grant no
  authority.
- **FR-039A**: The sealed fixture-only engineering-quality workflow shall be
  selected only by trusted application code from an exact live Personal
  Assistant root. `AgentOrchestrator` alone shall create Coding, QA &
  Validation, and Security & Risk as three sequential depth-one siblings under
  exact four-task, three-child, five-run, one-active-child, 32-event,
  16-workflow-record, and zero-retry limits. Generic delegation, D-085, D-086,
  and engineering selectors shall be mutually exclusive.
- **FR-039B**: Engineering results shall be strict, bounded, versioned,
  proposal-only data. `ChangeProposal` shall reference only application-issued
  fixture/evidence IDs; `ValidationReport` shall reconcile every criterion
  exactly once without fabricating execution; `RiskAssessment` shall remain
  evidence-bound or explicitly hypothetical and advisory. Final synthesis
  shall disclose fixture/proposal-only input and no execution, derive
  `RequiredBeforeMutation` only when a patch is proposed, and create no
  approval request or execution authority.
- **FR-039C**: Trusted application code shall select exactly one of two separate
  fixture-only infrastructure/operations workflows from a live Personal root.
  `AgentOrchestrator` alone shall create either Cloud Infrastructure or Systems
  Operations, then QA and Security, as sequential depth-one siblings under
  exact four-task, three-child, five-attempt, one-active-child, 32-event,
  16-workflow/audit-record, and zero-retry limits. The selectors shall be
  mutually exclusive with each other and every existing workflow.
- **FR-039D**: Cloud and Systems outputs shall be strict bounded proposal-only
  data preserving exact application-issued scenario, fixture, criterion,
  evidence, predecessor, and result identity. Cloud shall accept only the
  built-in synthetic Terraform/Azure decision fixture; Systems shall accept
  only the built-in synthetic service/log/recovery fixture. QA and Security
  shall remain advisory; consequential capabilities shall be denied and never
  dispatched; final synthesis shall create no approval request or execution
  authority.
- **FR-039E**: Trusted application code shall select one sealed Personal ->
  Workflow Automation -> Personal proposal lifecycle. The application shall
  own five immutable templates and validate exact step shape, dependencies,
  agents, cycles, limits, bounded inputs, strict structured output, and
  application-derived disposition. Complete A-D proposals alone may issue one
  opaque, process-local, expiring take-once manual token whose fresh destination
  maps only to the matching existing fixture-only/no-I/O selector. Template E
  shall remain proposal-only; no agent or runtime may construct the token or
  select the destination.
- **FR-039F**: Workflow Automation shall remain memory-disabled, tool-
  ineligible, non-spawning, and non-authorizing. Unknown tool/version/arguments
  shall fail closed; known tool and approval steps shall be recognized but
  remain non-executable and shall issue no token or approval request. The
  original 120-second monotonic deadline shall propagate through manual A-D
  dispatch and be checked cooperatively at trusted destination lifecycle
  ingress with child-first cancellation and no successor after expiry. It does
  not promise hard preemption of an in-flight synchronous runtime call.
- **FR-039G**: Trusted application code alone shall select the sealed D-091
  bounded-parallel workflow. `AgentOrchestrator` may retain multiple independent
  depth-one specialist runs under exact default-active two, hard-active and
  total-child three, four-task, five-run-attempt, zero-retry, eight-event-per-
  run, 32-record, 120-second-root, and 60-second-child limits. Specialists and
  Workflow Automation shall not create children or nested workflows; excess
  cataloged work shall queue locally or reject with typed behavior.
- **FR-039H**: Parallel outcomes shall be exactly succeeded, failed, cancelled,
  timed out, or skipped and shall be stored and synthesized in catalog ordinal
  order. `ContinuePartial`, `CancelDependentOnly`, and specialist-lane
  `FailFast` shall be explicit application policy. Root cancellation/expiry
  shall sweep children deterministically; independent child cancellation shall
  not cancel an unrelated sibling unless policy requires it. Final synthesis
  shall expose each source agent, status, validated findings, failures, and
  unresolved issues without concealing partial results.
- **FR-039I**: Same-thread event multiplexing shall preserve exact task/run/
  profile attribution, isolated context and task memory, separate cancellation
  handles, cooperative deadlines, rejected-run cleanup, and no orphan run.
  It shall not be represented as provider or CPU concurrency, hard preemption,
  provider-session isolation, an app-global capacity coordinator, scheduler,
  general workflow engine, or distributed execution.
- **FR-039J**: The deterministic Command Center fixture projection and its
  controls shall remain a lazy, frontend-owned `command-center-demo-v1`
  projection with one distinct
  `AgentOrchestrator`, all nine exact agent roles, five view-only groups,
  bounded closed scenarios, persistent simulated-data disclosure, and no
  Rust-agent IPC or consequential control.
- **FR-039K**: The Command Center graph shall have a synchronized grouped
  structured view and relationship table, keyboard-operable selection and
  viewport controls, ordinary page-wheel pass-through, feature-local filters,
  reachable inspector/activity content, and no claim of live health, approval,
  policy, execution, provider, tool, or telemetry state.

Only bootstrap metadata storage, a turn-bound volatile in-memory approval-audit
adapter, and D-085's unwired workflow-local volatile memory and approved-
document boundary are current. D-085 implements four bounded memory namespaces,
explicit selected-record context, versioned shared review, selected lowercase
`.txt`/`.md` reading, and one direct Personal Assistant-to-Knowledge
deterministic task. It does not persist across its one-root orchestrator, expose
a user-facing file or memory surface, or satisfy durable product-memory
requirements. D-086 separately implements only the sealed fixture-only
Personal-to-Research-to-Knowledge-to-Personal application-service sequence. It
has no generic or caller-selectable consumer; one fixed instance is consumed
only by the no-input simulated demo host and panel described in the current
baseline below. Its
strict parsers cap every structured output envelope at 8,192 scalar values and
16,384 bytes and the final answer itself at 2,048 scalars/8,192 bytes. Research
and Knowledge inputs are capped at 26,624 bytes each, synthesis input at 36,864
bytes, runtime and generic events at 32 each, and workflow events/audit records
at 16 each. It adds no live research, provider, persistence, durable or generic
audit, or general agent UI. Product persistence, durable or user-facing memory,
task repositories, and durable audit remain planned.

D-087 separately implements only the unwired fixture-only, proposal-only
Coding-to-QA-to-Security workflow. It adds no live repository/filesystem/
process/Git/package/network access, registered engineering tool, executor,
approval request, mutation, memory, persistence, Tauri/React consumer, IPC,
provider, external runtime, or device behavior. Coding, QA, and Security are
`Initial` only for that sealed workflow; generic routes and policy/memory
profiles remain unchanged and execution remains `NotAttempted`.

D-088 separately implements two unwired fixture-only/no-I/O selectors. The
Cloud built-in is synthetic Terraform configuration plus Azure architecture
and validation evidence; the Systems built-in is a synthetic service snapshot,
sanitized log excerpt, recovery scenario, and validation evidence. It adds no
Terraform/platform/OS command, live inventory or diagnostic, credential, tool,
executor, approval dispatch, provider, IPC/UI, dependency, persistence,
permission, external runtime, or effect. Cloud and Systems are `Initial` only
for those sealed selectors; QA/Security remain advisory and all four roles stay
tool-ineligible and memory-disabled.

D-090 separately implements one unwired strict Personal-to-Workflow-
Automation-to-Personal proposal selector and a take-once manual bridge to the
existing sealed A-D fixture workflows. Workflow Automation is `Initial` only
for that selector; generic delegation remains denied. Template E, all tool and
approval steps, scheduling, persistence, and every external effect remain
non-executable.

D-091 separately implements one unwired fixture-only/no-I/O bounded-parallel
selector with three immutable scenarios, explicit failure policy, stable
ordinal results, cooperative cancellation/deadlines, strict synthesis, and no
runtime/provider/thread/general-engine widening. Its same-thread retained-run
model is not production/provider concurrency evidence.

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
- **SR-011**: No external model processing shall occur until an approved
  identity configuration, deployed gateway, disclosure, and that AI provider's
  retention, data-use, logging, region, and security evidence satisfy their
  exact gates.
- **SR-012**: Before provider-approved ZDR is verified for the exact provider,
  organization, project, endpoint, model, and region configuration, external
  testing shall use synthetic data only. After verification, initial real-user
  processing is limited to explicitly submitted, non-sensitive text.
- **SR-013**: Credentials, attachments, regulated data, financial or healthcare
  data, and sensitive personal data shall not be transmitted in the initial
  consumer or enterprise processing boundary.
- **SR-014**: Gateway logs shall contain operational metadata only, retain it
  for no more than seven days, and never contain content. External-processing
  disclosure shall precede the first transmission and remain visible in
  Settings.

`SECURITY.md` and `SECURITY_CHECKLIST.md` provide the normative guardrails.

## Non-functional requirements

- **NFR-001 Portability**: keep platform-neutral behavior in Rust and React
  modules that do not depend on macOS APIs. Keep a future gateway containerized
  enough for separately justified cloud portability without claiming current
  multicloud deployment or failover.
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
- a Rust-internal workflow-local volatile memory and selected-document boundary,
  including one deterministic Personal Assistant-to-Knowledge task, with no
  persistence, IPC, provider, or user-facing consumer;
- a sealed fixture-only Personal-to-Research-to-Knowledge-to-Personal workflow
  with strict source provenance, typed partial results, child-first
  cancellation, and content-free volatile workflow evidence. It has no generic
  or direct consumer; only the fixed demo-host instance below is connected;
- a manually stepped, process-local lifecycle owner for one fixed instance of
  that sealed Research/Knowledge workflow, exposed only through four no-input
  Tauri commands, one non-authoritative notification, and one prop-free visibly
  simulated panel in the selected Command Center scenario; it retains a closed
  bounded snapshot, private success/failure schedule, cancellation and late
  event rejection, and fail-closed cleanup/replacement quarantine;
- an unwired sealed fixture-only Personal-to-Coding-to-QA-to-Security-to-
  Personal proposal workflow with strict evidence provenance, typed partial
  results, closed capability denial, and no execution;
- two unwired sealed fixture-only Personal-to-Cloud-or-Systems-to-QA-to-
  Security-to-Personal proposal workflows with strict fixture provenance,
  typed partial results, closed capability denial, and no I/O or execution;
- one unwired strict Personal-to-Workflow-Automation-to-Personal proposal
  lifecycle with five immutable templates, A-D-only take-once manual sealed
  dispatch, proposal-only E, and no tool or approval execution;
- one unwired fixture-only/no-I/O bounded-parallel selector with three sealed
  same-thread event-multiplexed scenarios, explicit failure policy, stable
  ordinal outcomes, truthful synthesis, and no provider/session/thread work;
- one frontend-owned deterministic Command Center fixture projection with a
  lazy route, exact architecture labels, structured alternative, and IPC-free
  fixture controls. Its required real-browser/Tauri viewport/input/accessibility
  matrix is complete and verified; the separately rendered read-only projection
  and lifecycle panels do not populate or control that fixture projection;
- no live model, gateway, tool execution, durable product persistence, integration,
  privileged permission, or durable audit path.

This baseline is an engineering proof, not the complete user-ready MVP.

## First usable v0 target

D-094 defines a deliberately narrower private v0 before the controlled MVP.
It is Personal Assistant only and permits one foreground, explicitly submitted
text request at a time, bounded streamed text, one bounded final answer,
terminal cancellation, and closed redacted failure.

Trusted Rust owns every run/configuration identity and exposes no caller
selector for agent, task, run, profile, runtime, workflow, instructions,
provider/model, tools, endpoint, limits, retry, or fallback. The exact tool set
is empty. The v0 has no files, persistence, memory, scheduling, background
autonomy, specialist delegation, action execution, approval dispatch, durable
audit, or device effect.

The v0 is reached only after three truths remain separate:

1. a live synthetic-text OpenAI-through-Cloudflare proof using a fixed
   application-owned fixture and a no-text synthetic-v1 UI contract;
2. separately activated private processing of explicitly submitted,
   non-sensitive personal text through a distinct real-content-v2 contract only
   after identity, provider/hosting, ZDR, disclosure, logging, deletion,
   operations, and target-Mac gates pass; and
3. later action-taking or production scope under new decisions.

Only item 2 is the first usable v0. Its identity/provider/hosting topology is
not yet selected; the synthetic service-token exception cannot be reused. The
synthetic proof is not a personal assistant, and the v0 is not the complete MVP
below. Current implementation evidence does not satisfy any live milestone.

## MVP target scope

The first usable controlled macOS MVP should include:

- native launch, menu access, main workspace, and text interaction;
- individual accounts, simple onboarding, and personal workspaces;
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
- Enterprise tenant administration, organization policy administration,
  organization-wide deployment controls, SAML, or SCIM in the Phase 1 consumer
  and prosumer launch.
- An implemented `AgentProvider`, AI model-provider integration, Azure, AWS, or
  Google Cloud deployment, active-active multicloud, cloud failover, or a
  three-cloud initial release.
- Automatic cross-provider fallback or treating one provider's retention
  approval as approval for another.

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
