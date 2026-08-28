# Personal Assistant v0 capability planning post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git diff --check",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "docs/PROJECT_DIRECTION.md",
    "docs/increments/personal-assistant-v0-capability-planning.md",
    "docs/plans/2026-08-28-personal-assistant-v0-access-auth-rehearsal.md",
    "docs/plans/2026-08-28-personal-assistant-v0-empty-tool-turn.md",
    "docs/plans/2026-08-28-personal-assistant-v0-gateway-admission.md",
    "docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md",
    "docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md",
    "docs/plans/2026-08-28-personal-assistant-v0-no-traffic-provisioning.md",
    "docs/plans/2026-08-28-personal-assistant-v0-program.md",
    "docs/plans/2026-08-28-personal-assistant-v0-provider-no-traffic-provisioning.md",
    "docs/plans/2026-08-28-personal-assistant-v0-real-demo-credential-ingestion.md",
    "docs/plans/2026-08-28-personal-assistant-v0-real-prompt-activation.md",
    "docs/plans/2026-08-28-personal-assistant-v0-session-host.md",
    "docs/plans/2026-08-28-personal-assistant-v0-signed-client.md",
    "docs/plans/2026-08-28-personal-assistant-v0-synthetic-gateway.md",
    "docs/plans/2026-08-28-personal-assistant-v0-synthetic-transport.md",
    "docs/plans/2026-08-28-personal-assistant-v0-tauri-presentation.md",
    "docs/reviews/2026-08-28-personal-assistant-v0-capability-planning-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": false,
      "category": "Roadmap",
      "effort": "Future evidence-gated increments",
      "milestone": "V0-9 through V0-14",
      "risk": "Stale provider, authentication, logging, or retention assumptions could be mistaken for current traffic or real-content authority.",
      "severity": "Advisory",
      "summary": "All external and real-content lanes remain Blocked and require fresh primary evidence plus separate owner approval."
    }
  ],
  "increment_id": "personal-assistant-v0-capability-planning",
  "manual_verification": [
    {
      "check": "Target-Mac native application, viewport, accessibility, theme, reduced motion, focus, resize, console, permission, and device-effect validation",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Signed identity, Keychain, credential transfer, rotation, revocation, and cleanup validation",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Cloudflare/OpenAI provisioning, authentication, provider traffic, vendor logs, kill switch, and rollback validation",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Ready",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run docs:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run repository:check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run security:scan",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-08-28
Increment: `personal-assistant-v0-capability-planning`
Branch: `main`
Baseline: `41ff7141007c8c0a684a5e2434ecf81dd4707418`

## Executive summary

PASS WITH ADVISORIES. This owner-authorized documentation-only increment is
complete. It defines D-094, the exact Personal Assistant v0 boundary, and a
dependency-ordered fourteen-increment program. V0-1 alone is Ready for separate
owner implementation approval and a fresh gate; V0-2 through V0-14 remain
Blocked. No Personal Assistant source path is implemented.

The initial workspace was clean synchronized `main`; `HEAD` and `origin/main`
were both `41ff7141007c8c0a684a5e2434ecf81dd4707418` after fetch. Git metadata was
healthy; only unreachable dangling objects were observed. The repository-
prescribed toolchains were Node 26.3.0, npm 11.16.0, and Rust/Cargo 1.90.0.

No production source, test, dependency, manifest, lockfile, capability, CSP,
permission, credential, Keychain/signing state, Cloudflare/OpenAI resource,
provider request, network transmission, Tauri IPC, persistence, memory, tool,
device effect, commit, or publication was created or changed.

## Verification results

- `npm run docs:check` — Passed. Prettier and repository link checks passed.
- `npm run repository:check` — Passed. Repository-health all checks passed.
- `npm run security:scan` — Passed. Repository secret scan passed.
- `git diff --check` — Passed with no whitespace error.
- `python3 .codex/hooks/session_end_gate.py` — Passed with no conflicts or
  staged paths; the complete changed set is documentation only.
- Target-Mac native UI/accessibility/lifecycle checks — Not run; no executable
  UI or native behavior changed.
- Signed identity, Keychain, credentials, Cloudflare/OpenAI, network, vendor
  logging, kill-switch, and rollback checks — Not run; those actions were
  prohibited and remain separately approval-bound.
- `npm run verify`, `npm audit`, product/Rust/frontend tests, Tauri build/dev,
  and hosted CI — Not run; documentation-tier policy did not require them and
  this increment changed no executable or dependency surface.

## Architecture findings

PASS. The plans align with current source: `NativeAgentRuntime` remains the
sole/default transport-free runtime, `AgentRuntime::start` remains the sole
start boundary, the current initial turn still owns two fixed tool schemas, and
no Personal Assistant host, transport, provider adapter, Tauri contract, or
live conversation reducer exists.

V0-1 freezes a distinct empty-tool request, truthful Idle/Starting/Streaming/
terminal projection, exact returned-runtime identity/status validation, and
F-02 run-plus-lease quarantine. All three existing concrete Native frame/
approval methods retain exact signatures and closed wrong-profile behavior.
V0-2 freezes mutable snapshot/update/cancel APIs, closed non-Serde Rust types,
one reducer, synchronous monotonic time, and Drop-time quarantine ownership.
Independent architecture/readiness review found no blocker.

## Security findings

PASS. The program keeps Rust authoritative for identity, instructions,
provider/model profile, empty tools, limits, cancellation, cleanup, and late-
event rejection. It separates the authentication-only disclosure admission
from the synthetic model disclosure admission, and it preserves F-01, F-02,
F-07, F-08, F-12, and F-15.

Cloudflare's service-token resource `id` and `client_id` are distinct; their
policy, JWT, Worker-binding, Keychain, transfer, rotation, and revocation
ownership is exact. The future Worker admits only one closed Responses event
grammar, records rather than assumes its sequence origin, keeps provider IDs
private, rejects refusal/reasoning/tool material, owns abort/late-event
containment, and denies new traffic before teardown/revocation waits.

Current primary evidence supports the selected synthetic direction but not
activation: the OpenAI model page identifies Responses/streaming support, the
Responses reference defines the event families, OpenAI's data guide requires
truthful retention/ZDR distinction, and Cloudflare documents Access JWT and log
semantics. V0-10/V0-12/V0-13 must reverify all facts before readiness or
traffic. See the official [GPT-5.6 Luna model page](https://developers.openai.com/api/docs/models/gpt-5.6-luna),
[Responses streaming events](https://developers.openai.com/api/reference/resources/responses/streaming-events),
[OpenAI data controls](https://developers.openai.com/api/docs/guides/your-data),
[Cloudflare Access application tokens](https://developers.cloudflare.com/cloudflare-one/access-controls/applications/http-apps/authorization-cookie/application-token/),
and [Cloudflare Access logs](https://developers.cloudflare.com/cloudflare-one/insights/logs/).

## Code-health findings

PASS. The complete diff is documentation-only, formatted, link-valid, secret-
scan clean, and internally consistent. Current, synthetic, real-content,
action-taking, Ready, and Blocked states are distinct. No implementation,
dependency, lint suppression, unsafe code, dead-code seam, or generic authority
was added.

## Technical debt

No current implementation debt was introduced. One Roadmap advisory remains:
future provider, gateway, identity, credential, retention, logging, and target-
Mac facts are deliberately unproved. The exact later plans own that work and
remain Blocked; the advisory does not block this planning increment or V0-1's
transport-free implementation readiness.

## Roadmap findings

Ready. V0-1 is the smallest genuinely unblocked prerequisite: one sealed,
transport-free, application-owned empty-tool turn and volatile host, with no
I/O, caller configuration, dependency, IPC, credential, external state, or
product claim. It still needs explicit owner implementation approval and a
fresh increment gate. V0-2 through V0-14 remain Blocked. Milestone 1 is Blocked
through V0-13, milestone 2 is Blocked at V0-14, and milestone 3 has no ExecPlan.
The local-model alternative is compared only as an unauthorized option and its
D-060 through D-094 decision impact is recorded.

## Completion decision

PASS WITH ADVISORIES

## Next-increment readiness

Ready. The exact next candidate is
[`V0-1 — sealed Personal Assistant empty-tool turn and volatile host`](../plans/2026-08-28-personal-assistant-v0-empty-tool-turn.md).
Do not implement it until the owner separately approves that exact source
increment and a fresh gate begins.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PRODUCT_REQUIREMENTS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `docs/PROJECT_DIRECTION.md`
- `docs/increments/personal-assistant-v0-capability-planning.md`
- `docs/plans/2026-08-28-personal-assistant-v0-access-auth-rehearsal.md`
- `docs/plans/2026-08-28-personal-assistant-v0-empty-tool-turn.md`
- `docs/plans/2026-08-28-personal-assistant-v0-gateway-admission.md`
- `docs/plans/2026-08-28-personal-assistant-v0-https-dependency-decision.md`
- `docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md`
- `docs/plans/2026-08-28-personal-assistant-v0-no-traffic-provisioning.md`
- `docs/plans/2026-08-28-personal-assistant-v0-program.md`
- `docs/plans/2026-08-28-personal-assistant-v0-provider-no-traffic-provisioning.md`
- `docs/plans/2026-08-28-personal-assistant-v0-real-demo-credential-ingestion.md`
- `docs/plans/2026-08-28-personal-assistant-v0-real-prompt-activation.md`
- `docs/plans/2026-08-28-personal-assistant-v0-session-host.md`
- `docs/plans/2026-08-28-personal-assistant-v0-signed-client.md`
- `docs/plans/2026-08-28-personal-assistant-v0-synthetic-gateway.md`
- `docs/plans/2026-08-28-personal-assistant-v0-synthetic-transport.md`
- `docs/plans/2026-08-28-personal-assistant-v0-tauri-presentation.md`
- `docs/reviews/2026-08-28-personal-assistant-v0-capability-planning-post-increment-review.md`

## Exact commands executed

- `npm run docs:check` — Passed.
- `npm run repository:check` — Passed.
- `npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `python3 .codex/hooks/session_end_gate.py` — Passed.
