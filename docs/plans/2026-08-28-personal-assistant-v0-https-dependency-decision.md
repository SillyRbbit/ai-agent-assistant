# V0-6 — direct Rust HTTPS dependency decision

Status: Blocked; no reviewed direct client is selected
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: V0-1/V0-2 architecture baseline and fresh dependency evidence

## Goal and outcome

Select or reject one direct Rust HTTPS/streaming implementation for the fixed
Cortexa gateway origin. The current Tauri crate has no direct HTTP client;
transitive lockfile entries do not authorize use. This documentation-only
decision neither changes a manifest/lockfile nor sends traffic.

## Exact files

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md`
- `docs/increments/personal-assistant-v0-https-dependency-decision.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-https-dependency-decision-post-increment-review.md`
  (new)

No production source, test, `Cargo.toml`, `Cargo.lock`, npm file, capability,
CSP, permission, workflow, credential, cloud resource, or external system may
change.

## Decision evidence required

Compare the smallest maintained direct-client candidates using primary source,
license, advisory, and target-Mac evidence. The selected record must pin:

- exact crate/version/features and the resulting complete lockfile change;
- TLS backend and root store, minimum TLS behavior, certificate/hostname
  verification, and no certificate bypass/pinning invention;
- fixed-origin enforcement, redirect policy, environment proxy behavior,
  connection pooling, DNS behavior, response decompression, and HTTP versions;
- incremental response-body/SSE reads with byte caps before application-state
  retention, backpressure, and no full unbounded buffering;
- connect, idle, provider, and total deadlines using a monotonic clock;
- cancellation that closes the original in-flight request/socket—never a
  second model request—and a terminal cleanup result that Rust can quarantine;
- platform support, build impact, transitive native code, MSRV, maintenance,
  audit posture, and removal/rollback; and
- why a dependency-free implementation, WebView `fetch`, shell/subprocess,
  plugin, or transitive/private API is rejected.

## Threats and tests for the later implementation

The decision must make V0-7 able to test redirects, proxy variables, wrong
host/scheme/port, TLS failure, slow connect, idle stream, oversized headers and
chunks, decompression expansion, disconnect races, cancellation, cleanup
failure, late bytes, and canary redaction. No provider-specific type may enter
the runtime or Tauri contract.

## Verification

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

`npm audit`, Cargo tests/builds, target-Mac transport, and network checks are
`Not run`; no dependency or source changes in this decision increment.

## Rollback and stop conditions

Rollback reverts the exact documentation decision/closeout files. Stop if no
candidate can provide bounded streaming and prompt socket abort without unsafe
TLS/proxy/redirect behavior; if license/advisory/target-Mac evidence is
ambiguous; if more than one runtime client or Tauri plugin is required; or if
evaluation would require manifest, lockfile, credential, or network changes.

## Readiness

**Blocked.** A reviewed candidate and complete dependency evidence do not yet
exist. V0-7 cannot proceed without an accepted decision and separate source
approval.
