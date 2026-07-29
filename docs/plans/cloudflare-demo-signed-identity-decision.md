# Cloudflare demo signed macOS identity decision plan

Status: Complete documentation-only decision; implementation remains Blocked
Date: 2026-07-28
Decision authority: D-064, D-068, D-069, D-070, and D-071

## Goal

Record the owner-selected stable signed macOS application identity as the only
future control model for the Cloudflare demo credential boundary. Define the
future signing provenance, least-privilege Keychain, secret-memory, lifecycle,
rollback, and private target-Mac evidence requirements without creating or
configuring any control.

## Decision

The future boundary must use one owner-controlled signed macOS application
identity. The previously considered narrow alternative Keychain ACL is not the
selected model. An unsigned executable and an interactive login-keychain prompt
are never an approved fallback.

## Required future implementation evidence

Before an implementation may be approved, a separate exact plan must define:

- accountable signing authority, application identifier, reproducible build and
  signing provenance, certificate/profile/entitlement ownership, renewal,
  revocation, and incident response, all held outside this repository;
- the minimal Keychain access group and exactly fixed credential labels, with no
  arbitrary lookup, write, enumeration, or broad process access;
- a bounded private secret owner with one-time consumption, no clone,
  serialization, debug, display, log, IPC, WebView, SQLite, test-fixture, or
  generic accessor path; practical overwrite and zeroization limitations must
  be explicit rather than claimed away;
- direct owner-only transfer from Cloudflare to Keychain without chat, source,
  files, shell arguments, terminal output, screenshots, WebView, IPC, SQLite,
  logs, tests, CI, or repository evidence;
- D-068's one-token, 30-day demo duration, inventory-free confirmation,
  rotation, expiry, immediate provider revocation, local removal, route-disable
  conditions, and fail-closed read errors; and
- renewed dependency maintenance, advisory, license, toolchain, and native-build
  review before real secret handling.

## Private target-Mac evidence

A later owner-operated implementation increment must require private,
sanitized evidence for stable signed-identity access across restart,
rejection for unsigned/non-authorized executables, missing/denied/cancelled/
malformed/expired/revoked outcomes, rotation and removal, and signed update or
reinstall behavior. Repository evidence may contain only sanitized pass/fail
outcomes and no signing identifiers, certificates, secret values, screenshots,
or Keychain material.

## Risks and controls

- **Unauthorized signing scope:** documentation selects a future model only;
  certificate, entitlement, profile, signing, and notarization remain absent.
- **Broad Keychain access:** future access is bound to one signed identity and
  fixed labels; unsigned fallback and arbitrary process trust are prohibited.
- **Secret leakage:** future ownership is bounded and one-time; raw values are
  excluded from all public, persisted, diagnostic, and repository paths.
- **Lifecycle failure:** the future plan must prove rotation, revocation,
  removal, expiration, incident response, and route disablement.
- **Boundary conflation:** signed identity does not authorize a real token,
  Access application, Worker, route, DNS record, deployment, traffic, or
  provider request.

## Verification

This documentation-only increment requires `npm run format`, `npm run
docs:check`, `npm run repository:check`, `npm run security:scan`, `git diff
--check`, and `python3 .codex/hooks/session_end_gate.py`. It performs no manual
signing, Keychain, credential, Cloudflare, provider, or network action.

## Rollback

Before commit, revert only this plan and its nine approved documentation
records. No external state, signing asset, Keychain item, credential, or runtime
behavior exists to revoke or remove.

## Non-goals

- No code, dependency, certificate, signing, notarization, entitlement,
  provisioning profile, Keychain action, credential, service token, Access
  application, policy, Worker, route, DNS record, secret, deployment, provider
  request, traffic, IPC, WebView, storage, or UI.
- No change to D-064's 15-minute production requirement or D-068's 30-day
  demo-only exception.

## Acceptance criteria

- [x] Stable signed macOS identity is the sole selected future model.
- [x] Future signing, Keychain, secret-memory, lifecycle, and private-evidence
      gates are exact and separately approval-bound.
- [x] No external or runtime capability is created.

## Readiness

Blocked. A signed identity selection is not signing authority or implementation
approval. A later exact implementation increment requires separate owner
approval.
