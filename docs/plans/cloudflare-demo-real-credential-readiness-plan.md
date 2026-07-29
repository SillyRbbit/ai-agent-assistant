# Cloudflare demo real-credential readiness plan

Status: Complete documentation-only plan; real ingestion remains Blocked
Date: 2026-07-28
Decision authority: D-064, D-068, and D-069

## Goal

Define the exact security, ownership, manual-evidence, rollback, and approval
prerequisites that must pass before a future increment may propose ingestion of
one real Cloudflare Access demo service-token pair. This is a planning record
only. It neither approves nor begins a real-credential implementation.

## Current evidence and boundary

- The owner-only fake-data demo has a configured Free-plan Zero Trust
  organization, restricted to account members, with zero Access applications,
  service tokens, Workers, routes, DNS changes, secrets, deployments, provider
  requests, or traffic.
- The verified fake-only macOS proof reads two fixed labels and returns status
  only. It used fake values, and both fake Keychain items were removed.
- Repeated authorization prompts did not establish stable app-specific access
  for the unsigned development executable.
- D-064's production gateway-access-token maximum remains 15 minutes. D-068's
  one-token demo exception remains fake-data-only and has a maximum 30-day
  duration; it does not authorize a real token or alter production policy.

## Required future evidence

No future real-ingestion implementation is Ready until a separately approved
security design and target-Mac evidence establish every item below.

1. **Stable application identity or narrow ACL:** choose and document either a
   signed, owner-controlled macOS application identity with a reproducible
   Keychain access-group/ACL design, or a narrowly reviewed alternative
   app-specific access-control design. The design must define identity
   ownership, entitlement/profile provenance if applicable, access scope,
   denial/revocation behavior, update/reinstall behavior, and how broad process
   access is prevented. An unsigned prompt is not evidence of stable access.
2. **Secret-memory lifecycle:** define bounded byte handling, ownership,
   one-time consumption, redaction, error/debug behavior, drop/zeroization
   limits, crash-report exclusion, and failure handling. It must not claim that
   best-effort buffer overwrite is a production zeroization guarantee.
3. **Owner-only transfer:** define an owner-operated, one-time handoff from
   Cloudflare directly into Keychain that never passes a secret through chat,
   source, files, shell arguments, terminal output, WebView, IPC, SQLite, logs,
   tests, CI, screenshots, or repository evidence. The procedure itself must
   remain private and contain no token value.
4. **Lifecycle and rollback:** define a single accountable owner, maximum
   30-day D-068 duration, inventory-free confirmation, expiration handling,
   rotation, immediate Cloudflare-side revocation, local removal, failed-read
   closure, incident response, and the conditions requiring route disablement.
5. **Dependency reassessment:** reassess exact package maintenance, licenses,
   advisories, Rust/toolchain compatibility, native build behavior, and viable
   alternatives because the fake-proof wrapper declares `looking-for-maintainer`.
6. **Manual target-Mac evidence:** use only private owner-held evidence to
   demonstrate stable access, missing-item behavior, denial/cancellation,
   revocation, cleanup, and no raw-value exposure. Repository records may state
   only sanitized outcomes.

## Required approval sequence

Each transition is independently project-owner-approved; completion of one does
not authorize the next:

1. This documentation-only plan.
2. A separate design/decision increment, if needed, for the chosen signed
   identity or alternative ACL and production secret-memory model.
3. A separate implementation increment with exact code, dependency, test,
   manual-evidence, and rollback scope. It still may not create a Cloudflare
   token or other external resource unless that increment explicitly receives
   separate external-action approval.
4. A separately approved no-traffic operational increment, if and only if the
   prior local evidence passes, for the exact Cloudflare Access/Worker/token
   boundary defined by the no-traffic deployment plan.
5. A separately approved synthetic-transport increment; real content remains a
   later independent decision and is outside this plan.

## Risks and controls

- **Credential exposure:** no secret is created, received, copied, or recorded
  by this plan; future transfer is owner-operated and direct to Keychain.
- **Unstable unsigned access:** repeated prompts are a blocker, not an approval
  or fallback. Stable identity/ACL evidence is mandatory.
- **Misleading memory assurances:** future code must state platform and language
  limits honestly and undergo dedicated security review before real bytes exist.
- **Lifecycle gaps:** rotation, revocation, expiry, removal, incident handling,
  and disablement are required before any external credential action.
- **Boundary conflation:** planning, local code, external provisioning,
  synthetic transport, and real-content activation retain separate approvals.

## Verification and manual evidence

This documentation increment verifies only documentation integrity and policy
consistency: `npm run format`, `npm run docs:check`,
`npm run repository:check`, `npm run security:scan`, `git diff --check`, and
`python3 .codex/hooks/session_end_gate.py`.

It has no Keychain, Cloudflare, provider, network, native-UI, credential, or
manual operational verification. Future credential work must define its own
automated and target-Mac manual evidence before implementation approval.

## Rollback

Before commit, revert only this plan and its nine approved documentation
records. No external state, Keychain item, credential, code, dependency, or
deployment exists to revoke, remove, or roll back.

## Non-goals

- No code, dependency, Keychain read/write, credential, service token, Access
  application, policy, Worker, route, DNS record, secret, deployment, provider
  request, traffic, runtime wiring, IPC, WebView, storage, or UI.
- No signing, notarization, entitlement, profile, certificate, application
  identity, provider configuration, or external resource action.
- No change to D-064's production 15-minute requirement or D-068's 30-day
  demo-only exception.

## Acceptance criteria

- [x] Stable identity/ACL, secret-memory, ownership, lifecycle, and dependency
      gates are explicit before any real credential may be proposed.
- [x] Manual evidence is private, sanitized, and target-Mac-specific.
- [x] Every external and runtime action remains separately approval-bound.
- [x] D-064 and D-068 remain unchanged.

## Readiness

Blocked. This plan authorizes no real credential, Cloudflare action, runtime
implementation, or deployment. A future exact design or implementation
increment requires separate owner approval.
