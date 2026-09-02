# Personal Assistant v0 account-directory boundary planning post-increment review

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
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-account-directory-boundary-planning",
    "npx prettier --write docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md",
    "npx prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-account-directory-boundary-planning.md docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md",
    "npx prettier --write docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md docs/reviews/2026-09-01-personal-assistant-v0-account-directory-boundary-planning-post-increment-review.md",
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
    "docs/increments/personal-assistant-v0-account-directory-boundary-planning.md",
    "docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md",
    "docs/reviews/2026-09-01-personal-assistant-v0-account-directory-boundary-planning-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Medium",
      "milestone": "Before any operational P2, signing, or product successor",
      "risk": "Documentation cannot enforce application-resolution prohibition, exact scope provenance, platform-effect exclusion, private capability lifecycle, or closed evidence handling; treating it as operational proof could repeat the historical boundary failure.",
      "severity": "Advisory",
      "summary": "D-101 and its P1-compatible predicates are documentation only; no supported platform API, authoritative platform-contract evidence, parser/sanitizer, adapter, capability, target-Mac behavior, or runtime tests exist, and every operational predicate remains Not run."
    }
  ],
  "increment_id": "personal-assistant-v0-account-directory-boundary-planning",
  "manual_verification": [
    {
      "check": "The complete change set is exactly the declared fifteen documentation paths with no protected product, dependency, workflow, hook, script, credential, certificate, database, log, generated, or external-system path",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-097 remains failed/FAIL/Blocked with its original report and digests, Failed privacy finding, Pending Open Directory boundary, Not-run signing, missing completion marker, and consumed-query no-rerun rule unchanged; D-098/D-099/D-100 remain historical",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "The policy separates non-waivable application resolution/input/provenance from independent directory, cache, log, socket, trust-service, process-metadata, and network predicates; cleanup success destroys the native reference and cleanup failure retains private quarantine until process exit without replacement, retry, reuse, exposure, or early ownership loss",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "The twelve unique check IDs and six unique outcomes are closed, non-authorizing, lowercase ASCII, include boundary_failed for every check, are bounded to 64/32 bytes, and produce a longest canonical three-field record of 121 bytes within D-100's 256-byte ceiling",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "ARCHITECTURE.md accurately records the disconnected fixed-service/fixed-account fake-only Keychain reader while preserving the absence of a generic PlatformAdapter, product credential consumer, or P2 implementation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "No account-directory, Apple, Xcode, Keychain, Security.framework, trust-service, certificate, private-key, signing, build, credential, provider, network, product, target-Mac operational, or external-system action ran",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Target-Mac account-directory, Apple, Xcode, Keychain, Security.framework, trust-service, certificate, private-key, signing, build, credential, provider, network, and product-runtime validation",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Supported platform API/contract evidence, executable parser/sanitizer, adapter/capability lifecycle, and operational P2 runtime tests",
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
Increment: `personal-assistant-v0-account-directory-boundary-planning`
Branch: `main`
Baseline: `5bf37681a2554094575f17025f47b7cf2b6b5d36`

## Executive summary

`PASS WITH ADVISORIES`. The owner-approved documentation-only increment defines
D-101 and `ExplicitAccountResolutionPolicyV1::Prohibited`. Future application
code may not explicitly resolve or derive an account, home, or Keychain path.
Independent contracts are required for no application input, exact fixed-domain
scope provenance, and each directory, cache, log, socket, trust-service,
process-metadata, and network effect. The application wrapper and adapter-
private native-reference lifecycle are separate, cleanup is attempted on every
terminal path, and failed cleanup remains privately quarantined until process
exit without replacement or reuse.

The increment implements none of those controls. It selects no platform API,
collects no operational evidence, accesses no Apple or external system, changes
no product source, and leaves every operational predicate Not run. Its advisory
is therefore intentional: operational P2 and all later operational work remain
Blocked.

## Scope and boundaries

The exact change set is fifteen documentation paths: twelve existing current-
state/security/testing records plus the new plan, increment record, and this
review. Protected product source, dependencies, lockfiles, workflows, hooks,
scripts, configuration, databases, credentials, certificates, logs, generated
files, and external systems are unchanged.

The documentation distinguishes three claims. Application source may be
reviewed for a non-waivable explicit-resolution prohibition. Static platform
contracts must independently establish no application-supplied scope and one
fixed application credential domain without ambient/default authority. Every
OS effect class has its own closed predicate or remains `contract_unproven`.
None of these non-authorizing records establishes readiness by itself.

D-097 remains `failed` / `FAIL` / `Blocked` without a completion marker. Its
report, digests, Failed privacy result, Pending Open Directory boundary, and
Not-run signing result remain unchanged. The consumed query was not rerun;
D-098/D-099/D-100 remain historical; P3/P4 and every operational successor
remain Proposed/Blocked.

## Verification results

- Git baseline and integrity: Passed. `main`, `HEAD`, and `origin/main` resolved
  to `5bf37681a2554094575f17025f47b7cf2b6b5d36`, ahead/behind was `0/0`, the
  working tree was clean before `begin`, and `git fsck --full --no-dangling`
  exited zero.
- Toolchain/platform prerequisite: Passed. Node `26.3.0`, npm `11.16.0`, Cargo
  `1.90.0`, and Rust `1.90.0` matched repository pins. The read-only platform
  prerequisite completed; no host-specific value is retained as protocol
  evidence.
- Gate begin/status: Passed. The ignored gate names only
  `personal-assistant-v0-account-directory-boundary-planning` as active before
  finalization.
- `npm run docs:check`: Passed in the final run. Two intermediate runs stopped
  only because the new plan required formatting after bounded edits; the
  repository formatter changed only that plan, and subsequent formatting/link
  validation passed.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- Protected product/source diff proof: Passed with no output.
- `python3 .codex/hooks/session_end_gate.py`: Passed with no conflicts or staged
  paths and the exact fifteen-path unstaged/untracked inventory.
- Target-Mac account-directory, Apple, Xcode, Keychain, Security.framework,
  trust-service, certificate, private-key, signing, build, credential, provider,
  network, and product-runtime checks: Not run by design.
- Supported API/contract evidence, executable parser/sanitizer, adapter/
  capability lifecycle, and operational runtime tests: Not run because no such
  implementation exists.

## Architecture findings

PASS. Independent architecture review found no remaining issue after the policy
was narrowed to application resolution, exact fixed-domain provenance and every
effect predicate were separated, ownership and cleanup/quarantine semantics
were made coherent, and the platform inventory was reconciled with the existing
disconnected fixed-label fake-only Keychain reader. No runtime, IPC,
persistence, dependency, product, external-system, or device edge was added.

## Security findings

PASS WITH ADVISORIES. Independent security review found no Critical, High,
Medium, Low, or additional Advisory defect. The twelve check IDs and six
outcomes are unique, closed, non-authorizing, include `boundary_failed`, and
meet D-100 grammar and 64/32/256-byte limits. The residual advisory is that
these are documentation requirements without executable enforcement or
platform evidence.

## Code-health findings

PASS WITH ADVISORIES. Independent code-health re-review found no remaining
finding after terminal cleanup success/failure semantics were separated, the
missing trust-service predicate was added, and literal-bound review became
explicit. Naming, authority, links, current/planned/prohibited states, exact
scope, and historical preservation are consistent.

## Technical debt

Introduced debt: None. Existing debt attributable to this diff: None. The absent
platform API, executable controls, and unresolved historical boundary are
explicitly sequenced blockers rather than hidden or newly introduced debt.

## Roadmap findings

`Blocked` for any next increment. The P2 documentation closeout is coherent,
but this uncommitted change is not yet published, no exact P3 plan exists, and
no later work is owner-authorized. Every operational P2 predicate remains Not
run and the historical finding remains Pending. After owner review and
publication to clean synchronized `main`, the smallest possible next action is
separate approval to draft—not begin—one exact documentation-only P3 build-
child-containment plan.

## Completion decision

PASS WITH ADVISORIES

This result applies only to D-101 policy documentation. It grants no evidence,
API, adapter, capability, account, Keychain, signing, product, device, network,
or external-system authority.

## Next-increment readiness

Blocked. No successor is Ready. The current change first requires owner review
and explicit publication authorization. An exact P3 documentation plan may be
drafted only after clean synchronized published `main` and separate owner
approval; it may not begin a gate or operational work.

## Exact files changed

1. `ARCHITECTURE.md`
2. `CHANGELOG.md`
3. `DECISIONS.md`
4. `HANDOFF.md`
5. `NEXT_STEPS.md`
6. `PLANS.md`
7. `PROJECT_STATUS.md`
8. `ROADMAP.md`
9. `SECURITY.md`
10. `SECURITY_CHECKLIST.md`
11. `TESTING_GUIDE.md`
12. `TROUBLESHOOTING_LOG.md`
13. `docs/increments/personal-assistant-v0-account-directory-boundary-planning.md`
14. `docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md`
15. `docs/reviews/2026-09-01-personal-assistant-v0-account-directory-boundary-planning-post-increment-review.md`

## Exact commands executed

- `git status --short --branch`: Passed; baseline was clean and synchronized,
  and final inventory contains only the fifteen declared paths.
- `git diff --stat`: Passed; documentation-only scope.
- `git rev-parse HEAD`: Passed; `5bf37681a2554094575f17025f47b7cf2b6b5d36`.
- `git rev-parse origin/main`: Passed; same commit.
- `git rev-list --left-right --count HEAD...origin/main`: Passed; `0 0`.
- `git fsck --full --no-dangling`: Passed.
- `node --version`: Passed; `v26.3.0`.
- `npm --version`: Passed; `11.16.0`.
- `cargo --version`: Passed; `cargo 1.90.0`.
- `rustc --version`: Passed; `rustc 1.90.0`.
- `sw_vers`: Passed as a read-only platform prerequisite; no host value is
  retained as protocol evidence.
- `python3 .codex/hooks/post_increment_gate.py status`: Passed; the exact P2
  increment was active before finalization.
- `npm run docs:check`: Passed in the final run; two intermediate formatting-
  only failures were corrected by the scoped formatter.
- `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-account-directory-boundary-planning`:
  Passed once from the clean synchronized baseline.
- `npx prettier --write docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md`:
  Passed in the two scoped formatting corrections.
- `npx prettier --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-account-directory-boundary-planning.md docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md`:
  Passed; only the new plan required formatting.
- `npx prettier --write docs/plans/2026-09-01-personal-assistant-v0-account-directory-boundary-planning.md docs/reviews/2026-09-01-personal-assistant-v0-account-directory-boundary-planning-post-increment-review.md`:
  Passed for final plan/report formatting.
- `npm run repository:check`: Passed.
- `npm run security:scan`: Passed.
- `git diff --check`: Passed.
- `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts`:
  Passed with no output.
- `python3 .codex/hooks/session_end_gate.py`: Passed with no conflicts, no staged
  paths, and exactly fifteen documentation paths.
