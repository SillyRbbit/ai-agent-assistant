# Next steps

Last updated: 2026-08-11

This file is the ordered implementation queue. Work only on the first item marked **Ready**. A verification-pending increment must close before later feature work begins.

## Completed increments

- Increment 2A — core interfaces and deterministic mocks: **Verified complete**.
- Increment 2B-0 — SQLite storage decision: **Complete**.
- Increment 2B-1 — SQLite dependency and migration skeleton: **Verified complete**.
- Increment 2B-1A — Rust 1.90 compatibility repair: **Verified complete**.
- Increment 2C — storage startup integration: **Verified complete**.
- Increment 2D — macOS menu-bar and window lifecycle: **Verified complete**.
- Increment 2E — React application shell: **Verified complete**.
- Increment 2F — mocked assistant interaction shell: **Verified complete**.
- Increment 2G — integration hardening: **Verified complete**.
- Increment 3A — in-memory conversation sessions: **Verified complete**.
- Increment 3B — mock context provenance: **Verified complete**.
- Increment 3C — simulated tool result: **Verified complete**.
- Phase 3 completion gap analysis: **Complete**.
- Increment 3D — bounded mock-loop completion: **Verified complete**.
- Phase 4 gateway and Responses security-boundary planning: **Complete**.
- Increment 4A — deterministic gateway protocol contract: **Verified complete**.
- Increment 4B — exact local tool-schema validation: **Verified complete**.
- Increment 4C — trusted policy-input binding: **Verified complete**.
- Increment 4D — exact approval binding: **Verified complete**.
- Increment 4E — trusted approval-decision source: **Verified complete**.
- Increment 4F - Cortexa product display rename: **Verified complete by project-owner direction**.
- Repository Workflow Increment 4G - automated post-increment gate: **Verified complete**.
- Increment 4H - typed approval-audit adapter: **Verified complete**.
- Increment 4I - remove generic audit scaffold: **Verified complete**.
- Repository Workflow Increment 4J - deletion-stable post-increment fingerprint: **Verified complete**.
- Increment 4K - remove legacy provider scaffold: **Verified complete**.
- Increment 4L - remove legacy memory scaffold: **Verified complete**.
- Increment 4M - remove legacy platform scaffold: **Verified complete**.
- Increment 4N - bounded initial gateway request: **Verified complete**.
- Increment 4O - bound initial gateway turn: **Verified complete**.
- Increment 4P - schema-bound initial gateway events: **Verified complete**.
- Increment 4Q - terminally release initial function call: **Verified complete**.
- Increment 4R - bind terminal initial function call to policy: **Verified complete**.
- Increment 4S - bind terminal initial approval presentation: **Verified complete**.
- Increment 4T - bind terminal initial approval resolution: **Verified complete**.
- Increment 4U - bind initial approval run-termination: **Verified complete; published and merged at `61525bf`**.
- Meta Increment 1 - branding and identity foundation: **Verified complete**.
- Meta Increment 2 - engineering operating system: **Verified complete**.
- Meta Increment 3 - Codex automation and post-increment quality gates:
  **Verified complete; squash-merged at `ad9042c`**.
- Meta Increment 5 - repository health and GitHub hygiene:
  **Verified complete; published and squash-merged at `6b149fa`**.
- Meta Increment 6 - Product Readiness Audit:
  **Documentation-only audit complete; result NOT READY (57/100)**.
- Meta Increment 7 - verified application icon rollout:
  **Verified complete with advisories; squash-merged through PR #19 at
  `96ba6ae`**.
- Remediation ARB-022 - project-memory reconciliation:
  **Verified complete and squash-merged through PR #22 at `7c79e65`**.
- Repository workflow - trusted self-hosted runner routing:
  **Verified complete with advisories and squash-merged through PR #24 at
  `eaf6c9f`**.
- Repository workflow - risk-based GitHub Actions validation:
  **Verified complete with advisories; published through PR #30 and
  squash-merged at `1780d7f`; its post-publication project-memory reconciliation
  was published through PR #31 and squash-merged at `74a8d2c`**.
- Meta Increment 8 - Prompt Library Reorganization:
  **Verified complete; published through PR #25 and squash-merged at
  `d26b5e1`**.
- Increment 4V / ARB-001 - terminal approval audit binding:
  **Verified complete; published through PR #23 and squash-merged at
  `6e6f91d`**.
- High-severity advisory disposition:
  **Verified complete with advisories; published through PR #33 and
  squash-merged at `7bf1a5c`; no immediate code remediation authorized**.
- O-006/O-007 staged gateway identity and retention decisions:
  **Provider-boundary documentation amendment verified complete with
  advisories; published through PR #35 and squash-merged at `853da62`; no
  product implementation authorized**.
- O-006 Phase 1 identity decision:
  **Microsoft personal identity selected as the sole Phase 1 provider under
  D-062; documentation-only decision record verified complete with advisories,
  published through PR #37, and squash-merged at `c458f27`; no implementation
  authorized and no publication action remains**.
