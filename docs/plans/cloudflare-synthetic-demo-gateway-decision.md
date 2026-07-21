# Cloudflare synthetic-demo gateway decision

Status: Complete documentation-only deployment decision
Date: 2026-07-20

## Goal

Select Cloudflare Workers Free as the remote trusted gateway target for the
internal, owner-only OpenAI synthetic demo. This decision creates no Worker,
DNS route, secret, credential, deployment, provider request, or runtime path.

## Approved boundary

- A future Cloudflare Worker is the sole demo gateway candidate.
- The OpenAI project remains `cortexa-internal-synthetic-demo`, restricted to
  `gpt-5.6-luna`, 20,000 TPM, 2 RPM, a $10 spend limit with $5 and $10 alerts,
  and disabled API-call logging. The repository records no project identifier.
- The future OpenAI API key is stored only as a Cloudflare Worker secret. It
  never enters source, Git, Tauri, trusted Rust, the WebView, SQLite, tests,
  logs, screenshots committed to the repository, or chat.
- Future requests use only `/v1/responses`, owner-approved synthetic text,
  `store: false`, foreground mode, no files, images, hosted tools, web search,
  persistence, automatic retry, or provider fallback.
- The Worker must fail closed when disabled, enforce limits below the provider
  project, redact failures, and log no prompt or response content.

## Unresolved before implementation

- D-068 separately permits one demo-only, 30-day-maximum Cloudflare Access
  service-token design with Keychain and trusted-Rust ownership. It creates no
  token, Keychain item, Access application, or runtime credential path and does
  not authorize any production authentication pattern.
- The exact Worker route, domain, deployment method, logging configuration,
  secret-rotation procedure, rollback, and manual security checks require a
  separate plan and project-owner approval.

## Non-goals

- No production-hosting change. Azure Container Apps remains historical planned
  production architecture; Cloudflare is selected only for this internal demo.
- No account mutation, Worker, DNS, secret, Keychain, source, dependency,
  infrastructure-as-code, deployment, networking, provider traffic, UI, IPC,
  persistence, identity, or multi-agent runtime behavior.

## Acceptance criteria

- [x] Cloudflare Workers Free is selected only for the internal synthetic demo.
- [x] Provider, secret, data, logging, limit, and no-fallback boundaries are explicit.
- [x] D-068 isolates the demo client-authentication decision, while token
      creation and deployment remain separately blocked.
- [x] No secret or non-public project identifier is recorded.

## Readiness

Blocked. A separately approved no-traffic Access and Worker deployment plan is
required before any external resource or runtime code.
