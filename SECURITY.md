# Security policy and development guardrails

Status: Authoritative security policy
Last updated: 2026-08-20

Use `SECURITY_CHECKLIST.md` for change and release review. `ARCHITECTURE.md`
identifies which security boundaries are current, mocked, planned, or
prohibited.

## Security model

Cortexa treats the model as an untrusted planner. The architecture requires the
trusted Rust core to validate requests, apply deterministic policy, obtain exact
approval where required, execute only registered tools, and record redacted
audit events.

The current repository implements transport-free validation, policy, approval,
cancellation, and a turn-bound in-memory approval-audit adapter. It also has an
unwired non-executing per-agent governance foundation: nine closed profiles,
orchestrator-derived live attribution, profile-aware deterministic policy,
agent-origin approval, an exact delegation matrix, and a bounded volatile audit
family. D-085 adds a separately bounded workflow-local volatile memory store,
selected-record context assembly, an approved-document reader, and one direct
Personal Assistant-to-Knowledge document task. D-086 adds one separately
selected, fixture-only Personal-to-Research-to-Knowledge-to-Personal sequence
with strict structured results, source-ID provenance, truthful partial outcomes,
child-first cancellation, and a content-free volatile workflow journal and
attribution audit. D-087 adds one separately selected fixture-only,
proposal-only Personal-to-Coding-to-QA-to-Security-to-Personal sequence. Its
strict results preserve only application-issued fixture, criterion, proposal,
QA, and evidence references; consequential capabilities are denied data, QA
and Security remain advisory, and final approval requirement is derived without
creating an approval request or execution subject. D-088 adds two further
separately selected fixture-only/no-I/O sequences: Cloud
Infrastructure or Systems Operations followed by QA, Security, and Personal
synthesis. Their immutable built-ins contain only synthetic Terraform/Azure or
sanitized service/log/recovery evidence. Consequential capabilities remain
denied data and no command, credential, live access, approval dispatch, or
effect exists. D-090 adds one strict fixture-only Personal-to-Workflow-
Automation-to-Personal proposal lifecycle and a take-once manual bridge from
complete A-D proposals to those existing sealed selectors. Template E and all
tool/approval steps remain non-executable. Workflow Automation cannot construct
trusted identity, create tasks, issue a token, approve, execute, or select a
destination. D-091 adds one further application-only, fixture-only/no-I/O
bounded-parallel selector. It binds every live event to exact task/run/profile
identity, orders outcome and cancellation handling by sealed catalog ordinal,
isolates task memory and cancellation handles, validates strict bounded
results/synthesis, and quarantines rejected runtime identities until cleanup
succeeds. Its same-thread retained-run model is neither provider/CPU
concurrency nor an app-global budget or session boundary. A successful terminal native or run-termination resolution
cannot leave the initial turn without one typed legacy audit receipt. Agent
governance reserves one audit slot before downstream mutation and records
execution only as `NotAttempted`. There is no live provider transport,
dispatcher, executor, durable or user-facing product memory, platform adapter,
or durable audit. Current resolutions, memory, records, references, and receipts
are volatile and non-authorizing and must not be mistaken for an end-to-end
security path.

The Active Command Center prototype is a separate untrusted WebView
presentation boundary. Its `command-center-demo-v1` data is frontend-owned,
closed, bounded, fixture-derived, redacted, and persistently labeled simulated.
Search, filters, selection, graph viewport controls, inspector, and activity
change only feature-local presentation state. They invoke no Tauri command,
event listener, network, clipboard, storage, filesystem, provider, tool,
approval, policy, audit, runtime, or device action. Fixture IDs are not trusted
Rust identities.

The owner approved exact `@xyflow/react@12.11.3` and
`lucide-react@1.33.0` after direct/transitive, license, peer, bundle, and
security review. Their lockfile consequence is 19 reviewed transitives. At the
Command Center checkpoint, the production audit reported zero vulnerabilities
while five pre-existing development-only advisories remained unchanged; the
superseding PR #57 remediation below resolves them. React Flow types/imports
stop at one feature adapter. No Rust/Tauri capability, CSP, IPC, permission, or
native dependency changed.

