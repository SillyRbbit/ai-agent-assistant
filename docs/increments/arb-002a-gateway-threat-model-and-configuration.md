# ARB-002A gateway threat model and closed configuration

Status: Verified complete with advisories; awaiting publication
Date: 2026-07-19
Owner: Project owner
Baseline: clean synchronized `main` at `92bd2c3`
Gate ID: `arb-002a-gateway-threat-model-and-configuration`

## Goal

Define the documentation-only O-006 and D-061 security contract required before
any identity, cloud, gateway, or AI-provider implementation can be planned.

## Scope

The increment creates an authoritative Phase 4 gateway threat model, a closed
configuration specification, an implementation plan, this record, and a
post-increment review. It updates fourteen current governance, architecture,
security, product, roadmap, and advisory documents for an exact 19-path
documentation-only scope.

## Decision boundary

D-064 separates four evidence stages:

1. documentation-only design;
2. no-traffic registration and resource provisioning;
3. separately approved synthetic-only transport; and
4. separately approved real-content activation.

The accepted closed defaults cover the Microsoft personal desktop/API
registration split, `gateway.access`, the `127.0.0.1` ephemeral callback at
`/oauth/callback`, a dedicated user-assigned managed identity, exact-resource
RBAC, private Azure OpenAI access, versioned disclosure acknowledgement, and
restricted operational evidence handling.

## Current boundary

No registration, identity client, OAuth/OIDC flow, loopback listener, token,
Keychain adapter, Azure resource, DNS record, certificate, managed identity,
RBAC assignment, private endpoint, gateway transport, `AgentProvider`, provider
connection, disclosure UI, or external processing exists or is authorized.

D-060 through D-063 remain authoritative. D-064 adds design closure without
representing any operational evidence as present.

## Non-goals

- No product source, tests, dependencies, lockfiles, workflow, hook, skill,
  Tauri, IPC, CSP, capability, permission, SQLite, or behavior change.
- No provisioning, deployment, credential, identity, networking, disclosure
  UI, provider, runtime, enterprise, licensing, signing, or release work.
- No ARB-002 runtime remediation, ZDR claim, real-content permission, or next
  increment start.

## Verification status

Passed: documentation formatting and local links, repository policy, secret
scan, whitespace, exact 19-path scope, protected paths, source-absence and
decision-consistency scans, complete diff review, architecture, security,
code-health, technical-debt, readiness, session-end, and mandatory
post-increment gate.

Failed and corrected: the first documentation check found Prettier-only issues
in four approved files; a later check found two security files needed
reformatting after evidence language changed. The first exact-scope command
also collapsed the untracked `docs/security/` directory and undercounted it;
the corrected command enumerates all untracked files. These corrections changed
no scope or security decision. The first final pass found one additional
formatting-only wrap in `ROADMAP.md`; formatting that approved path corrected
it.

Not run: frontend tests, Rust tests, application builds, native launch,
Microsoft registration, Azure, DNS/TLS, identity, managed identity, RBAC,
provider, networking, disclosure UI, ZDR, and operational checks. They are
outside the documentation tier and remain later-stage evidence. Manual
verification pending: none.

Result: `PASS WITH ADVISORIES`. ARB-002 remains High and unresolved. The
documented Microsoft token-lifetime and IP-literal callback gaps block Stage C,
not Stage A completion.

## Readiness

ARB-002 remains High and unresolved. Later Stage B, Stage C, and Stage D work
requires separate plans and project-owner approval. No runtime remediation is
Ready from this record.

## Publication status

Uncommitted and unpublished. No publication or later-stage action is authorized
by this record.

## Rollback

Before publication, restore the exact nineteen documentation paths. After
publication, revert only the bounded documentation commit and supersede D-064
additively if needed. No remote resource or data rollback applies.
