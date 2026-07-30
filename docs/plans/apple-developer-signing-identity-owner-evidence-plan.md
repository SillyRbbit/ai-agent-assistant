# Apple Developer signing-identity owner-evidence plan

- Status: Documentation-only read-only evidence plan; no signing action authorized
- Date: 2026-07-29
- Decision authority: D-064, D-068, D-069, D-070, D-071, D-072, and D-073

## Goal

Define the owner-only, read-only evidence needed to determine whether the
existing Apple Developer account could later provide an owner-controlled macOS
application-signing identity for the fake-only Cloudflare credential proof.

This plan determines account eligibility only. It does not establish that an
identity exists, that an identity is installed on the target Mac, that a future
proof is Ready, or that any signing, Keychain, credential, Cloudflare, provider,
traffic, deployment, or runtime action is authorized.

## Current facts and decision boundary

- D-072 selects a stable, owner-controlled signed macOS application identity as
  the future credential-control model.
- D-073 limits a later fake-only implementation proof to exactly three existing
  Rust paths and requires separate owner approval.
- The owner reported that no valid macOS code-signing identity is currently
  installed. This plan does not verify, create, download, import, or use one.
- Repeated authorization prompts from the unsigned fake Keychain proof are not
  an accepted fallback or identity-control model.
- D-064's production 15-minute token maximum and D-068's 30-day demo-only
  exception remain unchanged.

## Exact scope

### Included

One owner-operated, browser-only review of already available Apple Developer
account pages, limited to visually reading account membership, role, and
signing-asset availability indicators. The owner may record only the sanitized
outcome categories in the private evidence store.

### Excluded

- Enrollment, renewal, purchase, payment, agreement acceptance, role change,
  invitation, support request, or account-profile change.
- Certificate, signing identity, key, CSR, provisioning profile, entitlement,
  App ID, device, download, export, import, revocation, or installation action.
- macOS `security` commands, Keychain access, Keychain writes, local signing,
  notarization, build signing, or target-Mac execution.
- Cloudflare, OpenAI, Azure, Microsoft, Apple API, provider, DNS, Worker,
  Access, gateway, secret, credential, deployment, traffic, or runtime action.
- Repository storage of Apple ID, email, team identifier, membership number,
  certificate subject, serial, hash, expiry, screenshot, browser history,
  private evidence-store location, or any signing material.

## Owner-only read-only procedure

The project owner performs this procedure manually. This documentation increment
does not open Apple pages, submit a provider request, or inspect the account.

1. Use the owner-controlled browser profile and sign in only if the owner
   chooses to do so. Do not save credentials in the browser, repository,
   terminal, Keychain, notes, screenshots, or chat.
2. Navigate manually to the Apple Developer account pages. Treat every control
   that changes state, requests agreement, begins enrollment, adds an asset,
   downloads an item, invites a member, or contacts support as out of scope.
   Stop instead of clicking it.
3. Read the membership status and whether the page indicates an active program
   relationship. Record only one private outcome: `active`, `inactive`,
   `not enrolled`, `unavailable`, or `not determined`.
4. Read whether the signed-in owner can view the signing-asset area and whether
   the interface visibly indicates authority to manage signing assets. Do not
   press an Add, Create, Continue, Download, Revoke, or similar action. Record
   only `visible with apparent authority`, `visible without apparent authority`,
   `not visible`, `access denied`, or `not determined`.
5. If an existing signing-identity list is visible, observe only whether it is
   `none visible`, `one or more visible`, `access denied`, or `not determined`.
   Do not open an item, copy its details, download it, or test it locally.
6. Close the browser page when the read-only observation is complete. Do not
   make a support request to resolve ambiguity in this increment.
7. Store a sanitized attestation privately. It must state the date, that the
   owner performed a read-only review, the three closed outcome categories, and
   that no mutation occurred. It must omit all account, membership, identity,
   certificate, profile, and browser details.

## Closed interpretation rules

