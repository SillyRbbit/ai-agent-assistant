# Changelog

All notable repository changes are documented here. Entries distinguish verified work from implementation awaiting target-platform checks.

## Unreleased

- Implemented D-083's bounded native task and orchestration foundation above
  the unchanged `AgentRuntime`: closed task identity/lineage/state/result
  types, trusted live execution contexts, exact Personal Assistant-to-Research
  delegation, direct Personal responses, attributed child outcomes, fresh
  Personal synthesis runs, fail-closed runtime-event routing, and child-first
  cancellation. Added a shared test-only deterministic runtime fixture and
  contracts for identity isolation, denial, bounds, failures, cancellation,
  redaction, and exact output attribution. The foundation is Rust-only and
  unwired; Native remains sole/default, the other seven roles remain Deferred,
  and no provider, tool, policy, memory, IPC, UI, dependency, Hermes, external
  I/O, or user-visible behavior changed.

- Implemented the inert application-owned nine-role agent catalog with closed
  typed IDs, bounded purposes, exact versioned embedded instruction sources,
  descriptive `Initial`/`Deferred` activation metadata, and a deterministic
  immutable registry. Added fail-closed validation, typed redacted errors, and
  deterministic definition/registry contracts. Personal Assistant and Research
  alone are marked `Initial`, but no role is operational. Native remains
  sole/default; no task, orchestration, delegation, tool/policy authority,
  memory, provider, process, dependency, Tauri/React path, Hermes integration,
  or visible behavior changed.

- Accepted D-082's application-owned native multi-agent architecture above the
  implemented one-run `AgentRuntime` and sole/default `NativeAgentRuntime`.
  Added the evidence-based assessment, nine-role catalog and staged-activation
  roadmap, one Ready AgentDefinition/AgentRegistry plan, and separately Blocked
  orchestration, governance, workflow, parallelism, UI, demonstration, and
  final-review plans. Only Personal Assistant and Research are initially
  selected for a future flow; none is operational. Hermes remains
  Deferred/Blocked with every negative transport record preserved. No
  production source, test, dependency, provider, process, Tauri/React path, or
  visible behavior changed.

- Rejected Hermes ACP for the pinned `0.20.0` / `v2026.8.3` release under
  D-081. The structured JSON-RPC stdio contract is supported, but every ACP
  session hardcodes privileged internal terminal, filesystem, browser, memory,
  skill, code-execution, and delegation tools without a supported
  conversation-only mode or Cortexa-owned pre-execution gate. The candidate
  also lacks complete immutable runtime provenance and its installed ACP SDK.
  Five deterministic fixture tests pass; real Hermes was not executed. Native
  remains sole/default and no production source, dependency, UI, provider,
  credential, or behavior changed.

- Recorded the Hermes serve WebSocket containment spike's Milestone 0 FAIL.
  The supplied `0.20.0` / `v2026.8.3` candidate passed source, critical-hash,
  package-metadata, and isolated module-discovery checks, but lacked a complete
  immutable runtime/interpreter manifest. Pinned startup has no supported
  complete no-update/no-credential/no-plugin/zero-tool mode, and reviewed
  target-Mac `sandbox-exec` could not meet exact listener, package-manager,
  Unix-socket, or detached-descendant controls. The stop condition fired before
  any Hermes process, WebSocket, provider, credential, harness, dependency, or
  adapter change; Native remains sole/default and Prompt 4D was not started.

- Implemented the application-owned `AgentRuntime`/`RuntimeRun` foundation and
  sole/default `NativeAgentRuntime` as a thin composition over the unchanged
  `InitialGatewayTurn`. Added bounded typed runtime events, closed capabilities
  and errors, exact run cancellation, bidirectional input-lane isolation,
  redacted domain types, and a private deterministic `MockAgentRuntime` contract
  fixture. Focused runtime, unchanged gateway regressions, the all-target Rust
  suite, full repository verification, Tauri release build, and security scan
  pass. No Hermes, provider, network, process, dependency,
  Tauri/React wiring, selector, automatic fallback, or visible behavior changed.

- Accepted D-079's application-owned native-first runtime architecture and
  D-080's conditional contained `hermes serve` JSON-RPC/WebSocket evaluation
  direction. The native boundary ExecPlan is Ready for a later separately
  authorized run; the WebSocket spike remains Blocked on verified native
  completion and fresh containment review; the Hermes adapter remains
  Draft/Blocked on both phases. Raw TUI-gateway stdio stays NO-GO. This
  documentation-only increment adds no runtime source, test, dependency, Hermes
  execution, process, socket, credential, provider, UI, or behavior.

- Completed a documentation-only revision of the Proposed multi-runtime ADR after
  the completed Hermes transport spike rejected raw TUI-gateway stdio as a
  supported production contract. At that increment's closeout the revision
  recorded native-only, Hermes ACP, and Hermes serve as unselected paths;
  D-079/D-080 now supersede that former status. It added no Hermes runtime,
  dependency, process, source, or application behavior.

- Added an isolated Rust/Python fixture spike for the pinned Hermes TUI-gateway
  wire. Seven ordinary tests verify bounded fake-process lifecycle, framing,
  cancellation, timeout, malformed/forbidden output, stderr separation,
  environment isolation, redaction, and direct-child cleanup; the real-Hermes
  version probe remains ignored. Official `0.20.0` / `v2026.8.3` evidence makes
  raw TUI-gateway stdio NO-GO as a supported production contract. No Hermes,
  production source, dependency, application runtime, or native behavior was
  added or changed.

- Added D-078 and durable project-direction guidance for Cortexa's current
  private, owner-only, local-first personal scope; preserved the verified native
  path; documented a conceptual framework-neutral native/Hermes adapter
  direction; and expanded the existing living ExecPlan convention. No product
  source, dependency, runtime behavior, or future capability was added.

- Recorded the closed D-077 owner-contact increment: no Apple Support contact
  was attempted, no guidance was received, no state change was observed, and
  TS-017's cause remains undetermined.

- Added D-077's documentation-only owner decision to conditionally reopen
  consideration of one future Apple Support TS-017 contact under the existing
  assistance plan. The decision authorizes no contact or signing-related action.

- Recorded the owner-operated paired deletion of the unuploaded CSR and its
  filesystem private key. Sanitized owner evidence reports both exact targets
  deleted, no additional material or remaining copy, no upload or use, and no
  certificate. Ordinary APFS/SSD deletion is not claimed as cryptographic
  erasure, and D-072/D-076 remain unchanged.

- Added a documentation-only containment and disposition plan selecting future
  abandonment and paired deletion of the unuploaded CSR and filesystem private
  key. Identification and deletion remain separately approval-bound; no signing
  material or product state changed.

- Recorded the safely stopped Apple Support TS-017 contact increment. No Apple
  contact or Developer access occurred; one unuploaded CSR file and one unused,
  unexported filesystem private-key file exist with encryption and permissions
  undetermined. No certificate exists, and the material does not satisfy D-072.

- Added a documentation-only future Apple Support assistance plan for TS-017.
  It defines minimum sanitized disclosure, owner-only contact controls, and
  stop conditions without contacting Apple or authorizing any Apple, Keychain,
  signing, credential, Cloudflare, provider, deployment, traffic, code,
  dependency, or runtime action.

- Added D-076's documentation-only owner decision to defer the signed macOS
  identity path after TS-017. Apple Support and any alternate CSR workflow
  remain future, separately approval-bound options; no Apple, Keychain,
  signing, credential, Cloudflare, provider, deployment, traffic, code,
  dependency, or runtime action was added.

- Recorded the owner-operated, local-only TS-017 read-only diagnostic outcome:
  user and default Keychain configuration were observed, the valid
  code-signing-identity count was zero, no authorization prompt or state change
  was observed, and the cause remains undetermined. No retry, remediation,
  signing, credential, Cloudflare, provider, deployment, traffic, code,
  dependency, or runtime action was added.

- Added a documentation-only remediation plan for TS-017's unavailable macOS
  Certificate Assistant CSR outcome. It specifies a future owner-operated,
  read-only diagnostic boundary and private sanitized evidence without
  authorizing Apple access, CSR retry, Keychain modification, signing,
  credential, Cloudflare, provider, deployment, traffic, code, dependency, or
  runtime work.

- Recorded the safely stopped owner-operated Developer ID Application
  certificate-creation attempt. macOS Certificate Assistant reported that the
  specified item could not be found in the Keychain before creating a CSR; the
  owner confirmed no CSR file, certificate, or new named private key was
  created. The cause remains undetermined, and all signing, Keychain,
  credential, Cloudflare, provider, deployment, traffic, and runtime work
  remains blocked pending a separately approved remediation plan.

- Added D-075 and a documentation-only future Developer ID Application
  identity-creation and private target-Mac evidence plan. They select no actual
  certificate or key and authorize no Apple, signing, Keychain, credential,
  Cloudflare, provider, deployment, traffic, code, dependency, or runtime work.

- Recorded the owner-attested completion of the separately approved individual
  Apple Developer Program enrollment: membership is active and no signing asset
  was created. This operational reconciliation adds no certificate, Keychain,
  credential, Cloudflare, provider, deployment, traffic, or runtime capability.

- Added a documentation-only future execution plan for D-074's conditional
  individual Apple Developer enrollment model. It defines owner gates, private
  evidence, stop conditions, and non-reversible commitment handling without
  accessing Apple or creating any signing, Keychain, credential, Cloudflare,
  provider, traffic, deployment, or runtime capability.

