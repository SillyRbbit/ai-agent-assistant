# V0-9 — authentication-only Access rehearsal

Status: Blocked; prerequisite source, identity, credentials, and external authority are absent
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-4, V0-5, V0-7, V0-8, and a fresh traffic approval

## Goal and outcome

Temporarily expose one Access-protected route and send only fixed, zero-content
authentication probes from the approved signed target-Mac app. Prove the real
JWT/JWKS boundary and credential denial/revocation before any provider secret,
body parsing, model request, prompt, or UI exists. Finish with the route
disabled, token revoked, and Keychain items removed unless the owner separately
approves a bounded handoff window.

## Exact repository files

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
- `docs/security/phase4-gateway-threat-model.md`
- `docs/plans/2026-08-28-personal-assistant-v0-access-auth-rehearsal.md`
- `docs/increments/personal-assistant-v0-access-auth-rehearsal.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-access-auth-rehearsal-post-increment-review.md`
  (new)

No production source/test, manifest/lockfile, Tauri/frontend, capability/CSP,
provider adapter/secret, or workflow file may change. Private identifiers,
credentials, JWTs, claims, headers, routes, screenshots, raw logs, and evidence
locations never enter repository evidence.

## Exact rehearsal

- Reconfirm the V0-5 deployment hash, default-off state, exact issuer/AUD/JWKS
  bindings, V0-8 expected-Client-ID binding, their sanitized fingerprints, one
  Service Auth policy, one at-most-30-day token, and at-most-15-minute
  application JWT lifetime. The policy's token-resource-ID fingerprint and the
  expected-client-ID fingerprint must be distinct and must be traced to the
  same reviewed token record; equality between those two values is a failure.
  Every binding must match V0-4's closed grammar before a route is mapped.
- Temporarily map one fixed HTTPS route and enable only the V0-4 auth-check
  path at the exact V0-5 origin/path. `PA_V0_TRAFFIC_ENABLED=true` is set only
  for the acknowledged probe window and restored to `false` before route
  removal. The path validates the Access JWT before reading a body and returns
  the fixed `authenticated_provider_disabled` result. It has no provider
  secret/fetch, body handler, storage, queue, retry, or `waitUntil`.
- From the signed target Mac, each explicit probe opens one request with no
  query/body/user content. Before every probe, the example displays exactly:

  ```text
  AUTHENTICATION-ONLY NETWORK CHECK. This sends no prompt or model request. It contacts the fixed Access-protected Cloudflare Worker route with the app's demo authentication headers to verify admission and denial controls. On the current Cloudflare Free plan, Access authentication metadata is retained for 24 hours and mandatory admin-action audit records are retained for 18 months. Type nothing unless you accept this check.
  ```

  It then requires exact interactive input at `Type AUTHENTICATE to continue:`
  for acknowledgment version `personal-assistant-access-auth-probe@1`. Trusted
  Rust consumes the resulting one-use admission before credentials or a socket;
  there is no flag, environment, default, piped-input, or reusable bypass. Run
  closed cases separately: correct credential, missing items, wrong token,
  expired application JWT where practical, unknown or rotated `kid` where
  practical, and revoked token. Never auto-preflight before a later model
  request; one live model run must still use only its original request.
  Retention or field drift requires a new acknowledgment version and a revised
  plan before another probe.

- Observe one fixed JWKS fetch policy: configured HTTPS endpoint, no redirect,
  five-second/64 KiB/four-key limits, 15-minute in-memory cache, and one forced
  refresh for unknown `kid`. Record only closed outcomes and safe counts.
- Inspect Access/Worker logs and confirm no header, JWT, claim, credential,
  query, body, or content is recorded/reflected. Record actual vendor retention
  and account-admin/audit-log constraints. Access/runtime operational metadata
  must be at most seven days. The separately classified, content-free
  Cloudflare admin-action audit class may retain the D-094-accepted current
  18-month period for the synthetic lane; any secret/content field or changed
  class/retention stops before provider traffic.

## Manual acceptance matrix

Each row is Passed, Failed, or Not run with a sanitized reason: exact disclosure
copy/version; wrong, absent, default, or non-terminal acknowledgment makes zero
network attempts; one acknowledgment authorizes one probe only; authorized
signed app; unsigned/unauthorized app; missing Client ID; missing Secret; wrong
token; expired/rotated JWT where practical; revoked token; disabled Worker;
disabled route; offline/JWKS failure; fixed response; no prompt/device effect;
no content-bearing log; no residual request.

## Verification

```bash
cargo build --manifest-path src-tauri/Cargo.toml --example personal_assistant_v0_access_auth_probe
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

## Rollback and stop conditions

Rollback order is set `PA_V0_TRAFFIC_ENABLED=false`, disable/remove the route,
and revoke the Access token immediately to deny new authentication. Then wait
for any already admitted probe to close, retaining cleanup ownership on
ambiguity; token revocation is not claimed to abort that active probe. Remove
the Service Auth policy, delete both Keychain items, delete the expected-
Client-ID binding, prove denial/missing state, then optionally delete the
Access application and Worker.

Stop on missing traffic approval; absent/wrong/non-interactive disclosure
acknowledgment; a socket before Rust consumes the one-use admission; provider
configuration; body/content handling; an unexpected hostname/redirect; TLS/JWT
mismatch; log/reflection leak; authentication bypass; inability to
abort/disable/revoke/delete; a permission prompt/device effect; or any
personal/sensitive input. Also stop if any installed binding differs from its
V0-5/V0-8 owner or any runtime log exceeds seven days.

## Readiness

**Blocked.** This is the first authorized network test in the program and
requires all named prerequisites plus a fresh external-traffic approval. Its
success does not authorize OpenAI traffic.