- O-006 Phase 1 AI-provider decision:
  **D-063 selects Azure OpenAI as the synthetic-evaluation candidate;
  documentation-only record verified complete with advisories, published
  through PR #39, and squash-merged at `4abd49d`; no implementation authorized
  and no publication action remains**.
- ARB-002A - gateway threat model and closed configuration:
  **Verified complete with advisories under D-064; published through PR #41 and
  squash-merged at `36ce9ab`; no provisioning, transport, real-content
  activation, or runtime implementation authorized**.
- Repository project direction and runtime boundaries:
  **Verified complete with advisories under D-078; present personal scope,
  native preservation, conceptual adapter boundaries, and the living ExecPlan
  convention are documented; at that checkpoint no product or external-runtime
  implementation was authorized or Ready**.
- Isolated Hermes transport spike:
  **Verified complete with advisories; raw TUI-gateway stdio is NO-GO as a
  supported production contract for Hermes Agent `0.20.0` / `v2026.8.3`; seven
  deterministic fixture tests pass, the real-Hermes probe was not run, and no
  production runtime or native behavior changed**.
- Hermes runtime architecture decisions:
  **Documentation-only decision increment complete with advisories in the
  uncommitted working tree; D-079 accepts the native-first runtime architecture,
  D-080 conditionally selects a contained `hermes serve` WebSocket spike, and
  no runtime source, dependency, Hermes execution, or behavior changed**.

## Queue status

### Next implementation: Native agent runtime boundary

**Status:** **Ready**; sole next implementation plan, not active. The
[`2026-08-11-native-agent-runtime-boundary.md`](docs/plans/2026-08-11-native-agent-runtime-boundary.md)
plan implements only the small application-owned `AgentRuntime` contract,
`NativeAgentRuntime` composition over the verified native turn, and a private
deterministic no-I/O `MockAgentRuntime` contract fixture. Hermes, transport,
provider, process, dependency, UI, and runtime selection remain out of scope.
The plan may begin only after this uncommitted decision increment is separately
reviewed and published to a clean synchronized baseline and the owner gives an
exact implementation prompt.

### Blocked later spike: Hermes serve WebSocket containment

**Status:** **Blocked**. The
[`2026-08-11-hermes-serve-websocket-spike.md`](docs/plans/2026-08-11-hermes-serve-websocket-spike.md)
plan is owner-approved for later execution only after the native boundary has a
valid completion marker and a fresh security/readiness review proves the exact
read-only distribution, installed extras, no-install/no-update controls,
dotenv/managed-secret denial, endpoint-level network/Unix-socket containment,
and cleanup of detached descendants. It is not current work and authorizes no
Hermes execution now.

### Draft later adapter: HermesAgentRuntime

**Status:** **Draft; Blocked**. The
[`2026-08-11-hermes-agent-runtime-adapter.md`](docs/plans/2026-08-11-hermes-agent-runtime-adapter.md)
plan remains blocked until the native boundary is verified, the contained spike
passes, exact dependencies and scope are approved, and a fresh readiness review
passes. Native remains default. No adapter, selector, process, dependency, or
tool capability exists.

### Completed documentation-only increment: Hermes runtime architecture decisions

**Status:** Complete with advisories in the uncommitted working tree; no
implementation authorized by this completion. The
[`2026-08-11-hermes-runtime-architecture-decisions.md`](docs/plans/2026-08-11-hermes-runtime-architecture-decisions.md)
plan records D-079/D-080, preserves the raw-stdio NO-GO, readies only the native
boundary plan, and independently gates the contained WebSocket spike and Draft
adapter. It uses Hermes Agent `0.20.0`, tag `v2026.8.3`, commit
`3c27eb6234bf91b8ceee9e9071591b31e9b148cb` as the canonical source. No
production source, test, dependency, process, socket, credential, provider, UI,
or behavior changed.

### Completed documentation-only increment: Hermes ADR transport revision

**Status:** Verified complete with advisories; no runtime implementation
authorized. The
[`2026-08-11-hermes-adr-transport-revision.md`](docs/plans/2026-08-11-hermes-adr-transport-revision.md)
plan revised the Proposed ADR to retain the raw TUI-gateway stdio NO-GO and, at
that increment's closeout, compared native-only with unselected public Hermes
surfaces. D-079/D-080 now supersede that former decision status. The increment
did not execute Hermes, add dependencies, or create runtime source.

### Completed isolated spike: Hermes raw TUI-gateway stdio

**Status:** Verified complete with advisories; no runtime implementation
authorized. The
[`2026-08-11-hermes-transport-spike.md`](docs/plans/2026-08-11-hermes-transport-spike.md)
plan and
[`HERMES_TRANSPORT_SPIKE.md`](docs/spikes/HERMES_TRANSPORT_SPIKE.md) establish
that the selected raw-stdio mechanism lacks a supported public launcher,
initial version/capability negotiation, and a gateway-shutdown RPC at the pinned
release. The fixture proves host mechanics only, not Hermes conformance,
containment, descendant cleanup, packaging, or target-platform readiness. The
multi-runtime ADR was Proposed and required a separately selected revision at
that spike checkpoint. D-079/D-080 now preserve this evidence while accepting
the native-first architecture and only a conditional contained `hermes serve`
spike; ACP remains deferred.

