# Cloudflare demo macOS identity and secret-memory boundary plan

Status: Complete documentation-only plan; selection and implementation remain Blocked
Date: 2026-07-28
Decision authority: D-064, D-068, D-069, and D-070

## Goal

Define the decision criteria, security controls, target-Mac evidence, lifecycle,
and rollback requirements for a future stable macOS application identity or
narrow Keychain ACL and production secret-memory boundary. This plan does not
select, create, configure, or test either control.

## Current evidence

- The fake-only Keychain proof used only two fake items and returned status
  only; both items were removed.
- The unsigned development executable required repeated login-keychain prompts,
  so it did not prove stable app-specific access.
- No real credential, signing asset, certificate, entitlement, provisioning
  profile, Access application, service token, Worker, route, DNS record,
  secret, provider request, traffic, deployment, or runtime consumer exists.
- D-064's production access-token maximum remains 15 minutes. D-068's narrow
  fake-data demo exception remains one token for at most 30 days.

## Required future decision record

A separate owner-approved decision must select exactly one documented control
model before implementation:

1. **Signed application identity:** define accountable owner, signing authority,
   reproducible identity provenance, least-privilege Keychain access scope,
   update/reinstall behavior, revocation, and failure closure. It must not
   create a certificate, entitlement, profile, or signing configuration during
   decision work.
2. **Narrow alternative ACL:** define the exact app-specific principal,
   allowed operation and fixed labels, denial behavior, persistence semantics,
   revocation, update/reinstall behavior, and proof that arbitrary local
   processes cannot access the item. Broad login-keychain prompts or process
   trust are insufficient.

The record must reject a model if it cannot provide stable, least-privilege,
owner-controlled access without raw-secret exposure. It may not silently choose
an unsigned fallback.

## Future secret-memory boundary

Before real bytes exist, a later approved implementation plan must define:

- a bounded private secret type with one owner and no `Clone`, serialization,
  debug, display, logging, IPC, WebView, SQLite, test fixture, or generic
  accessor path;
- exact one-time transfer and consumption semantics, including cancellation,
  missing-item, denial, malformed-value, expiry, rotation, and post-consumption
  failure behavior;
- explicit handling limits for Rust, compiler optimization, copies, OS memory,
  crash reporting, and best-effort overwrite. No design may claim guaranteed
  zeroization without evidence adequate to that claim;
- redacted closed errors and an inventory-free status boundary; and
- dependency, licensing, maintenance, advisory, native-build, and toolchain
  review before any new or continued secret-handling dependency is accepted.

## Owner-only transfer and lifecycle

Any later real-token increment must define a private owner-run procedure that
places a value directly from Cloudflare into Keychain. It must exclude chat,
source, files, shell arguments, terminal output, screenshots, WebView, IPC,
SQLite, logs, tests, CI, and repository evidence. The procedure must define one
accountable owner, maximum D-068 duration, inventory-free confirmation,
rotation, expiry, immediate Cloudflare-side revocation, local removal, incident
handling, route-disable conditions, and a fail-closed result for every read.

## Required private target-Mac evidence

The later implementation plan must require owner-held, sanitized evidence for:

1. stable access by the selected identity/ACL across an application restart;
2. rejection for an unsigned or otherwise non-authorized executable;
3. missing-item, denial/cancellation, malformed-value, and expired/revoked
   outcomes without raw-value or native-error leakage;
4. rotation and revocation followed by local removal and re-observed missing
   behavior; and
5. update/reinstall and failure behavior appropriate to the selected model.

Evidence remains private. Repository records may contain only passed/failed
sanitized outcomes, never identifiers, passwords, values, screenshots, or
Keychain database material.

## Risks and controls

- **Broad access:** require one chosen, app-specific, least-privilege model and
  reject interactive unsigned fallback behavior.
- **Secret leakage:** prohibit raw-value paths, private evidence only, and use
  closed redacted outcomes.
- **False memory guarantees:** record practical limits and require dedicated
  review rather than equating overwrite with zeroization.
- **Lifecycle failure:** require owner, duration, rotation, revocation, local
  removal, incident, and disablement controls before any real token action.
- **Boundary conflation:** keep design, implementation, no-traffic provisioning,
  synthetic transport, and real-content activation independently approved.

## Verification

This documentation-only increment requires `npm run format`, `npm run
docs:check`, `npm run repository:check`, `npm run security:scan`, `git diff
--check`, and `python3 .codex/hooks/session_end_gate.py`. It performs no manual
target-Mac, Keychain, signing, Cloudflare, provider, or network action.

## Rollback

Before commit, revert only this plan and its nine approved documentation
records. No signing asset, Keychain item, credential, external resource, or
runtime behavior exists to revoke or remove.

## Non-goals

- No code, dependency, certificate, signing, notarization, entitlement,
  provisioning profile, application identity, Keychain action, credential,
  service token, Access application, policy, Worker, route, DNS record, secret,
  deployment, provider request, traffic, IPC, WebView, storage, or UI.
- No change to D-064's 15-minute production requirement or D-068's 30-day
  demo-only exception.

## Acceptance criteria

- [x] The two possible macOS controls have exact selection and rejection
      criteria without choosing either one.
- [x] Secret-memory, owner transfer, lifecycle, rollback, and private evidence
      controls are explicit before real credential work may be proposed.
- [x] All external actions and implementation remain separately approval-bound.

## Readiness

Blocked. This plan grants no authority to sign, configure macOS access controls,
handle a real credential, or create a Cloudflare resource. A later exact
decision or implementation increment requires separate owner approval.
