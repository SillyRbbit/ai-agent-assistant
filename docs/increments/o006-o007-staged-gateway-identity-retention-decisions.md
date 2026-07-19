# O-006/O-007 staged gateway identity and retention decisions

Status: Provider-boundary amendment verified complete with advisories; uncommitted and unpublished
Date: 2026-07-19
Owner: Project owner
Baseline: clean synchronized `main` at `ef8083d`
Original gate ID: `o006-o007-staged-gateway-identity-retention-decisions`
Amendment gate ID: `o006-provider-boundary-amendment`

## Goal

Record a consumer-first, enterprise-ready gateway identity and external-data
policy without creating or authorizing any product implementation.

The approved amendment separates identity-provider support, cloud hosting, and
AI model-provider support without changing that non-implementation boundary.

## Recorded direction

- Current: no deployed gateway, networking, identity provider, credential
  path, or external transmission.
- Phase 1: consumer/prosumer individual accounts, personal workspaces, simple
  onboarding, provider-neutral system-browser OAuth/OIDC Authorization Code
  Flow with PKCE, separately selected consumer-compatible providers, and a
  Cortexa-operated Azure gateway.
- Phase 2: organization accounts, team workspaces, administration, RBAC,
  organization policy and audit, Entra workforce SSO, tenant-aware
  authorization, and group controls. SAML, SCIM, and other enterprise providers
  remain demand-driven future decisions.
- D-061: verified provider-approved ZDR before real content, synthetic-only
  pre-verification testing, explicitly submitted non-sensitive text as the
  initial post-verification class, prohibited sensitive categories,
  content-free seven-day operational logs, and persistent disclosure.
- Identity: pluggable OAuth/OIDC, with Phase 1 Microsoft, Google, and Apple
  candidates and later enterprise OIDC/SAML targets; the gateway validates
  issuer, audience, signature, expiration, tenant, and authorization context.
- Hosting: one primary Azure Container Apps deployment is planned initially;
  AWS, Google Cloud, active-active multicloud, failover, and three-cloud release
  remain deferred.
- AI providers: a future trusted `AgentProvider` boundary may support multiple
  individually approved providers. No implementation exists, desktop provider
  credentials are prohibited, and every provider requires separate O-007
  evidence.

## Decision status

- D-060 accepts separate identity, cloud-hosting, and AI model-provider
  boundaries plus the gateway operator, initial Azure platform and region,
  reserved inactive origin, token and secret boundaries, and accountable
  owners.
- O-006 remains open for exact identity and AI-provider configurations and any
  future cloud expansion.
- D-061 accepts O-007's product policy and accountable owners. Provider ZDR
  approval and configuration evidence remain mandatory before real content.
- ARB-002 remains High, unresolved, and `DECISION REQUIRED`. No live transport
  or remediation implementation is Ready.

## Scope and boundaries

The exact 18-path amended documentation scope is authoritative in
`docs/plans/o006-o007-staged-gateway-identity-retention-decisions.md`. No
application source, test, dependency, lockfile, workflow, hook, skill, Tauri,
IPC, SQLite, capability, permission, CSP, credential, network, identity,
enterprise, or runtime path changes.

## Verification

The original 17-path decision record and exact 18-path amendment pass
documentation formatting and links, repository policy, secret scan, whitespace,
exact protected-path review, targeted consistency, current `AgentProvider`
absence, original-report byte preservation, complete diff review, engineering
reviews, session-end inspection, and mandatory finalization.

Failed: none.

Not run: frontend tests, Rust tests, application builds, native launch, Azure,
DNS, TLS, identity-provider, Keychain, and provider ZDR operational checks.
These are outside the documentation-only risk tier.

Manual verification pending: none. The project owner approved the decisions
and exact 17-path plan before editing.

Amended result: `PASS WITH ADVISORIES`. ARB-002 remains High and blocks live
networking and the next product increment, but it does not block this decision
record.

## Risks and rollback

The primary risks are false activation; conflating identity, cloud, and AI
providers; implying a current `AgentProvider`; arbitrary provider
configuration; unsupported ZDR claims; presenting portability as active-active
multicloud; and falsely resolving ARB-002. Explicit state labels, additive
decisions, targeted consistency scans, and protected-path review control those
risks.

Rollback restores only amendment hunks from the preserved pre-amendment
snapshot. Any future reversal of an owner decision requires an additive
superseding record.

## Next task

Review the exact 18-path documentation-only amendment for publication and wait
for separate Git direction. Do not begin ARB-002 or any identity, cloud,
gateway, networking, enterprise, or AI-provider implementation.