PR #57's baseline full development audit reports five vulnerable indirect
package-level findings—four High and one Moderate—across six lockfile nodes,
while its production-only audit and repository secret scan pass. The completed
`pr57-transitive-advisory-remediation` gate advanced only those six
development-only nodes within existing parent ranges. Exact graph, license,
integrity, engine, install-hook, clean-install, full/production zero-audit, and
complete repository verification pass with no manifest, override,
install-script allowlist, audit-policy, application, or governance change.
Independent architecture/security/code review and every classifier-selected
check on exact remediation `c3cc49e` also pass, including the unchanged
accepted Cargo advisory baseline. The completion report is `PASS WITH
ADVISORIES`; no dependency or security finding remains, and the sole advisory
is that no next implementation plan is owner-selected or Ready. The
deterministic marker is complete and valid.

## Non-negotiable invariants

- The WebView cannot execute a generic local action.
- Unknown tools and invalid arguments fail closed.
- Tool schemas use strict validation and reject additional properties.
- Class 3 personal-data modifications always require an exact trusted preview and explicit approval.
- Class 4 actions are not registered in the MVP.
- Class 5 behavior is prohibited.
- Approval is bound to canonical arguments, tool identity, expiry, and one-time consumption.
- Agent-origin approval is additionally bound to exact live agent/task/root/
  parent/runtime/profile/depth/run/request attribution.
- Runtime tool proposals remain rejected; synthetic agent governance has no
  executor and every execution disposition is `NotAttempted`.
- Delegation stays outside `ToolRegistry`; only the orchestrator may create a
  child after the exact Personal Assistant-to-Research matrix and finite limits.
- Memory access requires sealed definition/task/live-runtime attribution,
  including the exact memory profile; no caller or untrusted content supplies a
  grant, namespace, owner, or application-review authority.
- Proposed shared memory is unreadable as approved shared until an exact
  version-checked application review promotes it. Private and task memory never
  transfer across agents or workflows.
- Approved-document access uses opaque workflow-bound references selected by
  trusted application code. Paths remain private, roots cannot be enumerated,
  and one-time read authority grants no general filesystem permission.
- The direct Personal Assistant-to-Knowledge document route and D-086's sealed
  fixture workflow do not alter generic Personal-to-Research delegation.
  Generic or direct Research-to-Knowledge remains denied, specialists cannot
  spawn agents, and only the orchestrator may create the Knowledge sibling after
  validating the exact Research result.
- The sealed workflow permits only three tasks, two non-replenishing sequential
  depth-one children, one active child, four run attempts, 32 runtime events,
  32 generic events, 16 workflow events, 16 matching workflow audit records,
  and zero automatic retries. Overflow fails closed and never replenishes a
  task, child, run, event, or retry budget.
- Research output may reference only IDs from the immutable one-to-eight-source
  application fixture catalog. Knowledge output may preserve only references
  from the validated predecessor Research task and version. Unknown, duplicate,
  remapped, malformed, oversized, or reasoning-bearing output fails closed;
  a valid Research result with missing references remains partial and skips
  Knowledge, a valid incomplete Knowledge result remains partial, and citations
  are never invented.
- D-086 terminal parsing, terminal task output, remaining capacity, next task,
  request, descriptive attribution, and fallback/synthesis input are prepared
  before terminal runtime acceptance. Preparation failure causes zero workflow
  mutation. A continuation start failure cannot reverse an accepted terminal
  event and is exposed only through a closed content-free failure category.
- Research or Knowledge failure/cancellation never forwards raw invalid output;
  only validated results plus typed stage status may reach Personal synthesis.
  Personal synthesis itself must pass the strict bounded V1 result contract,
  preserve exactly the Research outcome's source-ID set, affirm fixture-only
  evidence, disclose partial status when applicable, and match the derived
  complete/partial status; invented references, URLs, live-research claims,
  reasoning, false disclosure, and unknown fields fail the root.
  Research missing-source partials skip Knowledge; Knowledge partials preserve
  validated Research. Root cancellation resolves pending governance, cancels
  the active child before the root, starts no later stage, preserves retryable
  live state on cancellation failure, and rejects late events.
- `ResearchKnowledgeAttribution` is a non-authoritative descriptive snapshot.
  Its run/request identity remains private and redacted, it cannot reconstruct a
  live execution context or grant policy/approval/memory/runtime/execution
  authority, and its journal records contain no objective, fixture content,
  findings, summary, proposal, path, URL, output, or reasoning.
