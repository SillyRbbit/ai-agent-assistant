# OpenAI synthetic-demo runtime implementation plan

Status: Complete documentation-only runtime plan
Date: 2026-07-20

## Goal

Define the smallest future code increment for a trusted, synthetic-only OpenAI
gateway path after separate secret/deployment approval. This plan adds no code,
credential, provider request, or external transmission.

## Future exact scope

- Add a gateway-owned configuration interface in trusted Rust only; the WebView
  never receives a provider URL, model, or credential.
- Add a closed synthetic-request validator and redacted result/error contract.
- Add one gateway client boundary that rejects disabled state, non-synthetic
  content, tools, files, images, unknown fields, oversized requests, timeout,
  cancellation, and provider errors without fallback.
- Add focused Rust tests for every rejection, fixed limits, `store: false`,
  no background mode, no tool path, redaction, and kill switch.

## Required decisions before code

- Project owner approves an exact server-side secret-store and gateway-deployment
  design; no secret enters this repository, desktop, WebView, test, or log.
- Project owner approves exact source/test file inventory, manual target
  verification, rollback/key revocation, and disable-switch operation.
- No API request occurs until implementation and its manual security gate pass.

## Non-goals

No key creation, deployment, infrastructure, provider traffic, user data,
tools, persistence, identity, UI, IPC expansion, Azure, or multi-agent runtime.

## Acceptance criteria

- [x] Future trusted code boundary, tests, and rejection behavior are defined.
- [x] Secret/deployment and manual gates remain separate owner decisions.
- [x] No runtime behavior is authorized by this plan.
