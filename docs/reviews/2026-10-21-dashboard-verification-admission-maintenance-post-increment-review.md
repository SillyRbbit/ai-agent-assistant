# Dashboard verification admission maintenance review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "dashboard-verification-admission-maintenance",
  "quality_gate": "FAIL",
  "next_increment_readiness": "Blocked",
  "files_changed": [
    ".codex/hooks/lifecycle_dashboard_publication.py",
    ".codex/hooks/lifecycle_dashboard_verification.py",
    ".codex/hooks/post_increment_gate.py",
    ".codex/hooks/tests/test_lifecycle_dashboard_publication.py",
    ".codex/hooks/tests/test_lifecycle_dashboard_verification.py",
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "ENGINEERING_GUIDE.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/USAGE_EFFICIENCY_DEMO.md",
    "docs/plans/2026-10-20-account-capacity-preview.md",
    "docs/plans/2026-10-20-capacity-disclosure-label-repair.md",
    "docs/plans/2026-10-20-dashboard-dependent-recovery-maintenance.md",
    "docs/plans/2026-10-20-publication-documentation-correction.md",
    "docs/plans/2026-10-20-usage-dashboard-hierarchy.md",
    "docs/plans/2026-10-20-usage-dashboard-publication-preparation.md",
    "docs/plans/2026-10-20-usage-dashboard-publication-recovery.md",
    "docs/plans/2026-10-20-usage-efficiency-demo.md",
    "docs/plans/2026-10-20-usage-efficiency-presentation.md",
    "docs/plans/2026-10-20-usage-visual-polish.md",
    "docs/plans/2026-10-21-dashboard-verification-admission-maintenance.md",
    "docs/reviews/2026-10-20-account-capacity-preview-post-increment-review.md",
    "docs/reviews/2026-10-20-account-capacity-preview-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-capacity-disclosure-label-repair-post-increment-review.md",
    "docs/reviews/2026-10-20-capacity-disclosure-label-repair-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-dashboard-dependent-recovery-maintenance-post-increment-review.md",
    "docs/reviews/2026-10-20-publication-documentation-correction-assessment.md",
    "docs/reviews/2026-10-20-usage-dashboard-hierarchy-approved-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-dashboard-hierarchy-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-dashboard-hierarchy-readiness-assessment.md",
    "docs/reviews/2026-10-20-usage-dashboard-hierarchy-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-dashboard-publication-preparation-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-dashboard-publication-preparation-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-dashboard-publication-recovery-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-dashboard-publication-recovery-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-efficiency-demo-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-efficiency-demo-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-efficiency-presentation-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-efficiency-presentation-readiness-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-visual-polish-post-increment-review.md",
    "docs/reviews/2026-10-20-usage-visual-polish-readiness-post-increment-review.md",
    "docs/reviews/2026-10-21-dashboard-verification-admission-maintenance-post-increment-review.md",
    "scripts/repository_health.py",
    "scripts/tests/test_repository_health.py",
    "src-tauri/src/lib.rs",
    "src-tauri/src/usage_efficiency.rs",
    "src-tauri/tests/usage_efficiency_contract.rs",
    "src/features/activity/ActivityPage.tsx",
    "src/features/usage-efficiency/AccountCapacityPanel.test.tsx",
    "src/features/usage-efficiency/AccountCapacityPanel.tsx",
    "src/features/usage-efficiency/UsageCharts.tsx",
    "src/features/usage-efficiency/UsageEfficiencyPanel.test.tsx",
    "src/features/usage-efficiency/UsageEfficiencyPanel.tsx",
    "src/features/usage-efficiency/UsageSummary.test.tsx",
    "src/features/usage-efficiency/UsageSummary.tsx",
    "src/features/usage-efficiency/account-capacity.css",
    "src/features/usage-efficiency/usage-disclosure.css",
    "src/features/usage-efficiency/usage-efficiency.css",
    "src/features/usage-efficiency/usage-presentation.test.tsx",
    "src/features/usage-efficiency/usage-presentation.ts",
    "src/infrastructure/tauri/usage-efficiency-client.test.ts",
    "src/infrastructure/tauri/usage-efficiency-client.ts",
    "src/test/usage-efficiency-fixture.ts"
  ],
  "commands_executed": [
    "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v",
    "npm run tauri -- build --no-bundle",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "npm run verify"
  ],
  "verification": [
    {
      "command": "/usr/bin/python3 -B -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "npm run tauri -- build --no-bundle",
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
    },
    {
      "command": "npm run verify",
      "required": true,
      "status": "Not run"
    }
  ],
  "manual_verification": [
    {
      "check": "Preservation and scope review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture, security, code-health and readiness review",
      "required": true,
      "status": "Passed"
    }
  ],
  "findings": [
    {
      "category": "Roadmap",
      "severity": "High",
      "summary": "Required full-command verification has no new successful invocation; composite Passed reporting was rejected.",
      "risk": "Maintenance cannot supply passing closure evidence until supported full verification is established.",
      "effort": "Establish a safe resource/cache route and actual accepted verification evidence without relabeling old exit -15.",
      "milestone": "Complete M verification before C/V",
      "blocks_completion": true,
      "blocks_next_increment": true
    },
    {
      "category": "Technical debt",
      "severity": "Advisory",
      "summary": "Local hashes establish custody, not operator authentication or semantic proof that commands ran.",
      "risk": "Evidence reuse requires source/environment and substantive review; schema validity alone is insufficient.",
      "effort": "Retain named review and actual result provenance.",
      "milestone": "M review and later C/V",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Roadmap",
      "severity": "Advisory",
      "summary": "Real C/V and post-commit publication handling remain separate and unexecuted.",
      "risk": "Fixed HEAD/index lineage cannot authorize staging or erase historical failures.",
      "effort": "Separate owner decisions after verified prerequisites.",
      "milestone": "Later closure, admission and publication decisions",
      "blocks_completion": false,
      "blocks_next_increment": true
    }
  ]
}
-->