- D-087's sealed engineering workflow permits only four tasks, three non-
  replenishing sequential depth-one children, one active child, five run
  attempts, 32 runtime/generic events, 16 workflow events/audit records, and
  zero retries. Generic, document, D-086, and engineering selectors are
  mutually exclusive, and no specialist can spawn.
- Engineering fixtures, criteria, and evidence use only immutable application-
  issued IDs. Evidence is `ObservedFixture` or `NotRun`; it cannot represent a
  live test pass. Unknown, duplicate, malformed, oversized, reasoning-bearing,
  URL-bearing, identity-supplying, or authority-claiming output fails closed.
- Coding patch text is inert proposal data. File mutation/deletion, path escape,
  dependency/package/test/formatter execution, Git operations, destructive
  shell, credential access, and network access are denied and never dispatched.
  QA cannot approve or fabricate evidence; Security cannot authorize,
  remediate, replace policy, or access/expose secret values.
- Final engineering synthesis must disclose fixture-only, proposal-only, and
  no-execution status. `RequiredBeforeMutation` is application-derived only
  when a patch is proposed; no approval request or execution authority exists.
  Coding, QA, and Security remain tool-ineligible, memory-disabled, and
  `NotAttempted` for execution.
- Each D-088 selector is application-only and mutually exclusive with the other
  selector and every prior workflow. Cloud or Systems, QA, and Security are
  sequential depth-one siblings under four-task, three-child, five-attempt,
  one-active-child, 32-event, 16-workflow/audit-record, and zero-retry limits.
- Cloud accepts only the sealed synthetic Terraform-configuration/Azure-
  architecture scenario. Systems accepts only the sealed synthetic service-
  snapshot/sanitized-log/recovery scenario. Findings and stage transfers may
  reference only application-issued scenario, fixture, criterion, evidence,
  predecessor, and result identities.
- Terraform/platform commands, live inventory/diagnostics, cloud or system
  mutation, IAM/firewall/account changes, service/process control, reboot/
  shutdown, configuration/package/patch operations, privileged shell, VMware/
  backup mutation, credential access/rotation, filesystem/network access, and
  every other consequential operation are denied inert proposal data.
- QA cannot approve or treat `NotRun` as passing evidence. Security cannot
  authorize, remediate, become policy, or claim unavailable credential/
  platform evidence. Final synthesis creates no approval request or executable
  subject. Cloud, Systems, QA, and Security remain tool-ineligible, memory-
  disabled, and `NotAttempted` for execution.
- D-088 string and credential-pattern guards are defense in depth only. They do
  not prove secret absence and cannot authorize any later live, credential,
  tool, command, or effect path; those require separate trusted containment and
  governance.
- D-090's proposal selector is application-only and mutually exclusive with
  every other selector. It permits two sequential depth-one tasks, three run
  attempts, one child, zero retries, and bounded content-free event/audit
  evidence. Only exact immutable A-D templates can become manually dispatchable;
  E remains proposal-only.
- Proposal validation fails closed on unknown/disabled agents, unknown or
  mismatched tools, malformed arguments, duplicate or cyclic dependencies,
  unsupported steps, excessive limits, nested execution, self-modification,
  false authority/effect claims, and noncanonical template content. Known tool
  and approval steps remain non-executable, create no approval subject, and
  cannot issue a token.
- A dispatch token is opaque, non-cloneable, non-serializable, process-local,
  taken once, and consumed on success or every error. A fresh destination maps
  it only to an existing sealed A-D selector. The propagated 120-second
  monotonic deadline is cooperatively checked at trusted destination ingress;
  expiry cancels child-first and starts no successor, but cannot preempt a
  synchronous runtime call already in flight.
- Workflow Automation remains tool-ineligible and memory-disabled; its generic
  delegation route is denied. D-090 invokes neither QA nor Security in the
  proposal lifecycle and adds no policy decision, approval request, executor,
  scheduler, persistence, IPC/UI, provider, external runtime, I/O, credential,
  or device effect.
