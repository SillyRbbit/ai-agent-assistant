# Cloudflare demo local security-boundary plan

Status: Complete documentation-only plan; implementation remains Blocked
Date: 2026-07-28
Decision authority: D-064, D-067, and D-068

## Goal

Define the local credential-ingestion and deny-only Worker boundaries required
before any further Cloudflare resource, service token, Keychain item, Worker,
route, DNS record, secret, deployment, provider request, or traffic may be
proposed for the internal owner-only fake-data demo.

This plan adds no source, test, dependency, configuration, credential, Keychain
item, external resource, network path, or runtime behavior.

## Current evidence

- The project owner attested that one Free-plan Cloudflare Zero Trust
  organization exists with Cloudflare's automatic identity provider restricted
  to account members.
- Access applications, policies, service tokens, Workers, routes, DNS changes,
  device enrollments, secrets, provider requests, and traffic remain absent.
- Cortexa has no Keychain adapter, Worker artifact, live gateway transport,
  provider client, credential loader, or external-content path.
- D-064's production gateway access-token maximum remains 15 minutes. D-068's
  maximum 30-day service-token exception remains demo-only.

## Required sequencing

No step authorizes or automatically begins the next:

1. **Documentation:** this plan defines the local boundaries only.
2. **Fake Keychain proof:** a separately approved target-Mac increment may add
   one narrow trusted Rust read adapter and use only fake credentials.
3. **Local deny-only Worker:** a separately approved local-only increment may
   add the Worker artifact and tests without deploying it.
4. **No-traffic provisioning:** only after both local increments pass may a new
   owner-approved plan propose Cloudflare Access, a service token, Keychain
   ingestion, or a Worker deployment.
5. **Synthetic transport:** remains a later, independently approved stage and
   requires all D-061, provider, disclosure, and security evidence.

## Future fake Keychain proof boundary

The first future code increment must use fake values only and remain local to
the target Mac:

- trusted Rust owns the Keychain read and returns no raw credential through
  Tauri IPC, the WebView, logs, errors, debug output, SQLite, tests, or ordinary
  CI;
- fixed non-secret Keychain labels identify one demo token pair under service
  `io.cortexa.demo.cloudflare-access`, with separate `client-id` and
  `client-secret` account labels;
- the project owner creates and removes fake generic-password items directly
  through an approved macOS Keychain procedure; the application adds no secret
  write or import surface;
- fake values prove exact lookup, missing-item closure, access denial,
  cancellation, redacted errors, and cleanup on the target Mac;
- the proof must determine whether the current unsigned development identity
  can enforce stable app-specific Keychain access control. A prompt, unstable
  identity, broad process access, or unverifiable ACL keeps real ingestion
  Blocked; and
- a real service token cannot be generated until a later plan defines the
  owner-operated one-time transfer from Cloudflare directly into Keychain and
  proves that the secret never enters chat, source, files, WebView, SQLite,
  logs, terminal arguments, tests, or CI.

The future adapter must not add a generic secret store, arbitrary service or
account lookup, credential enumeration, delete operation, WebView command, or
network client.

## Future local deny-only Worker boundary

The second future code increment must be local-only and dependency-minimal. Its
proposed exact artifact inventory is:

```text
cloudflare/demo-worker/package.json
cloudflare/demo-worker/src/index.js
cloudflare/demo-worker/test/index.test.js
cloudflare/demo-worker/wrangler.jsonc
```

The future artifact must:

- use Node's built-in test runner and add no runtime or development dependency;
- return one fixed closed denial response without reading or reflecting the
  request body, headers, query, path, or Cloudflare metadata;
- perform no outbound `fetch`, provider call, retry, fallback, background work,
  logging, analytics, content capture, or persistence;
- define `workers_dev: false`, `preview_urls: false`, `observability.enabled:
false`, no routes, and no custom domains;
- define no secret, variable, KV, R2, D1, Durable Object, Queue, service,
  Hyperdrive, AI, Vectorize, Browser, or other binding;
- remain undeployed and make no Wrangler login, API, account, or provider
  request; and
- fail verification if its source or configuration gains a route, binding,
  credential, outbound call, request reflection, or externally reachable URL.

Access JWT signature, issuer, and exact-audience validation remains required by
D-068 for a later authenticated Worker. The deny-only local artifact does not
implement or simulate that later authentication boundary; it rejects every
request before provider handling.

## Risks and controls

- **Credential leakage:** use fake values until the target-Mac read path and
  one-time owner procedure are separately approved; raw values never cross IPC
  or enter repository evidence.
- **Unstable Keychain authorization:** treat unsigned-app identity or broad ACL
  behavior as a failed proof requiring an additive decision, not a fallback.
- **Accidental public Worker exposure:** require persistent no-route,
  `workers_dev: false`, and preview-disabled configuration before any future
  deployment proposal.
- **Provider egress or content logging:** keep the local handler deny-only, add
  no secrets or bindings, disable observability, and reject any outbound call or
  request reflection in review.
- **Boundary conflation:** keep the Rust Keychain proof, local Worker artifact,
  external provisioning, and synthetic transport as separate approval-bound
  increments.

## Future verification and manual evidence

The future fake-Keychain increment must name exact Rust source and test files,
run focused and complete Rust/repository checks, and include owner-held target-
Mac evidence for fake item creation, exact read behavior, denial, cleanup, and
ACL scope. Repository records contain only sanitized outcomes.

The future Worker increment must run focused Node tests, configuration
assertions, dependency and protected-path review, complete repository checks,
and static review for routes, bindings, secrets, logs, request reflection, and
outbound calls. No Cloudflare dashboard or API check applies while it remains
local and undeployed.

## Rollback

Rollback for this plan removes only its additive documentation and restores the
five project-memory edits. A future fake-Keychain increment removes only its
fake items and bounded read adapter under its approved procedure. A future
Worker increment removes only its undeployed local artifact. No real credential
or Cloudflare object exists to revoke or delete under this plan.

## Non-goals

- No code, tests, dependencies, permissions, capabilities, IPC, UI, Keychain
  item, credential, token, Worker, Access application, policy, route, DNS
  record, secret, deployment, provider request, traffic, or external evidence.
- No OpenAI key, provider client, model selection, synthetic transport, real
  data, tools, persistence, retry, fallback, or multi-agent runtime.
- No change to the current production identity or hosting design and no change
  to D-064's 15-minute production or D-068's 30-day demo-only limits.

## Acceptance criteria

- [x] The fake-credential-only Keychain proof is separate from real ingestion.
- [x] Trusted Rust owns future reads without a generic secret or WebView path.
- [x] The local Worker artifact is persistently route-free, deny-only,
      dependency-minimal, and incapable of provider egress.
- [x] Local code, external provisioning, and transport remain separately
      approval-bound.
- [x] Risks, verification, manual evidence, rollback, and non-goals are exact.

## Readiness

Blocked. This plan authorizes no implementation or external action. The future
fake-Keychain proof and local deny-only Worker artifact each require a separate
exact plan, project-owner approval, focused verification, security review, and
valid completion marker.
