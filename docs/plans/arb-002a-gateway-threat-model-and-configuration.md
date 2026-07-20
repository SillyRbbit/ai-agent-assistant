# ARB-002A gateway threat model and closed configuration

Status: Verified complete with advisories; awaiting publication
Owner: Project owner
Date: 2026-07-19
Baseline: clean synchronized `main` at `92bd2c3`
Gate ID: `arb-002a-gateway-threat-model-and-configuration`

## Goal

Close the pre-implementation O-006 and D-061 design boundary with one
authoritative threat model and one closed Phase 1 configuration specification.
The increment separates decisions from operational evidence so that no cloud
resource or traffic is required to approve the design and no design document is
mistaken for permission to provision or transmit data.

## Approved staged evidence model

| Stage                        | Boundary                             | Required exit evidence                                                                                                                                                                       |
| ---------------------------- | ------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| A - design                   | This documentation-only increment    | Approved threat model, closed non-secret configuration, security-test matrix, evidence schema, and rollback                                                                                  |
| B - no-traffic provisioning  | Separately approved future increment | Exact registrations and intended Azure resources exist; sanitized configuration and ownership evidence pass; no authentication or provider traffic                                           |
| C - synthetic-only transport | Separately approved future increment | System-browser identity, gateway authentication, private provider transport, disclosure, logging, redaction, cancellation, and negative security tests pass with synthetic data only         |
| D - real-content activation  | Separately approved future increment | Exact provider-approved ZDR, `ContentLogging=false`, stateless Responses, disclosure, retention, deletion, operational, and security evidence pass for the intended production configuration |

No stage authorizes or starts the next stage. Failure at any stage keeps later
stages blocked.

## Approved closed defaults

- Microsoft personal identity remains the sole Phase 1 provider.
- The desktop and gateway API use separate registrations.
- The gateway API Application ID URI format is
  `api://<gateway-api-client-id>` and the only delegated scope name is
  `gateway.access`.
- The loopback callback uses `127.0.0.1`, an operating-system-assigned ephemeral
  port, and the fixed path `/oauth/callback`.
- One dedicated, non-shared user-assigned managed identity authenticates the
  gateway to Azure OpenAI.
- Azure RBAC is scoped to the exact Azure OpenAI resource and grants only the
  approved inference role.
- Azure OpenAI uses a private endpoint and disabled public network access before
  any synthetic or real provider traffic.
- External-processing disclosure requires an explicit versioned
  acknowledgement before the first transmission and after any material
  provider, retention, data-classification, or disclosure change.
- Operational evidence belongs in a restricted owner-approved evidence store;
  this repository contains only sanitized references and decisions.
- Stage C and Stage D each require separate project-owner approval.

Angle-bracket values are required provisioning-time substitutions, not optional
runtime choices. A future implementation must fail closed while any placeholder
is unresolved or any observed token/resource value differs from the approved
evidence.

## Exact scope

Created:

- `docs/plans/arb-002a-gateway-threat-model-and-configuration.md`
- `docs/security/phase4-gateway-threat-model.md`
- `docs/security/phase4-gateway-configuration-spec.md`
- `docs/increments/arb-002a-gateway-threat-model-and-configuration.md`
- `docs/reviews/2026-07-19-arb-002a-gateway-threat-model-and-configuration-post-increment-review.md`

Modified:

- `AGENTS.md`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `docs/plans/README.md`
- `docs/reviews/2026-07-16-advisory-remediation-backlog.md`

## Explicit non-goals

- No application source, test, dependency, lockfile, workflow, hook, skill,
  Tauri, IPC, CSP, capability, permission, SQLite, or runtime change.
- No Microsoft registration, OAuth/OIDC client, browser launch, loopback
  listener, token exchange, token validation, persistent session, Keychain use,
  account store, or account-linking implementation.
- No Azure subscription, resource group, Container Apps environment, container,
  managed identity, RBAC assignment, virtual network, private endpoint, DNS,
  certificate, secret, provider deployment, or network traffic.
- No `AgentProvider`, model-provider adapter, disclosure UI, logging service,
  retention job, transport, runtime coordinator, dispatch, execution, or
  enterprise control.
- No real user content, provider ZDR claim, production-readiness claim, direct
  OpenAI integration, provider fallback, AWS, Google Cloud, multicloud,
  licensing, signing, or notarization work.
- No ARB-002 runtime remediation and no automatic ARB-002B start.

## Deliverables

### Threat model

`docs/security/phase4-gateway-threat-model.md` defines assets, actors, trust
boundaries, data flows, abuse cases, controls, required negative tests,
residual-risk ownership, activation gates, and emergency shutdown behavior.

### Closed configuration

`docs/security/phase4-gateway-configuration-spec.md` defines the only permitted
Phase 1 identity, gateway, Azure, provider, data, logging, disclosure, and
evidence values. Caller-controlled origins, issuers, audiences, providers,
models, tools, and retention modes remain prohibited.

