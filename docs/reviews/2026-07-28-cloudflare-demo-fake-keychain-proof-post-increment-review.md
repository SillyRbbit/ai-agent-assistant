# Cloudflare demo fake Keychain proof post-increment review

<!-- post-increment-gate-manifest
{"commands_executed":["npm run format","npm run verify","npm run docs:check","npm run security:scan","cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework","cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework-sys","git diff --check","python3 .codex/hooks/session_end_gate.py"],"files_changed":["ARCHITECTURE.md","CHANGELOG.md","DECISIONS.md","HANDOFF.md","NEXT_STEPS.md","PLANS.md","PROJECT_STATUS.md","SECURITY.md","docs/plans/cloudflare-demo-fake-keychain-proof.md","docs/reviews/2026-07-28-cloudflare-demo-fake-keychain-proof-post-increment-review.md","src-tauri/Cargo.lock","src-tauri/Cargo.toml","src-tauri/examples/cloudflare_access_keychain_probe.rs","src-tauri/src/credentials/cloudflare_access.rs","src-tauri/src/credentials/mod.rs","src-tauri/src/lib.rs","src-tauri/tests/cloudflare_access_credential_boundary.rs"],"findings":[{"blocks_completion":false,"blocks_next_increment":true,"category":"Technical debt","effort":"separate approved signed-identity or narrowly reviewed stable ACL and real secret-memory lifecycle increment","milestone":"before any real Cloudflare Access service-token creation or ingestion","risk":"the unsigned development executable required repeated login-keychain prompts and did not prove stable app-specific access; its fake buffer overwrite is not a production zeroization guarantee","severity":"Advisory","summary":"Real credential ingestion remains blocked despite the successful fake-only read proof."},{"blocks_completion":false,"blocks_next_increment":false,"category":"Technical debt","effort":"reassess maintenance, advisories, and replacement options during the next credential-boundary dependency review","milestone":"before real credential ingestion and at each dependency update","risk":"security-framework 3.7.0 declares looking-for-maintainer in its package metadata","severity":"Advisory","summary":"The pinned native wrapper has an explicit upstream maintenance-health advisory."}],"increment_id":"cloudflare-demo-fake-keychain-proof","manual_verification":[{"check":"Initial read-only target-Mac probe reports the fixed client-ID item missing before fake-item creation","required":true,"status":"Passed"},{"check":"Owner creates exactly two fake generic-password items under the fixed service and accounts without exposing values","required":true,"status":"Passed"},{"check":"Native denial or cancellation returns only the closed cancelled outcome","required":true,"status":"Passed"},{"check":"Owner-authorized target-Mac probe reports only available for the complete fake pair","required":true,"status":"Passed"},{"check":"Repeated prompts are recorded as failure to prove stable unsigned-executable app-specific access","required":true,"status":"Passed"},{"check":"Owner removes both fake items and the final probe reports the fixed client-ID item missing","required":true,"status":"Passed"}],"next_increment_readiness":"Blocked","quality_gate":"PASS WITH ADVISORIES","schema_version":1,"verification":[{"command":"npm run format","required":true,"status":"Passed"},{"command":"npm run verify","required":true,"status":"Passed"},{"command":"npm run docs:check","required":true,"status":"Passed"},{"command":"npm run security:scan","required":true,"status":"Passed"},{"command":"cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework","required":true,"status":"Passed"},{"command":"cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework-sys","required":true,"status":"Passed"},{"command":"git diff --check","required":true,"status":"Passed"},{"command":"python3 .codex/hooks/session_end_gate.py","required":true,"status":"Passed"}]}
-->

Date: 2026-07-28
Increment: cloudflare-demo-fake-keychain-proof
Branch: main

## Executive summary

PASS WITH ADVISORIES. The exact 17-file increment adds one fake-only,
status-only macOS Keychain read proof. Automated and owner-operated target-Mac
evidence passed, both fake items were removed, and no real credential or
external resource exists. Repeated authorization prompts did not prove stable
unsigned-executable access, so real credential ingestion remains Blocked.