- D-091 is application-selected and mutually exclusive with every prior
  selector. Specialists and Workflow Automation cannot select it, create a
  child, delegate, or nest a workflow. Its exact limits are depth one, default
  active two, hard active and total child three, four tasks, five run attempts,
  zero retries, eight events per run, 32 applicable records, a 120-second root
  lease, and 60-second child leases capped by the root.
- Every admitted D-091 child has exact distinct task/run/context/profile/memory
  attribution and an application cancellation handle. Cross-run, stale, late,
  terminal, wrong-sequence, or over-cap events fail closed. Task memory remains
  isolated and terminally cleaned; successful result transfer contains only
  strict bounded fixture data, never memory or authority.
- D-091 outcomes are exactly succeeded, failed, cancelled, timed out, or
  skipped in catalog ordinal order. Explicit `ContinuePartial`,
  `CancelDependentOnly`, and specialist-lane `FailFast` policy controls sibling
  and dependent handling. Final synthesis must expose exact source agents,
  statuses, finding IDs, failures, and unresolved issues.
- Root cancellation/expiry sweeps active children deterministically before the
  root/synthesis run. Cancellation failure preserves closed resumable live
  state, and a rejected returned run remains quarantined until cleanup succeeds;
  no root terminal may conceal a live rejected run.
- Every legacy and bounded-parallel runtime start validates the exact
  application-created run/request identity and rejects duplicate live identity.
  A rejected nonterminal run is retained for explicit cancellation cleanup, and
  no new or fallback start proceeds while cleanup is pending.
- Cooperative deadlines cannot preempt a synchronous runtime call already in
  flight. Per-orchestrator limits do not prove provider or app-global capacity,
  and distinct runtime-run identity does not prove provider-session isolation.
  The lexical fixture authority-claim filter is defense in depth only and must
  never authorize a future live/provider/tool/effect path.
- Governance audit is closed, redacted, volatile, capped at 32 subjects, and
  never authorizes an action.
- Untrusted content cannot grant permission or change policy.
- Credentials, authentication codes, private keys, and production API keys are never logged or stored in SQLite.
- Privileged macOS permissions are requested only from a user-initiated feature flow.

## Phase 4 gateway boundary

- Every production AI model-provider credential belongs only to the authenticated product gateway and must be loaded there from server-side managed secret storage. It must never enter the desktop application, WebView, Tauri IPC, SQLite, local logs, crash reports, or audit records.
- No gateway is deployed, no identity provider is integrated, no cloud deployment exists, and no gateway or AI model-provider networking or external transmission is currently permitted.
- D-060 separates identity-provider, cloud-hosting, and AI model-provider approval. Approval in one category grants no approval in another.
- D-062 selects Microsoft personal identity as the sole Phase 1 provider without authorizing implementation. The planned flow uses the system browser, OAuth 2.0 Authorization Code Flow, PKCE S256, `state`, and OIDC `nonce`. The personal-account authority, exact discovery-derived issuer, tenant, separate public desktop client and gateway API resource, gateway audience, loopback redirect, and delegated scope are closed configuration. Work, school, guest, and arbitrary Entra tenants fail closed. Google is deferred until demonstrated demand after Microsoft verification; Apple is deferred until Mac App Store planning or demonstrated demand.
- The canonical external identity key is provider ID plus normalized issuer plus subject. Email is never an identity key and cannot automatically link accounts. Any future explicit cross-provider linking requires a separate approved threat model and reauthentication design.
- Phase 2 may add Entra workforce SSO, tenant-aware validation, group authorization, and enterprise administration. Those controls and other compatible enterprise OIDC or SAML identity providers remain separately approved future capabilities. AWS and Google Cloud accounts are not treated as consumer identity providers.
- Azure Container Apps in Central US is the initial planned hosting target, and `https://api.cortexaai.io` is the reserved origin. The origin remains inactive until DNS, TLS, deployment, authentication, authorization, logging, and security verification pass. Initial production uses one primary cloud. Container portability does not authorize AWS, Google Cloud, active-active multicloud, cloud failover, or a three-cloud release.
- A future trusted `AgentProvider` abstraction may select only separately approved AI model providers under trusted gateway or core policy. No implementation currently exists. Each AI provider requires independent retention, ZDR, data-use, logging, region, and security approval.
- D-066 selects OpenAI as the sole synthetic-demo candidate. Any future credential is gateway-owned and never reaches the desktop or WebView; exact data-control evidence, disclosure, limits, and a separate implementation plan remain mandatory. The owner configured a non-secret project boundary, but no API credential, endpoint integration, or network path exists.
- D-067 selects Cloudflare Workers Free only as the future internal-demo gateway
  candidate. The OpenAI project exists with owner-confirmed synthetic-only
  restrictions, bounded spend/rate settings, one allowed model, and disabled
  API-call logging; its identifier and any future key remain outside the
  repository. A future key may exist only as a Worker secret. No Worker, route,
  secret, client authentication, deployment, or provider traffic exists.