### Decision record

D-064 records the staged evidence model and the approved closed defaults. It
does not supersede D-060 through D-063 and grants no provisioning or traffic
authority.

## Risks

- A reader could treat this design as authorization to create resources or send
  traffic.
- `store: false` or `ContentLogging=false` could be incorrectly represented as
  complete ZDR evidence.
- App-registration or Azure identifiers could be copied into the public
  repository without an evidence-handling review.
- Token audiences or loopback behavior could differ from the planned values;
  Stage C must prove the issued values and fail closed on mismatch.
- Microsoft's documented default access-token lifetime exceeds the accepted
  15-minute gateway-token maximum. Stage B must prove an enforceable compatible
  configuration or a later decision must define a different short-lived
  gateway-session boundary; Stage C remains blocked until then.
- Broad RBAC, public Azure OpenAI access, unrestricted egress, or an alternate
  gateway hostname could create bypasses.
- Product code could become coupled to Azure-specific provider details instead
  of the normalized gateway contract.
- Model availability, region support, and provider policy can change before
  provisioning; deployment-time revalidation remains mandatory.
- Duplicated authority could drift unless D-064 points to the two detailed
  security documents and preserves D-060 through D-063.

## Remaining prerequisites after ARB-002A

- Stage B requires separate approval for no-traffic resource and registration
  provisioning, exact owners, the restricted evidence store, and a rollback
  plan for every created resource.
- Stage B must also prove the manifest form and later testability of the
  `127.0.0.1` ephemeral callback and identify an approved way to enforce the
  maximum 15-minute gateway-token lifetime.
- Stage C requires a separately approved implementation plan, exact dependency
  review, threat-model test implementation, disclosure copy approval, synthetic
  test corpus approval, and target-Mac verification.
- Stage D requires Microsoft/provider approval and exact operational evidence
  for ZDR, `ContentLogging=false`, stateless Responses, region/model/version,
  metadata retention, deletion, incident response, and user disclosure.
- The exact registration identifiers, Azure subscription/resource identifiers,
  DNS records, certificate, deployment name, model version, and evidence-store
  locations remain operational evidence. They must not be invented here.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
test "$(git status --porcelain=v1 --untracked-files=all | wc -l | tr -d ' ')" = "19"
git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts
rg -n "D-064|Stage A|Stage B|Stage C|Stage D|gateway.access|oauth/callback|ContentLogging=false|ARB-002" AGENTS.md ARCHITECTURE.md PRODUCT_REQUIREMENTS.md SECURITY.md SECURITY_CHECKLIST.md DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md ROADMAP.md docs/security docs/plans/arb-002a-gateway-threat-model-and-configuration.md docs/increments/arb-002a-gateway-threat-model-and-configuration.md docs/reviews/2026-07-16-advisory-remediation-backlog.md
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Frontend tests, Rust tests, application builds, native launch, Azure commands,
identity flow tests, provider calls, and operational ZDR checks are not run
because this increment changes documentation only.

## Rollback

Before publication, delete the five created documents and restore the fourteen
modified documentation paths. After publication, revert only the bounded
documentation commit and supersede D-064 additively if a decision changes.
D-060 through D-063 and all dated historical reports remain intact. There is no
resource, credential, migration, deployment, network, or data rollback.

## Acceptance criteria

- [x] D-064 records the approved staged evidence model and closed defaults.
- [x] The threat model defines all requested actors, boundaries, abuse cases,
      controls, tests, and activation gates.
- [x] The configuration specification contains one closed Phase 1 value for
      every pre-implementation choice and fail-closed handling for unresolved
      operational evidence.
- [x] Stage B, Stage C, and Stage D remain separately approved future work.
- [x] ARB-002 remains High and unresolved; no runtime increment is marked Ready.
- [x] Exactly 19 documentation paths change and protected paths remain clean.
- [x] Required documentation-tier checks pass and the completion marker is
      valid.

## Actual results

The exact 19-path documentation scope passes formatting, local-link,
repository-health, secret, whitespace, protected-path, source-absence,
decision-consistency, architecture, security, code-health, technical-debt,
readiness, and mandatory gate review. The result is `PASS WITH ADVISORIES`.

The advisory is the preserved High ARB-002 future boundary. In particular,
Microsoft's documented default token lifetime exceeds Cortexa's accepted
15-minute gateway-token maximum, and the manifest-based `127.0.0.1` ephemeral
callback remains unproven. Both block Stage C but do not block this
documentation-only Stage A completion.

Frontend tests, Rust tests, application builds, native launch, registration,
Azure, networking, provider, ZDR, and product manual checks were not run because
no executable or operational path changed. No manual verification is pending.

## Publication

Uncommitted and unpublished. Publication requires separate project-owner
direction and does not authorize Stage B, Stage C, Stage D, or runtime work.
