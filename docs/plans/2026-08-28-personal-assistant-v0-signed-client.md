# V0-3 — signed client identity and bounded secret owner

Status: Blocked by D-076 and TS-017
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: D-076/TS-017 resolution and a separately accepted restart of the signed-identity lane

## Goal

Prove, with fake values only, that one owner-controlled signed macOS application
identity can read exactly two fixed Cloudflare Access labels into a private,
single-consumption secret owner while an unsigned or unauthorized copy fails.
This plan neither ingests a real token nor authenticates a gateway request.

## Exact files

D-073 continues to limit the future implementation to:

- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/tests/cloudflare_access_credential_boundary.rs`
- `src-tauri/examples/cloudflare_access_keychain_probe.rs`
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-signed-client.md`
- `docs/increments/personal-assistant-v0-signed-client.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-signed-client-post-increment-review.md`
  (new)

Any manifest, lockfile, entitlement, profile, Tauri, IPC, startup, runtime,
frontend, network, or additional source file is a stop condition requiring a
new plan.

## Interfaces and invariants

- Fixed service and account labels only; no arbitrary lookup, enumeration,
  write, update, delete, or generic secret-store API.
- Private bytes are non-cloneable, non-serializable, non-debuggable,
  non-displayable, non-persisted, and unavailable through a public accessor.
- One scoped consumption transfers ownership only to a future trusted
  transport adapter. Consumption cannot be retried from a copied value.
- Missing, denied, cancelled, malformed, expired, identity-mismatched, and
  platform-failure outcomes are closed and redacted.
- Best-effort overwrite is described honestly; Rust, compiler, OS memory,
  swapping, and crash-dump behavior prevent a guaranteed zeroization claim.
- No fake or real value enters source, tests, shell arguments, terminal output,
  screenshots, chat, WebView, SQLite, logs, CI, or repository evidence.

## Threats and tests

Automated fake-source tests cover fixed labels, absence of generic access,
single consumption, repeated consumption, malformed size/grammar, every closed
error, debug/display redaction, ownership/drop behavior, and no public raw-value
path. They never access Keychain or signing state.

Private owner-operated target-Mac evidence must prove stable access across
restart/rebuild for the approved signed identity, rejection for an unsigned or
unauthorized copy, missing/denied/cancelled/malformed outcomes, one-time
consumption, and removal returning to missing. Repository evidence records only
Passed/Failed and no identity, certificate, requirement, entitlement, label
value, screenshot, Keychain database, or private evidence location.

## Manual gate

Before source work, the owner must separately approve the exact signing
authority, identifier, certificate/profile/entitlement provenance, renewal,
revocation, update/reinstall behavior, private evidence procedure, and incident
response. An interactive login-Keychain prompt or unsigned executable is never
a fallback.

## Verification

```bash
cargo test --manifest-path src-tauri/Cargo.toml credentials::cloudflare_access
cargo test --manifest-path src-tauri/Cargo.toml --test cloudflare_access_credential_boundary
cargo build --manifest-path src-tauri/Cargo.toml --example cloudflare_access_keychain_probe
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

## Non-goals

No real secret, service token, Access application, Worker, route, DNS, provider
key, provider request, network call, Tauri command, WebView, persistence,
notarization, distribution, tool, file, memory, or device effect.

## Rollback

Revert only the exact three source/test paths and closeout docs. Remove fake
Keychain items under the approved private procedure. No Cloudflare or provider
resource exists to revoke in this increment.

## Stop conditions

Stop if D-076 has not been additively reopened, TS-017 remains unresolved, the
signed identity is unstable, an unauthorized copy succeeds, a prompt is needed
as accepted evidence, a dependency/configuration change is required, a raw
value reaches diagnostics, or any external credential/resource action is
requested.

## Readiness

**Blocked.** D-076 explicitly deferred this lane after TS-017. This plan cannot
become Ready through documentation alone.