| Evidence outcome                                                                                    | Allowed conclusion                                                                                                                          | Required next state                                                          |
| --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------- |
| Active membership, signing area visible with apparent authority, and no identity visible            | The account may be capable of a future owner-controlled identity-creation proposal. No asset exists or is authorized from this result.      | Blocked pending a separate creation/installation decision and plan.          |
| Active membership, signing area visible with apparent authority, and one or more identities visible | Existing signing assets may exist under owner control. Recoverability, suitability, target-Mac installation, and ACL behavior are unproven. | Blocked pending a separate identity-selection and target-Mac evidence plan.  |
| Membership inactive, not enrolled, unavailable, or not determined                                   | Eligibility is not proved.                                                                                                                  | Blocked. Do not enroll, renew, purchase, or request support under this plan. |
| Signing area hidden, access denied, or authority unclear                                            | Owner-controlled signing authority is not proved.                                                                                           | Blocked. Do not change roles or invite members under this plan.              |

No outcome permits a certificate, signing identity, key, profile, entitlement,
Keychain item, fake credential, real credential, or implementation increment.

## Security and privacy controls

- The owner alone handles browser authentication and private evidence.
- Use the closed outcome categories exactly; do not preserve raw page text or
  identifiers in repository records, chat, logs, terminal output, screenshots,
  or ordinary notes.
- Avoid clipboard copies, downloads, browser autofill changes, and export
  flows. A page that requires a mutation to reveal a fact produces
  `not determined`, not a workaround.
- Account eligibility is not credential authority. No identity or secret may
  enter the repository, desktop application, WebView, SQLite, logs, tests, or
  ordinary CI.
- The owner must stop on unfamiliar wording, an agreement prompt, a payment
  surface, a creation/download control, or any prompt that could modify account
  state.

## Risks and controls

- **Accidental account mutation:** use a read-only checklist, closed stop list,
  and no-click rule for any mutating control.
- **Private-account disclosure:** retain only sanitized categories in the
  private evidence store and no account-specific artifact elsewhere.
- **False readiness:** distinguish account eligibility from a signing identity,
  target-Mac installation, signed-probe stability, and implementation approval.
- **Authority ambiguity:** fail closed as `not determined` or `access denied`;
  do not resolve it through role, enrollment, support, or asset actions.
- **Scope expansion:** any account action, signing asset, Keychain action, or
  code change stops this plan and needs a separate approved increment.

## Verification for this documentation increment

- Confirm this plan names no mutation, download, installation, API call, or
  provider request as a permitted step.
- Confirm D-064's 15-minute production requirement and D-068's 30-day
  demo-only exception are unchanged.
- Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, and `python3 .codex/hooks/session_end_gate.py`.
- Review changed paths and confirm no source, manifest, lockfile, Tauri,
  signing, Keychain, credential, Cloudflare, provider, traffic, or runtime file
  changed.

## Manual evidence status

The owner-operated read-only account review is intentionally **not performed**
by this documentation increment. It becomes a separately owner-directed manual
evidence step only after this plan is accepted and published. Its absence keeps
all later signing and fake-only implementation work Blocked.

## Rollback

Before commit, revert only this plan and its approved documentation closeout.
There is no account, signing, Keychain, certificate, credential, Cloudflare,
provider, traffic, deployment, or runtime state to undo because this increment
does not perform any external or local operational action.

## Acceptance criteria

- [x] The plan defines an owner-only read-only account-evidence procedure and
      closed result categories.
- [x] The plan prohibits enrollment, purchase, asset creation, download,
      installation, revocation, Keychain action, credential use, and external
      operational activity.
- [x] The plan preserves D-064's production and D-068's demo-only token rules.
- [x] The plan states that every outcome leaves implementation Blocked pending
      separate approval and target-Mac evidence.

## Readiness

Blocked. This plan is documentation only. A private owner attestation could
clarify account eligibility later, but no signing, credential, Keychain,
Cloudflare, provider, deployment, traffic, or runtime increment is Ready.
