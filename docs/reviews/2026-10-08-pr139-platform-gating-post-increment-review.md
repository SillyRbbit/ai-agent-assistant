# PR139 platform gating post-increment review

<!-- post-increment-gate-manifest
{
  "schema_version": 1,
  "increment_id": "pr139-platform-gating",
  "quality_gate": "PASS WITH ADVISORIES",
  "next_increment_readiness": "Ready with advisories",
  "verification": [
    {
      "command": "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "tauri build --no-bundle --no-sign --config external/reuse-frontend-config.json -- --locked --offline",
      "required": true,
      "status": "Passed"
    },
    {
      "command": "composite npm run verify unchanged-stage provenance audit",
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
      "command": "external scope/historical/artifact preservation comparison",
      "required": true,
      "status": "Passed"
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
    },
    {
      "check": "Unchanged product verification provenance review",
      "required": true,
      "status": "Passed"
    }
  ],
  "findings": [
    {
      "category": "Code health",
      "severity": "Advisory",
      "summary": "Linux exact-head CI and Mac fixed-stage/exit evidence require observation after the authorized push; historical CI failures remain.",
      "risk": "Local Mac success is not Linux acceptance or a proven Mac CI cause.",
      "effort": "Observe new workflows; propose any further repair separately.",
      "milestone": "PR139 CI diagnosis",
      "blocks_completion": false,
      "blocks_next_increment": false
    },
    {
      "category": "Security",
      "severity": "Advisory",
      "summary": "Retain inherited same-user receipt, platform/runtime/timing, Docker-host and native build workaround advisories.",
      "risk": "Local receipt hashes do not authenticate malicious same-user changes; bounded evidence is not all variants.",
      "effort": "No expansion in this successor.",
      "milestone": "Owner-selected follow-up",
      "blocks_completion": false,
      "blocks_next_increment": false
    }
  ],
  "files_changed": [
    "CHANGELOG.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TESTING_GUIDE.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/plans/2026-10-08-git-fixture-diagnostics.md",
    "docs/plans/2026-10-08-pr139-platform-gating.md",
    "docs/reviews/2026-10-08-git-fixture-diagnostics-post-increment-review.md",
    "docs/reviews/2026-10-08-git-fixture-diagnostics-readiness-post-increment-review.md",
    "docs/reviews/2026-10-08-pr139-platform-gating-post-increment-review.md",
    "docs/reviews/2026-10-08-pr139-platform-gating-readiness-post-increment-review.md",
    "src-tauri/src/approvals/manager.rs",
    "src-tauri/src/approvals/types.rs",
    "src-tauri/src/isolated_action/tests.rs"
  ],
  "commands_executed": [
    "cargo fmt --manifest-path src-tauri/Cargo.toml -- --check",
    "cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings",
    "cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline",
    "tauri build --no-bundle --no-sign --config external/reuse-frontend-config.json -- --locked --offline",
    "composite npm run verify unchanged-stage provenance audit",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "python3 -B .codex/hooks/session_end_gate.py",
    "external scope/historical/artifact preservation comparison"
  ],
  "milestone": {
    "criteria": {
      "platform-boundary": {
        "status": "automatically_verified",
        "evidence": "Consistent macOS-only private isolated-change constructor/decision/subject and dependent match arms; public metadata and non-macOS rejection unchanged; no dead-code suppression. External saved outputs/exits and provenance: /private/tmp/cortexa-pr139-platform-repair-tl9x12y6"
      },
      "local-verification": {
        "status": "automatically_verified",
        "evidence": "Fresh affected Rust format, strict Clippy, unit/integration/all-target and native compile coverage, with explicit unchanged-stage provenance for composite verify coverage; no live QA. External saved outputs/exits and provenance: /private/tmp/cortexa-pr139-platform-repair-tl9x12y6"
      },
      "preservation": {
        "status": "automatically_verified",
        "evidence": "Inherited eleven-path diagnostic and historical completion snapshots preserved; existing documents extended additively; all unrelated product, gate, dependencies and artifact bytes unchanged. External saved outputs/exits and provenance: /private/tmp/cortexa-pr139-platform-repair-tl9x12y6"
      },
      "publication-readiness": {
        "status": "automatically_verified",
        "evidence": "Truthful reviews, additive documentation, schema, session and preservation pass; exact candidate frozen for one owner-authorized commit/push updating PR139. New exact-head CI remains a separate observed result, never a presumed pass. External saved outputs/exits and provenance: /private/tmp/cortexa-pr139-platform-repair-tl9x12y6"
      }
    },
    "paths": {
      "HANDOFF.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "TROUBLESHOOTING_LOG.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/plans/2026-10-08-pr139-platform-gating.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "NEXT_STEPS.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "TESTING_GUIDE.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "docs/reviews/2026-10-08-pr139-platform-gating-post-increment-review.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "PLANS.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/approvals/manager.rs": {
        "criterion": "platform-boundary",
        "rationale": "macOS cfg attributes and equivalent split match arms only; runtime validation/public metadata/non-macOS rejection unchanged.",
        "within_objective": true,
        "preserves_existing": true
      },
      "src-tauri/src/approvals/types.rs": {
        "criterion": "platform-boundary",
        "rationale": "macOS cfg attributes and equivalent split match arms only; runtime validation/public metadata/non-macOS rejection unchanged.",
        "within_objective": true,
        "preserves_existing": true
      },
      "PROJECT_STATUS.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      },
      "CHANGELOG.md": {
        "criterion": "publication-readiness",
        "rationale": "Additive current-state successor plan/review/documentation; frozen historical diagnostic reports and existing document prefixes preserved.",
        "within_objective": true,
        "preserves_existing": true
      }
    }
  }
}
-->

