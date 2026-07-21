# Phase 4 Stage B no-traffic provisioning plan

Status: Superseded before publication by the OpenAI synthetic-demo provider decision; no Azure resource was created
Owner: Project owner
Date: 2026-07-20
Decision authority: D-060 through D-064

## Goal

Create the approved Microsoft personal desktop and gateway API registrations and
the closed Azure gateway resources with every traffic path disabled, then record
only sanitized ownership, configuration, and rollback evidence in the restricted
owner-approved evidence store. The outcome is evidence sufficient to decide
whether Stage C synthetic-only transport may be planned; it does not enable any
authentication, gateway, provider, or user-content traffic.

## Scope

- Provision the two separately owned Microsoft personal registrations with the
  fixed `gateway.access` API boundary and sanitized manifest evidence.
- Provision the single Central US Azure Container Apps gateway boundary, its
  dedicated user-assigned managed identity, the exact-resource Azure OpenAI
  role assignment, private endpoint/private DNS, and disabled Azure OpenAI
  public access.
- Keep gateway ingress disabled or otherwise incapable of accepting traffic;
  create no desktop client, token exchange, provider deployment, or provider
  request path.
- Store operational identifiers, manifests, role assignments, ownership, and
  rollback evidence only in the restricted evidence store; commit only
  sanitized evidence references and results.

## Explicit non-goals

- No application source, dependency, Tauri, IPC, credential, Keychain,
  browser, loopback listener, OAuth/OIDC, token, gateway transport, provider
  call, model deployment, DNS cutover, certificate activation, disclosure UI,
  user-content transmission, or runtime behavior.
- No Stage C or Stage D work, including synthetic tests, real-content
  activation, `ContentLogging=false` claims, ZDR claims, or provider traffic.
- No secrets, tokens, actual registration/resource IDs, endpoints, certificates,
  or evidence-store locations in the repository or ordinary CI.

## Acceptance criteria

- [ ] A project owner approves this operational increment before any external
      provisioning command or console action.
- [ ] The restricted evidence store and responsible owners are identified
      outside this repository before provisioning.
- [ ] Sanitized evidence proves two registrations, the exact API URI pattern,
      `gateway.access`, public-client/credential absence, excluded scopes, and
      the manifest representation of `http://127.0.0.1:<ephemeral>/oauth/callback`.
- [ ] Sanitized evidence proves the intended Central US gateway boundary,
      dedicated managed identity, exact-resource inference role, private
      endpoint/private DNS, and disabled Azure OpenAI public network access.
- [ ] No gateway ingress, identity flow, provider request, or model traffic is
      possible; an owner records the no-traffic verification.
- [ ] The intended method for the maximum 15-minute Microsoft personal gateway
      token is evidenced, or the result remains a documented Stage C blocker
      requiring an additive decision.
- [ ] Every created object has a tested owner-led rollback/deletion procedure.
- [ ] Repository updates contain only sanitized references; documentation
      validation and the applicable operational evidence review pass.

## Risks and controls

- Accidental traffic or public exposure: keep ingress disabled, Azure OpenAI
  public access disabled, and do not create client/runtime paths.
- Credential or identifier disclosure: use the restricted evidence store and
  secret scans; repository text may contain only sanitized references.
- Broad Azure authority: use one non-shared managed identity and the exact
  Azure OpenAI resource role only; reject broader scopes or API-key fallback.
- Unsupported callback or token lifetime: treat either mismatch as a hard
  Stage C blocker; do not substitute `localhost`, fixed ports, refresh tokens,
  or longer-lived defaults.

## Verification and rollback

Before executing Stage B, obtain separate owner approval and record the exact
operational runbook, responsible owner, evidence-store references, resource
inventory, and rollback order. Validate manifests, role scopes, network state,
disabled ingress, and the absence of traffic without placing operational values
in the repository. Rollback disables/deletes Stage B resources and revokes role
assignments; it never preserves authority for Stage C or Stage D.

## Readiness boundary

This plan does not make Stage C Ready. Stage C remains blocked until Stage B
evidence proves compatible token-lifetime and callback boundaries and a separate
implementation, disclosure, synthetic-corpus, and security-test increment is
approved.
