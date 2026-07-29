# Cloudflare demo fake Keychain proof

Status: Verified complete with advisories; real credential ingestion remains Blocked
Date: 2026-07-28
Decision authority: D-068 and D-069

## Goal

Add one local, fake-value-only macOS Keychain proof that lets trusted Rust read
exactly two fixed Cloudflare Access demo labels and return only closed status or
redacted errors. Determine whether the unsigned development executable can
receive stable app-specific access before any real credential or external
Cloudflare work is proposed.

## Approved exact scope

- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/src/lib.rs`
- `src-tauri/src/credentials/mod.rs`
- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/examples/cloudflare_access_keychain_probe.rs`
- `src-tauri/tests/cloudflare_access_credential_boundary.rs`
- `ARCHITECTURE.md`
- `SECURITY.md`
- `DECISIONS.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- this plan
- `docs/reviews/2026-07-28-cloudflare-demo-fake-keychain-proof-post-increment-review.md`

No path outside this 17-file inventory changed.

## Implementation boundary

The trusted Rust `credentials::cloudflare_access` module:

- reads only generic-password service
  `io.cortexa.demo.cloudflare-access`;
- reads only accounts `client-id` and `client-secret`, in that order;
- uses macOS-only `security-framework = "=3.7.0"` and
  `security-framework-sys = "=2.17.0"` with default features disabled;
- accepts only non-empty, visible-ASCII values of at most 512 bytes;
- returns only `Available` or typed closed errors for missing, access denied,
  cancelled, invalid, unsupported-platform, or redacted platform failure;
- exposes no raw-value accessor;
- overwrites its fake-value buffer on drop as a best-effort proof cleanup, not
  a production zeroization claim; and
- has no write, update, delete, enumeration, arbitrary-label, Tauri command,
  IPC, WebView, SQLite, startup, log, network, or runtime-consumer path.

Automated unit tests use only deterministic fake sources. They do not access
the operating-system Keychain. The public integration test checks the closed
redacted error contract. The manual example prints only `available` or one
closed error.

## Dependency review

The exact cached `security-framework 3.7.0` package is MIT OR Apache-2.0,
declares Rust 1.85, has no build script, and wraps Apple's native
Security.framework. The exact transitive/direct system binding is
`security-framework-sys 2.17.0`. The final lockfile adds only these two packages
and their root-package references; no unrelated version changes remain.

The wrapper's package metadata says `looking-for-maintainer`. That is accepted
only for this fake proof and remains an advisory requiring reassessment before
real credential ingestion.

## Target-Mac manual evidence

The project owner performed the following with only fake values:

1. The initial unsandboxed probe returned the closed missing-client-ID outcome.
2. The owner created exactly two generic-password items in the login keychain
   with the fixed service and account labels.
3. Native denial/cancel interaction returned the closed `read was cancelled`
   outcome without a value or native error detail.
4. Repeated owner-authorized reads eventually returned the closed `available`
   outcome.
5. macOS required multiple login-keychain authorization prompts. No stable
   unsigned-executable app-specific access was proved.
6. The owner removed both fake items.
7. The final probe again returned the closed missing-client-ID outcome.

No password, fake value, screenshot, Keychain database, or private terminal
evidence is stored in the repository. The sanitized outcomes above are the
complete manual record.

## Risks and controls

- **Raw-value leakage:** no public accessor, serialization, IPC, logging, or
  error source exists; tests scan closed output.
- **Arbitrary Keychain access:** service and account labels are private fixed
  constants; the public probe accepts no caller input.
- **Accidental Keychain mutation:** production code imports only the read
  function and defines no write, update, delete, or enumeration API.
- **Unsigned identity instability:** repeated prompts are recorded as a failed
  stable-access proof and block real credential ingestion.
- **Misleading memory guarantees:** fake buffers receive best-effort overwrite
  only; future real handling requires a separate reviewed memory-lifecycle
  design.
- **Dependency health:** exact versions, licenses, Rust compatibility, feature
  set, lockfile delta, and maintenance advisory are recorded.

## Verification

Required completion evidence:

```bash
npm run format
npm run verify
npm run docs:check
npm run security:scan
cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework
cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework-sys
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Focused Rust unit and integration tests, strict Clippy, the example build, and
the complete Rust suite also passed during implementation before the final
complete verification.

## Rollback

Before commit, revert only the 17 approved files. Remove the two pinned
dependencies and the credential module, example, integration test, plan,
review, and additive current-state documentation. The owner already removed
both fake items. No real credential or Cloudflare resource exists to revoke or
delete.

## Non-goals

- No real credential, service token, Access application, policy, Worker, route,
  DNS record, secret, deployment, provider request, traffic, or external
  evidence.
- No Tauri command, IPC, WebView, startup, provider, network, request header,
  runtime credential consumer, persistence, audit, or UI.
- No Keychain write, update, delete, enumeration, arbitrary-label lookup,
  generic secret store, `Always Allow`, keychain reset, or login-keychain
  modification.
- No claim that unsigned development access is stable or suitable for a real
  service token.
- No change to D-064's production 15-minute requirement or D-068's demo-only
  30-day maximum.

## Acceptance criteria

- [x] Only the two fixed service/account labels can be read.
- [x] No raw fake value crosses the public boundary, errors, debug, or output.
- [x] Missing, cancellation/denial-as-cancelled, invalid, and available
      behavior is closed and tested.
- [x] Automated checks never access or mutate Keychain.
- [x] Owner-operated target-Mac evidence proves availability and cleanup.
- [x] Repeated prompts are recorded as a failed stable unsigned-access proof.
- [x] No fake item, real credential, Cloudflare resource, request, or traffic
      remains.

## Readiness

Blocked. The fake proof is complete, but real credential ingestion is not
Ready. A future local deny-only Worker artifact also remains separately
approval-bound and has no Ready implementation plan.
