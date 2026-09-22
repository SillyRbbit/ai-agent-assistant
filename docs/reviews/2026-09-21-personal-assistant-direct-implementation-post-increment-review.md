# Direct Personal Assistant implementation review

Date: 2026-09-21. Increment: `personal-assistant-direct-implementation`.
Workspace: `/private/tmp/cortexa-personal-assistant-direct-implementation`.
Detached baseline: `0ed15810e90b6a4bd312a8096c61b0abb1ab7eff`.

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "personal-assistant-direct-implementation",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "files_changed": [
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md",
    "docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/Cargo.lock",
    "src-tauri/Cargo.toml",
    "src-tauri/src/agent/gateway_request.rs",
    "src-tauri/src/agent/native_runtime.rs",
    "src-tauri/src/agent/runtime.rs",
    "src-tauri/src/lib.rs",
    "src-tauri/src/personal_assistant_direct.rs",
    "src-tauri/src/personal_assistant_direct_tauri.rs",
    "src-tauri/src/personal_assistant_v0.rs",
    "src/App.test.tsx",
    "src/features/conversations/ConversationWorkspace.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.test.tsx",
    "src/features/conversations/PersonalAssistantDirectDemo.tsx",
    "src/infrastructure/tauri/personal-assistant-direct-client.test.ts",
    "src/infrastructure/tauri/personal-assistant-direct-client.ts"
  ],
  "commands_executed": [
    "npm ci --ignore-scripts",
    "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
    "npm run test:frontend -- src/features/conversations/PersonalAssistantDirectDemo.test.tsx src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/App.test.tsx",
    "python3 -B -m unittest scripts.tests.test_repository_health -q",
    "npm run verify",
    "npm audit",
    "npm audit --omit=dev",
    "/private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit audit --no-fetch --json --file src-tauri/Cargo.lock > /private/tmp/cortexa-direct-implementation-cargo-audit-final.json\naudit_exit=$?\npython3 scripts/cargo_audit_gate.py /private/tmp/cortexa-direct-implementation-cargo-audit-final.json --cargo-audit-exit \"$audit_exit\"",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py"
  ],
  "verification": [
    {
      "command": "npm ci --ignore-scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run test:frontend -- src/features/conversations/PersonalAssistantDirectDemo.test.tsx src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/App.test.tsx",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B -m unittest scripts.tests.test_repository_health -q",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm audit --omit=dev",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "/private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit audit --no-fetch --json --file src-tauri/Cargo.lock > /private/tmp/cortexa-direct-implementation-cargo-audit-final.json\naudit_exit=$?\npython3 scripts/cargo_audit_gate.py /private/tmp/cortexa-direct-implementation-cargo-audit-final.json --cargo-audit-exit \"$audit_exit\"",
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
      "command": "git diff --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 -B .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    }
  ],
  "manual_verification": [
    {
      "check": "Exact 30-path scope, additive historical text, dependency versions and preservation of all predecessor checkouts",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Composed architecture, security, code-health, technical-debt and readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Native window, Settings IPC, Conversations interaction and existing Research/Knowledge GUI smoke",
      "required": false,
      "status": "Manual verification pending"
    },
    {
      "check": "Actual credential ingestion and paid native OpenAI request",
      "required": false,
      "status": "Not run"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Native GUI interaction and actual provider activation are unverified.",
      "risk": "Builds and deterministic fixtures do not establish native IPC, account access or a paid successful response.",
      "effort": "One owner-led native rehearsal",
      "milestone": "Before claiming live-demo verification",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Temporary native environment credentials and request abort have the explicit private-demo limitations recorded in D-128.",
      "risk": "Same-user inspection, inherited environments, nonzeroized memory, OS DNS activity and provider computation/billing are not prevented or proved quiescent; HTTP/TLS libraries allocate before application bounds.",
      "effort": "Retain disclosure and owner-only synthetic use",
      "milestone": "Reconsider before any real-content or distributed use",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "The inherited D-127 Cargo vulnerability and warning baseline remains accepted unchanged.",
      "risk": "Two quick-xml vulnerabilities and eight warning tuples remain in the dependency graph; a passing exact gate is not zero vulnerabilities.",
      "effort": "Separate owner-approved dependency work when selected",
      "milestone": "Existing dependency-risk tracking",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ]
}
-->

## Executive summary

Implemented the owner's bounded direct synthetic demo through existing
Conversations, three closed Tauri commands, the existing Personal Assistant
host/runtime and one fixed OpenAI Responses HTTPS adapter. Both checkpoints
are implemented and automated local acceptance passed. Result:
**PASS WITH ADVISORIES**. Native GUI interaction and an actual paid native
response remain unverified. No real credential access or provider request,
commit, push, branch, publication or deployment occurred.

## Scope and boundaries

Exactly 30 changed repository paths: 17 source/test/dependency paths and
13 documentation paths, listed below. Original local main `87d52a6`, its seven
unpublished commits and 43 existing dirty/untracked paths are unchanged.
Earlier publication, reconciliation, successful audit and terminal-failed
worktrees retain their heads, statuses and dirty-byte fingerprints. The
documentation-organization worktree remains at `d1a6ad1`; the baseline/scope
worktree remains detached at `0ed1581` with its valid complete marker.
No finalized predecessor report was edited. All 12 pre-existing documentation
bodies remain byte-identical after their headings; updates are additive.

D-128 records the owner's explicit private runtime-key exception and superseding
abort contract. Historical Cloudflare/local-candidate decisions are retained.
D-125/M1/M2 remain parked. The UI does not activate other agents, tools,
approvals or graph events; Structured view remains absent. No CSP, capabilities,
native permissions, workflows, hooks, skills, harnesses, Cargo audit baseline
or JavaScript package changes occur.

## Verification results

- `npm ci --ignore-scripts`: **Passed**, 288 packages, zero vulnerabilities.
  No untrusted install-script bypass or package-policy change was introduced.
- Focused Rust Personal Assistant tests: **Passed**, 39 tests. Initial
  checkpoint A had 36; the additional tests prove actual provider identity
  reaches the host, known metadata handling and owned-future drop on timeout/Stop.
- Focused frontend client/panel/App tests: **Passed**, 48 tests; after strict
  async-test callback corrections the panel's four tests passed again.
- Focused repository-health regression module: **Passed**, 52 tests.
- `npm run verify`: **Passed** on the final source and lockfile. This includes
  formatting, repository checks, strict ESLint/Clippy, 74 hook tests, 83 repository
  tests, 377 frontend tests across 24 files, 310 Rust unit tests, 247 integration
  tests, TypeScript/frontend build and the Tauri no-bundle release build.
  The integration invocation also repeats the 310 library tests. One existing
  Hermes executable probe remains intentionally ignored because it requires
  explicit opt-in and an operator-supplied pinned executable; it was not
  enabled, waived or reported as passing. The release build finished in 1m54s.
- Full and production npm audits: **Passed**, zero vulnerabilities.
- Pinned cargo-audit 0.22.2 refreshed RustSec data at commit
  `d5c17953a895cf19e8d3ce66eaa42b6fcfe1fb16`. After removing an unnecessary
  tokio macro feature, a final audit reused that current database with
  `--no-fetch`. The repository gate: **Passed**, exactly the inherited two
  quick-xml vulnerability tuples and eight warnings. Raw cargo-audit exit 1
  is the expected finding status; it is not a claim of zero Rust vulnerabilities.
- Lock inspection: only reqwest 0.13.4 is replaced, by pinned 0.13.5.
  All unrelated existing versions remain. The final graph adds 38 lock nodes
  for the TLS/client feature closure, including the new reqwest version.
  Tokio 1.52.3 was already locked; it is now an explicit runtime/time dependency.
  Reqwest declares Rust 1.85; inspected newly downloaded packages do not exceed
  the repository's Rust 1.88 minimum. The actual tested toolchain is 1.90.
  Foreign-target-only packages and execution were not tested on other platforms.
- Documentation/link, repository, secret, whitespace and session inventory
  checks: **Passed**. No conflicts, staged files, secret artifacts or unexpected
  scope. Final report-only changes receive focused documentation checks without
  repeating the successful application suite.
- Native visual/IPC interaction: **Manual verification pending**. No native
  window interaction or browser visual walkthrough was performed in this
  increment; the prior unbundled-app automation limitation and native GUI
  advisory remain. DOM tests are deterministic browser-environment evidence only.
- Actual credential ingestion, account/model access, billing setup and paid
  Responses request: **Not run**, explicitly outside this task's authorization.

Intermediate failures were diagnosed and repaired within the task: offline
resolution lacked tokio-macros before minimal online resolution; initial
integration compilation needed its not-yet-added adapter types; new frontend
callbacks needed strict lint-compatible forms; and the first full verify
stopped at forbidden test `unwrap()` calls. Tests now propagate failures with
Result/Option rather than suppressing a lint. Strict Clippy and final verify
passed afterward. The no-longer-needed tokio macros feature was removed before
the final lock, audit and completion run. No distinct recoverable failure
received more than two additional repair attempts. No toolchain, gate or
historical failure was repaired as part of this increment.

## Architecture findings

**Passed.** The existing host remains the one lease/identity/journal/deadline
owner and the runtime remains I/O-free. A private request profile preserves
historical gateway behavior while selecting the fixed Responses body. Actual
provider response IDs and deltas are translated into the existing validated
lifecycle; no Access/gateway authentication is fabricated. The small Tauri
session wrapper owns only task lifetime, closed error status and host access.
Network awaits never hold the shared-state mutex.

Start accepts only version and disclosure acknowledgment. Poll accepts only
handle/cursor and returns a bounded snapshot. Cancel closes ingress before
aborting and awaiting the owned request task; a new run cannot inherit task,
handle or text. Late identity/content is rejected. Fixed endpoint/model, no
provider router, alternate server, execution interface or persistence is added.

## Security findings

**Passed with advisories.** The native debug/opt-in/Start boundary reads the key
only at explicit acknowledged Start. No key appears in frontend storage, IPC,
configuration, source, logs, error text or compile-time input. Tests use dummy
values. Header sensitivity and redacted Debug/error tests cover the new path.
Provider error bodies are not forwarded. The WebView receives only bounded text,
closed status and an opaque presentation handle; React escapes text.

The HTTPS client keeps TLS verification and disables proxies, redirects,
automatic retries and fallback. Foreground `store=false` requests transmit
only fixed instructions and the existing sample, with empty tools and bounded
tokens/time. SSE decoding checks byte/event/text limits, fragmented UTF-8 and
event boundaries, identities, sequence, known metadata and explicit final-text
agreement. Refusal/failure/truncation and unexpected tool output terminate
without execution. Existing exact import/invoke/registration checks remain
strict and gain negative control-payload tests.

Accepted D-128 limitations: environments are not same-user/debugger isolation
or secure erasure; request abortion does not prove OS DNS or remote computation/
billing cessation; library HTTP/TLS buffers allocate before application bounds.
Synthetic-only use and the disclosure are mandatory. The inherited D-127 audit
findings remain accepted, not hidden or fixed by this task.

## Code-health findings

**Passed.** Types, closed errors, strict lint and the bounded snapshot decoder
preserve existing conventions. Tests exercise success, rejection, limits,
duplicates, cancellation races, stale output and restart behavior. The native
mode displays the actual fixed sample and has no editable ignored composer;
mock approvals/activity are not reused for live results. Existing mock
conversation behavior remains covered by App tests. No new CSS/theme/graph
polish or generalized abstraction is introduced.

## Technical debt

The three manifest advisories retain their concrete risks, effort and milestone:
unverified native/provider behavior, explicit private-environment/transport
limits, and inherited accepted dependency findings. None blocks this authorized
fixture-only implementation closeout. They prohibit claiming live verification,
secure credential custody, complete DNS/remote quiescence or zero dependency
vulnerabilities. Do not silently remediate them or widen the increment.

## Roadmap findings

The only proposed next task is an owner-led bounded native rehearsal of this
candidate, beginning with no-key UI/mock verification. Paid traffic and private
key supply require separate explicit owner authorization. API/model access,
billing and native interaction are prerequisites to claiming a successful live
demo; lack of a key did not block implementation. No D-125/M1/M2, gateway,
real-content, graph or other-agent successor is selected.

## Completion decision

**PASS WITH ADVISORIES** for implemented, locally validated code. Ordinary
finalize/status/Stop must validate this frozen report and workspace before the
increment is treated as complete. No paid/native success is asserted.

## Next-increment readiness

**Ready with advisories** for the separately authorized owner-led live rehearsal.
The [plan](../plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md)
contains exact key-free build/Vite and separate native process launch commands,
temporary private zsh input, limits, billing disclosure and stop conditions.
Do not automatically start that task.

## Exact files changed

- `ARCHITECTURE.md`
- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `ROADMAP.md`
- `SECURITY.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-08-28-personal-assistant-v0-live-synthetic-rehearsal.md`
- `docs/reviews/2026-09-21-personal-assistant-direct-implementation-post-increment-review.md`
- `scripts/repository_health.py`
- `scripts/tests/test_repository_health.py`
- `src-tauri/Cargo.lock`
- `src-tauri/Cargo.toml`
- `src-tauri/src/agent/gateway_request.rs`
- `src-tauri/src/agent/native_runtime.rs`
- `src-tauri/src/agent/runtime.rs`
- `src-tauri/src/lib.rs`
- `src-tauri/src/personal_assistant_direct.rs`
- `src-tauri/src/personal_assistant_direct_tauri.rs`
- `src-tauri/src/personal_assistant_v0.rs`
- `src/App.test.tsx`
- `src/features/conversations/ConversationWorkspace.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.test.tsx`
- `src/features/conversations/PersonalAssistantDirectDemo.tsx`
- `src/infrastructure/tauri/personal-assistant-direct-client.test.ts`
- `src/infrastructure/tauri/personal-assistant-direct-client.ts`

## Exact commands executed

All final required commands below passed. Earlier failed diagnostics are
described above rather than mislabelled as final passing acceptance.

- `npm ci --ignore-scripts` — Passed.
- `cargo test --manifest-path src-tauri/Cargo.toml --lib --locked personal_assistant -- --test-threads=1` — Passed.
- `npm run test:frontend -- src/features/conversations/PersonalAssistantDirectDemo.test.tsx src/infrastructure/tauri/personal-assistant-direct-client.test.ts src/App.test.tsx` — Passed.
- `python3 -B -m unittest scripts.tests.test_repository_health -q` — Passed.
- `npm run verify` — Passed.
- `npm audit` — Passed.
- `npm audit --omit=dev` — Passed.
- `/private/tmp/cortexa-cargo-audit-0.22.2/bin/cargo-audit audit --no-fetch --json --file src-tauri/Cargo.lock > /private/tmp/cortexa-direct-implementation-cargo-audit-final.json; audit_exit=$?; python3 scripts/cargo_audit_gate.py /private/tmp/cortexa-direct-implementation-cargo-audit-final.json --cargo-audit-exit "$audit_exit"` — Passed.
- `npm run docs:check` — Passed.
- `npm run repository:check` — Passed.
- `npm run security:scan` — Passed.
- `git diff --check` — Passed.
- `python3 -B .codex/hooks/session_end_gate.py` — Passed.

Additional bounded diagnostics: `npm run typecheck`, `npm run lint:frontend`,
`npm run lint:rust`, `cargo check --manifest-path src-tauri/Cargo.toml --locked`,
and final `cargo check --manifest-path src-tauri/Cargo.toml --offline` passed.
Cargo fmt and Prettier formatted only authorized paths. Read-only Python
comparisons checked retained baseline hashes, exact scope, additive historical
bytes and old/new lock package identities. Current remote main, all worktree
heads/statuses, the active gate and the baseline/scope completion status were
inspected before closeout.