- D-068 permits one future 30-day-maximum Cloudflare Access service token only
  for the owner-only fake-data demo. Its secret is macOS-Keychain-only and
  trusted-Rust-only; the Access policy is restricted to one application and the
  Worker must validate JWT signature, issuer, and exact audience. Revocation and
  route disablement fail closed. No token or Keychain item currently exists,
  and D-064's production 15-minute access-token maximum is unchanged.
- D-069 accepts only the fake-value macOS Keychain read proof. The trusted Rust
  adapter uses fixed service/account labels, exposes no raw value, returns
  closed redacted outcomes, and has no write, delete, enumeration, IPC, WebView,
  SQLite, startup, network, or runtime-consumer path. Owner-operated evidence
  removed both fake items after successful proof. Repeated authorization
  prompts did not prove stable unsigned-executable access; real credential
  ingestion remains blocked.
- D-070 requires a separately approved stable signed application identity or
  narrowly reviewed app-specific Keychain ACL, production secret-memory
  lifecycle, direct owner-only transfer, rotation, revocation, rollback,
  dependency reassessment, and target-Mac evidence before real demo-token
  ingestion can be proposed. This is a planning gate, not credential or
  Cloudflare-action authority.
- D-071 requires a separate owner-approved selection between signed identity
  and narrow app-specific ACL; it also requires a distinct secret-memory model.
  Unsigned prompt behavior is not an approved fallback, and documentation
  creates no signing, Keychain, credential, or Cloudflare authority.
- D-072 selects stable signed macOS identity as the future model. It does not
  authorize signing assets or real credential handling; a later exact plan must
  define least-privilege scope, secret-memory ownership, lifecycle, and private
  target-Mac evidence.
- D-073 constrains a later fake-only signed-identity and secret-memory proof to
  three existing Rust paths. Any dependency, configuration, entitlement, IPC,
  networking, or runtime need requires new owner approval.
- D-064 closes the pre-implementation configuration and separates design,
  no-traffic provisioning, synthetic-only transport, and real-content
  activation. Its authoritative configuration is
  [`docs/security/phase4-gateway-configuration-spec.md`](docs/security/phase4-gateway-configuration-spec.md),
  and its threat actors, trust boundaries, abuse cases, controls, and required
  tests are in
  [`docs/security/phase4-gateway-threat-model.md`](docs/security/phase4-gateway-threat-model.md).
  No stage authorizes or starts the next stage.
- The closed registration uses separate Microsoft personal desktop and gateway
  API applications, `api://<gateway-api-client-id>`, delegated scope
  `gateway.access`, and a `127.0.0.1` ephemeral callback at
  `/oauth/callback`. Actual identifiers and observed claims remain restricted
  operational evidence; mismatch fails closed.
- The closed Azure boundary below is retained as historical D-064 design
  evidence only; D-066 supersedes it as the synthetic-demo provider direction.
  It uses one dedicated non-shared user-assigned managed
  identity, exact-resource `Cognitive Services OpenAI User` RBAC, a private
  Azure OpenAI endpoint, and disabled provider public network access before any
  provider traffic. API-key fallback, broad runtime roles, unrestricted egress,
  alternate origins, and automatic fallback are prohibited.
- Other providers remain separately approved future adapters. Automatic or
  silent provider fallback is prohibited because it can cross retention, region,
  and contractual boundaries.
