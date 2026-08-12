# Security policy and development guardrails

Status: Authoritative security policy

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
attribution audit. A successful terminal native or run-termination resolution
cannot leave the initial turn without one typed legacy audit receipt. Agent
governance reserves one audit slot before downstream mutation and records
execution only as `NotAttempted`. There is no live provider transport,
dispatcher, executor, durable or user-facing product memory, platform adapter,
or durable audit. Current resolutions, memory, records, references, and receipts
are volatile and non-authorizing and must not be mistaken for an end-to-end
security path.

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