- Added D-074's documentation-only Apple Developer enrollment recommendation:
  defer enrollment now; conditionally prefer individual membership for a later
  separately approved owner-only proof while Cortexa remains personally owned;
  re-evaluate organization enrollment before company ownership, seller identity,
  or shared certificate control is needed. No Apple, signing, Keychain,
  credential, Cloudflare, provider, traffic, deployment, or runtime action was
  added.

- Added a documentation-only, owner-only Apple Developer signing-identity
  evidence plan. It defines a read-only private account review with closed
  sanitized outcomes while prohibiting enrollment, purchase, support requests,
  role changes, signing assets, downloads, installation, Keychain actions,
  credentials, Cloudflare, provider, traffic, deployment, and runtime behavior.

- Redesigned the final-vision executive architecture as a concise visual story:
  user goal, trusted assistant, orchestration, policy and human approval,
  controlled tools, intelligence and secure data, and business-ready
  governance. The technical architecture and every product, runtime,
  permission, credential, provider, cloud, traffic, and deployment boundary
  remain unchanged.

- Added a documentation-only final-vision architecture bundle with simplified
  executive and detailed technical diagrams, editable SVG sources,
  presentation-ready PNG exports, and an evidence-based summary. The visuals
  distinguish current, planned, optional, and external components without
  changing product source, runtime behavior, permissions, credentials, cloud
  resources, traffic, or deployment state.

- Added a documentation-only implementation plan for the future signed-identity
  and bounded secret-memory proof. It locks a fake-only future increment to
  three existing Rust paths without adding code, dependencies, signing,
  Keychain, credentials, Cloudflare, traffic, or runtime behavior.

- Selected stable signed macOS application identity as the future Cloudflare
  demo credential-control model in documentation only. The decision defines
  later signing provenance, secret-memory, lifecycle, and private target-Mac
  evidence gates without adding signing, Keychain, credential, Cloudflare,
  provider, traffic, or runtime capability.

- Added a documentation-only Cloudflare macOS identity and secret-memory
  boundary plan. It defines future selection criteria for stable signed identity
  or narrow Keychain ACL, bounded secret handling, lifecycle, and private
  target-Mac evidence without choosing a control or adding signing, Keychain,
  credential, Cloudflare, provider, traffic, or runtime capability.

- Added a documentation-only Cloudflare real-credential readiness plan. It
  records the stable macOS identity/ACL, secret-memory, direct owner transfer,
  rotation/revocation/rollback, dependency-review, and target-Mac evidence
  gates required before any future real demo-token proposal. No credential,
  Keychain action, Cloudflare resource, provider request, traffic, code, or
  runtime behavior was added.

- Added a fake-only, local macOS Keychain proof for the Cloudflare Access demo
  boundary. Trusted Rust reads exactly two fixed generic-password labels through
  pinned Security.framework bindings and returns only closed status or redacted
  errors. Target-Mac evidence passed missing-item, cancelled/denied-as-cancelled,
  successful-read, and cleanup checks. Repeated unsigned-executable prompts
  keep real credential ingestion blocked. No real credential, Keychain item,
  runtime wiring, IPC, Cloudflare change, request, deployment, or traffic
  remains.

- Added the documentation-only Cloudflare demo local security-boundary plan.
  It defines fake-credential-first Keychain proof, trusted-Rust-only credential
  reads, and a future local deny-only Worker with no route, preview URL,
  provider egress, secret, deployment, or traffic. No code, dependency,
  credential, Keychain item, or Cloudflare configuration was added.

- Recorded the owner-attested, Free-plan Cloudflare Zero Trust organization
  onboarding for the owner-only demo boundary. Cloudflare's default
  account-member identity provider is present; no Access application, policy,
  service token, Worker, route, DNS change, device enrollment, secret,
  provider request, or traffic was added.

- Added the documentation-only Cloudflare Access and Worker no-traffic
  deployment plan. It defines future owner approval, disabled-worker,
  one-application/one-token, rollback, and manual-evidence requirements without
  creating a token, Keychain item, Access application, Worker, route, DNS
  record, secret, deployment, provider request, traffic, or runtime behavior.

- Added D-068's documentation-only, demo-only Cloudflare Access service-token
  exception: 30-day maximum, macOS-Keychain-only secret, trusted-Rust-only
  access, one application, Worker JWT validation, and immediate revocation. No
  credential, cloud resource, deployment, or runtime behavior was added.

- Selected Cloudflare Workers Free in D-067 as the documentation-only remote
  gateway candidate for the internal OpenAI synthetic demo. No Worker, DNS,
  secret, credential, deployment, provider request, or runtime behavior was
  added.

- Recorded the owner-only synthetic-demo decisions for fake data, disclosure,
  fixed limits, disable switch, and server-side-only future key ownership. No
  credential, provider request, networking, external transmission, or runtime
  behavior was added.

- Added the documentation-only readiness plan for a future OpenAI synthetic
  demo gateway. It records required evidence and approvals; no account,
  credential, networking, provider request, external transmission, or runtime
  behavior was added.

- Superseded the unpublished Azure Stage B plan with D-066's documentation-only
  OpenAI synthetic-demo direction. No account, credential, networking, provider
  request, external transmission, or runtime behavior was added.

- Added the documentation-only Codex instruction hierarchy: concise root
  `AGENTS.md`, detailed `docs/governance/MASTER_PROMPT.md`, aligned active
  prompts/templates, D-065, and the advisory model-and-effort recommendation
  policy. No product, dependency, workflow, hook, deployment, or runtime
  behavior changes.
- Recorded ARB-002A publication through PR #41 and squash commit `36ce9ab`
  without authorizing Stage B, Stage C, Stage D, or ARB-002 runtime work.
- Added D-064 and the documentation-only ARB-002A Phase 4 gateway threat model
  and closed configuration specification. The design separates no-traffic
  provisioning, synthetic-only transport, and real-content activation; closes
  the non-secret Microsoft registration, Azure network, managed-identity, RBAC,
  disclosure, evidence, and security-test defaults; and preserves every later
  approval gate.
- No identity, cloud resource, DNS, credential, Keychain, gateway networking,
  `AgentProvider`, Azure OpenAI connection, disclosure UI, external processing,
  dependency, Tauri boundary, or application behavior is added. ARB-002 remains
  High and unresolved.
- Published the documentation-only D-063 Phase 1 Azure OpenAI provider decision
  through PR #39 from source commit `e432681` and squash-merged it at
  `4abd49d`. Documentation run `29706772519` passed, the source and squash trees
  are identical, and no publication action remains. D-060 through D-063,
  ARB-002's High unresolved status, and all implementation prohibitions remain
  unchanged.
- Selected Azure OpenAI in Microsoft Foundry as the Phase 1
  synthetic-evaluation provider candidate under D-063. The planned boundary
  uses one Central US regional deployment, managed identity and
  least-privilege RBAC, foreground Responses with storage disabled, strict
  custom functions, and no automatic provider fallback. No cloud resource,
  networking, credential, provider adapter, real-content approval, or product
  behavior is added; D-061 evidence and ARB-002 remain blocking.

- Published the documentation-only D-062 Phase 1 Microsoft personal identity
  decision through PR #37 from source commit `e39523f` and squash-merged it at
  `c458f27`. Branch Documentation run `29705183818` and post-merge
  Documentation run `29705209977` passed. D-060, D-061, D-062, the original
  completion report, ARB-002's High unresolved status, and all implementation
  prohibitions remain unchanged.
- Selected Microsoft personal identity as the sole Phase 1 identity provider in
  D-062 without implementing authentication. Google and Apple are deferred;
  `offline_access`, persistent sessions, automatic email linking, workforce
  tenants, Microsoft Graph scopes, and all identity, gateway, credential, and
  networking implementation remain excluded. ARB-002 remains High and not
  Ready.
- Published the documentation-only O-006/O-007 provider-boundary amendment
  through PR #35 from source commit `4b474b4` and squash-merged it at `853da62`.
  Branch Documentation run `29703530854` and post-merge Documentation run
  `29703588215` passed. D-060, D-061, both completion reports, ARB-002's High
  unresolved status, and all implementation prohibitions remain unchanged.
- Recorded D-060 and D-061 as a documentation-only consumer-first,
  enterprise-ready gateway decision boundary, then amended D-060 to separate
  identity-provider support, Azure-first portable cloud hosting, and future
  trusted AI model-provider support. Microsoft, Google, and Apple remain Phase
  1 identity candidates; Entra and compatible enterprise OIDC/SAML providers
  remain Phase 2 targets. Azure Container Apps in Central US and the inactive
  `https://api.cortexaai.io` origin remain the one-primary-cloud initial target;
  AWS, Google Cloud, active-active multicloud, and three-cloud deployment remain
  deferred. Every AI provider requires independent O-007 evidence. No
  `AgentProvider`, networking, identity, credential, cloud, enterprise, or
  runtime behavior changed, and ARB-002 remains High and unresolved.
- Published the evidence-based High-severity advisory disposition through PR
  #33 from source commit `26f68b4` and squash-merged it at `7bf1a5c`. Branch
  Documentation run `29676662232` and post-merge Documentation run
  `29676693814` passed. ARB-001 remains resolved, ARB-002 remains
  decision-required, four missing product capabilities remain blocked as
  future work, ARB-006 and ARB-007 remain High with explicit legal and release
  triggers, and ARB-044 remains superseded. D-059 records the secure-default,
  no-false-resolution policy; O-006 and O-007 still block live model traffic.
- Published the documentation-only D-058 project-memory reconciliation through
  PR #31 and squash-merged it at `74a8d2c`. Live governance documents now treat
  D-058 as closed without requesting another recursive publication closeout;
  dated plan, increment, and review evidence remains unchanged.