- A future desktop gateway access token must be short-lived with a maximum 15-minute lifetime, audience-bound, read only by trusted Rust through the platform secret-store abstraction, and stored in process memory. Initial scopes are `openid`, `email`, and one exact delegated gateway scope. `profile`, Microsoft Graph, directory, group, mail, calendar, file, contact, and `offline_access` scopes are excluded. Persistent sessions and `offline_access` require a separate decision; any later approved refresh or session credential must use platform-secure credential storage. The WebView must never receive any credential.
- Microsoft's documented default access-token lifetime does not satisfy the
  accepted 15-minute maximum. Stage B must prove an enforceable compatible
  personal-account configuration or an additive decision must define another
  short-lived gateway-session boundary. The manifest-based `127.0.0.1`
  ephemeral callback also requires Stage B and target-Mac Stage C evidence. No
  silent lifetime or redirect fallback is permitted.
- The Rust core may send only a closed, size-bounded request contract to one configured HTTPS gateway origin. The WebView must not choose the gateway URL, identity provider, AI model provider, model, provider parameters, tool schemas, or authorization headers.
- The gateway authenticates and authorizes the desktop principal, applies request/rate/model/tool-set limits, selects an approved AI model provider under trusted policy, injects only that server-held provider credential, and normalizes upstream events. It cannot approve or execute local tools.
- The gateway selects exact server-side tool-set versions. It must not forward arbitrary caller-supplied OpenAI tools, hosted tools, MCP servers, shell tools, or provider parameters.
- OpenAI function definitions must use strict mode, require every property, reject additional properties, and disable parallel tool calls for the first production loop. Strict provider generation is defense in depth, not local authorization.
- The gateway must validate recognized upstream event types and required fields, enforce sequence and size limits, and reject unknown event types. Additive fields on a recognized OpenAI event may be ignored because the upstream API documents them as backward-compatible additions; no unrecognized field may cross the normalized protocol.
- The gateway-to-Rust protocol must be versioned, closed, size-bounded, sequence-checked, and reject unknown fields. Function calls remain untrusted until trusted Rust independently validates the call ID, registered tool name, tool-contract version, argument JSON, exact per-tool schema, risk, permission, policy, and approval state.
- Use foreground Responses streaming with `store: false` and `background: false`. Cancellation transitions the local validator to a terminal cancelled state, aborts the desktop-to-gateway request and gateway-to-provider stream, discards late events, and is idempotent. Do not expect a cancellation event over the aborted stream or claim provider-side cancellation completion.
- `store: false` minimizes Responses application-state storage but does not eliminate provider abuse-monitoring retention. D-061 requires verified provider-approved ZDR independently for each AI provider's exact production organization, project, endpoint, model, and region configuration before real user content. Until then, only synthetic data may be considered under a separately approved future transport test.
- Azure real-content approval additionally requires exact resource evidence for `ContentLogging=false` and documented confirmation that the selected stateless Responses configuration retains no application state. Hosted tools, files, retrieval, Agents, Assistants, Batch, stored completions, web search, MCP, code execution, and response retrieval remain excluded.
- After ZDR verification, the initial real-user data class is explicitly submitted, non-sensitive text only. Credentials, attachments, regulated data, financial or healthcare data, and sensitive personal data remain prohibited.
- Provider and gateway failures cross into Rust only as closed redacted error codes, retryability, bounded retry delay, and opaque correlation IDs. Raw response bodies, headers, stack traces, prompts, model output, function arguments, and credentials must not cross this boundary.
- The gateway operational log and the local trusted audit log are separate. Gateway logs contain only authentication outcome, opaque principal/correlation IDs, contract/model versions, timing, status, rate-limit metadata, and aggregate usage, with a maximum seven-day retention. Content logging is prohibited. The local audit owns model-proposal, policy, approval, execution, cancellation, and outcome evidence without duplicating prompts, raw arguments, raw results, or raw errors.
- External-processing disclosure must be presented before the first transmission and remain visible in Settings.
- Disclosure requires explicit versioned acknowledgement before the first
  external transmission, including a synthetic-only Stage C transmission, and
  after material provider, retention, data-classification, or disclosure
  changes. Exact copy and acknowledgement storage require separate Stage C
  approval and evidence.

## Prohibited changes before an approved live-gateway increment

Do not add:

- Accessibility APIs.
- ScreenCaptureKit.
- Apple Events.
- Microphone permissions.
- Unrestricted shell execution.
- Generic filesystem access.
- Production model API credentials.
- Automatic external communication or destructive actions.

