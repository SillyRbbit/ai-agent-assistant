# macOS Certificate Assistant CSR-remediation plan

- Status: Documentation-only plan; diagnostic execution remains unauthorized
- Date: 2026-08-01
- Evidence authority: TS-017, D-064, D-068, D-070, D-072, D-073, and D-075

## Goal

Define the future owner-operated, local-only, read-only diagnostic boundary for
the unavailable macOS Certificate Assistant CSR outcome in TS-017. The goal is
only to establish whether basic Keychain selection state can be observed without
mutation. It is not to reproduce the failure, determine a root cause by
assumption, repair Keychain, or create signing material.

The confirmed baseline remains: no CSR file, certificate, or new named private
key was created, and the cause is `not determined`.

## Future diagnostic scope

A later separate owner-approved operational increment may perform only the
following local observations on the approved target Mac, once each, with no
output copied into the repository, chat, screenshots, shared storage, cloud
sync, or ordinary notes:

1. Observe the configured user Keychain list through `security list-keychains -d user`, recording only `observed`, `unavailable`, or `not determined`.
2. Observe the configured default user Keychain through `security default-keychain -d user`, recording only the same closed outcome category.
3. Observe the summary count from `security find-identity -v -p codesigning`,
   recording only `zero`, `nonzero`, `unavailable`, or `not determined`. Do not
   retain, repeat, share, or transcribe any identity name, hash, certificate
   subject, or other raw output.

The owner may visually confirm that Keychain Access opens and that the login
Keychain can be selected, but must not search, enumerate, inspect, open,
rename, export, modify, or delete any item. The plan authorizes no command or
UI action beyond these observations.

## Required owner gates and procedure

The following procedure is future-facing and must not run under this plan.

1. Obtain separate approval that names this exact plan and confirms the target
   Mac remains personally owner-controlled.
2. Privately reconfirm the TS-017 baseline before starting: no CSR file,
   certificate, or new named private key exists.
3. Close Apple Developer and do not visit Apple, provider, support, or external
   troubleshooting pages. Disconnect no services and make no account change.
4. Run each allowed observation at most once. Keep the terminal local and do
   not save, pipe, redirect, copy, screenshot, or share its output.
5. Report only the closed categories above, whether a password or unlock prompt
   occurred, and `state changed: not observed`. If the evidence does not prove a
   cause, report `cause: not determined`.
6. Stop after the observations. Do not attempt a CSR, key, certificate,
   download, installation, signing, or remediation.

## Stop conditions

Stop immediately, close the relevant application or terminal window without
saving output, and report a sanitized `unavailable` or `not determined` result
if any of the following occurs:

- an unlock, password, Touch ID, authorization, import, export, creation,
  deletion, reset, repair, or replacement action is requested;
- an unexpected certificate, private key, signing identity, or credential is
  observed;
- raw output would expose a Keychain path, account detail, item label,
  certificate subject, hash, serial, team value, or private key material;
- the procedure would require Apple Developer, Apple Support, a browser search,
  a third-party utility, Terminal or OpenSSL key generation, or a CSR retry; or
- ownership, target-Mac control, or the original baseline cannot be confirmed.

## Security and privacy controls

| Risk                         | Required control                                                                                                                                                                      |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Accidental Keychain mutation | Only the three listed read-only observations and passive visual confirmation; no Keychain item action or CSR retry.                                                                   |
| Sensitive identity exposure  | No raw output retention, identifiers, paths, screenshots, transcripts, or shared evidence.                                                                                            |
| False diagnosis              | Use only closed outcome categories; preserve `cause: not determined` unless a later approved investigation produces reproducible evidence.                                            |
| Scope expansion              | No Apple or external access, support contact, alternate utility, key generation, certificate action, signing, credential, Cloudflare, provider, traffic, deployment, or runtime work. |
| Misleading readiness         | A clean observation does not authorize another CSR attempt, identity creation, fake-only proof, or real credential ingestion.                                                         |

## Private target-Mac evidence

The owner keeps any ephemeral local observation private and reports only:

- `user keychain configuration: observed | unavailable | not determined`;
- `default keychain configuration: observed | unavailable | not determined`;
- `valid code-signing identities: zero | nonzero | unavailable | not determined`;
- `authorization prompt: not observed | observed`; and
- `state changed: not observed`.

Do not record a path, username, account, certificate, key, item name, label,
identifier, raw command output, screenshot, terminal transcript, or browser
artifact. The owner must not make a root-cause claim from these categories alone.

## Rollback and follow-up

The diagnostic procedure creates no state, so rollback is limited to closing
the local application and terminal without saving output and reporting the
sanitized outcome. If state change is suspected, stop and require a separate
owner-approved incident or recovery plan; do not delete, reset, replace, or
repair any Keychain state.

Whether the observations are clean, unavailable, or inconclusive, no CSR retry
or signing action is authorized. Any proposed next step—including a retry,
Apple Support contact, Keychain repair, alternate CSR workflow, certificate
creation, signing, or fake-only proof—requires its own separately approved plan
and owner approval.

## Verification for this documentation increment

- Confirm TS-017's no-asset baseline and `not determined` cause remain intact.
- Confirm the plan defines exactly three future read-only observations and does
  not permit their execution under this plan.
- Confirm stop conditions, no-state-change rollback, private evidence, and all
  Apple, Keychain, signing, credential, Cloudflare, provider, deployment,
  traffic, code, dependency, and runtime prohibitions are explicit.
- Confirm D-064's 15-minute production maximum and D-068's 30-day demo-only
  exception remain unchanged.
- Run `npm run docs:check`, `npm run repository:check`, `npm run security:scan`,
  `git diff --check`, `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock`, and
  `python3 .codex/hooks/session_end_gate.py`.

## Non-goals and readiness

No diagnostic execution, Apple Developer access, CSR retry, certificate or key
creation, Keychain unlock, reset, deletion, replacement, Terminal or OpenSSL
key generation, Apple Support contact, signing, notarization, profile, App ID,
entitlement, credential, Cloudflare change, provider request, deployment,
traffic, code, dependency, or runtime behavior.

Blocked. This plan authorizes neither diagnostic execution nor recovery. D-064's
15-minute production requirement and D-068's 30-day owner-only demo exception
remain unchanged.