### Completed documentation-only project direction and runtime boundaries

**Status:** Verified complete with advisories; no product implementation
authorized.
[`2026-08-11-project-direction-runtime-boundaries.md`](docs/plans/2026-08-11-project-direction-runtime-boundaries.md)
records present private owner-only scope, preserves verified native Rust and
deterministic mock assets, and limits `AgentRuntime`, `NativeAgentRuntime`, and
optional experimental `HermesAgentRuntime` to conceptual future direction.
OpenClaw remains only a possible later evaluation. D-078 does not cancel
accepted future targets or make Hermes, provider, transport, execution,
multi-agent, deployment, or product work Ready. Any later implementation
requires a new exact owner-selected plan and gate.

### Completed implementation: Cloudflare demo fake Keychain proof

**Status:** Verified complete with advisories; no real ingestion authorized.
[`cloudflare-demo-fake-keychain-proof.md`](docs/plans/cloudflare-demo-fake-keychain-proof.md)
adds one macOS-only, status-only trusted Rust probe for exactly two fixed
generic-password labels. Automated checks use deterministic fake sources and
never touch Keychain. Owner-operated target-Mac evidence passed missing,
cancelled/denied-as-cancelled, available, and cleanup outcomes. Repeated
authorization prompts did not establish stable unsigned-executable access.
Both fake items were removed. A real credential, runtime consumer, Worker, or
Cloudflare action remains Blocked and requires a separate exact plan and owner
approval.

### Completed documentation-only planning: Cloudflare real-credential readiness

**Status:** Complete; real ingestion remains Blocked.
[`cloudflare-demo-real-credential-readiness-plan.md`](docs/plans/cloudflare-demo-real-credential-readiness-plan.md)
defines the stable identity/ACL, secret-memory, direct owner transfer,
rotation/revocation/rollback, dependency reassessment, and private target-Mac
evidence required before a future real demo-token proposal. It creates no code,
credential, Keychain action, Cloudflare resource, provider request, traffic, or
runtime behavior. A later exact design or implementation increment requires
separate owner approval.

### Completed documentation-only planning: Cloudflare macOS identity and secret-memory boundary

**Status:** Complete; selection and implementation remain Blocked.
[`cloudflare-demo-macos-identity-secret-boundary-plan.md`](docs/plans/cloudflare-demo-macos-identity-secret-boundary-plan.md)
defines the selection criteria, bounded secret-memory controls, lifecycle, and
private target-Mac evidence for a future real demo-token proposal. It chooses
neither a signed identity nor narrow ACL and creates no signing, Keychain,
credential, Cloudflare, provider, traffic, or runtime capability.

### Completed documentation-only decision: Cloudflare signed macOS identity

**Status:** Complete; implementation remains Blocked.
[`cloudflare-demo-signed-identity-decision.md`](docs/plans/cloudflare-demo-signed-identity-decision.md)
selects stable signed macOS application identity for the future demo credential
boundary and defines later signing provenance, secret-memory, lifecycle, and
private-evidence gates. It creates no signing asset, Keychain action,
credential, Cloudflare resource, provider request, traffic, or runtime behavior.

### Completed documentation-only planning: signed-identity and secret-memory proof

**Status:** Complete; implementation remains Blocked.
[`cloudflare-demo-signed-identity-secret-memory-implementation-plan.md`](docs/plans/cloudflare-demo-signed-identity-secret-memory-implementation-plan.md)
limits a later fake-only implementation to the existing credential module,
integration test, and status-only example. It creates no code, dependency,
signing, Keychain, credential, Cloudflare, traffic, or runtime behavior.

### Completed documentation-only planning: Apple Developer signing-identity owner evidence

**Status:** Complete; private read-only review remains unperformed and every
later action remains Blocked.
[`apple-developer-signing-identity-owner-evidence-plan.md`](docs/plans/apple-developer-signing-identity-owner-evidence-plan.md)
defines the owner-only sanitized account-evidence procedure for membership,
signing-asset visibility, and apparent authority. It prohibits enrollment,
purchase, support requests, role changes, signing assets, downloads,
installation, Keychain actions, credentials, Cloudflare, provider, traffic,
deployment, and runtime behavior. Account eligibility cannot authorize a
future proof or real credential ingestion.

### Completed documentation-only decision: Apple Developer enrollment model

**Status:** Complete; enrollment and all signing actions remain Blocked.
D-074 defers Apple Developer Program enrollment now and conditionally recommends
individual enrollment only if a later owner-only proof is separately approved
while Cortexa remains personally owned. It requires a fresh organization review
before company ownership, seller identity, or shared certificate control is
needed. It creates no account access, purchase, agreement, signing asset,
Keychain item, credential, Cloudflare, provider, traffic, deployment, or
runtime authority.

