# Cloudflare Access service-token demo exception

Status: Complete documentation-only security decision
Date: 2026-07-20

## Goal

Record one narrow authentication exception for the internal, owner-only
Cloudflare synthetic demo. This decision creates no service token, Keychain
item, Access application, Worker, route, DNS record, secret, or network path.

## Approved exception

- One Cloudflare Access service token may authenticate the future internal demo
  client to one Cloudflare Access application.
- The service-token duration is at most 30 days. It must be revoked immediately
  after the demo, on suspected exposure, on owner request, or before changing
  the protected application.
- The Client Secret exists only in macOS Keychain and is readable only by
  trusted Rust. The Client ID is treated as internal configuration. Neither may
  enter the WebView, SQLite, source, Git, logs, tests, screenshots committed to
  the repository, or chat.
- Trusted Rust sends the standard Cloudflare Access service-token headers only
  to the one fixed HTTPS gateway origin. The WebView cannot choose or read the
  origin or headers.
- Cloudflare Access uses a `Service Auth` policy restricted to the one token and
  one application. The future Worker independently validates the
  `Cf-Access-Jwt-Assertion` signature, issuer, and exact application audience;
  missing, malformed, expired, or mismatched evidence fails closed.
- Revocation of the service token and disabling the Worker route are the
  immediate rollback controls. There is no retry, alternate credential,
  bypass, or provider fallback.

## Production boundary

This is a demo-only exception for fake synthetic data and an owner-only client.
It does not change D-064's maximum 15-minute production gateway access-token
requirement, authorize persistent production sessions, or establish a general
desktop authentication pattern. Production identity remains separately blocked.

## Non-goals

No token generation, Keychain integration, Access application, Worker, DNS,
route, secret, deployment, provider request, real data, production use,
identity implementation, UI, IPC, persistence, or multi-agent runtime behavior.

## Acceptance criteria

- [x] The 30-day maximum, Keychain ownership, trusted-Rust boundary, one-app
      policy, JWT validation, and immediate revocation are explicit.
- [x] The exception is limited to owner-only fake-data demo use.
- [x] The production 15-minute requirement remains unchanged.
- [x] No credential or private identifier is recorded.

## Readiness

Blocked. A separately approved no-traffic Cloudflare Access and Worker
deployment plan is required before any external resource or credential exists.