## Scope and boundaries

The implementation reads only two fixed generic-password labels through trusted
Rust and returns only `Available` or closed redacted errors. It adds no write,
delete, enumeration, arbitrary-label, Tauri command, IPC, WebView, SQLite,
startup, network, runtime credential consumer, or Cloudflare action. The
complete Git inventory matches the approved 17 paths.

## Verification results

All required automated and manual checks passed. The owner used only fake
values, observed closed missing, cancelled/denied-as-cancelled, and available
outcomes, removed both items, and re-observed missing. No credential value,
native error detail, private screenshot, or terminal evidence entered the
repository.

During implementation, the first non-offline Cargo check attempted and failed
to resolve the crates.io index before compilation. The already-cached exact
approved dependencies were then resolved offline. Offline resolution initially
proposed three unrelated package downgrades; review restored the baseline
versions, leaving only the two approved packages in the final lockfile. This
was corrected before final verification and caused no provider or Cloudflare
request.

## Architecture findings

None. The narrow `credentials::cloudflare_access` module owns fixed-label
Keychain access, exposes only a status boundary, and remains disconnected from
Tauri, runtime startup, IPC, WebView, storage, and networking. Platform-specific
bindings are target-gated and the off-macOS boundary fails closed.

## Security findings

No completion-blocking finding. Raw values have no public accessor, serializer,
log, error source, or IPC path. The adapter imports only read functionality,
validates bounded visible-ASCII fake values, uses closed error mapping, and
overwrites fake buffers on drop without claiming production-grade zeroization.
The owner removed all fake items. Repeated authorization prompts correctly keep
real ingestion blocked.

## Code-health findings

None. Types and errors are closed, the public example emits status only,
automated tests use deterministic fake sources, target-specific code remains
portable through conditional compilation, and strict Clippy plus the complete
suite pass.

## Technical debt

1. Category: credential-boundary readiness.
   Severity: Advisory.
   Risk: unsigned development identity required repeated prompts and the fake
   buffer cleanup is not a production zeroization guarantee.
   Effort: separate signed-identity or narrowly reviewed ACL and real
   secret-memory lifecycle increment.
   Milestone: before any real service-token creation or ingestion.
   Completion impact: does not block this fake-only proof.
   Next-increment impact: blocks real credential ingestion.
2. Category: dependency health.
   Severity: Advisory.
   Risk: `security-framework 3.7.0` declares `looking-for-maintainer`.
   Effort: reassess maintenance, advisories, and alternatives during the next
   credential-boundary dependency review.
   Milestone: before real credential ingestion and on each dependency update.
   Completion impact: does not block this bounded fake proof.
   Next-increment impact: does not block unrelated local-only planning.

## Roadmap findings

Blocked. No real credential, Worker, Cloudflare, provider, or runtime increment
is Ready. The smallest possible future work requires a new exact plan and
project-owner approval.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Blocked. Real credential ingestion requires stable app-specific identity/ACL
evidence and production secret-memory handling. The local deny-only Worker
artifact remains a separate unapproved candidate.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `SECURITY.md`
- `docs/plans/cloudflare-demo-fake-keychain-proof.md`
- `docs/reviews/2026-07-28-cloudflare-demo-fake-keychain-proof-post-increment-review.md`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/examples/cloudflare_access_keychain_probe.rs`
- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/src/credentials/mod.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/tests/cloudflare_access_credential_boundary.rs`

## Exact commands executed

- `npm run format` — Passed
- `npm run verify` — Passed
- `npm run docs:check` — Passed
- `npm run security:scan` — Passed
- `cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework`
  — Passed
- `cargo tree --manifest-path src-tauri/Cargo.toml --locked --offline -i security-framework-sys`
  — Passed
- `git diff --check` — Passed
- `python3 .codex/hooks/session_end_gate.py` — Passed
