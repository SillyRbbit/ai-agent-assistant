# V0-4 — local deny-only Cloudflare admission verifier

Status: Blocked; V0-1 is the sole Ready plan in the execution queue
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: D-094, verified/accepted V0-1 queue baseline, and an accepted dependency-free JWT/JWKS implementation design

## Goal and outcome

Add one source-controlled Cloudflare Worker boundary that verifies the exact
Cloudflare Access service-token application JWT and then returns a fixed
content-free disabled result. Automated tests use generated keys and injected
JWKS only. This increment creates no Cloudflare account object, route, token,
policy, secret, provider adapter, or traffic.

## Exact files

- `gateway/cloudflare-worker/src/personal-assistant-v0-worker.mjs` (new)
- `gateway/cloudflare-worker/test/personal-assistant-v0-worker.test.mjs` (new)
- `gateway/cloudflare-worker/wrangler.jsonc` (new; `workers_dev = false`, no
  account, route, secret, provider, or identifier)
- `package.json` (one pinned built-in test script only)
- `scripts/ci_change_scope.py`
- `scripts/tests/test_ci_change_scope.py`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
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
- `docs/plans/2026-08-28-personal-assistant-v0-gateway-admission.md`
- `docs/increments/personal-assistant-v0-gateway-admission.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-gateway-admission-post-increment-review.md`
  (new)

`package-lock.json`, workflows, runner selectors, permissions, Rust, frontend,
Tauri, capability, CSP, and every other gateway/deployment file are excluded.
If pinned Node/Web Crypto is insufficient, stop; do not add `jose`, Wrangler,
Miniflare, or any other dependency under this plan.

## Exact admission contract

The verifier accepts one request plus closed application configuration. It
reads exactly one bounded `Cf-Access-Jwt-Assertion` value and rejects duplicate
or alternate authorization material. It verifies:

- exactly three JWT segments; bounded header/payload/signature and an 8 KiB
  total token cap;
- header keys only `alg`, `typ`, and `kid`; `alg == "RS256"`, `typ == "JWT"`,
  bounded `kid`, and no `crit`, `jku`, `x5u`, or embedded key;
- the configured HTTPS team-domain JWKS endpoint only,
  `https://<team-domain>/cdn-cgi/access/certs`, never `iss` or request input;
- one exact `kid`, `kty == "RSA"`, `use == "sig"`, `alg == "RS256"`, and an
  RSA modulus of at least 2048 bits;
- `iss` exactly equals the configured team issuer;
- `aud` is an array containing exactly one value equal to the configured
  Access application AUD;
- `type == "app"`, `common_name` exactly equals the configured expected
  service-token Client ID, and `sub == ""`;
- required integer NumericDates `iat` and `exp`; optional `nbf`, when present,
  is an integer; `0 < exp - iat <= 900`, `iat <= now + 60`,
  `exp > now - 60`, and `nbf <= now + 60`; and
- bounded additive signed claims are ignored and grant no authority. Duplicate
  JSON keys, non-finite/nonnumeric dates, ambiguous encodings, or unexpected
  header fields deny closed.

`nbf` is not required because Cloudflare's documented service-token JWT
example omits it. Service-token Client ID/Secret lifetime remains at most 30
days and is distinct from the at-most-15-minute application JWT lifetime.

The JWKS policy is module-memory only: 64 KiB response cap, at most four keys,
five-second fetch deadline, at most 15-minute freshness, no redirect, and no
KV, Durable Object, R2, Cache API, or persistence. Unknown `kid` permits one
synchronous forced JWKS refresh, not a request/provider retry. Refresh failure
may use only an unexpired known-key cache; absent, expired, or still-unknown
keys deny.

The global `PA_V0_TRAFFIC_ENABLED` binding is evaluated before JWT parsing or
any fetch. Only the exact string `true` enables admission; missing, malformed,
or `false` denies. When disabled, the only output is a fixed
`gateway_disabled` response with one opaque Worker correlation and no
request-derived content. The verifier requires exactly four closed deployment
bindings—`PA_V0_ACCESS_ISSUER`, `PA_V0_ACCESS_AUD`,
`PA_V0_ACCESS_JWKS_URL`, and `PA_V0_EXPECTED_ACCESS_CLIENT_ID`—and rejects a
missing, duplicate, empty, malformed, or internally inconsistent binding.
V0-5 owns the first three and V0-8 owns the expected Client ID; neither can
enable traffic. When enabled in a later plan, the only upstream allowed by this
increment is the fixed Cloudflare JWKS fetch; OpenAI/provider egress remains
structurally absent.

## Invariants, threats, and tests

- Access headers are untrusted until independent signature and claim checks
  pass; neither Access policy nor a valid JWT can enable provider work here.
- The Worker never parses a body, reflects headers/claims, calls `waitUntil`,
  logs request content, or stores state.
- Fixed tests cover valid current/previous keys, rotation, forced refresh,
  unknown key, algorithm confusion, corrupted signature, every exact claim and
  time boundary, duplicate JSON/header material, size caps, missing config,
  disabled-first behavior, no provider fetch, and redacted fixed outputs.
- Classifier tests prove every `gateway/**` production path selects frontend,
  Rust, and audit jobs; the exception allowlist stays empty. Existing docs-only,
  frontend-only, audit-only, isolated-Rust, schedule, dispatch, deletion, and
  unsafe-path behavior remains exact.
- Repository-health tests pin `workers_dev = false`, default-off admission,
  exact binding names/deny behavior, absent provider fetch/secret/content log,
  and the exact verifier surface.

## Verification and manual gates

```bash
npm run test:gateway-admission
python3 -m unittest scripts.tests.test_ci_change_scope -v
python3 -m unittest scripts.tests.test_repository_health -v
npm run verify
npm audit --audit-level=low
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Manual inspection must prove no external resource, reachable route, credential,
provider configuration, live JWKS fetch, or traffic exists. Target-Mac,
Cloudflare, OpenAI, signing, and network checks are `Not run`.

## Non-goals and rollback

No provisioning, service token, Service Auth policy, DNS, route, deployment,
credential ingestion, provider key/request, Rust transport, Tauri, UI,
persistence, tool, file access, or device effect. Rollback reverts only the
exact repository files; no external cleanup exists.

## Stop conditions

Stop if a dependency or lockfile is needed; the claim/JWKS contract cannot be
implemented exactly; gateway paths escape all three CI jobs; request-derived
data can reach responses/logs; provider/body handling appears; a public route
is configured; or any external action is requested.

## Acceptance and readiness

- [ ] Generated-key verifier tests and repository trust-boundary tests pass.
- [ ] The source is deny-only, default-off, content-free, and provider-free.
- [ ] Independent architecture/security review has no blocker.

**Blocked.** It is independently implementable only after V0-1 is accepted as
the queue baseline and the dependency-free design is reviewed; no work starts
automatically.
