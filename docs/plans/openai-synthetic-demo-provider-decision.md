# OpenAI synthetic-demo provider decision

Status: Complete documentation-only decision
Date: 2026-07-20

## Goal

Replace the unpublished Azure Stage B direction with one disposable, synthetic-data-only OpenAI demo path. This records the conditions for a later bounded implementation; it creates no API request, account, credential, gateway, or runtime behavior.

## Scope

- OpenAI is the sole candidate for a future synthetic demo.
- A future trusted gateway, not the WebView or desktop, owns any API credential.
- Only owner-approved synthetic text may cross the boundary after a separate implementation approval and disclosure.

## Non-goals

- No OpenAI API key, account setup, networking, provider request, model selection, production traffic, user content, tools, persistence, fallback, or multi-provider routing.
- No Azure provisioning; the former Stage B plan is superseded before publication.

## Acceptance criteria

- [x] Current-state records identify OpenAI as the synthetic-demo candidate and Azure Stage B as superseded.
- [x] A later implementation plan requires a trusted gateway, server-side credential handling, fixed limits, disclosure, redacted errors, and synthetic-only tests.
- [x] Direct desktop/WebView credentials and automatic provider fallback remain prohibited.

## Readiness

Implementation remains Blocked after the completed documentation-only security
plans. A separately approved no-traffic Cloudflare deployment plan, exact
runtime source/test scope, manual security gate, rollback evidence, and a later
runtime implementation approval are still required. No credential may enter
the repository or chat.