Any request to introduce one of these requires a new architecture decision and a later-phase threat-model review. Live AI provider networking, gateway authentication, identity federation, OAuth/OIDC, SAML, PKCE, token validation, platform-secure credentials, cloud deployment, `AgentProvider`, and a deployed gateway also remain prohibited until their own exact implementation plan is approved and all O-006 and D-061 evidence gates pass.

## Dependency review

Before adding a production dependency:

1. Explain the exact capability it provides.
2. Confirm the standard library or an existing dependency cannot satisfy the need.
3. Review maintenance, licensing, native build impact, and transitive dependencies.
4. Record the decision in `DECISIONS.md`.
5. Pin the version and update lockfiles.
6. Run the full relevant verification suite.

Apply the dependency and supply-chain sections of `SECURITY_CHECKLIST.md` and
`RELEASE_CHECKLIST.md` when those scopes apply.

## Data handling

- Keep local data local unless the user explicitly sends it.
- Minimize data before passing it across IPC or network boundaries.
- Redact content from logs and audit details unless the exact field is required for an approved action.
- Use opaque references rather than arbitrary paths where possible.
- Reject symlink escapes and unsupported executable content when file tooling is implemented.
- D-085 memory is process-local to one bounded orchestrator. Approved shared,
  private, task-temporary, and proposed-shared content has exact ownership,
  quotas, versioned review, deletion, terminal cleanup, disable, and drop
  behavior; it never enters SQLite or another workflow.
- The approved-document reader accepts only exact registered lowercase `.txt`
  or `.md` nonempty UTF-8 files within the documented bounds. It rejects
  traversal, noncanonical members, symlinks, hard-link aliases, unsupported or
  changed targets, replay, revocation, and cross-workflow references.
- On supported Unix targets, registered, opened-handle, and final-path identity
  are compared before and after the bounded read. The pure-standard-library open
  sequence retains a narrow TOCTOU residual advisory. Non-Unix targets report
  this boundary unavailable rather than weakening it.
- Raw document text and explicitly selected approved-shared context are labeled
  untrusted, bounded before runtime request construction, and excluded from
  Debug, errors, orchestration events, governance audit, and automatic memory.
- D-085 adds no IPC, file picker, provider transmission, unrestricted file tool,
  persistence, vector search, background indexing, document writing, or device
  effect. Any later consumer requires a separate approved plan and privacy
  evidence.
- D-086 transports specialist data only through validated structured results;
  it never reads sibling memory. Research and Knowledge retain access only to
  their own live agent-private and task-temporary namespaces, and terminal
  cleanup removes task-temporary records.
- A reusable Knowledge value remains `PendingReview`. It is not inserted into
  `MemoryStore`, assigned a shared-proposal identity, approved, selected,
  persisted, or treated as fact by synthesis.
- D-086 fixture labels/evidence, objective text, findings, summaries, proposals,
  runtime output, arbitrary references, paths, URLs, and reasoning remain absent
  from Debug, errors, workflow events, workflow audit, logs, SQLite, IPC, and
  automatic memory. Its fixture-only disclosure does not prove live retrieval
  or factual correctness.
- D-086 adds no provider, model, credential, network, process, filesystem read,
  tool, executor, persistence, IPC, UI, dependency, capability, permission,
  Hermes/OpenClaw adapter, or `AgentRuntime`/`NativeAgentRuntime` widening.
- D-087 fixture content, free-form output, patch descriptions, criterion text,
  evidence descriptions, and fake-secret sentinels remain absent from Debug,
  errors, workflow events/audit, logs, SQLite, IPC, and automatic memory.
- D-087 adds no live repository/filesystem/process/Git/package/network access,
  registered tool, executor, approval request, mutation, persistence, IPC/UI,
  dependency, capability, permission, provider, external runtime, or
  `AgentRuntime`/`NativeAgentRuntime` widening. Native remains sole/default.
- D-088 fixture content, sanitized log text, assessment/change/diagnostic plan,
  criterion/evidence descriptions, and synthetic sentinels remain absent from
  Debug, errors, events/audit, logs, SQLite, IPC, and automatic memory.
- D-088 adds no Terraform/platform/OS command, live inventory or diagnostic,
  credential lookup/use, tool, executor, approval request/dispatch, provider,
  IPC/UI, dependency, permission, persistence, external runtime, or effect and
  does not widen `AgentRuntime` or `NativeAgentRuntime`.

