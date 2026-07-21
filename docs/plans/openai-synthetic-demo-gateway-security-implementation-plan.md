# OpenAI synthetic-demo gateway security implementation plan

Status: Complete documentation-only implementation plan
Date: 2026-07-20
Prerequisites: D-066; owner decision record; configured internal demo project

## Goal

Define one future, bounded implementation increment for an internal,
synthetic-only OpenAI demo gateway. This plan creates no API key, provider
request, network path, external transmission, or runtime behavior.

## Required future implementation scope

- One trusted gateway boundary owns a project-scoped secret from an approved
  server-side secret store; Rust, desktop, WebView, SQLite, source, tests, and
  logs never receive it.
- Accept only an allowlisted text request containing owner-approved synthetic
  data; reject files, images, tools, web search, background mode, persistence,
  user content, unknown fields, and oversize input.
- Call only `/v1/responses` with `gpt-5.6-luna`, `store: false`, no background
  mode, no tools, bounded output, timeout, and no automatic retry or fallback.
- Enforce lower gateway limits than the provider project: one in-flight request,
  fixed input/output bounds, timeout, daily usage budget, and an owner-controlled
  fail-closed disable switch.
- Return only closed, redacted error codes and opaque correlation IDs.
- Add focused tests for synthetic-data admission, rejection, limits, timeout,
  cancellation, disabled state, redaction, and no-fallback behavior.

## Non-goals

- No direct desktop/WebView provider call, user content, production traffic,
  tools, persistence, identity, Azure, Terraform, Bicep, deployment, or
  multi-agent runtime behavior.

## Entry criteria

- Project owner supplies the future secret through an approved server-side
  secret path; it is never recorded in this repository or chat.
- The exact gateway deployment/secret-store boundary and manual security gate
  are separately approved before code begins.
- A future implementation plan names exact source/test files, validates the
  target environment, includes rollback/key-revocation evidence, and receives
  project-owner approval.

## Acceptance criteria

- [x] Exact trusted-boundary, request, provider, limits, redaction, and test
      requirements are defined.
- [x] Current provider settings and owner decisions are reflected without
      exposing a credential.
- [x] Implementation remains Blocked pending the separate approved runtime
      increment and secret/deployment decision.
