# Personal Assistant v0 containment primitive selection post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-build-child-containment-primitive-selection",
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-containment-primitive-selection",
    "npm exec prettier -- --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-containment-primitive-selection.md docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md",
    "npm exec prettier -- --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-containment-primitive-selection.md docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md docs/reviews/2026-09-02-personal-assistant-v0-containment-primitive-selection-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npm run verify",
    "npm run tauri -- build --no-bundle"
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
    "docs/increments/personal-assistant-v0-containment-primitive-selection.md",
    "docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md",
    "docs/reviews/2026-09-02-personal-assistant-v0-containment-primitive-selection-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "Before any P3-3 containment controller implementation",
      "risk": "No eligible candidate in the frozen reviewed set establishes D-102 pre-effect effect control and complete descendant ownership.",
      "severity": "Advisory",
      "summary": "Operational build-child containment remains unavailable."
    }
  ],
  "increment_id": "personal-assistant-v0-containment-primitive-selection",
  "manual_verification": [
    {
      "check": "Frozen six-entry candidate set, ten conjunctive eligibility gates, closed API set, and bounded no-selection wording",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Official-source claim-to-link review with no inferred current availability or signature-class equivalence",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Exact ten not-run source, fourteen contract-unproven, and eight not-run contract disposition partition plus boundary-failed handling",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-097 through D-102, original failed evidence, historical privacy failure, Pending Open Directory boundary, and Not-run signing preservation",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Complete fifteen-path documentation scope, protected-source review, and independent architecture/security/code/debt/readiness review",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Approved public-documentation-only external contact and no operational target-Mac, Apple, signing, build, probe, credential, provider, product, or state-changing external action",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Build, child, process, filesystem, network, and target-Mac containment checks",
      "required": false,
      "status": "Not run"
    },
    {
      "check": "Apple, Xcode, Keychain, certificate, signing, credential, provider, product, and state-changing external-system checks",
      "required": false,
      "status": "Not run"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
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
      "command": "git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json",
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
    },
    {
      "command": "npm run verify",
      "required": false,
      "status": "Not run"
    },
    {
      "command": "npm run tauri -- build --no-bundle",
      "required": false,
      "status": "Not run"
    }
  ]
}
-->

Date: 2026-09-02
Increment: `personal-assistant-v0-containment-primitive-selection`
Branch: `main`

## Executive summary

The approved fifteen-path documentation-only P3-2 static review is complete.
D-103 records exactly **no eligible candidate in the reviewed set**. The only
deep-review candidate fails the independent no-new-entitlement/no-prerequisite-
signing gate, and its reviewed public process contracts do not prove D-102's
complete detached-descendant lifecycle. Fixed negative controls and scope-only
classes are also rejected without admitting a catch-all. No primitive,
controller, product behavior, or operational proof was added. Acceptance
criteria are met for this bounded documentation decision only. Quality gate:
**PASS WITH ADVISORIES**.

## Scope and boundaries

The exact approved scope was a static authoritative-source review of six frozen
candidate entries, with only the App Sandbox helper plus closed libSystem/POSIX
composition entering D-100 contract review. Approved read-only public Apple
documentation access was the sole external contact. No authenticated or state-
changing external action occurred. The complete changed inventory is exactly
the declared fifteen documentation paths. No product source, dependency,
lockfile, build configuration, Tauri capability, CSP, permission, entitlement,
workflow, hook, credential, Apple account, Xcode, Keychain, signing state, or
external resource changed.

The result is bounded to the reviewed set and is not a universal impossibility
claim. It cannot authorize an unreviewed candidate, a relaxed eligibility rule,
a controller, an operational test, or P3-3. D-097 and every historical Failed,
Pending, and Not-run fact remain unchanged.

## Verification results

| Check                                                            | Status  | Evidence                                                                                                  |
| ---------------------------------------------------------------- | ------- | --------------------------------------------------------------------------------------------------------- |
| `npm run docs:check`                                             | Passed  | Final Prettier and repository link-health checks passed.                                                  |
| `npm run repository:check`                                       | Passed  | Repository-health `all` passed.                                                                           |
| `npm run security:scan`                                          | Passed  | Repository secret scan passed.                                                                            |
| `git diff --check`                                               | Passed  | No whitespace errors.                                                                                     |
| Protected-path diff                                              | Passed  | No source, dependency, configuration, workflow, hook, skill, or script path changed.                      |
| Session inventory                                                | Passed  | No conflicts or suspicious paths; the complete inventory equals the declared fifteen documentation paths. |
| Active-gate status                                               | Passed  | The correct shortened P3-2 identifier remained active before finalization.                                |
| `npm run verify`                                                 | Not run | Documentation-only scope forbids complete product/build execution.                                        |
| `npm run tauri -- build --no-bundle`                             | Not run | Documentation-only scope forbids native build execution.                                                  |
| Build/child/process/filesystem/network/target-Mac checks         | Not run | No operational primitive or approval exists.                                                              |
| Apple/Xcode/Keychain/certificate/signing/provider/product checks | Not run | Explicitly outside approved operational scope.                                                            |

The frozen-set, eligibility, source-claim, D-100 partition, historical-
preservation, exact-scope, external-contact, and independent-review manual gates
passed. No manual verification remains pending for this documentation
increment.

## Architecture findings