- Published D-058 through PR #30 from implementation commit `9a2c75d` and
  documentation closeout commit `da08573`, then squash-merged it at `1780d7f`.
  Branch CI run `29670565671`, branch Documentation runs `29670565657` and
  `29671289962`, post-merge CI run `29672575232`, and post-merge Documentation
  run `29672575254` passed with the exact Linux runner 21 and macOS runner 22
  assignments. The marker remains valid after post-publication documentation
  reconciliation.
- Replaced blanket validation with two read-only, risk-based workflows.
  Documentation-only changes run focused documentation and repository checks;
  application changes select frontend, Linux Rust, target-Mac Rust, and
  dependency-audit jobs from a deterministic fail-closed classifier.
- Consolidated the weekly JavaScript and Rust advisory checks into CI, added 16
  classifier cases and expanded repository-health coverage to 37 tests, pinned
  official actions by verified v7.0.0 commit SHA, and preserved the complete
  local increment gate. D-058 adds a direct push-range fixture, raising the
  classifier suite to 17 cases and repository tests to 38.
- Recorded the failed PR #30 hosted allocation caused by exhausted Actions
  minutes or spending limit and routed trusted pushes to exact Linux and macOS
  `cortexa-ci` selectors under D-058. Persistent runners receive no pull-request
  event, secret, write, `sudo`, deployment, or publication path. No product
  source, dependency, lockfile, Tauri boundary, permission, CSP, or SQLite
  behavior changed.
- Published Increment 4V / ARB-001 through PR #23 from reconstructed source
  commit `ec919e9`; hosted CI, Documentation, and Security passed before the
  exact 19-path remediation was squash-merged at `6e6f91d`. The `04v` marker
  remains complete and valid, original reviewed commit `3440ce9` remains
  preserved, and no later remediation was started.
- Reconstructed the exact Increment 4V / ARB-001 19-path scope from preserved
  commit `3440ce9` onto synchronized `main` at `d81b73a`, retaining all later
  repository-governance changes while complete local verification passes.
  Publication still requires a valid committed marker, refreshed PR #23 hosted
  checks, and separate merge approval.
- Adopted risk-based repository validation: focused checks during
  implementation, one stable change-class completion gate, complete
  verification for cross-cutting work, and documentation-only validation
  without unrelated frontend, Rust, or application-build checks. Synchronized
  all six reusable increment prompts, both increment-authoring templates, the
  prompt index, and coordinating remediation and repository-health workflows
  without changing product, GitHub workflow, hook, skill, dependency, or runner
  configuration.
- Published Meta Increment 8 through PR #25 from verified source commit
  `2d3261a`; hosted CI, Documentation, and Security passed before the prompt
  library was squash-merged at `d26b5e1`. No product source, dependency, skill,
  hook, Tauri, storage, permission, or behavior changed. The post-publication
  project-memory reconciliation passed complete verification and re-finalized
  the existing gate with one non-blocking stale-roadmap advisory.
- Reorganized the copy-paste prompt library into increment, review, workflow,
  and authoring-template categories with one selection guide and consistent
  human-readable metadata across 23 prompt assets.
- Added bounded feature, bug-fix, refactor, severity-remediation,
  single-advisory-remediation, release, repository-health, and prompt-authoring
  instructions without changing repository skills, hooks, dependencies, or
  product behavior.
- Preserved useful flat-library content through 13 moves and two documented
  merges, repaired active prompt references, and retained old paths only in
  dated historical evidence.
- Routed the existing read-only CI, Documentation, and Security jobs to the
  repository's dedicated Linux x64 runner through the exact custom-labeled
  selector, with no pull-request trigger, a maintainer-controlled push-branch
  allowlist, and fail-fast host prerequisite checks. Complete local and remote
  verification passed on final branch commit `cfa976f`; PR #24 was
  squash-merged at `eaf6c9f`.
- Added self-hosted runner operating guidance, security and testing boundaries,
  and repository-health regressions that reject generic or unguarded
  self-hosted selectors. A separately approved two-file portability correction
  compiles private native decision-source support only on macOS. No public API,
  target-Mac behavior, dependency, Tauri boundary, permission, schema,
  identifier, secret, or product behavior changed.
- Reconciled ARB-022 as squash-merged through PR #22 at `7c79e65` and Increment
  4V / ARB-001 as verified at `3440ce9` on open PR #23. Its failed hosted checks
  did not start because of the account billing or spending-limit state.
- Resolved ARB-022 in the reviewed documentation state by recording the advisory
  backlog and post-Meta-7 reconciliation as squash-merged through PR #21 at
  `cc434d9`, removing already-completed publication work from the live queue,
  and preserving Increment 4V as Ready but unstarted. The resolving remediation
  commit remains pending until committed; product source and dated historical
  evidence are unchanged.
- Added the evidence-based advisory remediation backlog covering 64 source
  findings, 25 normalized active remediations, historical dispositions, and the
  ordered recommendation to close project-memory drift before Increment 4V.
- Reconciled Meta Increment 7 as squash-merged through PR #19 at `96ba6ae` with
  successful hosted CI, documentation, and security checks. Increment 4V is now
  the first Ready product increment, still unstarted and separately controlled.
- Reconstructed Meta Increment 7 on the repaired dependency baseline at
  `b298999`, reran the complete repository, artifact, bundle, audit, and
  target-Mac verification matrix, and preserved remote PR #19 untouched pending
  separately approved publication.
- Recorded that repeated Tauri CLI ICNS generation is pixel-deterministic but
  not container-byte deterministic. D-052 therefore requires decoded
  representation equality for regeneration and exact byte equality between the
  reviewed repository ICNS and each packaged app resource.
- Restored a valid, reproducible dependency baseline after overlapping
  Dependabot merges removed the direct Vitest entry, produced duplicate npm
  lock keys, selected Vite outside the React plugin peer range, and selected a
  rusqlite transitive build script incompatible with supported Rust.
- Reinstated one deduplicated `vite@7.3.5` graph, `vitest@3.2.6`, and
  `rusqlite@0.37.0` while preserving other compatible dependency updates.
  Clean installation, complete repository verification, npm audit, secret scan,
  and the exact RustSec advisory-baseline gate pass without product behavior or
  source changes.
- Replaced exactly the 16 existing Tauri icon outputs with derivatives of the
  canonical Cortexa app-icon source and verified PNG, ICO, ICNS, debug-bundle,
  release-bundle, Finder, menu, and macOS application-icon presentation.
- D-051 records the project-owner-approved advisory that raw unbundled
  `npm run tauri -- dev` retains macOS's generic `exec` icon while debug and
  release `.app` bundles use Cortexa. Default DMG creation also remains a
  release-readiness advisory after its Finder AppleScript step failed; both app
  bundle builds passed.
- Reconciled the unchanged verified application-icon rollout as Ready Meta
  Increment 7 after publication of the Meta Increment 6 readiness audit. D-050
  records the new live number, preserved 16-icon scope and verification matrix,
  separate implementation approval, and absence of any icon or product change.
- Added the documentation-only Meta Increment 6 Product Readiness Audit with a
  source-backed `NOT READY` result, 57/100 composite score, 16 category
  assessments, 18 classified findings, an ordered remediation backlog, roadmap
  recommendations, and explicit Passed/Failed/Not Run/Manual Pending evidence.
- Reconciled Meta Increment 5 as published and squash-merged at `6b149fa` with a
  valid marker. D-049 records the owner's reassignment of Meta Increment 6 to the
  audit, defers the unchanged historical icon plan pending later renumbering,
  and leaves Increment 4V Proposed as the smallest recommended remediation.
- Confirmed the complete repository quality gate passes while production
  readiness remains blocked by the absent end-to-end workflow, terminal audit
  gap, gateway/executor/persistence decisions, accessibility and non-functional
  evidence gaps, unresolved Rust advisories, and release/legal/enterprise gates.
- Added the mandatory `meta-06` consolidated post-increment review after the
  repository Stop hook requested closeout. The documentation increment passes
  with advisories; next-increment readiness remains Blocked.
- Verified Meta Increment 5 repository health and GitHub hygiene with an honest
  branded README, explicit contribution and no-license boundaries, owner review
  paths, structured issue and pull-request templates, review-only Dependabot
  proposals, and read-only GitHub quality workflows.
- Added standard-library repository checks for internal links, secret patterns,
  generated output, licensing status, documented commands, and workflow safety,
  plus an exact RustSec baseline parser and 16 focused regression tests.
- Added GitHub label and milestone policy, a release-notes template, dependency
  and security audit automation, and broader database/environment/log/backup
  ignore coverage without changing application behavior or dependencies.
- D-046 records least-privilege workflow and exact advisory-baseline behavior;
  D-047 records that no repository license is selected; D-048 records Meta 5
  queue selection, the stopped unimplemented Meta 4 request, and the unchanged
  application-icon rollout's historical Meta 6 number. D-049 supersedes that
  live number through the owner-assigned readiness audit and defers icon work.
- Reconciled Meta Increment 3 as squash-merged at `ad9042c`. Remote labels,
  milestones, branch protection, and CODEOWNERS enforcement remain unverified;
  Meta Increment 5 hosted CI, documentation, and security checks passed before
  squash merge.
- Verified Meta Increment 3 repository-local Codex automation with shared bounded
  Git/path/JSON validation, a read-only session-end inventory, expanded hook
  regressions, focused engineering-review skills, matching prompts, and review
  templates.
- D-045 preserves the supported Stop contract, records the project-owner Git
  naming and squash-PR policy, and renumbers the unchanged application-icon
  rollout to Meta Increment 4.
