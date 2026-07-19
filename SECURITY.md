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
cancellation, and a turn-bound in-memory approval-audit adapter. A successful
terminal native or run-termination resolution cannot leave the initial turn
without one typed audit receipt. It has no live provider transport, runtime
coordinator, dispatcher, executor, product memory, or durable audit. Current
resolutions, records, and receipts are volatile and non-authorizing and must not
be mistaken for an end-to-end security path.

## Non-negotiable invariants

- The WebView cannot execute a generic local action.
- Unknown tools and invalid arguments fail closed.
- Tool schemas use strict validation and reject additional properties.
- Class 3 personal-data modifications always require an exact trusted preview and explicit approval.
- Class 4 actions are not registered in the MVP.
- Class 5 behavior is prohibited.
- Approval is bound to canonical arguments, tool identity, expiry, and one-time consumption.
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
- A future desktop gateway access token must be short-lived with a maximum 15-minute lifetime, audience-bound, read only by trusted Rust through the platform secret-store abstraction, and stored in process memory. Initial scopes are `openid`, `email`, and one exact delegated gateway scope. `profile`, Microsoft Graph, directory, group, mail, calendar, file, contact, and `offline_access` scopes are excluded. Persistent sessions and `offline_access` require a separate decision; any later approved refresh or session credential must use platform-secure credential storage. The WebView must never receive any credential.
- The Rust core may send only a closed, size-bounded request contract to one configured HTTPS gateway origin. The WebView must not choose the gateway URL, identity provider, AI model provider, model, provider parameters, tool schemas, or authorization headers.
- The gateway authenticates and authorizes the desktop principal, applies request/rate/model/tool-set limits, selects an approved AI model provider under trusted policy, injects only that server-held provider credential, and normalizes upstream events. It cannot approve or execute local tools.
- The gateway selects exact server-side tool-set versions. It must not forward arbitrary caller-supplied OpenAI tools, hosted tools, MCP servers, shell tools, or provider parameters.
- OpenAI function definitions must use strict mode, require every property, reject additional properties, and disable parallel tool calls for the first production loop. Strict provider generation is defense in depth, not local authorization.
- The gateway must validate recognized upstream event types and required fields, enforce sequence and size limits, and reject unknown event types. Additive fields on a recognized OpenAI event may be ignored because the upstream API documents them as backward-compatible additions; no unrecognized field may cross the normalized protocol.
- The gateway-to-Rust protocol must be versioned, closed, size-bounded, sequence-checked, and reject unknown fields. Function calls remain untrusted until trusted Rust independently validates the call ID, registered tool name, tool-contract version, argument JSON, exact per-tool schema, risk, permission, policy, and approval state.
- Use foreground Responses streaming with `store: false` and `background: false`. Cancellation transitions the local validator to a terminal cancelled state, aborts the desktop-to-gateway request and gateway-to-provider stream, discards late events, and is idempotent. Do not expect a cancellation event over the aborted stream or claim provider-side cancellation completion.
- `store: false` minimizes Responses application-state storage but does not eliminate provider abuse-monitoring retention. D-061 requires verified provider-approved ZDR independently for each AI provider's exact production organization, project, endpoint, model, and region configuration before real user content. Until then, only synthetic data may be considered under a separately approved future transport test.
- After ZDR verification, the initial real-user data class is explicitly submitted, non-sensitive text only. Credentials, attachments, regulated data, financial or healthcare data, and sensitive personal data remain prohibited.
- Provider and gateway failures cross into Rust only as closed redacted error codes, retryability, bounded retry delay, and opaque correlation IDs. Raw response bodies, headers, stack traces, prompts, model output, function arguments, and credentials must not cross this boundary.
- The gateway operational log and the local trusted audit log are separate. Gateway logs contain only authentication outcome, opaque principal/correlation IDs, contract/model versions, timing, status, rate-limit metadata, and aggregate usage, with a maximum seven-day retention. Content logging is prohibited. The local audit owns model-proposal, policy, approval, execution, cancellation, and outcome evidence without duplicating prompts, raw arguments, raw results, or raw errors.
- External-processing disclosure must be presented before the first transmission and remain visible in Settings.

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
