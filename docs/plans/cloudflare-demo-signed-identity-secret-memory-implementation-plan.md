# Cloudflare demo signed-identity and secret-memory implementation plan

Status: Complete documentation-only plan; implementation remains Blocked
Date: 2026-07-28
Decision authority: D-064, D-068, D-069, D-070, D-071, and D-072

## Goal

Define one future fake-only implementation proof for the owner-selected signed
macOS application identity and bounded private secret-memory behavior. This plan
adds no code, signing state, Keychain item, credential, dependency, Cloudflare
resource, network path, or runtime consumer.

## Future exact source and test boundary

Only these three existing paths may change in the later implementation:

- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/tests/cloudflare_access_credential_boundary.rs`
- `src-tauri/examples/cloudflare_access_keychain_probe.rs`

The future increment must stop for a new exact scope approval if it needs any
other source, test, manifest, lockfile, Tauri configuration, entitlement,
profile, script, dependency, IPC, startup, WebView, networking, or runtime path.

## Future implementation contract

- Use fake values only under the existing fixed service and two fixed accounts.
- Replace proof-only byte ownership with one private bounded secret owner that
  is not cloneable, serializable, printable, debuggable, persisted, or publicly
  accessible and supports only scoped one-time consumption.
- Preserve closed redacted missing, denial, cancellation, invalid-value,
  unsupported-platform, and platform-failure outcomes.
- Keep the public boundary status-only and the example output limited to
  `available` or one closed error.
- Add no write, update, delete, enumeration, arbitrary-label, generic secret
  store, IPC, WebView, SQLite, startup, network, provider, or runtime-consumer
  behavior.
- State practical Rust, compiler, copy, OS-memory, and crash-report limitations;
  best-effort overwrite must not be described as guaranteed zeroization.

## Private signed target-Mac evidence

The later owner-operated proof must use only fake items and privately record:

1. verified owner-controlled signing provenance without putting certificate,
   identity, requirement, hash, profile, or entitlement data in the repository;
2. stable signed-probe access across restart or rebuild as defined by the
   approved local identity procedure;
3. rejection for an unsigned or non-authorized copy;
4. closed missing, denial/cancellation, malformed fake-value, and successful
   one-time consumption outcomes without raw-value or native-error exposure;
5. no repeated broad authorization behavior accepted as stable proof; and
6. removal of both fake items followed by re-observed missing behavior.

Only sanitized passed/failed outcomes may enter repository evidence. No value,
password, screenshot, signing identifier, certificate, Keychain database, or
private evidence-store location may be recorded.

## Risks and controls

- **Secret leakage or copies:** private ownership, no public accessor or
  diagnostics, scoped consumption, closed errors, and output scans.
- **False zeroization assurance:** document limits and verify bounded ownership
  without promising impossible guarantees.
- **Unstable or broad identity access:** require signed-probe stability and
  unauthorized-copy rejection; unsigned prompt behavior fails the proof.
- **Dependency drift:** no manifest or lockfile change; any dependency need
  stops the future increment for new review and approval.
- **Scope expansion:** three existing files only and no runtime wiring.

## Verification for the future implementation

The later increment must run focused module and integration tests, strict
Clippy, the example build, complete repository verification, security review,
diff checks, the session-end gate, and private owner-operated target-Mac
evidence. Automated tests must use deterministic fake sources and never access
Keychain or require signing.

## Verification for this documentation increment

Run `npm run format`, `npm run docs:check`, `npm run repository:check`, `npm run
security:scan`, `git diff --check`, and
`python3 .codex/hooks/session_end_gate.py`. No manual platform or external
verification applies.

## Rollback

Before commit, revert only this plan and its nine approved documentation
records. No fake item, signing state, credential, dependency, external resource,
or runtime behavior exists to revoke or remove.

## Non-goals

- No code, test, dependency, certificate, signing, notarization, entitlement,
  profile, Keychain action, credential, Cloudflare change, provider request,
  traffic, deployment, IPC, WebView, storage, networking, or runtime behavior.
- No real secret-memory or identity-readiness claim from documentation.
- No change to D-064's 15-minute production requirement or D-068's 30-day
  demo-only exception.

## Acceptance criteria

- [x] The future fake-only source/test boundary contains exactly three existing
      paths.
- [x] Secret ownership, closed outcomes, signed evidence, cleanup, and stop
      conditions are explicit.
- [x] No future signing, Keychain, credential, Cloudflare, or traffic action is
      authorized by this plan.

## Readiness

Blocked. The future implementation requires separate exact owner approval and
must use fake values only. Real credential ingestion remains prohibited.