- Documentation-only Meta Increment 2 engineering operating system with
  authoritative engineering, current architecture, normalized requirements,
  milestone roadmap, testing, security-review, and release guides.
- Explicit documentation precedence and current/mock/planned/prohibited status
  labels grounded in the actual React, Tauri, Rust, SQLite, gateway, policy,
  approval, audit, menu, and permission boundaries.
- D-044 renumbers the unimplemented verified application-icon rollout to Meta
  Increment 3 while preserving its exact source, non-goals, and separate
  approval gate.
- Reconciled Meta Increment 1 publication at `5edbf4d`, current repository
  synchronization, README capability claims, and resolved TS-010 public-branch
  lag condition.

### Added

- Meta Increment 1 canonical Cortexa identity assets under `assets/branding/`,
  preserving the owner source exactly for primary/light/dark use and adding
  proportionally padded favicon and future app-icon source derivatives.
- Brand guidelines for logo use, safe spacing, minimum size, backgrounds,
  prohibited treatments, color, contrast, typography, iconography,
  presentations, and architecture diagrams.
- Repository-local `$branding` skill for applying the official identity while
  preserving compatibility IDs and separately gating Tauri icon replacement.
- Documentation-only Meta Increment 2 Ready plan for generating and verifying
  the complete production Tauri icon family on a separately approved branch.
- Official logo use in the README and application sidebar plus a production
  Vite favicon reference, with focused coverage replacing the letter
  placeholder.
- D-043 recording the owner raster as brand authority, exact derivative rules,
  unchanged compatibility identifiers, and deferred production icon rollout.
- Post-publication reconciliation recording Increment 4U as committed, pushed,
  fast-forward merged, and synchronized at `61525bf`, with Increment 4V still
  unstarted and separately controlled.
- Increment 4U private pending-approval ownership inside `InitialGatewayTurn` plus one no-argument idempotent run-termination cancellation operation that delegates only to the existing private approval manager.
- Focused request and public-contract coverage for exact run-termination facts, no interaction evidence, no-pending and repeated-call idempotence, typed-error retention, expiry precedence, successful native cleanup, and late-native-outcome rejection.
- D-042 documenting private manager-issued ID ownership, manager-authoritative expiry and terminalization, run-termination non-authority, and retained stale-dialog/audit boundaries.
- Documentation-only Increment 4V proposal for recording exact native and run-termination approval resolutions in the turn's private typed in-memory audit adapter before returning a closed resolution-plus-receipt value.
- Exact two-file future source/test scope, manager-then-audit ordering, typed failure boundary, volatility and non-authority constraints, risks, non-goals, verification, closeout scope, and rollback for terminal approval audit binding.
- Independent clean-commit fingerprint reproduction confirming `244a1d8` still matches the stored valid `04t` completion marker while the live planning workspace correctly reports a stale fingerprint.
- Documentation-only Increment 4U plan for resolving the bound turn's exact pending approval as run-terminated through its private approval manager.
- Exact two-file future source/test scope, private pending-ID lifecycle, idempotent cancellation contract, expiry precedence, late-outcome rejection, risks, non-goals, verification, closeout scope, and rollback for initial approval run termination.
- Post-publication project-memory reconciliation recording Increment 4T as committed, pushed, fast-forward merged, and synchronized at `244a1d8` with a valid marker before 4U planning edits.

