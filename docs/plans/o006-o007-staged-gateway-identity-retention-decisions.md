# O-006/O-007 staged gateway identity and retention decisions

Status: Amendment complete with advisories; published through PR #35 and squash-merged at `853da62`
Owner: Project owner
Last updated: 2026-07-19
Original gate ID: `o006-o007-staged-gateway-identity-retention-decisions`
Amendment gate ID: `o006-provider-boundary-amendment`
Baseline: clean synchronized `main` at `ef8083d`

## Goal

Record the project owner's consumer-first, enterprise-ready gateway identity,
cloud-hosting, AI model-provider, credential, retention, data-classification,
disclosure, and accountability decisions without implementing or authorizing
any external processing path.

## User-visible outcome

None. This increment changes documentation and governance only. It defines
requirements for future onboarding and Settings disclosure but adds no UI,
account, authentication, gateway, or network behavior.

## Scope

Document three clearly separated states:

1. **Current**: no gateway deployment, networking, integrated identity
   provider, credential path, or external transmission; synthetic data is the
   maximum future pre-ZDR test boundary.
2. **Phase 1 target**: consumer and prosumer individual accounts, simple
   onboarding, personal workspaces, provider-neutral system-browser OAuth/OIDC
   Authorization Code Flow with PKCE, one or more separately selected
   consumer-compatible identity providers, and a Cortexa-operated Azure
   gateway.
3. **Phase 2 target**: organization accounts, team workspaces, centralized
   billing and administration, RBAC, organization policy and audit, Entra
   workforce SSO, tenant-aware authorization, and group-based controls. SAML,
   SCIM, and other enterprise identity providers remain demand-driven future
   decisions.

D-060 records three separate approval boundaries:

- Pluggable OAuth/OIDC identity providers, with consumer candidates in Phase 1
  and enterprise OIDC/SAML targets in Phase 2.
- One-primary-cloud Azure hosting initially, with container portability but no
  active-active multicloud or three-cloud requirement.
- A future trusted `AgentProvider` abstraction that may route only to
  separately approved AI providers and does not currently exist.

D-061 records the cross-phase, per-AI-provider retention, data, logging,
disclosure, privacy, and security policy.

## Explicit non-goals

- No application source, test, dependency, manifest, lockfile, workflow, hook,
  skill, script, Tauri, capability, permission, CSP, IPC, SQLite, migration,
  identifier, or runtime behavior change.
- No networking, Azure resource, DNS, TLS, gateway deployment, credential,
  secret, provider connectivity, or OpenAI account configuration.
- No AWS or Google Cloud deployment, active-active multicloud, cloud failover,
  three-cloud release requirement, or claim of current cloud portability.
- No Microsoft, Google, Apple, Entra, OAuth/OIDC, PKCE, token-validation,
  redirect-handler, Keychain, or other platform-secret implementation.
- No `AgentProvider`, AI model-provider integration, provider routing, provider
  credentials, or inheritance of one provider's O-007 approval by another.
- No team workspace, organization account, enterprise administration, billing,
  RBAC, organization policy, SAML, or SCIM implementation.
- No ARB-002 implementation, threat-model completion, severity reduction, or
  claim that live traffic is Ready.
- No rewrite of D-021, D-059, inception baselines, completed plans, increment
  records, or dated review evidence.
- No commit, push, merge, publication, deployment, or later increment.

## Existing behavior and constraints

- Source and manifests contain no HTTP client, provider SDK, gateway origin,
  identity integration, authorization header, credential loader, Keychain
  adapter, or live provider request.
- The React mock loop and transport-free Rust gateway turn are disconnected.
- D-021 already requires a credential-owning product gateway, a maximum
  15-minute audience-bound desktop token in trusted Rust memory,
  platform-secure session storage, content-free logs, and no provider secret on
  the desktop.
- D-059 correctly records that O-006 and O-007 were undecided when the
  High-severity disposition was published. It remains unchanged historical
  evidence.
- O-006 remains open for exact Phase 1 identity configurations, every AI
  model-provider approval, and any future cloud expansion. O-007's product
  policy is accepted, but provider-specific ZDR evidence remains a live-traffic
  prerequisite.