## GitHub automation boundary

- Pull-request code and dependency updates are untrusted. GitHub workflows run
  with `contents: read`, receive no repository secret context, do not use
  `pull_request_target`, and disable persisted checkout credentials.
- Active persistent runners receive only reviewed branch pushes, scheduled
  audit, and explicit dispatch. They never subscribe to `pull_request` or
  `pull_request_target`; fork and dependency-bot changes must be reproduced on
  a maintainer-controlled allowlisted branch before execution.
- Linux or target-Mac Rust verification does not prove native macOS menus,
  windows, dialogs, icons, permissions, signing, notarization, or installer
  behavior. Those checks remain target-Mac evidence.
- Every external action reference is pinned to an immutable commit digest.
  Dependabot proposes action updates for review; mutable action tags are not a
  trust decision.
- Workflows may inspect, compile, test, and retrieve public advisory data. They
  must not commit, push, merge, publish, deploy, sign, notarize, or begin another
  increment.
- The CI dependency-audit job runs the pinned JavaScript audit and Cargo audit
  for dependency, security-sensitive, workflow, scheduled, and manually
  dispatched validation. The
  Cargo result fails on any finding outside D-025's exact two-vulnerability
  `quick-xml 0.39.4` baseline and D-046's exact 18-warning lockfile baseline.
  Accepted findings remain unresolved and visible; the gate does not declare
  them fixed or generally safe.
- The tracked secret-pattern scan is defense in depth, not proof that a
  repository or artifact contains no secret. Release review still requires a
  complete secret and artifact assessment.
- CODEOWNERS, issue labels, milestones, badges, and workflow success are
  repository coordination evidence, not authorization, branch-protection proof,
  security approval, or release evidence.
- Security design-review issues must contain sanitized material only.
  Vulnerabilities and incidents follow the private reporting policy.

The authoritative runner labels, host prerequisites, trust policy, maintenance,
incident response, and rollback are in
`docs/github/SELF_HOSTED_RUNNER.md`.

## Repository hook boundary

- Repository-local hooks execute code with the developer's Codex session permissions and require review and trust through `/hooks` before use.
- `.codex/hooks/common.py` provides bounded repository, path, JSON, conflict,
  and suspicious-path validation to the two hook scripts.
- `.codex/hooks/post_increment_gate.py` may read repository metadata, changed
  files, its bounded structured report, and ignored gate state.
- `.codex/hooks/session_end_gate.py` may read only fixed Git status evidence and
  emit a structured staged, unstaged, untracked, and conflicted-path inventory.
  It is read-only and exits nonzero when conflicts exist.
- The hook scripts must not parse the unstable transcript, inspect model or
  personal content, access the network, execute arbitrary report content, or
  modify product source.
- Hook input is untrusted JSON. Validate event type, booleans, repository root, bounded size, report schema, and every path before use. Reject absolute paths, traversal, symlink escapes, merge conflicts, stale fingerprints, and suspicious changed paths. The workspace fingerprint covers only paths that exist in the current snapshot; reviewed deletions remain mandatory report-inventory entries but contribute no content before or after commit. Removing a path that existed at finalization must invalidate the marker.
- The script may invoke only fixed Git inspection commands. It must not use report content to construct shell commands.
- `.codex/state/post_increment_gate.json` contains no secrets and is ignored. Its completion marker is not approval, authorization, trusted audit evidence, or proof that commands ran; the report and actual command output remain the evidence.
- `stop_hook_active` must suppress a repeated continuation request. This loop guard does not waive the mandatory completion criteria.
- In an emergency, disable the hook through `/hooks` or start a session with `codex --disable hooks`. Record the bypass and rerun the complete gate before marking an increment complete. Do not routinely bypass hook trust.

## Reporting a security concern

Do not place secrets, personal files, tokens, or exploitable details in a public issue. Record a sanitized summary in the project handoff and notify the repository owner through a private channel.

A security fix is not complete until regression tests and the relevant threat-model documentation are updated.

Production release additionally requires every applicable item in
`RELEASE_CHECKLIST.md`, including signing, notarization, installer, secret,
artifact, and rollback evidence.