- Increment 4T macOS-gated same-manager resolution binding inside `InitialGatewayTurn`, consuming one sealed `TrustedApprovalSourceOutcome` and returning the existing exact non-authorizing `ApprovalResolution` or typed approval error.
- Focused gateway-request unit coverage for every closed native-result mapping, exact retained identity and preview facts, interaction evidence, cross-manager rejection without recipient mutation, and outcome/resolution redaction.
- Documentation-only Increment 4T plan for returning one sealed trusted approval source outcome to the exact private manager that issued its presentation.
- Exact two-file future source/test scope, same-manager resolution contract, test-only helper boundary, risks, non-goals, verification, closeout scope, and rollback for terminal initial approval resolution.
- Increment 4S terminal approval-manager binding inside `InitialGatewayTurn`, returning one exact owned `ApprovalPresentation` for terminal `RequireApproval` while preserving non-authorizing policy events for `Allow` and `Deny`.
- Public gateway-request contract coverage for exact presentation identity, classification, typed preview, manager-assigned ID, terminal ordering, protocol-error retention, failure/cancellation discard, and redacted event debug output.
- Documentation-only Increment 4S plan for consuming terminal `RequireApproval` through the existing exact approval manager and issuing one owned non-authorizing presentation.
- Exact two-file future source/test scope, approval-presentation event contract, risks, non-goals, verification, closeout scope, and rollback for terminal initial approval binding.
- Increment 4R terminal policy binding inside `InitialGatewayTurn`, returning one retained non-authorizing `PolicyDecision` only after accepted terminal completion.
- Public contract coverage for exact `Allow` and `RequireApproval` outcomes, retained typed call facts, terminal ordering, protocol-error retention, failure/cancellation discard, and redaction.
- Documentation-only Increment 4R plan for consuming the terminal initial function call through the fixed deterministic policy engine before it leaves the bound turn.
- Exact two-file future source/test scope, policy-decision event contract, risks, non-goals, verification, closeout scope, and rollback for terminal initial policy binding.
- Increment 4Q private pending-call ownership inside `InitialGatewayTurn`, with terminal release and failure/cancellation discard behavior.
- Public contract coverage for both local tools, transactional protocol-error retention, exact terminal release, terminal discard paths, text behavior, schema failure, state, and redaction.
- Documentation-only Increment 4Q plan for withholding a schema-validated initial function call until terminal response completion and discarding it on failure or cancellation.
- Exact two-file future source/test scope, optional-event contract, pending-call lifecycle, risks, non-goals, verification, closeout scope, and rollback for terminal function-call release.
- Increment 4P closed `InitialGatewayEvent` and typed `InitialGatewayTurnError` contracts for schema-bound initial gateway events.
- A private exact local registry owned by `InitialGatewayTurn`, exhaustive normalized-event conversion, terminal local-schema failure, and public tests for typed arguments, classification, failures, cancellation, and redaction.
- Documentation-only Increment 4P plan for schema-bound initial gateway events that prevent raw normalized function calls or caller-selected registries from leaving the bound turn.
- Exact two-file future source/test scope, typed event/error contract, terminal local-schema failure, risks, non-goals, verification, closeout scope, and rollback for initial-turn schema ownership.
- Documentation-only Increment 4O plan for one transport-free initial gateway turn that binds request bytes, correlation identities, exact local tool names/version, response validation, and cancellation.
- Exact two-file future source/test scope, public API narrowing, risks, non-goals, verification, closeout scope, and rollback for the bound initial gateway turn.
- Increment 4O non-cloneable `InitialGatewayTurn` that owns the closed initial request and response validator, derives exact correlation and local tool-contract configuration once, and exposes only bounded request/stream operations.
- Six public bound-turn contract tests covering exact request bytes, matching and mismatched identities, both allowed tools, unknown tools, version mismatch, cancellation, and content redaction.
- Documentation-only Increment 4N plan for one closed, transport-free, byte-bounded initial desktop-to-gateway request contract.
- Exact four-file future source/test scope, fixed request fields and limits, content-redaction rules, risks, non-goals, verification, closeout scope, and rollback for the initial gateway request.
- Increment 4N non-cloneable initial gateway request with private closed wire serialization, fixed `cortexa_desktop_mvp@1` tool-set identity, existing conservative limits, shared opaque-ID validation, and post-escaping 64 KiB enforcement.
- Six focused request tests and one public-boundary integration test covering exact fields, deterministic serialization, non-ASCII content, identity and size failures, redaction, and forbidden provider/credential/authority fields.
- Documentation-only Increment 4M plan to remove the disconnected caller-authored generic Rust platform capability scaffold while preserving future capability-specific adapter requirements.
- Exact four-file source scope, caller-absence and stale-symbol checks, focused app-info and Permission Center regression coverage, risks, non-goals, verification, closeout scope, and rollback for the legacy platform removal.
- Documentation-only Increment 4L plan to remove the disconnected unbounded arbitrary-content Rust memory scaffold while preserving the future user-controlled encrypted memory requirement.
- Exact four-file source scope, caller-absence and stale-symbol checks, focused storage regression coverage, risks, non-goals, verification, closeout scope, and rollback for the legacy memory removal.
- Documentation-only Increment 4K plan to remove the disconnected synchronous arbitrary-string provider scaffold while preserving the verified normalized gateway protocol and function-call validator.
- Exact three-file source scope, caller-absence and stale-symbol checks, focused gateway regression coverage, risks, non-goals, verification, closeout scope, and rollback for the legacy provider removal.
- Documentation-only Increment 4I plan to remove the unused public arbitrary-string audit scaffold before any production coordinator or persistence path can adopt it.
- Exact three-file source scope, stale-symbol checks, focused regression coverage, risks, non-goals, verification, and rollback for the audit-surface removal.
- Two repository-hook regressions proving that reviewed tracked deletions remain valid after commit and unreviewed post-finalization deletions invalidate completion evidence.
- Repository Workflow Increment 4J plan, record, troubleshooting entry, security boundary, and deterministic existing-content fingerprint decision.
- Increment 4H closed typed in-memory approval-audit records, non-authorizing sequence receipts, fixed typed errors, and a 1,024-record no-eviction adapter.
- Six focused adapter tests and one public gateway-to-audit integration test covering exact identity, metadata, redaction, expiry, run termination, invalid evidence, duplicates, capacity, and sequence overflow.
- Documentation-only Increment 4H plan for a bounded typed approval-audit adapter derived from one exact terminal `ApprovalResolution`.
- Proposed closed approval-audit record, complete terminal disposition/evidence validation matrix, 1,024-record in-memory limit, exact-subject duplicate rejection, checked sequencing, redacted debug/errors, and non-authorizing receipt.
- Repository Workflow Increment 4G post-increment review skill, trusted project Stop hook, deterministic Python standard-library validator, structured report template, review directory, and ignored completion state.
- Fifteen focused gate tests covering missing, failed, pending, passing, stale, conflicting, suspicious, malformed, symlink-escape, re-finalized, post-commit, and loop-guard behavior.
- Increment 4F display-name-only rename plan, record, and D-026 compatibility boundary for the `Cortexa` product name.
- Phase 4 Increment 4E manager-issued one-shot approval presentation and sealed exact-subject Rust-owned macOS native source outcome.
- Typed Edit/no-decision/source-failure cancellation, explicit `NotEvaluated` authentication evidence, and a non-executing target-Mac manual harness.
- Private pointer-identical manager-instance binding that rejects cross-manager source-outcome substitution even when public IDs collide.
- Fixed-field-first native preview construction with exact zero-width/default-ignorable/line/bidirectional-format rejection and a 1,024-Unicode-scalar complete-message limit.
- Fail-closed native button ordering with Reject as the first/default button and closed mapping for every `rfd` message-dialog result.
- Exact macOS-target `rfd = "=0.17.2"` dependency with default features disabled, no Tauri dialog plugin or WebView permission, and reviewed source, MIT license, target feature tree, duplicate graph, lockfile, and native boundary.
- Sixteen approval unit tests and two approval-binding integration tests covering one-shot issuance, cross-manager collision, identity mismatch, all native results, cancellation/expiry/replay precedence, Edit invalidation, presentation safety, message limits, source failure, and redaction.
- Documentation-only Phase 4 Increment 4E plan for a Rust-owned macOS native approval-decision source.
- Phase 4 Increment 4D exact approval binding from validator-owned run/request/call identity through one terminal approval resolution.
- Closed borrowed `create_local_task@1` approval previews, one-pending state, a 1,024-subject manager-lifetime cap, relative 120-second monotonic expiry, explicit cancellation, and non-evicting replay prevention.
- Exact gateway-to-approval integration coverage and adversarial lifecycle, expiry-boundary, capacity, replay, overflow, and redaction tests, bringing the Rust suite to 82 library and ten integration tests.
- Documentation-only Phase 4 Increment 4D plan for exact run/request/call-bound approval ownership.
- Proposed closed borrowed `create_local_task@1` approval preview, one-pending limit, 1,024-subject lifetime cap, relative 120-second monotonic expiry, cancellation, rejection, one-time consumption, and non-evicting replay prevention.
- Phase 4 Increment 4C ownership-consuming `PolicyInput` and input-retaining `PolicyDecision`, sourced only from one `SchemaValidatedFunctionCall`.
- Closed policy reasons with derived outcomes plus four conservative rule-table tests and two public gateway-to-policy boundary tests.
- Documentation-only Phase 4 Increment 4C plan for removing the unused raw Rust proposal bypass and binding deterministic policy input to one owned `SchemaValidatedFunctionCall`.
- A proposed canonical typed policy boundary with no caller-supplied context, conservative handling of missing trusted evidence, closed reasons, derived outcomes, exact input retention, redacted debug output, and no approval or execution authority.
- Phase 4 Increment 4B closed Rust schema catalog for exact `get_current_datetime@1` and `create_local_task@1` input contracts.
- Ownership-consuming gateway-to-local function-call validation with typed arguments, locally derived risk and permission, and explicitly non-authorizing output.
- Fifteen focused schema, registry, boundary, and redaction tests, bringing the Rust library suite to 79 tests.
- Documentation-only Phase 4 Increment 4B plan for closed local tool schemas and typed validation of normalized function calls before proposal or policy conversion.
- Proposed exact version-1 input contracts for `get_current_datetime` and `create_local_task`, with locally derived identity, risk, permission, and redacted typed argument handling.
- Phase 4 Increment 4A transport-free Rust gateway protocol with closed normalized events, typed redacted failures, conservative limits, sequence/state validation, and idempotent local cancellation.
- Explicitly non-actionable function-call values with allowed-name and tool-contract checks plus bounded, duplicate-free JSON-object argument validation.
- Seventeen focused gateway-protocol tests, bringing the Rust library suite to 67 tests.
- Documentation-only Phase 4 gateway and Responses security-boundary analysis grounded in the verified repository and current official OpenAI documentation.
- A proposed Increment 4A plan for a transport-free, versioned Rust gateway protocol and deterministic fixture validator.
- Explicit credential ownership, gateway responsibilities, dual event/function validation, cancellation, conservative limits, error-redaction, and audit-boundary contracts.
- Phase 3 Increment 3D fixed final answers bound to exact simulated results, runs, and conversations.
- A frozen conservative mock-loop contract covering model turns, tool calls, retries, assistant output, and unavailable network, tool-timeout, file, and search capabilities.
- Focused limit, output, identity, ordering, privacy, retry-cap, one-tool-call, empty-session, and restoration coverage, bringing the frontend suite to 124 tests.
- Documentation-only Phase 3 completion analysis and proposed Increment 3D plan for deterministic result-to-final sequencing and explicit mock-loop limits.
- Phase 3 Increment 3C fixed simulated tool results bound to exact runs, conversations, and mock proposals.
- Accessible approve-only result presentation with explicit `Simulated`, `No execution`, and fixed no-change labeling.
- Focused result validation, proposal-binding, decision, Retry, privacy, empty-session, and restoration coverage, bringing the frontend suite to 104 tests.
- Documentation-only Phase 3C gap analysis and proposed plan for approve-only simulated tool-result presentation.
- Phase 3 Increment 3B typed fixed-copy mock context provenance bound to exact run and conversation IDs.
- Accessible per-run context disclosure showing the current request as used and all unaccessed source categories as not used.
- Focused provenance validation, privacy, Retry, empty-session, and conversation-restoration coverage, bringing the frontend suite to 92 tests.
- Documentation-only Phase 3B gap analysis and proposed plan for volatile mock context provenance disclosure.
- Phase 3 Increment 3A volatile conversation sessions with deterministic IDs, bounded titles, and per-session messages and mock tool activity.
- Accessible New conversation and newest-first conversation-history controls in the application sidebar.
- Focused conversation-model, reducer, native-route, busy-state, restoration, and interaction coverage, bringing the frontend suite to 83 tests.
- Documentation-only Phase 3 gap analysis and proposed Increment 3A plan for volatile in-memory conversation sessions.
- Verified Phase 2 Increment 2G typed mock-run driver with explicit idempotent cancellation.
- Bounded mock failure presentation and deterministic Retry without duplicating the user message.
- Redacted in-memory Activity feed for accepted run, Stop, failure, approval-request, and approval-decision events.
- Focused privacy, stale-event, driver, cancellation, failure, Retry, and Activity tests.
- Verified Phase 2 Increment 2F deterministic in-memory assistant interaction shell.
- Fixed mock assistant streaming, Stop behavior, conversation messages, and tool activity presentation.
- Mock-only approval preview with deterministic approve, reject, and edit decisions that execute no tool.
- Focused script, reducer, cancellation, and interaction tests, bringing the frontend suite to 47 tests.
- Verified Phase 2 Increment 2E React application shell.
- Reducer-and-context navigation for Conversations, Tasks, Memory, Activity, Integrations, Permissions, and Settings.
- Conversation workspace with a controlled in-memory composer.
- Settings diagnostics, Permission Center placeholders, and reusable empty, loading, and error presentations.
- Strict frontend listener for the closed `assistant-menu-route` native event.
- Thirty focused frontend tests across reducer, event parsing, navigation, diagnostics, Settings, and Permissions.
- Verified Phase 2 Increment 2C managed `Storage` startup integration.
- Typed `app_initialized` metadata bootstrap with idempotent migrations.
- Verified Phase 2 Increment 2D macOS menu-bar and window lifecycle.
- macOS-only Tauri `tray-icon` support using the existing bundled icon as a temporary template icon.
- Fixed menu actions for Open Cortexa, New Request, Tasks (Coming Soon), and Quit Cortexa.
- Main-window close-to-hide behavior and macOS Dock/reopen restoration.
- Closed-enum `assistant-menu-route` backend event for later React-shell integration.
- Platform-neutral menu action and lifecycle policy contracts.
- Focused deterministic unit and integration tests for routing, failure behavior, close policy, and reopen policy.
- Increment 2D record and execution plan.

### Changed

