# Apple Support TS-017 assistance plan

- Status: Documentation-only plan; support contact remains unauthorized
- Date: 2026-08-01
- Decision authority: D-064, D-068, D-072, D-075, D-076, and TS-017

## Goal

Define a future owner-operated Apple Support assistance path for the unresolved
TS-017 Certificate Assistant CSR failure. This plan permits no contact now and
does not reopen D-076's deferred signed-identity path.

## Minimum sanitized future issue summary

The owner may disclose only this summary if a separate operational increment is
approved:

> On an owner-controlled macOS target Mac, Keychain Access Certificate
> Assistant reported that the specified item could not be found in the Keychain
> during a CSR-generation attempt. No CSR, certificate, or new named private key
> resulted. Approved read-only checks observed configured user/default Keychain
> state, zero valid code-signing identities, no authorization prompt, and no
> state change. Cause remains undetermined.

The owner may add only broad macOS version and architecture information when
necessary. Do not disclose an Apple Account, membership, team, email, payment,
Keychain path or label, item list, certificate, private key, hash, serial,
terminal output, screenshot, source, repository, credential, Cloudflare, or
provider detail.

## Future owner procedure and stop conditions

The following procedure is future-facing and must not run under this plan.

1. Obtain separate owner approval naming this plan and one support contact.
2. The owner alone contacts Apple Support and requests diagnostic guidance only.
   Do not use remote screen sharing, upload data, install a profile or utility,
   grant device access, or accept an account change.
3. Stop immediately if asked to disclose prohibited information or to unlock,
   reset, repair, delete, replace, export, import, create, download, or install
   any Keychain, CSR, certificate, private-key, signing, or Apple asset.
4. Do not execute any support recommendation during the contact. End the
   session and report only a closed sanitized category.

## Private evidence and rollback

Report only `contact attempted: yes | no`, `guidance: none | diagnostic-only |
state-changing-requested`, `state changed: not observed`, and `cause: not
determined | escalated`. Do not retain a case number, transcript, screenshot,
recording, path, account, or technical identifier in the repository or chat.

Rollback is ending the support session without acting on guidance. If any state
change is suspected, stop and require a separate owner-approved incident or
recovery plan; do not delete, reset, replace, or repair anything.

## Risks and controls

| Risk                       | Control                                                                                                                |
| -------------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| External disclosure        | Minimum summary only; owner-only contact; no screenshot, transcript, remote access, or upload.                         |
| Destructive support advice | Do not execute advice during contact; explicit stop conditions and separate follow-up approval.                        |
| Identity-boundary bypass   | No alternate CSR, key, certificate, or signing action; preserve D-072's owner-controlled non-exported-key requirement. |
| False diagnosis            | Retain `not determined` unless later approved evidence proves otherwise.                                               |

## Verification and readiness

- Confirm D-076 remains a deferral and this plan grants no contact authority.
- Confirm disclosures, stop conditions, private evidence, and no-state-change
  rollback are explicit.
- Confirm D-064's 15-minute production maximum and D-068's 30-day demo-only
  exception remain unchanged.
- Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock`, and
  `python3 .codex/hooks/session_end_gate.py`.

No Apple Developer access, Apple Support contact, CSR retry, certificate or key
creation, Keychain action, signing, credential, Cloudflare, provider,
deployment, traffic, code, dependency, or runtime behavior. Blocked pending a
separate owner-approved operational increment.