Date: 2026-10-21. Branch: `codex/usage-dashboard-publication`.
M only: explicit pre-admission maintenance without ordinary begin.
**FAIL / Blocked** for maintenance acceptance; implementation and bounded native
verification are retained. Real lifecycle state remains untouched.

## Executive summary

The fixed verification-only schema-7 module and narrow dispatcher integration
are implemented. All 217 hook tests passed, including 19 new cases and unchanged
independent/VPS/D-133/original dashboard tests. Actual offline native no-bundle
verification passed at exit 0 in 139 seconds, with source unchanged and no app
launch. This is new forward evidence for the formerly missing native stage.

A proposed report marking the required `npm run verify` row Passed from composite
coverage was rejected by automatic approval review. It was not written. This
replacement records that command as **Not run in M**. The historical full and
native invocations remain exit -15; no command result is relabeled.

A safe fresh full-command resource envelope is unverified. The retained debug
stage and successful native stage bind different Cargo source homes. The native
success therefore does not prove cheap full-run cache reuse. No full invocation,
cache-path experiment or cleanup was attempted. Stop before C/V.

## Scope and boundaries

Exactly 14 attributed maintenance paths: new lifecycle module/test, narrow
dispatcher integration, nine additive root documents, the
[plan](../plans/2026-10-21-dashboard-verification-admission-maintenance.md)
and this report. Other paths in the complete Git inventory are the preserved
65-path candidate. No product, Rust, IPC, fixtures, accounting, providers,
dependencies, security configuration, branding or navigation changed.

The original preparation seal/addendum, 1,171 custody bindings, actual index,
raw failed state and receipts verified before edits. A fresh overlapping snapshot
and frozen concrete design preceded code. Original dashboard, historical reports,
bundles, consumed markers and other worktrees remain preserved. All synthetic
Git/lifecycle effects occurred in temporary fixtures. No real begin, closure,
admission, finalize or close-failed occurred; no marker is issued.

## Verification results

Private evidence package: `cortexa-dashboard-verification-maintenance-20261021-01`.
Source bindings, logs, receipts, frozen design, snapshots and actual outcomes
are retained there. The final small-check receipts bind this finished document
workspace. Required command identities listed in the manifest include the
explicitly Not run full command; that listing is not an execution claim.

| Check                                                | Actual result                                                                                                                                                                                      |
| ---------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial focused synthetic run                        | 41 tests Passed, including 26 imported predecessor tests and 15 initial new cases.                                                                                                                 |
| Final hook suite                                     | 217 tests Passed, exit 0; all 19 final new cases and legacy routes.                                                                                                                                |
| Native no-bundle stage                               | Passed, exit 0, 139.008 seconds, source unchanged; no app launch.                                                                                                                                  |
| Documentation/repository/security/whitespace/session | Passed on affected input; final receipts bind completed documentation.                                                                                                                             |
| Full command in M                                    | Not run; required acceptance remains incomplete.                                                                                                                                                   |
| Earlier full command/native continuation             | Both retained exit -15 resource interruptions remain Failed.                                                                                                                                       |
| Unchanged product checks                             | Historical successful stages retained: frontend 663, repository 99, Rust library 464 and integration contracts, one existing opt-in Hermes probe ignored. They are not a new passing full command. |
| Frontend typecheck/build                             | Also Passed as stages of the new native command.                                                                                                                                                   |
| Browser                                              | Retained mock evidence reused; no browser or native app launched in M.                                                                                                                             |

Existing dependencies: 292 installed package versions match the lockfile,
41,231 public registry files verified, Node 26.3.0 / npm 11.16.0 / Rust 1.90.0 /
SDK 27.0. No installation or credential/configuration content access. Public
cache config/credential file absence was inspected by metadata only.