- The current source has no `AgentProvider`; Increment 4K deleted the legacy
  generic provider scaffold.
- ARB-002 remains High and `DECISION REQUIRED`; this increment does not make a
  transport increment Ready.

## Files expected to change

Modified:

```text
AGENTS.md
ARCHITECTURE.md
CHANGELOG.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PRODUCT_REQUIREMENTS.md
PROJECT_STATUS.md
ROADMAP.md
SECURITY.md
SECURITY_CHECKLIST.md
docs/plans/README.md
docs/reviews/2026-07-16-advisory-remediation-backlog.md
```

Created:

```text
docs/plans/o006-o007-staged-gateway-identity-retention-decisions.md
docs/increments/o006-o007-staged-gateway-identity-retention-decisions.md
docs/reviews/2026-07-19-o006-o007-staged-gateway-identity-retention-decisions-post-increment-review.md
docs/reviews/2026-07-19-o006-provider-boundary-amendment-post-increment-review.md
```

The exact amended scope is 18 documentation paths. The original valid review
report remains unchanged historical evidence; the amendment adds one review.

## Implementation steps

- [x] Begin mandatory gate state on clean synchronized `main` before edits.
- [x] Append D-060 and D-061 without rewriting D-021 or D-059.
- [x] Reconcile authoritative architecture, requirements, security policy, and
      checklist with current, Phase 1, and Phase 2 states.
- [x] Update current project memory and add an additive dated ARB-002 update.
- [x] Run documentation-tier verification and protected-path review.
- [x] Complete the consolidated post-increment review and valid marker.
- [x] Preserve the verified pre-amendment 17-path state outside the repository.
- [x] Begin the dedicated `o006-provider-boundary-amendment` gate before edits.
- [x] Separate identity-provider, cloud-hosting, and AI model-provider support
      across all authoritative live documents.
- [x] Preserve the original completion report and create the amendment report.
- [x] Rerun documentation-tier verification and finalize the amendment marker.

## Security and privacy considerations

- Provider-neutral means a closed application abstraction over separately
  approved configurations. It never means arbitrary issuer, endpoint, audience,
  client identity, redirect, or origin selection.
- Identity-provider, cloud-hosting, and AI model-provider support are separate;
  approval in one category grants no approval in another.
- The gateway validates trusted issuer, audience, signature, expiration,
  applicable tenant, and authorization context from closed server-owned
  configuration.
- Azure-first portability is not deployed AWS or Google Cloud support,
  active-active multicloud, cloud failover, or a three-cloud release.
- `AgentProvider` is future-only. No current abstraction or implementation may
  be inferred from the decision record.
- The reserved origin is inactive until DNS, TLS, deployment, authentication,
  authorization, logging, and security evidence pass.
- ZDR requires provider approval and exact organization/project evidence;
  `store: false` or policy intent is not proof.
- No real user content is permitted before verified ZDR. After verification,
  the initial class is explicitly submitted, non-sensitive text only.
- Content logging is prohibited. Operational metadata retention is capped at
  seven days.
- Full owner names and roles are recorded only where accountability requires
  them; no tenant, subscription, client, resource, credential, or secret value
  enters the repository.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex .agents scripts
rg -n "identity provider|OAuth|OIDC|SAML|issuer|audience|signature|expiration|tenant|Azure Container Apps|Central US|AWS|Google Cloud|active-active|multicloud|AI model provider|AgentProvider|O-006|O-007|ARB-002|ZDR|api\.cortexaai\.io" AGENTS.md ARCHITECTURE.md PRODUCT_REQUIREMENTS.md SECURITY.md SECURITY_CHECKLIST.md DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md ROADMAP.md
if rg -n "\bAgentProvider\b|\bMockAgentProvider\b|\bAgentProviderResponse\b|\bAgentRequest\b" src src-tauri/src src-tauri/tests; then exit 1; else test $? -eq 1; fi
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

Frontend tests, Rust tests, application builds, native launch, Azure, Entra,
identity-provider, DNS, TLS, Keychain, and ZDR operational checks are not
required for this documentation-only increment. They must be recorded as not
run rather than inferred.

## Rollback or failure strategy