**Result: no completion-blocking finding.** D-103 adds no runtime edge, module,
dependency, IPC surface, platform adapter, entitlement, service, or portability
claim. “libSystem supervision” is closed to `posix_spawn`, `waitpid`, `kqueue`
`EVFILT_PROC`, `setpgid`, and process-group signaling; none is represented as a
complete containment graph. The negative decision is properly bounded and
keeps planned and current behavior separate.

## Security findings

**Advisory — operational build-child containment remains unavailable.**
Location: D-103 and the candidate table in the P3-2 plan. Why it matters: no
candidate in the frozen set satisfies every D-102 pre-effect filesystem,
network, descendant-membership, shutdown, reaping, quiescence, cleanup, and
privacy predicate without prohibited entitlement/signing/privilege/dependency/
external-resource expansion or, for the container branch, without an exact
qualifying public contract. Smallest safe correction: none exists inside this
plan. A future separately owner-approved documentation decision must change an
eligibility constraint or admit one exact newly identified candidate before a
controller can be planned. This blocks the next increment, not this bounded
negative documentation closeout.

The App Sandbox helper review distinguishes entitlement-bearing development/ad
hoc signatures from P4's later Developer ID signer-binding proof. Missing,
archived-only, ambiguous, or inferred guarantees remain `contract_unproven`.
All ten nonexistent-controller source checks and the exact eight unreviewed
contract checks remain `not_run`. Candidate/check/attempt binding is private;
unexpected provenance or target-derived material is `boundary_failed` and stops
without retry. No credential, sensitive identifier, raw target data, screenshot,
trace, packet, private path, host/process/account value, or free-text evidence
entered the repository.

## Code-health findings

**Result: no finding.** No production or test code changed. Documentation names,
candidate identities, 32 check dispositions, D-100 semantics, decision lineage,
and project-memory state are internally consistent. Formatting, links,
repository health, scope, and inventory checks passed.

## Technical debt

| Category             | Severity | Risk                                                                     | Effort | Milestone                  | Blocks completion | Blocks next increment |
| -------------------- | -------- | ------------------------------------------------------------------------ | ------ | -------------------------- | ----------------- | --------------------- |
| Security containment | Advisory | No eligible frozen candidate establishes D-102's operational guarantees. | Large  | Before any P3-3 controller | No                | Yes                   |

No implementation debt was introduced by this documentation-only change.

## Roadmap findings

**Blocked.** P3-2 closes truthfully with a negative bounded decision, but P3-3
has no selected primitive and cannot begin. P3-4, P3-5, P4, signing, V0-3, and
every product/external successor remain Blocked. No successor is Ready.

## Completion decision

**PASS WITH ADVISORIES.** All required documentation checks and manual reviews
passed. Optional product/build and operational checks were Not run by approved
scope. The advisory records the intended negative selection and does not weaken
D-102.

## Next-increment readiness

**Blocked.** There is no admitted next task. A future proposal requires a
separately approved documentation decision that changes an eligibility
constraint or introduces one exact new candidate; neither is authorized here.

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
- `docs/increments/personal-assistant-v0-containment-primitive-selection.md`
- `docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md`
- `docs/reviews/2026-09-02-personal-assistant-v0-containment-primitive-selection-post-increment-review.md`

## Exact commands executed

| Command                                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Result                                                                                                     |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-build-child-containment-primitive-selection`                                                                                                                                                                                                                                                                                                                                                | Failed; the descriptive identifier exceeded the gate's 64-character kebab-case bound and changed no state. |
| `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-containment-primitive-selection`                                                                                                                                                                                                                                                                                                                                                            | Passed; the same approved scope became active under the shortened identifier.                              |
| `npm exec prettier -- --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-containment-primitive-selection.md docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md`                                                                                                        | Passed; only declared documentation paths were formatted.                                                  |
| `npm exec prettier -- --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-containment-primitive-selection.md docs/plans/2026-09-02-personal-assistant-v0-containment-primitive-selection.md docs/reviews/2026-09-02-personal-assistant-v0-containment-primitive-selection-post-increment-review.md` | Passed; final exact-scope formatting completed.                                                            |
| `npm run docs:check`                                                                                                                                                                                                                                                                                                                                                                                                                                                             | Passed.                                                                                                    |
| `npm run repository:check`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | Passed.                                                                                                    |
| `npm run security:scan`                                                                                                                                                                                                                                                                                                                                                                                                                                                          | Passed.                                                                                                    |
| `git diff --check`                                                                                                                                                                                                                                                                                                                                                                                                                                                               | Passed.                                                                                                    |
| `git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json`                                                                                                                                                                                                                                                        | Passed; no protected path changed.                                                                         |
| `python3 .codex/hooks/session_end_gate.py`                                                                                                                                                                                                                                                                                                                                                                                                                                       | Passed; no conflicts or suspicious paths and exactly the declared fifteen paths changed.                   |
| `python3 .codex/hooks/post_increment_gate.py status`                                                                                                                                                                                                                                                                                                                                                                                                                             | Passed; correct shortened P3-2 identifier remained active before finalization.                             |
| `npm run verify`                                                                                                                                                                                                                                                                                                                                                                                                                                                                 | Not run by approved documentation-only scope.                                                              |
| `npm run tauri -- build --no-bundle`                                                                                                                                                                                                                                                                                                                                                                                                                                             | Not run by approved documentation-only scope.                                                              |
