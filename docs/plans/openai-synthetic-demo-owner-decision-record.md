# OpenAI synthetic-demo owner decision record

Status: Complete documentation-only decision record
Date: 2026-07-20
Prerequisite decision: D-066

## Goal

Record the project owner's decisions that constrain a future synthetic-only
OpenAI demo. This record creates no provider account, credential, request,
external transmission, gateway, or runtime behavior.

## Owner decisions

- Demo content is limited to owner-approved fake synthetic data. User,
  customer, confidential, production, credential, regulated, and sensitive
  personal data are prohibited.
- The demo audience is internal and owner-only.
- Before any future provider request, the approved disclosure states that the
  demo sends synthetic text to OpenAI and prohibits personal, confidential, or
  production information.
- A future implementation must use fixed low request, response-size, timeout,
  and spend limits, plus an owner-controlled gateway disable switch that fails
  closed with no retry or provider fallback. Suspected key exposure also
  requires provider-key revocation or rotation through the provider project.
- The project owner will own the future OpenAI project. Any future API key must
  remain solely in an approved server-side secret path and must never be sent to
  Codex, stored in the repository, desktop, WebView, SQLite, logs, or tests.

## Unchanged blockers

- Exact provider data-control evidence remains required for the intended OpenAI
  account, project, endpoint, model, retention, data use, logging, and
  abuse-monitoring posture.
- A separate project-owner-approved gateway/security implementation plan and
  focused tests remain required before any provider transport proposal.
- No provider account, credential, networking, external transmission, runtime,
  infrastructure, Azure, Terraform, Bicep, or multi-agent work is authorized.

## Acceptance criteria

- [x] The five owner decisions are recorded without exposing a credential.
- [x] The synthetic-only, owner-only, disclosure, limits, and secret boundaries
      are explicit.
- [x] Provider-evidence and implementation blockers remain explicit.