- Reconciled project memory with Increment 4S commit `6d0bed4`, now pushed and fast-forward merged into clean synchronized `main`; its `04s` completion marker was valid before 4T planning edits.
- Made project-owner approval of the exact Increment 4T plan the next gate; no source, native invocation, cancellation/expiry orchestration, audit, transport, runtime, IPC, persistence, dispatch, or execution work has started.
- Marked Increment 4S verified complete with `PASS WITH ADVISORIES`; its exact two-file source/test scope and declared closeout scope pass focused and complete verification with no manual gate required.
- Made explicit project-owner direction to commit, push, and merge verified Increment 4S the next gate; no later increment is Ready.
- Reconciled project memory with Increment 4R commit `5e58edb`, now pushed and fast-forward merged into synchronized `main`; its `04r` completion marker was complete and valid before 4S planning edits.
- Made project-owner approval of the exact Increment 4S plan the next gate; no source, approval-manager, native-source, audit, transport, runtime, IPC, persistence, dispatch, or execution work has started.
- Completed Increment 4R within its exact two-file source/test scope: the terminal standalone-call event is replaced by a fixed deterministic policy-decision event, while lower-level validation, policy rules, approval, audit, transport, runtime, and execution boundaries remain unchanged.
- Made explicit project-owner direction to commit, push, and merge verified Increment 4R the next gate; no later implementation increment is Ready.
- Reconciled project memory with Increment 4Q commit `8598612`, now pushed and fast-forward merged into synchronized `main`; its `04q` completion marker was complete and valid before 4R planning edits.
- Made project-owner approval of the exact Increment 4R plan the next gate; no source, policy-rule, approval, transport, continuation, runtime, IPC, persistence, dispatch, or execution work has started.
- Completed Increment 4Q within its exact two-file source/test scope: accepted function frames now return `None`, and only accepted terminal completion releases the exact `SchemaValidatedFunctionCall`.
- Preserved lower-level protocol, schema-validation, registry, policy, approval, audit, transport, runtime, dependency, Tauri, persistence, capability, and permission boundaries; no user-visible behavior changed.
- Reconciled project memory with Increment 4P commit `8c1a2e0`, now pushed and fast-forward merged into synchronized `main`; its `04p` completion marker was complete and valid before 4Q planning edits.
- Made project-owner approval of the exact Increment 4Q plan the next gate; no source, policy, approval, transport, continuation, runtime, IPC, persistence, or execution work has started.
- Completed Increment 4P within its exact two-file source/test scope: valid initial-turn function calls now leave the bound turn only as `SchemaValidatedFunctionCall`, while local schema rejection closes the wrapper terminally.
- Preserved lower-level protocol, registry, function-validation, policy, approval, audit, transport, dependency, Tauri, persistence, capability, and permission boundaries; no user-visible behavior changed.
- Reconciled project memory with Increment 4O commit `87be00e`, now pushed and fast-forward merged into synchronized `main`; its `04o` completion marker was complete and valid before 4P planning edits.
- Made project-owner approval of the exact Increment 4P plan the next gate; no source, transport, authentication, credential, continuation, policy, runtime, IPC, persistence, or execution work has started.
- Reconciled project memory with Increment 4N commit `d7c4b69`, now pushed and fast-forward merged into synchronized `main`; its `04n` completion marker was complete and valid before 4O planning edits.
- Made project-owner approval of the exact Increment 4O plan the next gate; no source, transport, authentication, credential, continuation, runtime, IPC, persistence, or execution work has started.
- Completed Increment 4O within its exact two-file source/test scope with no dependency, transport, credential, continuation, coordinator, Tauri, persistence, policy, approval, audit, dispatch, execution, capability, or permission expansion.
- Marked Increment 4O verified complete after focused tests, Clippy, complete repository verification, dependency audit, exact-scope and trust-boundary review, documentation synchronization, and the mandatory post-increment gate passed.
- Reconciled project memory with Increment 4M commit `1f03d1e`, now pushed and fast-forward merged into synchronized `main`; its deletion-stable 04m marker remains valid.
- Made project-owner approval of the exact Increment 4N plan the next gate; no source, transport, authentication, credential, provider, continuation, runtime, IPC, persistence, or execution work has started.
- Completed Increment 4N within its exact four-file source/test scope with no dependency, transport, credential, provider, runtime, Tauri, persistence, policy, approval, audit, dispatch, execution, capability, or permission expansion.
- Marked Increment 4N verified complete after focused tests, Clippy, complete repository verification, dependency audit, exact-scope and trust-boundary review, documentation synchronization, and the mandatory post-increment gate passed.
- Removed the disconnected generic Rust `platform` module and its crate-root export while preserving typed app-info, the fixed Permission Center, and future capability-specific adapter requirements.
- Marked Increment 4M verified complete after focused app-info and Permission Center checks, Clippy, complete repository verification, dependency audit, stale-symbol and exact-scope review, documentation synchronization, and the mandatory post-increment gate passed.
- Reconciled project memory with Increment 4L commit `ecd49be`, now pushed and fast-forward merged into synchronized `main`; its deletion-stable `04l` marker remains valid.
- Made Increment 4M project-owner approval the next gate; no source implementation, replacement platform adapter, OS query, permission request, native framework, Keychain, LocalAuthentication, IPC, or UI work has started.
- Removed the disconnected Rust `memory` module and its crate-root export while preserving the verified SQLite bootstrap storage boundary and future product memory requirement.
- Marked Increment 4L verified complete after focused storage checks, Clippy, complete repository verification, dependency audit, stale-symbol and exact-scope review, documentation synchronization, and the mandatory post-increment gate passed.
- Reconciled project memory with Increment 4K commit `5415444`, now pushed and fast-forward merged into synchronized `main`; its deletion-stable `04k` marker remains valid.
- Made Increment 4L project-owner approval the next gate; no source implementation, replacement memory design, migration, encryption, persistence, context collection, IPC, or UI work has started.
- Removed the unused synchronous `agent::provider` and `agent::types` arbitrary-string scaffold and their exports while preserving the verified normalized gateway protocol and exact local function-call validator unchanged.
- Marked Increment 4K verified complete after focused gateway and validation checks, Clippy, complete repository verification, dependency audit, exact-scope and trust-boundary review, documentation synchronization, and the mandatory post-increment gate passed.
- Reconciled project memory with reconstructed Increment 4I commit `99f9279`, now pushed and fast-forward merged into synchronized `main`; its corrected `04i` marker remains valid and the pre-fingerprint backup remains at `cf9d701`.
- Made Increment 4K project-owner approval the next gate; no source implementation, replacement provider, networking, credential, coordinator, dispatch, executor, persistence, IPC, or UI work has started.
- Removed the unused public `audit::logger` and `audit::types` arbitrary-string scaffold while preserving the verified typed `audit::approval` boundary unchanged.
- Reconstructed Increment 4I on corrected 4J `main` by applying the preserved change without creating a second pre-verification commit; focused and complete verification, npm audit, exact-scope and security review, documentation reconciliation, and the mandatory post-increment gate passed before publication.
- Corrected the post-increment workspace fingerprint to omit paths absent from the working tree, keeping reviewed deletion evidence stable across commit without relaxing exact changed-file reporting or stale-workspace rejection.
- Marked Repository Workflow Increment 4J verified complete after 17 focused hook tests, complete repository verification, dependency audit, exact-scope review, security review, documentation synchronization, and the mandatory gate passed.
- Preserved the unpushed pre-fix Increment 4I commit at `cf9d701`; after 4J merged, it was renamed to the backup branch and reconstructed on corrected `main` rather than publishing its invalid marker.
- Completed Increment 4H within its exact four-file runtime/test scope and retained the generic arbitrary-string audit scaffold as disconnected non-production code.
- Extended native-source tests to pass every sealed button/no-decision/source-failure result through the adapter and reject extra or contradictory evidence before mutation, without changing production dialog behavior.
- Marked Increment 4H verified complete after focused checks, `npm run verify`, npm audit, complete diff and trust-boundary review, documentation synchronization, and the mandatory post-increment gate passed.
- During planning, made project-owner approval of the exact Increment 4H four-file runtime/test list and declared closeout scope the next gate; runtime implementation remained blocked until that approval.
- Kept the arbitrary-string generic audit scaffold disconnected and proposed no persistence, coordinator, execution, IPC, UI, gateway, credential, capability, permission, manifest, or lockfile change.
- Integrated hook tests into `npm run test` and `npm run verify`, and integrated the gate with verified-increment, code/security review, end-session, planning, and project-memory workflows.
- Advanced Workflow Increment 4G to verification pending with no application behavior, dependency, network, permission, persistence, or compatibility-identifier change.
- Passed 4G focused and complete repository verification plus exact scope, no-application-source, secret, generated-output, diff, code, and security review after correcting report/state parent-symlink escapes; normal `/hooks` trust and live Stop confirmation remain pending.
- Marked Workflow Increment 4G verified complete after the project owner passed normal `/hooks` trust and live active-state Stop confirmation; the consolidated result is `PASS WITH ADVISORIES` and no later product increment is Ready.
- Published Workflow Increment 4G from `codex/post-increment-gate`, fast-forward merged it into `main`, and pushed merged `main` at the project owner's request.
- Renamed the assistant's product-facing UI, window and menu metadata, Settings application value, native approval title, diagnostics, prompts, skills, and documentation to `Cortexa`.
- Preserved the `ai-agent-assistant` npm/Cargo/repository/executable names, `ai_agent_assistant_lib`, `com.aiagentassistant.desktop`, storage identifiers, IPC/event names, and other compatibility-sensitive code identifiers.
- Advanced Increment 4F to verification pending after the pre-edit `npm run build` baseline passed and every tracked exact former-name occurrence was reviewed and updated.
- Passed every requested Increment 4F automated command with 124 frontend, 92 Rust library, and ten Rust integration tests; passed target-Mac window/menu/Settings/regression checks and complete scope, compatibility, code, and security review.
- Kept Increment 4F verification pending because the mandatory `$post-increment-gate` skill/report workflow referenced by `AGENTS.md` is absent; no later increment started.
- Marked Increment 4F verified complete by explicit project-owner direction under D-027. The absent post-increment skill did not run and no result is claimed; skill creation is deferred to the next clean branch before later implementation work.
- Published Increment 4F implementation commit `972a874` on `phase4/increment-4f`, fast-forward merged it into `main`, and pushed merged `main` at the project owner's request.
- Advanced Increment 4E to verification pending after focused Rust checks, the complete repository gate, JavaScript dependency audit, dependency review, code review, and security review passed.
- Passed the target-Mac native-dialog gate: Approve, Reject, Edit, Return/default, fixed title/content order, terminal redaction, no action or persistence, and no permission prompt were confirmed; Escape had no effect and no window-close control was available. Kept Increment 4E open only for exact `cargo-audit 0.22.2` disposition; no ignore or dependency upgrade was added.
- Marked Increment 4E verified complete after the project owner accepted D-025's scoped reviewed baseline exception for two pre-existing `quick-xml 0.39.4` RustSec advisories. The failed scanner result remains recorded; no advisory ignore or dependency remediation change was added.
- Published Increment 4E commit `b0a3036`, fast-forward merged it into `main`, and pushed the merged branch state at the project owner's request.
- Deferred LocalAuthentication to a separately approved higher-risk policy adapter because the current reversible local-task subject requires no operating-system permission or device-owner-authentication claim.
- Carried validator-owned run and gateway-request IDs through schema validation and policy without adding a public identity constructor.
- Replaced detached approval tool names, `action_hash`, preview strings, clonable records, and caller-owned times with one manager-owned exact `RequireApproval` decision and typed borrowed view.
- Removed cloning and raw derived debug output from content-bearing gateway events, approval requests, previews, and resolutions.
- Marked Increment 4D verified complete after focused checks, the full repository gate, dependency audit, diff checks, code review, and security review passed.
- Advanced from verified Increment 4D into documentation-only Increment 4E trusted approval-decision-source planning.
- Made project-owner approval of the exact Increment 4D plan and five-file runtime/test list the next gate; implementation remains blocked.
- Proposed removal of caller-authored approval tool names, action hashes, preview strings, and clonable detached records without adding a digest or dependency.
- Removed `ToolCallProposal`, the unused provider tool-call response variant, caller-supplied `PolicyContext`, independently constructed `ProposedAction`, arbitrary policy reason strings, and fallible policy evaluation.
- Made information-only/no-permission calls allowable as non-authorizing data, reversible and personal-data calls approval-required, and permission-bearing, read-only-without-scope, external/high-impact, and prohibited calls denied.
- Marked Increment 4C verified complete after focused checks, the full repository gate, dependency audit, diff checks, code review, and security review passed.
- Made documentation-only planning for exact trusted approval binding the next Ready task.
- Made project-owner approval of the exact Increment 4C trusted policy-input plan the next gate; runtime implementation remains blocked.
- Proposed removal of `ToolCallProposal`, the unused provider tool-call response variant, caller-supplied `PolicyContext`, independently constructed `ProposedAction`, arbitrary reason strings, and fallible policy evaluation only within the approved future increment.
- Replaced arbitrary placeholder schema and independently supplied tool-definition metadata with schema-derived private definitions.
- Marked Increment 4B verified complete after focused checks, the full repository gate, dependency audit, diff checks, code review, and security review passed.
- Made documentation-only planning for trusted proposal and policy-input binding the next Ready task.
- Made project-owner approval of the exact Increment 4B schema-validation plan the next gate after documentation-only planning completed.
- Chose an existing-dependency design using `serde` and `serde_json`; the proposed implementation changes no Cargo manifest or lockfile.
- Added the exact direct `serde_json 1.0.150` dependency already present in the lockfile and exported the portable gateway-protocol module without wiring it to Tauri or the existing provider.
- Marked Increment 4A verified complete after focused checks, the full repository gate, dependency audit, diff checks, code review, and security review passed.
- Made documentation-only planning for exact local tool-schema validation the next Ready task.
- Made Increment 4A deterministic gateway protocol planning the approval-blocked next task after Phase 4 planning completed.
- Defined foreground `store: false` Responses streaming and transport-abort cancellation so background response storage and provider-side cancellation are not implied.
- Replaced the generic approve outcome message with one distinct deterministic final answer rendered immediately after its matching simulated result.
- Limited an initial failed run to one fresh Retry and removed retry eligibility after failure of retry attempt `1`.
- Advanced Increment 3D to verification pending after focused checks, the full automated gate, dependency audit, code review, security review, and native development launch passed.
- Marked Increment 3D and Phase 3 verified complete after project-owner interaction, restoration, layout, lifecycle, storage, and no-permission-prompt checks passed.
- Selected bounded mock-loop completion as the final Phase 3 gap and obtained project-owner approval for the exact file plan before implementation.
- Stored simulated results with each volatile conversation and counted result-bearing sessions as non-empty.
- Advanced Increment 3C to verification pending after the full automated gate and native development launch passed.
- Marked Increment 3C verified complete after project-owner native result, decision, restoration, layout, and regression checks passed.
- Selected simulated tool-result presentation as the smallest remaining Phase 3 gap; implementation remains blocked on project-owner approval of the exact file plan.
- Stored mock provenance with each volatile conversation and counted provenance-bearing sessions as non-empty.
- Advanced Increment 3B to verification pending after the full automated gate and native development launch passed.
- Marked Increment 3B verified complete after project-owner native interaction, restoration, layout, and regression checks passed.
- Selected mock context provenance as the smallest remaining Phase 3 gap; implementation remains blocked on project-owner approval of the exact file plan.
- Bound active and retryable mock runs to exact conversation IDs so asynchronous events cannot mutate another selected transcript.
- Made native New Request create or select an empty conversation while idle and only focus the active conversation while a run or approval is pending.
- Marked Increment 3A verified complete after full automated verification, native launch, and project-owner manual acceptance passed.
- Replaced direct timer ownership in the React hook with an injectable closed mock-run driver.
- Replaced the Activity placeholder with a volatile session feed containing fixed summaries and opaque run IDs only.
- Hardened late chunk, completion, and failure handling against inactive or mismatched runs.
- Marked Phase 2 verified complete after automated, native-launch, manual interaction, lifecycle, diagnostics, storage, and no-permission-prompt checks passed.
- Made Phase 3 gap analysis and increment planning the next Ready task.
- Enabled the conversation composer for non-empty local mock requests and kept all resulting state volatile.
- Updated Activity copy to distinguish current-conversation mock activity from future persisted audit history.
- Recorded successful full repository verification, native launch, streaming, Stop, approval-decision, layout, lifecycle, storage, and no-permission-prompt checks for Increment 2F.
- Made Increment 2G integration hardening the next Ready increment.
- Replaced the proof-of-connection page with the platform-neutral React shell while retaining `get_app_info` diagnostics.
- Resolved O-004 with React reducer plus context and no new state-management dependency.
- Routed menu-bar New Request to Conversations with a cleared draft and Tasks to the Tasks page.
- Marked Increment 2E verified complete after its target-Mac gate passed.
- Marked Increment 2C verified complete based on target-Mac results: 41 Rust unit tests, 3 Rust integration tests, TypeScript, Vite, and native launch passed.
- Marked Increment 2D verified complete based on target-Mac Rust, frontend, native launch, menu-action, close, reopen, Dock, quit, storage, and permission checks.
- Updated `npm run test:integration` to execute every Rust integration-test target rather than only the original smoke test.
- Changed the Rust entrypoint to build the Tauri app explicitly so macOS `RunEvent::Reopen` can restore the hidden main window.
- Updated project memory to make Increment 2E — React application shell — the next ready increment.
- Documented that the custom right-side menu-bar status item is distinct from the standard left-side macOS application menu.

