# V0-12 — OpenAI/Cloudflare no-traffic provider provisioning

Status: Blocked; provider/resource/secret authority is absent
Owner: Henry Dang
Last updated: 2026-08-28
Depends on: verified V0-10 and V0-11, current synthetic-provider evidence, and fresh external approval

## Goal and outcome

Provision the minimum private OpenAI project/key and install it as one
Cloudflare Worker secret while `PA_V0_TRAFFIC_ENABLED=false` and no route is
reachable. Configure and inspect bounded provider spend/rate controls,
vendor logging/retention, ownership, revocation, and rollback without sending a
provider request. This is D-064 Stage B only.

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
- `TROUBLESHOOTING_LOG.md`
- `docs/security/phase4-gateway-threat-model.md`
- `docs/plans/2026-08-28-personal-assistant-v0-provider-no-traffic-provisioning.md`
- `docs/increments/personal-assistant-v0-provider-no-traffic-provisioning.md`
  (new)
- `docs/reviews/2026-08-28-personal-assistant-v0-provider-no-traffic-provisioning-post-increment-review.md`
  (new)

No production source/test, manifest/lockfile, workflow, Tauri/frontend,
capability/CSP, signing, Keychain, model input, route, or public endpoint may
change. Repository evidence contains no secret, organization/project/account,
Worker/application/route, model endpoint, screenshot, billing detail, or
private evidence location.

## Exact external scope

- Reverify the exact OpenAI API organization/project, Responses endpoint,
  `gpt-5.6-luna` availability, streaming contract, region/residency if
  applicable, training/data-use setting, abuse-monitoring and prompt-cache
  retention, `store: false`, background-disabled behavior, and synthetic-only
  disclosure. Do not describe `store: false` as ZDR.
- Create one least-privilege project API key owned by the Worker secret store;
  it never enters the desktop, Keychain, WebView, source, CLI argument/history,
  environment file, screenshot, log, CI, or chat.
- Freeze the only approved key handoff as OpenAI Dashboard to Cloudflare
  Dashboard Worker-secret UI, one value and one attempt at a time. The owner
  views the OpenAI key once, disables screen sharing/recording, Universal
  Clipboard, and clipboard-history tooling, uses only the system pasteboard to
  paste it into the single target Worker secret field, immediately overwrites
  the pasteboard with a public canary, closes both source and target value
  fields, and confirms the Worker secret is write-only/unrecoverable. The key
  may not pass through shell, environment, file, editor, chat, Codex, CI,
  screenshot, browser history, password manager, or an ordinary saved form.
  The transient surfaces are the OpenAI Dashboard UI buffer, system pasteboard,
  Cloudflare Dashboard UI buffer, and final Cloudflare Worker secret store;
  Henry Dang owns their closure/cleanup and immediate key revocation on any
  ambiguity or exposure.
- Configure the narrowest available project model/rate/budget controls and one
  owner alert/incident procedure. Record whether each control is hard denial or
  advisory; no soft alert is treated as an enforced cap.
- Install exactly one provider-secret binding and retain the server-owned
  `PA_V0_TRAFFIC_ENABLED=false` binding. Only the literal `true` can admit a
  later request. Keep `workers_dev = false`, no route, and no provider request.
- Confirm Worker application logging/observability remains disabled and record
  the actual Access, Worker, OpenAI, and control-plane metadata retention/access
  constraints. Access/Worker runtime operational metadata must be content-free
  and retained at most seven days. Confirm the separately classified mandatory
  Cloudflare admin-action audit trail contains no prompt/output/secret and
  remains within D-094's synthetic-only acceptance of the current 18-month
  vendor retention. A class/field/retention change is a stop, not an advisory.
  No new Logpush, analytics, storage, queue, KV, R2, or Durable Object exists.

If exact global one-request/rate/budget containment cannot be achieved without
a new stateful resource, stop and record the blocker; do not provision that
resource under this plan.

## Verification and manual gates

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
python3 .codex/hooks/session_end_gate.py
```

Private OpenAI/Cloudflare control-plane checks are required and sanitized.
Provider request, prompt, network stream, signed target-Mac, Tauri, and UI
checks are `Not run`.

## Threats, rollback, and stop conditions

Threats include key exposure, an overbroad project key, soft-only spend limits,
unexpected vendor retention/logging, a reachable route, secret/version drift,
and a preview request. Rollback order is set
`PA_V0_TRAFFIC_ENABLED=false`, keep/remove route, revoke/delete the provider
key, delete the Worker secret and provider bindings, verify absence, then
delete newly created provider resources if approved. The flag/route deny new
admission only; no active-request abort is claimed in this no-traffic plan.

Stop on missing exact approval; provider/model/retention ambiguity; inability to
enforce the accepted budget/rate boundary; any real or synthetic request;
secret exposure; automatic preview; unexpected logging/storage; reachable
route; runtime metadata above seven days; content/secret in control-plane audit;
or inability to revoke/delete.

## Readiness

**Blocked.** V0-10/V0-11, current provider evidence, exact operational
controls, and external resource authority are absent. This plan does not
authorize traffic.
