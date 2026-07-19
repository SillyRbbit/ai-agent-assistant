# O-006 Phase 1 Microsoft personal identity decision

Status: Verified complete with advisories; published and closed
Date: 2026-07-19
Owner: Project owner
Baseline: clean synchronized `main` at `c90c77f`
Gate ID: `o006-phase1-microsoft-personal-identity-decision`

## Goal

Record Microsoft personal identity as the sole Phase 1 provider while keeping
all implementation and remaining ARB-002 prerequisites blocked.

## Approved direction

- Personal Microsoft accounts only; no workforce, guest, or arbitrary Entra
  tenant in Phase 1.
- System-browser Authorization Code Flow with PKCE S256, `state`, and OIDC
  `nonce`.
- Separate public desktop client and gateway API resource registrations.
- Minimum scopes: `openid`, `email`, and one delegated Cortexa gateway scope.
- `offline_access`, persistent sessions, profile and Graph scopes, automatic
  email linking, Google, Apple, and enterprise identity are excluded.
- Account identity is provider ID plus normalized issuer plus subject.

## Current boundary

No registration, client, redirect, OAuth/OIDC flow, credential, token, account,
Keychain, gateway, network, `AgentProvider`, AI-provider, or external-processing
path exists. D-062 authorizes documentation only.

## Remaining blockers

- Exact Microsoft registration, issuer, tenant, gateway audience, redirect,
  delegated scope, lifecycle, recovery, revocation, and threat-model evidence.
- O-006 AI-provider selection and configuration.
- D-061 provider-specific ZDR, logging, deletion, region, disclosure, and
  security evidence.
- A separate approved implementation plan.

ARB-002 remains High, unresolved, and not Ready.

## Verification status

Passed: documentation formatting and links, repository policy, secret scan,
whitespace, exact 17-path scope, protected-path review, D-060/D-061 and
historical-report preservation, current-state consistency, complete diff,
session-end inspection, and the mandatory post-increment gate.

Failed and corrected: the initial documentation check found only Prettier
formatting in approved-scope files. After formatting and correcting one
resulting paragraph split, all final checks passed.

Not run: frontend tests, Rust tests, application builds, native launch, and all
identity, registration, token, Keychain, gateway, network, and provider checks.
They are outside this documentation-only validation tier. Manual verification
pending: none; the project owner approved the decision and exact scope.

Result: `PASS WITH ADVISORIES`. ARB-002 remains the pre-existing High blocker
for any later identity, gateway, or model-networking implementation.

## Publication status

Source commit `e39523f` passed branch Documentation run `29705183818`. PR #37
squash-merged the decision record at `c458f27`, and post-merge Documentation
run `29705209977` passed. The source and squash trees are identical. No
publication action remains, and publication changes no implementation boundary
or ARB-002 disposition.

## Rollback

Restore only the exact 17 documentation paths before publication. After
publication, use an additive superseding decision for any policy change.
