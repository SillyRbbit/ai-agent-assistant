# V0-7 — bounded fixed-origin Rust streaming transport

Status: Blocked by V0-2, V0-3, V0-5, and V0-6
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-2, V0-3, V0-5, and V0-6

## Goal and outcome

Connect the volatile host to one compile-time fixed HTTPS gateway origin through
one reviewed direct Rust client, using fake credentials and deterministic fake
I/O only. Prove bounded incremental streaming, active deadline enforcement,
terminal socket cancellation, cleanup quarantine, and late-byte rejection. No
external request, real credential, provider adapter, Tauri IPC, or user text is
introduced.

## Exact files

- `src-tauri/Cargo.toml`
- `src-tauri/Cargo.lock`
- `src-tauri/src/credentials/cloudflare_access.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_v0.rs`
- `src-tauri/src/personal_assistant_v0_transport.rs` (new)
- `src-tauri/examples/personal_assistant_v0_access_auth_probe.rs` (new)
- `src-tauri/tests/cloudflare_access_credential_boundary.rs`
- `src-tauri/tests/personal_assistant_v0_contract.rs`
- `src-tauri/tests/personal_assistant_v0_transport_contract.rs` (new)
- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-synthetic-transport.md`
- `docs/increments/personal-assistant-v0-synthetic-transport.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-synthetic-transport-post-increment-review.md`
  (new)

Only the exact crate/version/features accepted by V0-6 may change the manifest
and lockfile. JavaScript manifests, gateway source, capabilities, CSP, Tauri
commands, frontend, workflows, and agent/workflow/tool files are excluded.

## Interfaces and invariants

- `PersonalAssistantV0Transport::start_synthetic(&mut host,
SyntheticDisclosureAdmissionV1, SealedAccessCredentials)` obtains request
  bytes from the host, consumes the admission and two secret values exactly
  once, and owns the sole in-flight request. It accepts no URL, header, body,
  agent, instruction, provider, model, tool, profile, identity, limit, retry,
  fallback, or fixture input.
- `SyntheticDisclosureAdmissionV1` is a public opaque, non-cloneable,
  non-serializable, non-debuggable type with private state and **no production
  constructor in V0-7**. Only a `cfg(test)` constructor exercises fake model
  I/O. V0-11 may add the sole crate-private production mint after exact Rust
  disclosure validation. Thus the model transport is implemented/testable but
  cannot be started in production before the disclosure boundary closes.
- The separate example exposes only one fixed no-body Access authentication
  probe. It accepts no content/configuration and cannot call the model start
  path; V0-9 is its only live-use plan. Before any socket can open, the example
  prints this exact disclosure:

  ```text
  AUTHENTICATION-ONLY NETWORK CHECK. This sends no prompt or model request. It contacts the fixed Access-protected Cloudflare Worker route with the app's demo authentication headers to verify admission and denial controls. On the current Cloudflare Free plan, Access authentication metadata is retained for 24 hours and mandatory admin-action audit records are retained for 18 months. Type nothing unless you accept this check.
  ```

  It then prompts exactly `Type AUTHENTICATE to continue:` on an interactive
  terminal. Only the exact `AUTHENTICATE` response under acknowledgment version
  `personal-assistant-access-auth-probe@1` mints a private, one-use,
  non-cloneable `AccessAuthProbeDisclosureAdmissionV1`. A command flag,
  environment variable, piped/non-terminal input, default, timeout continuation,
  or synthetic-model acknowledgment cannot mint it. The probe consumes this
  distinct admission before credential projection and before the first network
  operation. It is never serialized, logged, or accepted by model transport.
  Any field/class/retention change requires a new acknowledgment version and a
  revised plan before another probe.

- The origin and path are compile-time application configuration selected by
  V0-5 and pinned visibly in Rust; they are not credentials. The implementation
  and target-Mac signed build must match the privately verified V0-5 inventory
  exactly. HTTPS and the exact host/port/path are mandatory. Redirects, caller or
  environment proxies, alternate DNS/host/port/scheme, certificate bypass,
  automatic decompression beyond bounded handling, and ambient authentication
  are disabled or denied.
- The only credential projection is the exact Cloudflare Access Client ID and
  Secret headers to that origin. Secret bytes are non-cloneable and cleared
  best-effort on every exit without a guaranteed-zeroization claim.
- At most one foreground request exists. Connect, idle, provider, cleanup, and
  total deadlines are actively driven by the transport's monotonic timer even
  when the UI does not poll and no byte arrives.
- Response bytes are incrementally capped before application-state retention;
  the host remains authoritative for JSON/event identity, sequence, event,
  aggregate output, terminal state, and late-event acceptance.
- Cancellation closes host ingress first and aborts the original in-flight
  HTTP request/socket. It never sends a second model/cancellation request.
  Ambiguous or failed abort remains `Cancelling`, retains transport ownership,
  rejects restart, and permits only a bounded trusted cleanup retry. A clean
  abort publishes one cancellation exactly once.
- Drop attempts abort; inability to prove teardown retains the process-wide
  lease fail-closed until process exit. No detached task or background
  continuation survives ownership.
- Logs expose only closed stage/outcome, bounded safe counters, and a Rust-owned
  opaque support correlation. URL, header, credential, prompt, output, frame,
  provider/native error, and stack are absent.

## Threats and tests

All automated tests use an in-process fake transport/TLS seam and fake secret
source; no socket or Keychain is opened. Cover fixed-origin construction,
wrong host/scheme/port, redirect, proxy environment, TLS/hostname failure,
one-time credentials, exact request bytes, partial/multibyte/SSE splits,
oversized headers/chunks/decompression, EOF, slow connect, idle/provider/total
deadline, cancellation before/during/after connect, terminal-versus-cancel
races, abort ambiguity and retry, late bytes, process-wide single-flight, no
retry/fallback, restart only after cleanup, canary redaction, absence of a
production synthetic-disclosure constructor, exact authentication-only
disclosure/version/input, rejection of wrong/default/piped acknowledgment
without a socket, one-use auth-probe admission, and strict type/flow separation
of the content-free auth probe from model start.

The gateway later must bind downstream disconnect to immediate upstream abort,
must not use `waitUntil`, and must discard late upstream frames. This transport
plan cannot claim that remote behavior; V0-13 proves it.

## Verification and manual gates

```bash
cargo test --manifest-path src-tauri/Cargo.toml personal_assistant_v0
cargo test --manifest-path src-tauri/Cargo.toml --test cloudflare_access_credential_boundary
cargo test --manifest-path src-tauri/Cargo.toml --test personal_assistant_v0_contract
cargo test --manifest-path src-tauri/Cargo.toml --test personal_assistant_v0_transport_contract
cargo build --manifest-path src-tauri/Cargo.toml --example personal_assistant_v0_access_auth_probe
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features -- -D warnings
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

