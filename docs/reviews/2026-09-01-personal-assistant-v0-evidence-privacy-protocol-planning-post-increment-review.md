# Personal Assistant v0 evidence-privacy protocol planning post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git diff --stat",
    "git rev-parse HEAD",
    "git rev-parse origin/main",
    "git rev-list --left-right --count HEAD...origin/main",
    "git fsck --full --no-dangling",
    "node --version",
    "npm --version",
    "cargo --version",
    "rustc --version",
    "sw_vers",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npm run docs:check",
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-evidence-privacy-protocol-planning",
    "npx prettier --write docs/plans/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning.md",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts",
    "python3 .codex/hooks/session_end_gate.py"
  ],
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
    "SECURITY_CHECKLIST.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/increments/personal-assistant-v0-evidence-privacy-protocol-planning.md",
    "docs/plans/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning.md",
    "docs/reviews/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Medium",
      "milestone": "Before any evidence-producing, signing, or other operational successor",
      "risk": "Documentation alone cannot enforce source minimization, parsing, private attempt binding, replay rejection, or external-system behavior; treating it as operational proof could expose sensitive evidence or grant unsupported readiness.",
      "severity": "Advisory",
      "summary": "The P1 protocol is documentation policy only; no parser, sanitizer, trusted local minimization boundary, consumer, or operational evidence path exists, and P2-P4 remain Blocked."
    }
  ],
  "increment_id": "personal-assistant-v0-evidence-privacy-protocol-planning",
  "manual_verification": [
    {
      "check": "The complete change set is exactly the declared fifteen documentation paths with no protected product, dependency, workflow, hook, script, credential, certificate, database, log, or generated path",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-097 failed/FAIL/Blocked state, original report and digests, historical Failed privacy finding, Pending Open Directory boundary, Not-run signing, and absence of its completion marker remain unchanged; D-098/D-099 remain historical",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "The accepted and rejected contract tables cover exact schema, mandatory boundary_failed, lexical and size bounds, unique ordered string fields, non-authorizing predicates, source minimization, private once-only binding, replay/late rejection, and prohibited evidence",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "No prohibited security/signing evidence was requested, viewed, captured, copied, transmitted, or persisted, and no external or product operation occurred",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Target-Mac Apple, Xcode, Keychain, certificate, private-key, signing, build, credential, provider, network, and product-runtime operational validation",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Executable parser, sanitizer, trusted local minimization boundary, consumer, and evidence-path tests",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {
      "command": "git fsck --full --no-dangling",
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
      "command": "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/session_end_gate.py",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "python3 .codex/hooks/post_increment_gate.py status",
      "required": true,
      "status": "Passed"
    }
  ]
}
-->

Date: 2026-09-01
Increment: `personal-assistant-v0-evidence-privacy-protocol-planning`
Branch: `main`
Baseline: `096cdbe0139832010749039149fb2e335cfa19fd`

## Executive summary

`PASS WITH ADVISORIES`. The owner-approved documentation-only increment defines
D-100 and `evidence_privacy_v1`: one fixed protocol version, one future-plan-
owned check ID, and one closed outcome. It adds exact bounded static tokens,
canonical one-record serialization, mandatory `boundary_failed`, bounded non-
authorizing predicate semantics, source minimization, private once-only future
plan/check/attempt binding, replay/late rejection, and stop-without-retry
incident handling.

The increment collected no operational evidence and changed no product,
runtime, IPC, filesystem, dependency, workflow, hook, credential, target-Mac
security/signing, Apple, Keychain, provider, network, or external-system
behavior. Its sole advisory is intentional: no executable enforcement exists.

## Scope and boundaries

The exact change set is fifteen documentation paths: twelve existing current-
state/security/testing records plus the new plan, increment record, and this
review. Protected product/source, dependency, lockfile, workflow, hook, script,
configuration, database, credential, certificate, log, and generated paths are
unchanged.

The protocol governs a future separately approved security/signing check's
payload. Static repository-owned literals and necessary non-sensitive
repository validation/governance metadata remain separate. A three-field record
has no standalone provenance, freshness, authentication, authorization, audit,
readiness, or permission meaning. Future trusted enforcement and every
operational action require separate design, review, gates, and owner approval.

D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker. Its
original report, digests, and historical Failed privacy finding remain intact.
The Open Directory boundary remains Manual verification pending; signing
remains Not run; D-098/D-099 remain historical; P2–P4 remain Blocked.