### Completed documentation-only planning: Apple Developer individual enrollment execution

**Status:** Complete; enrollment remains Blocked pending separate owner approval.
[`apple-developer-individual-enrollment-execution-plan.md`](docs/plans/apple-developer-individual-enrollment-execution-plan.md)
defines the D-074 conditional individual-enrollment procedure, owner gates,
private evidence, stop conditions, and non-reversible commitment handling. It
authorizes no Apple account access, enrollment, purchase, agreement, signing
asset, Keychain item, credential, Cloudflare, provider, traffic, deployment, or
runtime behavior.

### Completed owner-operated reconciliation: Apple Developer individual enrollment

**Status:** Owner-attested complete; membership is active and all signing work
remains Blocked. The owner attested that the separately approved individual
Apple Developer Program enrollment completed and that no signing asset was
created. This reconciliation records no account, payment, membership identifier,
certificate, key, profile, entitlement, Keychain item, credential, Cloudflare,
provider, deployment, traffic, or runtime detail. Membership alone does not make
the fake-only signed proof or real credential ingestion Ready.

### Completed documentation-only decision and planning: Developer ID Application identity

**Status:** Complete; certificate creation and all signing work remain Blocked.
D-075 selects Developer ID Application only as the future certificate class for
the owner-selected stable macOS identity proof. The
[`macos-developer-id-identity-creation-plan.md`](docs/plans/macos-developer-id-identity-creation-plan.md)
defines the later owner-operated identity-creation, private-key, lifecycle,
private target-Mac evidence, and stop boundaries. It creates no Apple account
access, certificate, CSR, key, profile, App ID, entitlement, download, signing,
notarization, Keychain action, credential, Cloudflare, provider, deployment,
traffic, code, dependency, or runtime behavior.

### Safely stopped owner operation: Developer ID Application certificate creation

**Status:** Unavailable; no signing asset exists and all later work is Blocked.
The separately approved owner-operated attempt stopped after Certificate
Assistant reported that the specified item could not be found in the Keychain.
The owner confirmed no CSR file, certificate, or new named private key was
created. The cause is not determined. Do not retry, reset or delete Keychain
state, use Terminal or OpenSSL to create signing material, contact Apple
support, or select another certificate path under this increment. If the owner
chooses to continue, the smallest next task is a documentation-only remediation
plan defining exact read-only diagnostics, risks, rollback, stop conditions,
and sanitized private target-Mac evidence. It must not access Apple Developer or
create, modify, export, revoke, or use any signing or credential asset.

### Completed documentation-only planning: Certificate Assistant CSR remediation

**Status:** Complete; diagnostic execution remains Blocked.
[`macos-certificate-assistant-csr-remediation-plan.md`](docs/plans/macos-certificate-assistant-csr-remediation-plan.md)
defines the exact future owner-operated, local-only, read-only diagnostic
boundary for TS-017. It permits no execution now and prohibits Apple access,
CSR retry, Keychain state changes, signing material, Apple Support contact,
credentials, Cloudflare, provider, deployment, traffic, code, dependency, and
runtime behavior. A separate explicit owner approval is required before the
listed diagnostic observations could run. Inconclusive evidence preserves the
`not determined` cause and authorizes no recovery path.

### Completed owner operation: Certificate Assistant read-only diagnostics

**Status:** Complete with advisories; cause remains undetermined and all later
work is Blocked. The owner performed each of the plan's three local read-only
observations once and reported only sanitized categories: user and default
Keychain configuration were observed, valid code-signing identities were
`zero`, no authorization prompt appeared, and no state change was observed.
This evidence does not authorize repetition, remediation, CSR retry,
certificate creation, signing, credential handling, Cloudflare, provider,
deployment, traffic, code, dependency, or runtime work. A future action requires
a separately approved plan based on evidence beyond these inconclusive checks.

### Completed documentation-only decision: Defer TS-017 signed identity path

**Status:** Complete; no operational follow-up is Ready. D-076 records the
owner's decision to defer the signed macOS identity path. It preserves the
no-asset baseline and `not determined` cause. Apple Support assistance and an
alternate CSR workflow are considered future options only; neither is approved
or planned for execution. Each would require a separate owner-approved plan
that proves target-Mac-only, owner-controlled, non-exported private-key and
privacy boundaries. No diagnostic repetition, Apple, Keychain, signing,
credential, Cloudflare, provider, deployment, traffic, code, dependency, or
runtime action is authorized.

### Completed documentation-only planning: Apple Support TS-017 assistance

**Status:** Complete; support contact remains Blocked.
[`apple-support-ts-017-assistance-plan.md`](docs/plans/apple-support-ts-017-assistance-plan.md)
defines a possible future owner-only Apple Support contact using minimum
sanitized disclosure, no screen sharing, stop conditions, and no-state-change
rollback. It preserves D-076's deferral and does not authorize contact,
diagnostic repetition, remediation, Apple access, Keychain action, signing,
credential, Cloudflare, provider, deployment, traffic, code, dependency, or
runtime behavior.