On the target Mac, use repository-pinned tools to build and exercise only the
fake/offline transport, disclosure prompt without accepting it, deadline,
abort, restart, redaction, and no-permission paths. Cloudflare, OpenAI,
Keychain real values, external TLS, provider, and UI checks are `Not run`.

## Non-goals and rollback

No live request, real token, provider key/adapter, user text, Tauri command or
event, WebView, conversation UI, persistence, memory, durable audit, tool,
file/device effect, retry, fallback, second origin, or distribution. Rollback
reverts exactly the source/test/docs files and the reviewed manifest/lockfile
change; no external cleanup exists.

## Stop conditions and readiness

Stop if the V0-6 dependency diff changes; any unsafe/process/WebView/transitive
path is needed; origin/headers/config become caller-selected; a blocked read
cannot be aborted; cleanup cannot retain ownership; late data is accepted; or
content reaches logs. Also stop if production can construct
`SyntheticDisclosureAdmissionV1` before V0-11; if any auth-probe socket can
open without the exact interactive `personal-assistant-access-auth-probe@1`
acknowledgment; or if the auth probe can carry a body, reuse the synthetic-model
admission, or reach model start.

**Blocked.** V0-2, V0-3, V0-5, and V0-6 are unresolved. Approval of this plan
would not authorize real credentials or traffic.