## Verification results

- Git baseline and integrity: Passed. `main`, `HEAD`, and `origin/main` resolved
  to `096cdbe0139832010749039149fb2e335cfa19fd`, ahead/behind was `0/0`, the
  working tree was clean before `begin`, and `git fsck --full --no-dangling`
  exited zero.
- Toolchain/platform prerequisite: Passed. Node `26.3.0`, npm `11.16.0`, Cargo
  `1.90.0`, and Rust `1.90.0` matched repository pins. The read-only platform
  prerequisite command completed; no host-specific value is retained as
  protocol evidence.
- Gate begin/status: Passed. The ignored gate names only
  `personal-assistant-v0-evidence-privacy-protocol-planning` as active before
  finalization.
- `npm run docs:check`: Passed in the final relevant runs. Two intermediate
  runs stopped only because the new ExecPlan needed formatting after bounded
  edits; `npx prettier --write` ran on that one file twice, and subsequent
  formatting/link checks passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- Protected product/source diff proof: Passed with no output.
- `python3 .codex/hooks/session_end_gate.py`: Passed with no conflicts, staged
  paths, or unexpected files; its final inventory is the exact fifteen paths.
- Target-Mac Apple/Xcode/Keychain/certificate/private-key/signing/build,
  credential, provider, network, and product-runtime checks: Not run by design.
- Parser, sanitizer, local minimization boundary, consumer, and operational
  evidence-path tests: Not run because no such implementation exists.

## Architecture findings

PASS. Independent architecture review found no remaining issue after requiring
`boundary_failed` in every operational check table. No runtime, IPC,
filesystem, logging, dependency, product, external-system, or device edge was
added. Current, planned, and prohibited states remain distinct, and the
protocol is explicit repository-governance documentation.

## Security findings

PASS WITH ADVISORIES. Independent security review found no Critical, High,
Medium, or Low issue after the contract gained mandatory incident state, exact
bounded/canonical parsing rules, non-authorizing predicates, private once-only
attempt binding, replay/substitution/late rejection, source minimization, and
scoped target-derived evidence prohibitions. The residual advisory is that
these are requirements, not executable controls.

## Code-health findings

PASS WITH ADVISORIES. Independent code-health re-review found no remaining
finding after reconciling the plan index, adding missing-key/wrong-order/non-
string adversarial cases, and clarifying that checklist parser/consumer entries
are future contracts. Naming, links, organization, current-state claims, and
the exact scope are consistent.

## Technical debt

Introduced debt: None. The only residual advisory is deliberate product/security
sequencing: no parser, sanitizer, local minimization boundary, consumer, or
operational evidence path exists. Effort is Medium before operational work; it
does not block this truthful documentation closeout but blocks every
evidence-producing or signing successor.

## Roadmap findings

`Blocked` for any operational successor. P1 documentation is complete, but P2
account-directory disposition/containment, P3 executable-build-child
containment, and P4 immutable signer binding remain Proposed/Blocked. The
smallest possible next action is a separately owner-selected and approved
documentation-only P2 account-directory boundary plan. No queue reorder or
successor start is authorized by this review.

## Completion decision

PASS WITH ADVISORIES

This result applies only to the documentation protocol. It grants no evidence,
security, signing, product, device, or external-system authority.

## Next-increment readiness

Blocked. No operational successor is Ready. A P2 documentation-planning
increment requires its own exact plan, owner approval, clean baseline, and gate;
it must not rerun the consumed query or operate an external system.

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
- `SECURITY_CHECKLIST.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/increments/personal-assistant-v0-evidence-privacy-protocol-planning.md`
- `docs/plans/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning.md`
- `docs/reviews/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning-post-increment-review.md`

## Exact commands executed

- `git status --short --branch`
- `git diff --stat`
- `git rev-parse HEAD`
- `git rev-parse origin/main`
- `git rev-list --left-right --count HEAD...origin/main`
- `git fsck --full --no-dangling`
- `node --version`
- `npm --version`
- `cargo --version`
- `rustc --version`
- `sw_vers`
- `python3 .codex/hooks/post_increment_gate.py status`
- `npm run docs:check`
- `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-evidence-privacy-protocol-planning`
- `npx prettier --write docs/plans/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning.md`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts`
- `python3 .codex/hooks/session_end_gate.py`