### Security

- Increment 4H records only bounded opaque identity and closed local facts, never calls the title-bearing preview accessor, and excludes arbitrary event strings, raw content, raw errors, credentials, user-actor claims, and authentication-success claims.
- The adapter revalidates exact tool/policy and terminal evidence, rejects duplicate and bounded-storage failures without partial mutation, and exposes no policy, run-liveness, durable-audit, dispatch, executor, IPC, or tool-result conversion.
- Increment 4H planning excludes the title-bearing approval preview and all raw arguments, prompts, results, errors, credentials, summaries, and details from the proposed record; the adapter may project only bounded opaque identity and closed local facts.
- The proposed record assigns no user actor, preserves `NotEvaluated` authentication evidence, revalidates every disposition/evidence combination before mutation, and grants no run-liveness, durable-audit, dispatch, or execution authority.
- The project hook requires normal Codex trust review, reads only bounded repository/report/state evidence, uses fixed Git inspection commands, rejects unsafe or suspicious changed paths, and treats its ignored marker as non-authorizing workflow state.
- Emergency hook bypass is documented through `/hooks` or `codex --disable hooks`; a bypass must be recorded and cannot complete an increment until the full gate is rerun.
- D-025 limits the RustSec exception to RUSTSEC-2026-0194 and RUSTSEC-2026-0195 on the reviewed pre-existing `plist 1.9.0 -> Tauri` path. It does not declare the advisories fixed or generally safe and requires re-review if affected APIs become reachable or the dependency path changes.
- Increment 4E keeps production choices outside the untrusted WebView: only the Rust-owned macOS source can consume a manager-issued presentation and privately construct a sealed source outcome.
- The manager rechecks a private pointer-identical manager marker, approval/run/request/call identity, one-shot issuance state, cancellation, and monotonic expiry before a terminal outcome; Edit, native no-decision, source failure, late results, mismatches, and replay fail closed.
- The native preview places trusted facts before final untrusted title content, rejects the planned presentation-format set, and excludes title and OS/dependency error content from outcomes, errors, debug output, logs, audit, persistence, and terminal output.
- Every native result records `NotEvaluated` authentication evidence and grants no audit, dispatch, execution, provider, persistence, run-liveness, actor-identity, biometric, or device-owner-authentication claim.
- Increment 4E adds no shipping Tauri wiring, frontend, capability, CSP, LocalAuthentication, permission, audit, executor, persistence, provider, network, or credential path. The target-specific dialog dependency exposes no file-dialog or raw-handle API through the application.
- Increment 4D binds approval state to one owned policy decision and derives preview content from the same typed subject; callers cannot resupply identity, arguments, classification, preview, outcome, creation time, deadline, or resolution subject.
- Approve, reject, cancel, and expiry consume the subject once per manager instance; stale, duplicate, unsupported, ineligible, over-capacity, and overflow paths fail closed through typed redacted errors.
- An `Approved` disposition remains local transport-free state, not proof of a user gesture, user presence, local authentication, active run, dispatch eligibility, or execution authority.
- Increment 4D adds no audit, dispatch, executor, IPC, persistence, provider, network, credential, Tauri, frontend, dependency, lockfile, capability, CSP, packaging, permission, or user-visible path.
- Increment 4D planning binds approval to one owned `RequireApproval` policy decision with validator-owned run/request/call identity and a preview borrowed from the same typed arguments.
- The plan makes approve, reject, cancel, and expiry one-time terminal outcomes, rejects duplicate subjects, and exposes no audit, dispatch, executor, IPC, persistence, or provider-continuation conversion.
- The plan removes untrusted `action_hash` rather than treating a digest or approval ID as authority; it adds no dependency and keeps exact task content out of debug, errors, and generic audit strings.
- Increment 4D planning changes documentation only and adds no runtime, dependency, lockfile, credential, network, Tauri, capability, CSP, packaging, permission, or user-visible path.
- Policy action metadata can now enter production policy types only through ownership of a locally schema-validated call; no constructor accepts caller-supplied identity, version, arguments, risk, permission, intent, grant state, or scope.
- `PolicyDecision` retains the exact evaluated input, derives outcome from a closed reason, redacts argument content from debug output, and exposes no approval, audit, dispatch, or executor conversion.
- Increment 4C adds no dependency, lockfile, credential, network, IPC, persistence, Tauri, capability, CSP, packaging, permission, or user-visible path.
- Increment 4C planning makes locally schema-validated identity, version, typed arguments, risk, and permission the proposed sole source of policy action metadata; model or caller values cannot override them.
- The plan removes unbound intent and permission booleans: permission-bearing and read-only calls deny, while reversible actions require approval until exact call-bound evidence exists.
- The plan keeps policy allowance non-authorizing, retains the exact evaluated input, and leaves generic approval, audit, dispatch, executor, IPC, persistence, and network scaffolds disconnected.
- Increment 4C planning changes documentation only and adds no runtime, dependency, lockfile, credential, provider, Tauri, capability, CSP, packaging, permission, or user-visible path.
- Exact local schema validation now rejects unknown tools, contract mismatches, missing or additional fields, wrong types, malformed values, non-canonical titles, control characters, and title-limit violations before any proposal or policy conversion.
- Successful schema validation discards raw JSON, exposes content only through private typed arguments, redacts debug output, and derives classification only from the local catalog.
- Increment 4B adds no proposal conversion, authorization, policy call, approval, audit, executor, runtime tool registration, network, credential, IPC, persistence, capability, CSP, permission, dependency, or user-visible path.
- The Increment 4B plan keeps schema-valid function calls non-authorizing, derives risk and permission only from closed local definitions, discards raw JSON after typed parsing, and prohibits proposal, policy, approval, audit, IPC, or executor conversion.
- Increment 4B planning changes documentation only and adds no dependency, network, credential, provider, persistence, Tauri, capability, CSP, packaging, permission, or user-visible path.
- Normalized gateway frames are bounded before decoding and fail closed on unknown fields/variants, malformed identity, sequence/state violations, mixed or excess output, late events, and invalid terminal behavior.
- Parsed function calls remain private-field untrusted protocol data with no `ToolCallProposal`, policy, approval, IPC, or executor conversion path.
- Increment 4A adds no network client, gateway deployment, credential, WebView IPC, persistence, capability, CSP, packaging, operating-system permission, or user-visible behavior.
- Production OpenAI credentials are assigned exclusively to gateway server-side secret storage; future gateway tokens remain in trusted Rust and platform secret storage and never enter the WebView or SQLite.
- The gateway must select exact strict function contracts, normalize recognized provider events, reject unknown event types, and return only a versioned closed product protocol without local execution authority.
- Normalized function calls remain non-actionable until independent Rust name, contract-version, duplicate-free JSON, exact schema, policy, approval, and executor validation succeeds.
- Provider failures cross the boundary only as closed codes and opaque correlation metadata; gateway operational telemetry and local trusted audit remain separate and exclude raw content and credentials.
- Foreground cancellation is a local terminal transition followed by transport abort, not a gateway cancellation frame or confirmed provider-side cancellation.
- `store: false` is explicitly not treated as zero retention; provider retention mode and user disclosure must be approved before live traffic, and gateway operational metadata defaults to a maximum seven-day retention.
- Phase 4 planning changed documentation only and added no runtime, dependency, lockfile, Rust, IPC, Tauri, capability, CSP, persistence, credential, network, packaging, or permission path.
- Increment 3D final-answer construction accepts validated IDs only, uses fixed copy, and fails closed on result, run, conversation, proposal, tool-call-limit, output-limit, stale-event, and retry-limit mismatches.
- Increment 3D changes no dependency, lockfile, Rust, IPC, Tauri, SQLite, capability, CSP, credential, network, packaging, or operating-system permission file.
- The Phase 3D plan keeps final answers fixed, ID-only, volatile, and explicitly mock-generated; provider calls, result payloads, real tools, trusted output, persistence, networking, native changes, and permissions remain excluded.
- Result construction accepts validated opaque IDs only, derives the exact proposal ID, uses fixed output fields, and fails closed if the proposal is missing or mismatched.
- Reject, Edit, Stop, stale events, invalid decisions, and duplicate decisions cannot create results; `Approve mock` still executes nothing.
- Increment 3C changes no dependency, lockfile, Rust, IPC, Tauri, SQLite, capability, CSP, credential, network, packaging, or operating-system permission file.
- The Phase 3C plan limits results to fixed-copy volatile WebView presentation with `executed: false` and excludes request content, arbitrary payloads, real execution, trusted audit claims, persistence, networking, and native capability changes.
- The provenance constructor accepts validated opaque IDs only, uses a closed fixed source list, and cannot receive request text or personal content.
- The new disclosure is explicitly labeled as frontend mock data rather than trusted audit evidence and adds no authorization or execution path.
- Increment 3B changes no dependency, lockfile, Rust, IPC, Tauri, SQLite, capability, CSP, credential, network, packaging, or operating-system permission file.
- The Phase 3B plan limits provenance to volatile fixed-copy WebView presentation with opaque IDs and explicitly excludes request text, real context collection, trusted audit claims, persistence, networking, and native capability changes.
- Increment 3A keeps all conversation titles, messages, and mock tool activity in volatile WebView memory and copies none of that content into Activity.
- Conversation creation and selection fail closed during streaming or pending approval; Retry eligibility is cleared when leaving the failed conversation.
- Increment 3A adds no Rust, IPC, Tauri command, capability, CSP, dependency, lockfile, persistence, credential, network, packaging, or operating-system permission change.
- Increment 2G Activity records exclude request text, tool arguments, tool results, and underlying error details.
- Driver failures map to one bounded user-facing reason; startup exceptions are not rendered or logged.
- Increment 2G adds no Rust, IPC, Tauri, capability, CSP, dependency, persistence, credential, network, or OS permission change.
- Increment 2F adds no model network, credential, dependency, Rust, IPC, Tauri command, capability, CSP, persistence, or operating-system permission change.
- Mock approval decisions remain inside untrusted WebView memory, are labeled as non-executing, and cannot authorize or invoke a local action.
- Mock timer events are bound to an active run identifier; stale events and decisions outside the valid state fail closed.
- Added no Rust, SQLite, Tauri command, capability, CSP, dependency, persistence, credential, networking, or operating-system permission change in Increment 2E.
- Native event payloads enter as `unknown` and are ignored unless they exactly match one closed route object.
- Permission Center is status-only and contains no control that can request operating-system access.
- Kept `get_app_info` as the only custom Tauri command.
- Added no WebView-invokable capability, CSP change, plugin, global shortcut, API key, OAuth flow, model networking, database schema, or product-data persistence.
- Added no Accessibility, screen capture, Apple Events, microphone, shell, or broad filesystem access.
- Menu routes use closed enums, unknown menu IDs are ignored, and no arbitrary content is executed.
- Target-Mac verification produced no macOS permission prompt.

## 0.1.0 — 2026-06-18

### Added

- Smallest runnable Tauri 2 desktop application.
- React 19, TypeScript 5.9, and Vite 7 frontend.
- Typed `get_app_info` command across the Tauri IPC boundary.
- Typed Rust startup error propagation.
- Restrictive Content Security Policy and minimal Tauri capability set.
- ESLint, Prettier, rustfmt, Clippy, Vitest, Rust unit tests, and Rust integration smoke test.

### Changed

- Added compatibility for Node.js 26.3.0 and npm 11.16.0 while preserving strict engine enforcement.

### Security

- No API keys, shell plugin, opener plugin, broad filesystem plugin, Accessibility, screen capture, or Apple Events access.

## 2026-07-13 — Phase 2 Increment 2E verified

- Marked the React application shell as verified complete on the target Mac.
- Recorded successful frontend, Rust, native-launch, menu-routing, lifecycle, diagnostics, storage-idempotence, and no-permission-prompt checks.
- Marked Phase 2 Increment 2F as Ready.
