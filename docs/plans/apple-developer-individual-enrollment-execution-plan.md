# Apple Developer individual enrollment execution plan

- Status: Documentation-only execution plan; enrollment remains unauthorized
- Date: 2026-07-30
- Decision authority: D-064, D-068, D-070, D-072, D-073, and D-074

## Goal

Define the future owner-operated enrollment procedure if the owner later
separately approves Apple Developer Program enrollment as an individual for the
personally owned, owner-only fake-demo scope selected conditionally by D-074.

This plan does not authorize enrollment, payment, agreement acceptance, account
access, certificate creation, signing, Keychain activity, credential handling,
Cloudflare activity, provider traffic, deployment, or runtime behavior.

## Preconditions and explicit owner gates

Before any future enrollment action, the owner must separately confirm all of
the following outside the repository:

1. Cortexa remains personally owned by one individual or sole proprietor; no
   legal entity should instead own the Apple agreement, seller identity,
   certificates, or future team access.
2. The owner understands that Apple may display the individual's legal name as
   the App Store seller name and that organization enrollment must be reconsidered
   if this is not acceptable.
3. The owner accepts the then-current Apple membership price, agreement, tax,
   privacy, and regional terms after reviewing them privately.
4. The owner accepts that membership purchase and agreement acceptance are
   external commitments, not reversible engineering steps; no refund,
   cancellation, transfer, or support outcome is assumed by this plan.
5. D-074 still applies, and the owner has explicitly approved a separate
   enrollment increment with its exact operational scope.

If any condition is false, unclear, or changes during enrollment, stop. Do not
use this plan to choose organization enrollment, change account ownership, or
resolve ambiguity through support contact.

## Future operational procedure

The following procedure is intentionally future-facing. It must not be executed
under this documentation increment.

1. Reconcile D-074, this plan, the current working tree, and the current Apple
   enrollment guidance. Confirm individual enrollment is still the owner-selected
   model and that no company-ownership trigger has appeared.
2. Open a separately approved enrollment increment and obtain project-owner
   approval for the exact future action. The approval must name individual
   enrollment only and must exclude all signing assets, certificates, profiles,
   Keychain actions, credentials, Cloudflare, provider, traffic, deployment,
   and runtime work.
3. The owner uses an owner-controlled Apple Account with two-factor
   authentication and private browser session. Authentication data, legal name,
   contact details, payment data, membership identifiers, and browser artifacts
   remain private and never enter the repository, chat, terminal, logs, or
   ordinary notes.
4. The owner reviews Apple’s then-current individual enrollment requirements,
   legal-name display, membership price, agreement, and privacy information.
   Stop before submission if any condition conflicts with the explicit owner
   gates or the future increment’s approved scope.
5. Only after the owner confirms the final review and explicitly authorizes the
   purchase/contract action may the owner submit individual enrollment. Do not
   create a certificate signing request, certificate, key, profile, entitlement,
   App ID, device, download, or signing identity as part of enrollment.
6. Privately record one sanitized enrollment outcome: `not started`,
   `submitted`, `active`, `pending`, `declined`, `unavailable`, or `not
determined`. Record separately only `no signing asset created: confirmed`.
   Do not preserve account, payment, membership, certificate, or browser data.
7. After an `active` outcome, stop. Membership alone does not authorize a
   signing identity, target-Mac installation, Keychain proof, fake credential,
   or implementation. A later distinct plan must select the exact certificate
   type, private key lifecycle, target-Mac handling, and signed-probe evidence.

## Stop conditions

Stop immediately if any of the following occurs:

- Ownership, seller-name, legal-entity, team-access, or business-continuity
  requirements no longer match the D-074 individual model.
- The owner cannot privately confirm price, agreement, identity-verification,
  privacy, or regional requirements.
- Apple requests a step outside individual enrollment or a page offers a
  certificate, signing, profile, Keychain, credential, or developer-resource
  action.
- Enrollment is pending, declined, or ambiguous; do not contact support or
  retry unless a separate owner-approved support or remediation plan exists.
- Any future step requires source, dependency, configuration, entitlement,
  certificate, Keychain, credential, Cloudflare, provider, traffic, deployment,
  or runtime action not explicitly approved.

## Security and privacy controls

- Only the owner handles Apple authentication, legal identity, payment, and
  agreement review.
- Do not save Apple credentials, payment details, enrollment identifiers,
  screenshots, certificates, private keys, CSR files, profiles, or browser
  exports in the repository or product.
- No signing asset is a permitted enrollment by-product. Creation, download,
  import, export, revocation, or installation requires another exact increment.
- Enrollment state does not grant Keychain, secret-memory, Cloudflare, provider,
  gateway, traffic, deployment, or runtime authority.
- Preserve D-064’s 15-minute production token maximum and D-068’s 30-day
  owner-only demo exception without change.

## Verification for this documentation increment

- Confirm the plan requires a new owner approval before any future Apple action.
- Confirm it covers ownership, seller name, cost, agreement, privacy, evidence,
  stop conditions, and non-reversible-commitment risk.
- Confirm it prohibits signing assets and all product, Keychain, credential,
  Cloudflare, provider, traffic, deployment, and runtime actions.
- Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, and `python3 .codex/hooks/session_end_gate.py`.

## Rollback

Before a future submission, abandon the proposed enrollment action; no external
state should have changed. After a future submission or purchase, do not claim a
reversible rollback. Record only the sanitized status and stop for a separately
approved remediation, support, ownership, or distribution decision if needed.

## Non-goals

- No enrollment, purchase, agreement acceptance, Apple account access, support
  request, or role/ownership change.
- No certificate, CSR, signing identity, key, profile, entitlement, notarization,
  Keychain item, credential, fake credential, or real credential.
- No Cloudflare, provider, DNS, gateway, deployment, traffic, IPC, WebView,
  storage, networking, source, dependency, test, or runtime behavior.

## Readiness

Blocked. This plan defines future enrollment only. It does not make enrollment,
signing, the fake-only proof, real credential ingestion, or any product
increment Ready.