### Safely stopped owner operation: Apple Support contact and filesystem key

**Status:** Stopped with an unresolved signing-material disposition blocker.
No Apple Support or Apple Developer access occurred. One CSR file and one
filesystem private-key file exist; neither was uploaded, used, copied,
exported, or backed up, and no certificate exists. Encryption and permissions
are not determined. The material does not satisfy D-072. The smallest possible
future task is a documentation-only containment and disposition plan; no
inspection, movement, deletion, upload, certificate, signing, credential,
Cloudflare, provider, deployment, traffic, code, dependency, or runtime action
is Ready.

### Completed documentation-only planning: Filesystem signing-material disposition

**Status:** Complete; operational disposition remains Blocked.
[`filesystem-signing-material-disposition-plan.md`](docs/plans/filesystem-signing-material-disposition-plan.md)
selects future abandonment and paired deletion of the unuploaded CSR/private-key
pair. It requires separate approval, exact private target resolution, stop on
ambiguity, no content inspection, and no secure-erasure claim. No identification,
deletion, signing, credential, cloud, product, or runtime action is authorized.

### Completed owner operation: Paired filesystem signing-material deletion

**Status:** Complete with residual-media advisory; no follow-on increment is
Ready. Sanitized owner evidence reports exact identification of the intended
CSR/private-key pair, no additional material, completed deletion of both files,
no remaining copy, no upload or use, and no certificate. This closes the active
filesystem custody blocker but does not prove cryptographic erasure from
APFS/SSD remnants or snapshots, satisfy D-072, or supersede D-076. Any future
diagnostic, Apple Support, alternate CSR, signing, credential, Cloudflare,
provider, deployment, traffic, code, dependency, or runtime work requires a
separately approved exact increment.

### Completed documentation-only decision: Conditional Apple Support consideration

**Status:** Complete; Apple Support contact remains Blocked. D-077 conditionally
reopens consideration of one future owner-operated TS-017 contact under the
existing assistance plan while retaining D-076's signed-identity deferral. A
separate owner-approved operational increment is mandatory before contact. The
plan's minimum sanitized disclosure, no screen sharing/uploads/device access,
no-execution rule, stop conditions, and closed evidence remain binding. No Apple
Developer or Apple Support access, diagnostic, CSR, key, Keychain, certificate,
signing, credential, Cloudflare, provider, deployment, traffic, code,
dependency, or runtime action is authorized.

### Completed owner operation: D-077 Apple Support contact not attempted

**Status:** Complete with no external contact or state change. Sanitized owner
evidence records `contact attempted: no`, `guidance: none`, `state changed: not
observed`, and `cause: not determined`. The operational approval is closed and
does not carry forward. D-077 still allows only future consideration; another
contact would require a fresh explicit owner approval under the existing plan.
No Apple, diagnostic, signing, credential, Cloudflare, provider, deployment,
traffic, code, dependency, or runtime increment is Ready.

### Completed documentation-only planning: Cloudflare demo local security boundary

**Status:** Complete; no implementation authorized.
[`cloudflare-demo-local-security-boundary.md`](docs/plans/cloudflare-demo-local-security-boundary.md)
defines a fake-credential-first Keychain proof and future local deny-only Worker
boundary. It adds no code, dependency, credential, Keychain item, Worker,
Access application, policy, route, DNS change, secret, provider request, or
traffic. Separate exact implementation plans remain required, and neither is
Ready.

### Completed documentation-only reconciliation: Cloudflare Zero Trust onboarding

**Status:** Complete owner-attested current-state reconciliation. The Free-plan
Zero Trust organization exists with Cloudflare's default identity provider
restricted to account members. No Access application, policy, service token,
Worker, route, DNS change, device enrollment, secret, provider request, or
traffic exists. This control-plane onboarding does not make an operational or
product increment Ready.

### Completed documentation-only planning: Cloudflare Access and Worker no-traffic deployment

**Status:** Complete; it authorized no external action by that planning
increment.
[`cloudflare-access-worker-no-traffic-deployment.md`](docs/plans/cloudflare-access-worker-no-traffic-deployment.md)
defines the future one-application, one-token, disabled-Worker boundary and
required rollback/manual evidence. It creates no token, Keychain item, Access
application, Worker, route, DNS record, secret, deployment, provider request,
or traffic. A separate project-owner approval is required before an operational
increment can begin.

### Completed documentation-only task: Cloudflare Access service-token demo exception

**Status:** Complete. D-068 permits one 30-day-maximum, Keychain-held Cloudflare
Access service token only for the owner-only fake-data demo. It authorizes
documentation only; token creation, deployment, secrets, traffic, and runtime
remain blocked.

### Other runtime remediation remains Blocked

**Status:** The exact 19-path documentation-only ARB-002A scope in
`docs/plans/arb-002a-gateway-threat-model-and-configuration.md` is verified
complete with advisories and published through PR #41 at `36ce9ab`. D-064 separates
pre-implementation design, no-traffic provisioning, synthetic-only transport,
and real-content activation so operational proof is never fabricated and no
stage automatically starts the next.

