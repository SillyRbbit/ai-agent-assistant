# V0-13 — live synthetic-text Stage C rehearsal

Status: Blocked; no traffic authority or complete prerequisite chain exists
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-9 through V0-12, fresh V0-8 credential window, and exact Stage C approval

## Goal and outcome

On the signed target Mac, perform the first and only live synthetic-text proof:
an explicit acknowledged Start sends the fixed application-owned fixture to
OpenAI through the Access-protected Cloudflare Worker, streams bounded text,
and proves closed terminal behavior and full rollback. This is not a usable
personal assistant and authorizes no real/personal content.

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
- `docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md`
- `docs/increments/personal-assistant-v0-live-synthetic-rehearsal.md` (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal-post-increment-review.md`
  (new)

No production source/test, dependency/lockfile, Tauri/frontend, capability/CSP,
permission, gateway code/config shape, workflow, signing state, or new cloud
resource may change. Private credentials, identifiers, URLs, raw content/logs,
screenshots containing private state, and evidence locations stay outside Git.

## Preflight gates

- Reconfirm exact commit/toolchains, signed app identity, V0-8 token/policy and
  Keychain ACL/lifetime, V0-12 provider project/key/model/limits, deployed
  source hash, route/origin/TLS/JWT bindings,
  `PA_V0_TRAFFIC_ENABLED=false`, and
  rollback owners.
- Reconfirm current OpenAI synthetic retention/data-use evidence and actual
  Cloudflare Access/Worker/control-plane log retention. Confirm no custom
  content log/observability/storage exists. Access/Worker runtime metadata must
  be content-free and at most seven days; the separately classified mandatory
  admin-action audit trail must contain no app content/secret and match D-094's
  current synthetic-only 18-month acceptance.
- Display the exact V0-11 disclosure and fixed fixture. The checkbox begins
  unchecked. Rust must reject Start without the exact disclosure version and
  may write no socket before consuming its private admission.
- Prohibit personal/sensitive data. No composer or alternate input is present.
  One explicit Start equals one model request; there is no automatic preflight,
  retry, fallback, continuation, or second cancel request.

## Rehearsal matrix

Record each as Passed, Failed, or Not run with sanitized evidence:

1. `PA_V0_TRAFFIC_ENABLED=false` and disabled-route denial before activation;
   map only the fixed route, re-prove false-flag denial, then set the flag to
   exact `true` for the bounded rehearsal window.
2. Explicit disclosure acknowledgment followed by one fixed-fixture success;
   verify ordered stream, exact final equality, 128-event/output caps, no tool
   event, and one terminal state.
3. Closed failure through a safe pre-provider control
   (`PA_V0_TRAFFIC_ENABLED=false` before a fresh start) and, only where
   practical without changing scope, one upstream closed error. The false flag
   proves new-admission denial only.
4. Explicit cancellation during connect and streaming, each in a fresh run;
   UI closes ingress, Rust aborts the original request/socket, Worker aborts
   upstream on disconnect, no `waitUntil` continues work, cleanup terminalizes,
   and all late frames are rejected.
5. Connect/idle/provider/total deadline paths where practical, network loss,
   offline start, route disable, Access revocation, provider-key revocation,
   rate/budget denial, busy denial, and restart only after cleanup.
6. Route-away/back and minimize/hide/restore snapshot recovery,
   keyboard/focus, narrow/native resize, 200% zoom, light/dark, reduced motion,
   long output scroll, accessibility semantics, native/WebView console
   redaction, no permission prompt, no file, durable state, tool, device effect,
   or residual active request.
7. Vendor consoles/log surfaces show only accepted operational metadata; no
   fixture, instruction, output, token, JWT, claim, header, provider body/ID,
   raw error, or stack is present.

Unavailable controls are `Not run`, never inferred. A required row that cannot
be observed blocks completion rather than being replaced with fixture evidence.

## Verification

```bash
npm ci
npm audit --audit-level=low
npm run verify
npm run security:scan
npm run repository:check
npm run docs:check
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Use only repository-pinned Node/npm/Rust on the target Mac. Start the native app
through the prescribed Tauri command, capture sanitized results, and stop every
development process after the matrix.

## Rollback and stop conditions

Rollback first sets `PA_V0_TRAFFIC_ENABLED=false` and disables/removes the route
to deny new admission immediately. It then closes local ingress and triggers
the owned cancellation path for any active run; Access-token and OpenAI-key
revocation follow immediately after abort initiation without waiting for
teardown. The owner waits only for the bounded active cleanup, retaining cleanup
ownership on ambiguity, then deletes Keychain items, proves denial/missing
state, removes policy/secret/bindings, and deletes newly approved resources
where required. The traffic flag, route removal, and credential revocation are
never claimed to abort an already admitted request; the owned controller is the
active-request kill path.

Stop immediately on any real/personal input, unacknowledged transmission,
caller-selected configuration, provider/tool event, retry/fallback, content
log/reflection, unknown terminal state, abort/cleanup ambiguity, late accepted
event, unavailable new-admission flag, failed revocation/removal, permission prompt,
device effect, runtime metadata above seven days, content/secret in the
control-plane audit class, or source/config change need. Do not remediate within
rehearsal.

## Readiness

**Blocked.** This is the only Stage C live synthetic increment. It requires a
fresh explicit traffic approval after every prerequisite closes. Success does
not authorize milestone 2 or real prompts.
