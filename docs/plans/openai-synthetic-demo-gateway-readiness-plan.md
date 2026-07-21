# OpenAI synthetic-demo gateway readiness plan

Status: Complete documentation-only readiness plan
Date: 2026-07-20
Prerequisite decision: D-066

## Goal

Define the exact evidence, owner approvals, implementation boundaries, and
rollback conditions required before proposing a future synthetic-only OpenAI
demo gateway increment. This plan creates no account, credential, endpoint,
provider request, external transmission, gateway, or runtime behavior.

## Scope

- Require evidence for the exact future OpenAI account, project, endpoint, and
  data controls before a provider transport plan may be proposed.
- Require an owner-approved, disposable synthetic corpus with no personal,
  customer, credential, regulated, or production content.
- Require a pre-transmission disclosure and explicit acknowledgement design.
- Require a future trusted gateway to own any secret through an approved
  server-side secret path; desktop, WebView, source, logs, tests, and SQLite
  must never receive it.
- Require fixed request, response, time, rate, and cost limits; closed redacted
  errors; synthetic-only tests; and an owner-approved rollback or kill-switch.

## Non-goals

- No OpenAI account, API key, project, endpoint, model selection, provider
  request, network path, external transmission, or traffic.
- No gateway, desktop, WebView, Rust, IPC, UI, persistence, deployment,
  infrastructure, Azure provisioning, Terraform, Bicep, or release work.
- No production or user content, tools, hosted tools, automatic fallback,
  multi-provider routing, or multi-agent runtime behavior.

## Required evidence before a future implementation proposal

1. Exact provider data-control evidence for the intended account, project,
   endpoint, model, region, retention, abuse-monitoring treatment, data use,
   and logging configuration; unresolved or unavailable evidence fails closed.
2. Written owner approval of the bounded synthetic corpus and its source,
   classification, disposal method, and prohibition on real-user content.
3. Approved disclosure requirements covering the provider, synthetic-only data
   class, retention/data-use posture, and acknowledgement before transmission.
4. An approved secret-handling design proving a trusted gateway owns the
   credential and that no untrusted client, source file, ordinary log, test, or
   database can receive it.
5. A bounded security and test plan covering authorization, data classification,
   request/response size, timeout, rate, cost, cancellation, redacted-error,
   provider-failure, and kill-switch cases.
6. An owner-approved rollback plan that disables transport, revokes the
   provider credential, prevents retries or fallback, and records only
   permitted operational evidence.

## Future implementation-entry criteria

A future implementation increment may be proposed only after every required
evidence item above is available, reviewed, and explicitly approved by the
project owner. Its plan must name exact source and test files, preserve trusted
Rust and gateway ownership, exclude direct desktop/WebView credentials, forbid
automatic provider fallback, and include focused validation plus manual
security evidence. This readiness plan does not itself make that increment
Ready.

## Acceptance criteria

- [x] The plan names the exact provider, corpus, disclosure, secret, limits,
      redaction, test, and rollback evidence required before implementation.
- [x] The plan preserves D-066's synthetic-only, trusted-gateway, and no-fallback
      boundaries.
- [x] The plan explicitly prohibits all account, credential, networking,
      runtime, infrastructure, and multi-agent work.
- [x] Current project-memory records identify this plan as documentation-only
      and keep implementation Blocked.

## Readiness

Blocked. Do not propose or start provider implementation until the required
evidence and separate project-owner approval exist.