## Executive summary

Two-file consistent macOS private approval gating locally verified. Existing fixed Git diagnostic is preserved for owner-authorized PR139 publication. PASS WITH ADVISORIES.

## Scope and boundaries

Only approvals/manager.rs and types.rs implementation changes; seven additive current documents and new plan/readiness/final report. The eleven-path diagnostic is inherited, not replayed. Exact publication scope 16 paths; no unrelated work. Snapshot before schema-2 admission. No product behavior, public metadata, fixture/HOME, dependency, gate, capability, provider or workflow changes.

## Verification results

Fresh Rust format, strict all-target/all-feature Clippy, all-target tests (749 passed, 3 ignored, zero failures), and native no-bundle compilation passed. Unit approval hash/expiry/replay test passed; complete Mac fixture-dependent tests passed. Optional ignored checks remain unclaimed. Installed Python/Xcode SDK27/Cargo offline workaround, fresh cloned output and external build-only config reuse verified dist; no launch. Original product verify-host.json exit0 in /private/tmp/cortexa-qa-shape-1cv9b4i4 covers unchanged frontend format/lint/typecheck/tests/build and repository tests; 182 frontend and21 repository inputs match. /private/tmp/cortexa-governance-integration-rcuhdh6z/hooks.json exit0 on installed Python3.12.1 covers9 unchanged hook inputs. Seals and input hashes verify. Changed Rust stages ran freshly. This is composed npm run verify coverage, not a new full-run exit0. Prior verify.json exit1 and initial rustfmt exit1 remain historical; owner specifically approved the exact formatting correction. Newly triggered Linux/target-Mac CI is pending publication, not waived or called passed.

## Architecture findings

Architecture-review: private decision/subject construction availability now follows its sole macOS production owner. Public metadata and application-owned trust boundary unchanged. Equivalent match split preserves Mac semantics. No generic execution/runtime selection or framework coupling.

## Security findings

Security-review: no allow(dead_code), warning weakening, added fallback, approval bypass, credentials, raw Git output or altered native approval hashes/expiry. Non-macOS Workspace::native_approval remains Err(ActionError::Approval); source file byte-identical. All hooks/signing, isolation, journal/replay and cleanup boundaries unchanged.

## Code-health findings

Code-review: constructor, private enum variants, eleven accessors, preview conversion and split resolution arm consistently gated. Private subject key avoids secondary dead-code after decision removal. Existing exact native approval and public binding/audit tests passed. Formatter correction only collapses variant layout. No unnecessary new tests mirroring cfg attributes.

## Technical debt

Technical-debt: historical CI failures and discarded-response unknown causes retained. Mac Git failure did not reproduce; no speculative HOME or return repair. Native rollback restoration, Docker-active cancellation, all timing/platform/provider variants and isolated QA limitations remain as inherited. Ledger63/66, zero dispatches.

## Roadmap findings

Readiness-review: local repair/diagnostic candidate ready for specifically authorized publication. PR139 merge remains unauthorized and depends on actual exact-head checks/reviews. Old run37844810176 is not rerun; new push starts new source-bound workflows. No later milestone selected.

## Completion decision

PASS WITH ADVISORIES for local repair/publication readiness only. Actual finalization/full Stop and post-push CI are recorded externally after freeze.

## Next-increment readiness

Ready with advisories for authorized exact publication and CI inspection; necessary additional repair requires proposal/approval.

## Exact files changed

- `CHANGELOG.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TESTING_GUIDE.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/plans/2026-10-08-git-fixture-diagnostics.md`
- `docs/plans/2026-10-08-pr139-platform-gating.md`
- `docs/reviews/2026-10-08-git-fixture-diagnostics-post-increment-review.md`
- `docs/reviews/2026-10-08-git-fixture-diagnostics-readiness-post-increment-review.md`
- `docs/reviews/2026-10-08-pr139-platform-gating-post-increment-review.md`
- `docs/reviews/2026-10-08-pr139-platform-gating-readiness-post-increment-review.md`
- `src-tauri/src/approvals/manager.rs`
- `src-tauri/src/approvals/types.rs`
- `src-tauri/src/isolated_action/tests.rs`

## Exact commands executed

- `cargo fmt --manifest-path src-tauri/Cargo.toml -- --check`
- `cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets --all-features --locked --offline -- -D warnings`
- `cargo test --manifest-path src-tauri/Cargo.toml --all-targets --locked --offline`
- `tauri build --no-bundle --no-sign --config external/reuse-frontend-config.json -- --locked --offline`
- `composite npm run verify unchanged-stage provenance audit`
- `npm run docs:check`
- `npm run repository:check`
- `npm run security:scan`
- `git diff --check`
- `python3 -B .codex/hooks/session_end_gate.py`
- `external scope/historical/artifact preservation comparison`

External receipts retain exact executable/configuration paths, environments, outputs and exit statuses; normalized commands above describe the same invoked checks.