Before publication, restore only the exact amendment hunks from the preserved
`/tmp/o006-o007-pre-provider-boundary-amendment.tgz` snapshot. After a future
publication, revert only its bounded documentation commit. No runtime,
dependency, database, migration, credential, cloud-resource, identity-provider,
or network rollback applies. A later change to an accepted owner decision must
append a superseding decision rather than delete D-060, D-061, or historical
evidence.

## Acceptance criteria

- [x] D-060 records the current absence, Phase 1 consumer identity, Phase 2
      enterprise expansion, Azure target, credential boundaries, owners, and
      inactive-origin gate.
- [x] D-061 records verified-ZDR policy, synthetic-only pre-verification data,
      the initial allowed and prohibited data classes, logging, disclosure, and
      owners across both phases.
- [x] O-006 remains open for exact identity and AI-provider configurations and
      future cloud expansion; O-007's policy is accepted without claiming
      operational ZDR evidence.
- [x] ARB-002 remains High, unresolved, and not Ready for implementation.
- [x] Current, Phase 1, and Phase 2 claims are consistent across live docs.
- [x] The exact 17-path documentation scope and all required checks pass.
- [x] The report result is `PASS` or `PASS WITH ADVISORIES`, and the marker is
      complete and valid.
- [x] D-060 independently defines identity-provider, cloud-hosting, and AI
      model-provider support without implying cross-approval.
- [x] Current state explicitly has no gateway, network, identity integration,
      cloud deployment, `AgentProvider`, or AI model-provider networking.
- [x] Azure is the one-primary-cloud initial target; AWS, Google Cloud,
      active-active multicloud, failover, and three-cloud release remain
      deferred.
- [x] Each AI provider requires separate O-007 evidence before real content.
- [x] The exact 18-path amendment scope passes and the original report remains
      unchanged.

## Actual results

Passed: `npm run docs:check`, `npm run repository:check`,
`npm run security:scan`, `git diff --check`, the exact protected-path diff, the
targeted decision/state consistency scan, session-end inspection, complete
diff review, architecture review, security review, code-health review,
technical-debt review, readiness review, and mandatory marker finalization.

Failed: none.

Not run: frontend tests, Rust tests, application builds, native launch, Azure,
Entra, identity-provider, DNS, TLS, Keychain, and ZDR operational checks. They
are outside this documentation-only scope and no affected product path changed.

Manual verification pending: none. The project owner approved the decisions
and exact file plan before editing.

Passed: documentation formatting and local links, repository policy, secret
scan, whitespace, exact protected-path diff, current `AgentProvider` absence,
original-report byte preservation, targeted boundary consistency, session-end
inspection, complete diff and scope review, engineering reviews, and mandatory
amendment-marker finalization.

Failed: the first attempt to reopen the original gate was rejected because its
valid completion marker is immutable. No file had changed. The separately
approved one-report scope expansion and dedicated amendment gate resolved the
workflow constraint. The first amendment-finalization attempt then rejected a
duplicated `status` command in the report manifest; the manifest-only correction
changed no evidence or scope.

Not run: frontend tests, Rust tests, application builds, native launch, Azure,
AWS, Google Cloud, DNS, TLS, identity federation, Keychain, `AgentProvider`, AI
model-provider, and operational ZDR checks. They are outside the
documentation-only risk tier.

Manual verification pending: none. The project owner approved the amended
decisions, original 17-path plan, and one-report scope expansion.

Result: `PASS WITH ADVISORIES`. ARB-002 remains a pre-existing High finding
that blocks live networking and the next product increment, not this
documentation-only decision record. Source commit `4b474b4` passed branch
Documentation run `29703530854`; PR #35 squash-merged it at `853da62`, and
post-merge Documentation run `29703588215` passed. No publication action
remains.

## Documentation updates

- [x] `HANDOFF.md`
- [x] `PROJECT_STATUS.md`
- [x] `NEXT_STEPS.md`
- [x] `DECISIONS.md`
- [x] `CHANGELOG.md`
- [x] `ARCHITECTURE.md`
- [x] `PRODUCT_REQUIREMENTS.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`

`TROUBLESHOOTING_LOG.md` is unchanged because no issue was diagnosed.
