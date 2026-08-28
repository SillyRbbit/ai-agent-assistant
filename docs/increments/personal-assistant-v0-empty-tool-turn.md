# Personal Assistant v0 empty-tool turn increment

Status: Verified complete with advisories
Owner: Henry Dang
Date: 2026-08-28
Baseline: `0b22ee79a24e11d7c67cbace111a502608b57591`
Plan: [2026-08-28-personal-assistant-v0-empty-tool-turn.md](../plans/2026-08-28-personal-assistant-v0-empty-tool-turn.md)

## Goal

Implement V0-1's smallest transport-free Personal Assistant prerequisite: one
sealed, application-owned synthetic `empty@1` text-turn contract behind the
existing Native runtime start boundary and one volatile no-input Rust host.

## Scope

- Pin the exact synthetic fixture, immutable instructions, provider/model
  profile, lower closed limits, empty tools, zero retries, and no fallback.
- Route the sealed profile through the sole `AgentRuntime::start` boundary
  while preserving the existing Initial profile.
- Issue private process-local correlation identities and validate the complete
  returned runtime identity plus exact initial status before acceptance.
- Own one process-wide lease and terminal-clean or privately quarantine every
  rejected run without exposing or replacing ambiguous ownership.
- Prove bounded deterministic event, cancellation, failure, transactional
  rejection, and late-event behavior without I/O.

## Non-goals

No user text, response-frame ingress on the public host, Tauri/WebView, UI,
provider/model request, network, signed identity, credential, Keychain,
persistence, memory, tool, approval, audit, filesystem, background autonomy,
device action, dependency, manifest, lockfile, external state, commit, or
publication.

## Baseline evidence

- Clean synchronized `main`; `HEAD` and `origin/main` both equal the baseline
  above after fetch.
- Repository-pinned toolchains: Node 26.3.0, npm 11.16.0, Rust/Cargo 1.90.0.
- Gate `personal-assistant-v0-empty-tool-turn` began before source edits.
- Git metadata was healthy; integrity inspection reported only unreachable
  dangling objects.

## Acceptance evidence

- The request bytes pin every trusted identity/configuration field and
  `empty@1`; public callers cannot select or observe those values.
- The sole Native runtime start method owns both boxed profiles; Initial-only
  concrete methods fail closed or safely no-op on the Personal Assistant
  profile without mutating it.
- Checked start accepts only the exact returned identity and
  `AwaitingStart`. Every other identity/status is terminal-cleaned or retained
  with run-and-lease ownership in private quarantine.
- The public no-input host exposes only closed status, request length, and
  cancellation. It has no text or response ingress and retains no transcript.
- Rust tests cover deterministic success, failure, cancellation, lower scalar
  and byte bounds, explicit retry metadata, transactional rejection, redacted
  errors/debug, and late-event rejection without I/O.
- Independent architecture and security/code reviews found no remaining
  finding. Full validation, manual scope inspection, and the completion marker
  pass as recorded in the post-increment review.

## Result

`PASS WITH ADVISORIES`. V0-1 is locally verified, uncommitted, and unpublished.
Its success/failure/stream outcomes remain fixture-only because the public host
has no response ingress. Target-Mac UI, network, signing, Keychain, gateway,
provider, credential, and external rollback checks are `Not run` because no
such surface exists in this increment. V0-2 remains Blocked until this exact
source baseline is published or otherwise accepted by the owner.