Repository Governance - Codex instruction hierarchy is verified complete with
`PASS WITH ADVISORIES`. It established D-065 without beginning Stage B, Stage
C, Stage D, ARB-002 runtime work, or altering the approved product queue.

Stage C is specifically blocked until Stage B proves an enforceable maximum
15-minute Microsoft personal gateway token and the manifest-based
`127.0.0.1` ephemeral callback is verified on the target Mac. Microsoft's
documented defaults are not accepted as evidence, and no fallback is implied.

D-059 classifies all canonical High findings without lowering
severity or representing deferred work as resolved. ARB-001 is resolved.
ARB-002 remains High and unresolved. D-062 selects Microsoft personal identity
for Phase 1. D-066 supersedes D-063's unpublished Azure synthetic-demo
direction with OpenAI only for a future synthetic demo; exact identity,
OpenAI data-control, D-061, disclosure, and threat-model evidence remain
unresolved. D-060 separates pluggable
identity, Azure-first portable hosting, and future trusted AI-provider
selection; D-061 accepts O-007's product policy, but provider-specific ZDR
evidence remains mandatory. ARB-003, ARB-004, ARB-005,
and ARB-008 are blocked on future product capabilities. ARB-006 and ARB-007 are
deferred and non-blocking only until their explicit legal and release triggers.
ARB-044 is superseded.

The D-058 implementation and its post-publication reconciliation are closed;
no D-058 publication task remains in the queue.

The High-severity disposition is published and closed; no PR #33 publication
task remains in the queue.

The O-006/O-007 provider-boundary amendment is verified complete with
advisories, published through PR #35, and squash-merged at `853da62`. Its exact
18-path scope preserves the original reports unchanged, and no publication
action remains. It does not make ARB-002 Ready.

No other runtime remediation or product increment is Ready; D-079's narrow
native boundary plan above is the sole exception. Do not begin ARB-002B or
another ARB-002 implementation automatically. Do not add a provider
client, `AgentProvider`, gateway origin, cloud deployment, identity integration,
credential, Keychain adapter, or external content path before D-062's exact
evidence passes, D-066's exact OpenAI data-control and D-061 evidence pass, the
required disclosure exists, and a separate implementation
and threat model are approved. Container portability
does not authorize AWS, Google Cloud, active-active multicloud, failover, or a
three-cloud release.

Do not implement ARB-003, ARB-004, ARB-005, or ARB-008 during advisory
disposition work. Revisit ARB-006 before public distribution or external
contributions. Revisit ARB-007 before release-candidate or public-distribution
work. Until then, preserve the proprietary/all-rights-reserved posture, macOS
14+ Apple Silicon provisional baseline, and unsigned local-development
boundary without claiming Intel support or production release readiness.

Repository Workflow Increment 4G is **Verified complete**. It adds only the repository-local post-increment skill, deterministic Stop-hook validator, focused tests, report assets, and workflow documentation required to replace D-027's one-time exception. Its consolidated result is `PASS WITH ADVISORIES`; the advisory is the documented project-hook trust/bypass boundary.

Increment 4H typed approval-audit adapter is **Verified complete**. It adds one bounded transport-free in-memory Rust adapter that derives a closed redacted record from an exact terminal `ApprovalResolution`, revalidates the complete disposition/evidence matrix, and returns only a non-authorizing sequence receipt. It adds no durable persistence, runtime coordinator, dispatch, execution, IPC, UI, or live networking. Its consolidated result is `PASS WITH ADVISORIES`; the advisory is the explicit non-durable boundary and the absence of an approved next increment.

Repository Workflow Increment 4J is **Verified complete, published, and merged**. It corrects only the deterministic completion fingerprint for reviewed tracked deletions and adds positive and negative regression coverage. Its publication prerequisite for 4I reconstruction is satisfied.

Increment 4I remove generic audit scaffold is **Verified complete after reconstruction**. It deletes only the unused `audit::logger` and `audit::types` modules and removes their exports, preserving the typed `audit::approval` adapter unchanged. The corrected 4J gate remains intact, the stale-symbol scan has no matches, focused and complete checks pass, and no replacement abstraction, persistence, coordinator, dispatch, executor, IPC, UI, networking, dependency, migration, or permission was added.

Increment 4K remove legacy provider scaffold is **Verified complete**. It deletes
only `agent::provider` and `agent::types` and removes their two exports. The
verified normalized gateway protocol and exact function-call validator remain
unchanged; focused and complete checks pass, and no replacement provider,
transport, networking, credential, coordinator, dispatch, executor, persistence,
IPC, UI, dependency, capability, or permission was added.

Increment 4K is published and merged at `5415444`; its completion marker remains
valid for that committed content.

Increment 4L remove legacy memory scaffold is **Verified complete, published,
and merged at `ecd49be`**. It deletes only the three legacy memory files and their
single crate-root export. The verified storage module is unchanged; focused and
complete checks pass, and no replacement memory, persistence, migration,
encryption, Keychain, context selection, IPC, UI, dependency, capability, or
permission was added. Its consolidated result is `PASS WITH ADVISORIES`; the
advisory is the theoretical unsupported external consumer of the removed public
module.

