# Cloudflare Access and Worker no-traffic deployment plan

Status: Complete documentation-only operational plan
Date: 2026-07-20
Decision authority: D-067 and D-068

## Goal

Define the bounded, owner-approved operational plan that would be required
before creating a Cloudflare Access application or a Cloudflare Worker for the
internal, owner-only fake-data demo. Authoring this plan creates no token,
Keychain item, Access application, Worker, route, DNS record, secret,
deployment, provider request, or network path.

## Future operational scope

Only after a separate project-owner approval, one operational increment may
propose the minimum no-traffic setup needed to prove that the future demo
boundary can remain disabled:

- one Cloudflare Access application with one Service Auth policy restricted to
  one demo-only service token;
- one service token with a maximum 30-day duration, with its Client Secret
  written only to macOS Keychain and readable only by trusted Rust;
- one Worker deployment that is disabled by default, has no OpenAI credential,
  no provider egress, and rejects every request before provider handling; and
- sanitized owner-held evidence that the Access audience, JWT validation
  requirements, disable switch, token revocation procedure, and route-disable
  procedure are configured as approved.

The later operational increment must name its exact external resources,
responsible owner, restricted evidence store, manual security procedure, and
rollback order before any console or API action. Repository documentation may
record only sanitized evidence references and outcomes.

## Explicit non-goals

- No token generation, Keychain write, Access application creation, Worker
  deployment, route, DNS record, secret, provider request, or traffic occurs in
  this documentation-only increment.
- No OpenAI API key, Worker secret, desktop credential, client header,
  Keychain adapter, Rust source, WebView, IPC, UI, dependency,
  infrastructure-as-code, persistence, or runtime behavior is added.
- No real, personal, customer, confidential, regulated, production, or
  sensitive data is used. Synthetic-only transport is not authorized.
- No provider integration, provider egress, OpenAI request, automatic retry,
  fallback, production authentication pattern, or multi-agent runtime is
  authorized.
- D-064's production maximum 15-minute gateway access-token requirement is
  unchanged. D-068's 30-day service-token exception remains demo-only and
  never becomes a production credential pattern.

## Acceptance criteria for a future operational approval

- [ ] The project owner explicitly approves one no-traffic operational
      increment before any Cloudflare console, API, Keychain, DNS, or deployment
      action.
- [ ] The accountable owner and restricted evidence-store location are recorded
      outside this repository before operational work begins.
- [ ] A one-application Access boundary, one-token maximum, 30-day expiry,
      token revocation, and Access-policy scope are evidenced without recording
      identifiers or secrets in the repository.
- [ ] The Worker is proven disabled and incapable of provider egress or any
      provider request; no route or DNS record is made available for demo use.
- [ ] The intended Worker JWT checks are specified as signature, issuer, and
      exact Access-application audience validation, with malformed, missing,
      expired, or mismatched assertions failing closed.
- [ ] A tested owner-led rollback revokes the token and disables the route
      before any later synthetic-transport proposal can be considered.
- [ ] Repository updates contain only sanitized evidence references and pass
      the documentation and security review required by that operational plan.

## Risks and controls

- Accidental external access or provider traffic: keep the Worker disabled,
  configure no provider secret or egress, and treat any request capability as a
  failed no-traffic result.
- Credential or identifier disclosure: keep all Client Secret, Worker secret,
  token, audience value, operational identifier, and evidence-store location
  outside the repository, ordinary CI, logs, screenshots, and chat.
- Authorization drift: bind one token to one Access application, require the
  Worker to validate JWT signature, issuer, and exact audience, and prohibit
  alternate credentials or fallback.
- Production-boundary confusion: state the owner-only fake-data limit and
  preserve D-064's separate 15-minute production requirement in every later
  operational record.
- Incomplete rollback: require token revocation and route disablement as the
  first rollback actions, with no retry or alternate path.

## Verification and rollback

This plan-authoring increment requires documentation formatting, links,
repository-policy, security, whitespace, protected-scope, complete-diff,
session-end, and post-increment-gate verification. No application, native,
provider, Cloudflare, Keychain, DNS, or manual external verification applies
because this increment creates no executable or external artifact.

The future operational increment must include owner-led manual evidence that no
traffic, provider egress, or provider request is possible, and must test token
revocation and route disablement before any synthetic-transport proposal. Its
rollback revokes the service token, disables the Worker route, and removes the
Access/Worker objects only under its separately approved procedure.

## Readiness

Blocked. This documentation plan does not authorize the operational increment.
No external resource or credential may exist until the project owner separately
approves the exact no-traffic operational scope and manual security evidence.