New native output used a separate copy-on-write target. Protected floor:
43,553,652,736 bytes, retaining conservative 32 GiB future demand and avoiding a
second reservation of 12 GiB already allocated. The fresh monitored 768 MiB
envelope is distinct from exhausted historical 4/4.75 GiB budgets. Start available
44,646,400,000 bytes; post-build 44,732,846,080 bytes. Samples/final reading remained
above the floor. Post-build changed/new-file allocation estimate: 306,012,160
bytes, excluding small logs/temporary metadata. Free-space changes include
unattributed concurrent/APFS effects; no cleanup benefit or zero-allocation claim.
The monitor is not a filesystem quota. Reservations stayed unchanged.

Whole-file formatting initially failed with sandbox EPERM; automatic approval
review rejected escalation because it could rewrite historical content.
Read-only comparison identified only whitespace at the new section boundaries.
Explicit newline patches resolved it without rewriting historical bodies.
ATTEMPTS.md retains that refusal and the separate composite-report rejection.

## Architecture findings

Reviewed the new module, dispatcher/report/terminal boundaries, original dashboard
reader and general closure rules. The new module owns only the fixed route.
Existing independent/VPS/D-133/dashboard implementations are unchanged. Schema 7
cannot mix lineages or admit arbitrary tasks.

Preparation creates the fixed plan, current owner decision and readiness before
admission. Execution may change only seven current-state documents and the final
report. All other baseline paths, including plan/readiness/authority/governance,
are protected. All 21 product hashes are enforced in active and terminal states.
No product or device authority is added.

## Security findings

Reviewed bounded duplicate/nonfinite-free JSON, exact identity/source/index
bindings, symlink-safe evidence, separate current authority and exclusive
receipt/pointer/state effects with intervening checks. Interrupted or uncertain
effects block replay. New terminal validation rechecks forward evidence and
quality, rejecting fabricated completion.

Historical schema-6 authority is checked against immutable raw decision/request
bytes and transitive closure/admission evidence; the old validator is not invoked
against changed current DECISIONS with failure suppressed. An own new D-130 closure
is mandatory. Neither a request nor a plan grants its own approval.

Provenance binds actual status, inputs, environment, result and named scope/reuse
review. Whole-workspace checks cannot substitute product-only bindings. Semantic
truth still requires review of actual logs. No model/WebView, credential, network,
provider or device path is added. Local hashes are not same-user authentication.

## Code-health findings

Production retained inputs preserve exactly four criteria, nine original
mappings and ten newer unresolved records. The complete required sets from both
source reports include previously passing checks. Forward reporting requires
ten commands and thirty manual checks/evidence entries.

Synthetic tests reject omitted/optional-downgraded required checks, changed
criteria/mappings, unsupported status/input provenance, stale authority/readiness,
scope expansion, mixed lineage, historical drift, interruption at each effect,
replay and false completion. Positive completion and failure paths include actual
synthetic Stop payloads. The post-commit finding must stay unchanged in future
reports, with next readiness Blocked. No concrete code defect remained in this
bounded review. Real source-scale admission/Stop acceptance remains unexecuted.

## Technical debt

Fixed HEAD/index intentionally prevents staging or commits from silently retaining
valid lineage. Publication requires a separately supported transition and owner
authority. Governance scope is separate from the 21 product paths. Native/VoiceOver,
narrow-layout, bundle-size and live-data limitations remain. New compilation
success does not establish native UI acceptance.

## Roadmap findings

**Blocked** for C/V because required full-command acceptance is unresolved.
First assess a safe full-verification resource/cache route; preserve both old
interruptions and the new native success. No cleanup or reservation release is
implied. Step B stays blocked, exec deferred and hosted authentication unresolved.

## Completion decision

**FAIL** for M acceptance. This explicit pre-admission maintenance cannot finalize
or close the active predecessor. Original schema-6 failure remains failed / FAIL /
Blocked, byte-identical, with no completion marker. No passing intermediate real
Stop is asserted. This report does not execute any lifecycle transition.

## Next-increment readiness

**Blocked**. Before C, M needs accepted full-verification evidence and passing
review, without relabeling a failed command. A viable allocation-aware full-run
route must precede costly checks. C then requires separate owner retain/defer
authority, both preservation layers, actual index/raw records, unchanged four/
nine/ten mapping and supported close-without-completion prerequisites. After
actual C/Stop, V still requires separate approval and fresh plan/readiness/
authority bindings. Post-commit publication remains separately blocked.

## Exact files changed

The machine manifest is the complete Git change set. M is exactly the 14
attributed paths listed in the plan; other candidate paths predate M. Evidence,
logs and the new isolated native output remain outside the repository.

## Exact commands executed

Fresh hook discovery used system Python with bytecode writes disabled. Small
checks used a private HOME and offline npm configuration. Fresh native verification
used the recorded offline environment and separate monitored target. The required
full command is explicitly Not run in M; its old exit -15 remains unchanged.
Read-only source, hash, custody, resource and production-contract assessments are
recorded separately. No real gate or publication command was executed.