Increment 4M remove legacy platform scaffold is **Verified complete, published,
and merged at `1f03d1e`**. It deletes only the three legacy platform files
and their single crate-root export. Typed app-info and the fixed Permission Center
remain unchanged; focused and complete checks pass, and no replacement adapter,
OS query, native framework, permission request, Keychain, LocalAuthentication,
IPC, UI, dependency, Tauri capability, entitlement, or permission was added. Its
consolidated result is `PASS WITH ADVISORIES`; advisories are the intentionally
deferred future platform design and theoretical unsupported external consumer of
the removed public module.

Increment 4N bounded initial gateway request is **Verified complete, published,
and merged at `d7c4b69`**. It adds one non-cloneable transport-free request
value with closed private serialization, fixed initial-turn and tool-set fields,
all existing conservative limits, shared opaque-ID validation, redacted debug and
errors, and final 64 KiB enforcement after escaping. Six focused tests, one
public-boundary integration test, Clippy, complete repository verification, npm
audit, exact-scope review, code review, security review, documentation sync, and
the mandatory gate pass. No transport, credential, provider, continuation,
runtime, IPC, persistence, policy, approval, audit, dispatch, execution,
dependency, capability, or permission path was added.

Increment 4O bound initial gateway turn is **Verified complete, published, and
merged at `87be00e`**. It adds one non-cloneable transport-free turn that owns
the closed request bytes and response validator, derives both correlation IDs and
the exact two local tool names/version from trusted construction, and exposes only
borrowed request bytes, status, frame acceptance, and local cancellation. Raw
initial-request construction is private; the lower-level validator remains public
for protocol fixtures. Six preserved request tests, six public contract tests,
Clippy, complete repository verification, npm audit, code/security review,
documentation sync, and the mandatory gate pass. No transport, credential,
continuation, coordinator, IPC, persistence, dispatch, execution, dependency,
capability, or permission path was added.

Increment 4P schema-bound initial gateway events is **Verified complete,
published, and merged at `8c1a2e0`**. `InitialGatewayTurn` owns one private exact
registry built from the same fixed schema catalog as response validation and
returns only closed `InitialGatewayEvent` values. Function events contain a
`SchemaValidatedFunctionCall`; local schema rejection returns a typed redacted
error, reports `Failed`, rejects late frames, and makes cancellation a no-op.
Six request, 18 protocol, six function-validation, nine tool, eight public
contract, and two policy-binding tests pass with Clippy, complete repository
verification, npm audit, exact-scope review, code/security review, documentation
sync, and the mandatory gate. No transport, credential, continuation, policy,
approval, audit persistence, runtime, IPC, execution, dependency, capability, or
permission path was added.

Increment 4Q terminally release initial function call is **Verified complete,
published, and merged at `8598612`**. `InitialGatewayTurn` privately retains a
schema-validated call after its non-terminal function frame returns `None`.
Accepted terminal completion releases the exact call once; gateway failure and
successful local cancellation discard it; transactional protocol errors retain
it for the correct terminal frame. Six request, 18 protocol, six
function-validation, nine tool, nine public contract, and two policy-binding
tests pass with Clippy, complete repository verification, npm audit, exact-scope
review, code/security review, documentation sync, and the mandatory gate. No
transport, credential, continuation, policy, approval, audit persistence,
runtime, IPC, execution, dependency, capability, or permission path was added.

## Published completion

### Increment 4U - bind initial approval run-termination

**Status:** Verified complete, published, and merged at `61525bf`; the `04u`
completion marker remained valid after commit, merge, and push.

**Goal:** Let the bound turn terminally deny and consume its exact pending
approval when a trusted future orchestrator reports run termination, without
accepting a caller-selected approval ID, choice, native result, or evidence.

**Exact source/test scope:**

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The turn now retains its exact private manager-assigned pending approval ID and
exposes one narrow idempotent cancellation operation that delegates to the
existing manager's `cancel_for_run_termination`. Successful resolution clears
turn ownership and returns the existing non-authorizing `ApprovalResolution`.
Successful native resolution also clears ownership; typed errors retain it.
Expiry keeps precedence at or after the deadline, and every late native outcome
remains rejected after cancellation.

Native dialog invocation or closure, proactive expiry or timers, source traits,
runtime coordination, active-run validation beyond the trusted cancellation
call, audit, persistence, dispatch, execution, continuation, transport,
authentication, credentials, Tauri, frontend, SQLite, dependencies,
capabilities, entitlements, and permissions are non-goals.

Focused request, approval, public contract, approval-binding, and approval-audit
tests, strict Clippy, complete `npm run verify`, npm audit, diff review, and the
mandatory `04u` gate pass. No manual gate applies because no native UI or
production caller is introduced. D-042 records the durable boundary. Roll back
the two source/test files and only the declared closeout documentation before
commit, or revert one bounded 4U commit afterward.

No 4U, Meta 2, Meta 3, Meta 5, Meta 6, or Meta 7 implementation task remains.
Increment 4V has since been approved, verified, and squash-merged through PR #23
at `6e6f91d`. The self-hosted workflow prerequisite and Meta Increment 8
publication are complete; do not modify or replace the verified scope without a
separate increment.

## Published product remediation

### Increment 4V - bind initial terminal approval audit

**Status:** Verified complete and squash-merged through PR #23 at `6e6f91d`
from reconstructed source commit `ec919e9`. Hosted CI, Documentation, and
Security passed, and the `04v` marker remains complete and valid.

**Goal:** Prevent a future initial-turn caller from receiving a successful
native or run-termination approval resolution unless the turn's private typed
in-memory audit adapter has validated and recorded that exact manager-owned
resolution first.

**Exact source/test scope:**

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

The verified branch gives the turn one private `InMemoryApprovalAuditAdapter`
and returns only a closed non-cloneable value containing the exact
`ApprovalResolution` and its non-authorizing `ApprovalAuditReceipt`. Both
successful terminal paths use one private manager-then-audit helper. A typed
audit failure returns no resolution and could not restore already-consumed
manager state.

Durable audit persistence, SQLite, native invocation or closure, proactive
expiry, timers, source traits, runtime coordination, active-run validation,
dispatch, execution, continuation, transport, authentication, credentials,
Tauri, frontend, dependencies, capabilities, entitlements, and permissions are
non-goals.

Focused request, audit, approval, public-contract, approval-binding, and
approval-audit tests, strict Clippy, complete `npm run verify`, npm audit, diff
review, the mandatory `04v` gate, and all three hosted workflows passed. No
manual gate applies. Rollback reverts only squash commit `6e6f91d`.

Its exact reconciled plan remains at
`docs/plans/04v-bind-initial-terminal-approval-audit.md`. Do not start ARB-002;
no later product or remediation increment is Ready.

Increment 4R bind terminal initial function call to policy is **Verified
complete, published, and merged at `5e58edb`**. Accepted terminal completion
consumes the exact pending schema-valid call through the fixed deterministic
engine and returns one retained `PolicyDecision`; failure and cancellation still
discard the pending call without evaluation. Both exact tool outcomes, retained
typed facts, redaction, lower-level contracts, Clippy, complete repository
verification, npm audit, scope review, documentation sync, and the mandatory gate
pass. No approval, audit, transport, runtime, dispatch, execution, dependency,
capability, or permission path was added.

### Increment 4S - bind terminal initial approval presentation

**Status:** Verified complete, published, and merged at `6d0bed4`.

The bound initial turn now consumes terminal `RequireApproval` through its
private fixed approval manager and returns one owned non-authorizing
`ApprovalPresentation`. `Allow` and `Deny` remain closed non-authorizing policy
events.

**Exact source/test scope:**

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/tests/gateway_request_contract.rs
```

Accepted terminal completion creates one manager-owned approval request and
issues one exact presentation for `create_local_task@1`. The information-only
tool remains a non-authorizing `PolicyEvaluated` event. No trusted interaction,
approval disposition, audit, run-liveness, dispatch, or execution authority was
added.

No approval-manager, native-source, audit, policy-rule, transport, credential,
runtime, Tauri, frontend, SQLite, dependency, capability, entitlement, or
permission work is included.

Focused and complete verification, dependency audit, exact-scope, code,
security, documentation, and mandatory gate reviews pass. D-040 records the
durable boundary. No manual check is required.

### Increment 4T - bind terminal initial approval resolution

**Status:** Verified complete, published, and merged at `244a1d8`.

The bound turn now returns one sealed trusted approval source outcome to the
exact private manager that issued its presentation and exposes only the exact
non-authorizing terminal resolution.

**Exact source/test scope:**

```text
src-tauri/src/agent/gateway_request.rs
src-tauri/src/approvals/decision_source.rs
```

On macOS, the turn consumes one existing sealed
`TrustedApprovalSourceOutcome`, delegates directly to its private manager, and
returns the existing exact `ApprovalResolution` or typed approval error. The
native source's production behavior remains unchanged; only its existing
synthetic result mapper gains crate-wide test visibility under `cfg(test)` so
focused unit tests do not open a dialog.

No native invocation, run cancellation, proactive expiry, audit, persistence,
active-run validation, dispatch, execution, transport, credential, Tauri,
frontend, SQLite, dependency, capability, entitlement, or permission work is
included.

Eight request, 17 approval, nine public gateway-request, two approval-binding,
and one approval-audit test pass. Strict Clippy, complete `npm run verify`, npm
audit, exact-scope, code, security, documentation, and mandatory gate reviews
pass with no manual gate. D-041 records same-manager ownership and the
non-authorizing resolution boundary.

Commit `244a1d8` is pushed on `codex/phase4-increment-4t`, fast-forward merged
into synchronized `main`, and retained a valid `04t` marker immediately before
4U planning edits.
