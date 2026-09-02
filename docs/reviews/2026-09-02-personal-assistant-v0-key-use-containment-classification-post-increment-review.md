# Personal Assistant v0 in-process key-use containment classification post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git rev-parse HEAD",
    "git rev-parse origin/main",
    "git merge-base --is-ancestor HEAD origin/main",
    "git switch -c codex/p3-in-process-key-use-containment-classification",
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-in-process-key-use-containment-classification",
    "python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-key-use-containment-classification",
    "npm exec prettier -- --write ARCHITECTURE.md CHANGELOG.md DECISIONS.md HANDOFF.md NEXT_STEPS.md PLANS.md PROJECT_STATUS.md ROADMAP.md SECURITY.md SECURITY_CHECKLIST.md TESTING_GUIDE.md TROUBLESHOOTING_LOG.md docs/increments/personal-assistant-v0-key-use-containment-classification.md docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md",
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri package.json package-lock.json .npmrc rust-toolchain.toml .github .codex/hooks .agents scripts eslint.config.js vite.config.ts tsconfig.app.json tsconfig.json tsconfig.node.json",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py status",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment personal-assistant-v0-key-use-containment-classification --report docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md",
    "npm run verify",
    "npm audit --audit-level=low",
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
    "docs/increments/personal-assistant-v0-key-use-containment-classification.md",
    "docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md",
    "docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "Before any P3-3, P3-4, P3-5, P4, operational signing, or V0-3 work",
      "risk": "Eleven conjunctive contracts remain unproved; operating or admitting the candidate could use ambient identity authority, prompt, continue after cancellation, leak metadata, or bypass D-102 and product-signing prerequisites.",
      "severity": "Advisory",
      "summary": "The in-process challenge candidate is not admitted and no successor is Ready."
    }
  ],
  "increment_id": "personal-assistant-v0-key-use-containment-classification",
  "manual_verification": [
    {
      "check": "Exact candidate identity, frozen source register, nineteen contract rows, and documented=8/contract_unproven=11/not_run=0/boundary_failed=0 totals",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Canonical compact D-100 factual record remains separate from the D-107 governance result and contains no target-derived value",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Architecture review: the childless and fileless challenge class is scope reduction, not a D-102 containment primitive, waiver, product-signing proof, or current architecture edge",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Security review: D-101 identity provenance, exact signer binding, no-export reachability, interaction denial, fixed algorithm, cancellation, late-result handling, cleanup, platform effects, and D-102 split remain unresolved",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Privacy review: no identity, key, certificate, signature, challenge, account, path, label, serial, fingerprint, Team ID, raw error, prompt, or other target-derived value entered the diff",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "History review: D-097 failed/FAIL/Blocked evidence, report, digests, Failed privacy finding, Pending Open Directory finding, Not-run signing, absent completion marker, D-098 disposition, and D-096 through D-106 remain unchanged",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Code-health, technical-debt, documentation, and readiness review over the complete fifteen-path documentation-only change set",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "Apple account, Xcode, Keychain, certificate, identity, private-key, signing, verification, build, process, filesystem, network, target-Mac, provider, product, and external-system checks",
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
      "command": "npm audit --audit-level=low",
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
Increment: `personal-assistant-v0-key-use-containment-classification`
Branch: `codex/p3-in-process-key-use-containment-classification`
Baseline: `2287c1bfe999d733495ee78728f4dc7a653f393f`

## Executive summary

The approved fifteen-path documentation-only source classification is
complete. D-107 records `not_eligible_or_unproven` for exactly
`in_process_security_framework_ephemeral_challenge_proof_v1`: eight rows are
`documented`, eleven are `contract_unproven`, and none are `not_run` or
`boundary_failed`. The candidate is materially distinct because it removes a
proposed build, helper, child, bundle, artifact, filesystem write, and
`codesign` process from this narrow proof class. The source record does not
admit it, waive D-101 or D-102, or make a successor Ready. Quality gate: **PASS
WITH ADVISORIES**.

## Scope and boundaries

The exact scope was static review of one frozen candidate against the frozen
Apple public and checksum-resolved pinned Rust source register plus
reconciliation of fifteen documentation paths. Read-only Apple documentation
retrieval was the only external contact. No product/test source, dependency,
lockfile, configuration, capability, CSP, permission, entitlement, hook,
workflow, build, process, filesystem, Keychain/private-key operation, target-
Mac check, provider, product, or state-changing external action occurred.

D-097 remains `failed` / `FAIL` / `Blocked` with its original report and
digests, Failed screenshot/privacy finding, Pending Open Directory boundary,
Not-run signing result, and absent completion marker. D-098 remains a valid,
immutable, non-reusable schema-v3 disposition. D-096 and D-100 through D-106
remain unchanged.

## Verification results

| Check                                                            | Status  | Evidence                                                                                                                                                        |
| ---------------------------------------------------------------- | ------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `npm run docs:check`                                             | Passed  | Formatting and repository link-health checks passed.                                                                                                            |
| `npm run repository:check`                                       | Passed  | Repository-health `all` passed.                                                                                                                                 |
| `npm run security:scan`                                          | Passed  | Repository secret scan passed.                                                                                                                                  |
| `git diff --check`                                               | Passed  | No whitespace errors.                                                                                                                                           |
| Protected-path diff                                              | Passed  | No product source, dependency, configuration, workflow, hook, skill, or script path changed.                                                                    |
| Session inventory                                                | Passed  | No conflicts; the changed-file inventory equals the declared fifteen documentation paths.                                                                       |
| Active-gate status                                               | Passed  | The exact shortened increment identifier remained active before finalization.                                                                                   |
| Initial 67-character gate identifier                             | Failed  | The hook rejected the overlength identifier before state changed. The valid shorter identifier preserved candidate and scope.                                   |
| First sandboxed write using the valid identifier                 | Failed  | Filesystem sandboxing prevented the ignored state write; the owner-authorized exact command then succeeded with permitted access and no repository-file change. |
| Initial finalization attempt                                     | Failed  | Unsupported finding category `Security architecture` was rejected before completion state changed; correcting it to `Security` preserved the finding.           |
| Second finalization attempt                                      | Failed  | A duplicate identical gate-status entry in the machine-readable command list was rejected before completion state changed; deduplication preserved the history. |
| Third finalization attempt                                       | Failed  | The schema requires every verification command identifier, including a Not-run check, in the command inventory; adding the identifiers did not run the checks.  |
| `npm run verify`                                                 | Not run | Documentation-only scope introduced no runtime or dependency change.                                                                                            |
| `npm audit --audit-level=low`                                    | Not run | No dependency or lockfile changed.                                                                                                                              |
| `npm run tauri -- build --no-bundle`                             | Not run | Documentation-only scope prohibited native build execution.                                                                                                     |
| Apple/Xcode/Keychain/private-key/signing/build/target-Mac checks | Not run | Explicitly outside the approved operational scope.                                                                                                              |

No required check or manual gate remains Failed, Not run, or Pending. The five
failed setup/finalization commands changed no repository or gate state and were
resolved without changing candidate scope or weakening a boundary.

## Architecture findings

No completion-blocking finding. The candidate is a planned, macOS-only
containment-by-elimination concept and creates no current module, runtime, IPC,
or product edge. A future implementation would move sensitive signing
capability into the Cortexa process and would therefore require one private
adapter behind a no-input, non-serializable, single-use wrapper. Security
framework types, signer selection, challenge bytes, algorithms, and signing
authority may not reach domain code, Tauri, the WebView, persistence, logs, or
tests.

Removing a child and artifact is not a D-102 containment primitive or waiver.
D-102 remains fully applicable to every product/build-bearing path.

## Security findings

No completion-blocking finding for this documentation-only classification. The
candidate is blocked operationally by eleven unproved contracts. In particular:

- no D-101-compliant application credential-domain identity source exists;
- no immutable Developer ID signer binding exists;
- the exact no-export call graph and fixed algorithm are unproved;
- lookup-time UI controls do not prove prompt-free private-key use;
- synchronous key operations have no sourced hard cancellation boundary;
- result rejection does not stop late private-key use;
- cleanup/quarantine and OS-managed effects remain unproved; and
- D-102 contains no accepted applicability split for this proof class.

No secret, personal identifier, target-derived certificate metadata, raw
error, challenge, signature, or credential entered the diff. No permission,
capability, CSP, unsafe-Rust, dependency, filesystem, network, storage, IPC, or
execution boundary changed.

## Code-health findings

No finding. The change is documentation-only, uses one candidate identity and
one consistent closed vocabulary, links the accepted decision and plan, and
distinguishes current, planned, prohibited, and Not-run behavior. No source or
test code changed.

## Technical debt

One pre-existing security-architecture advisory remains: eleven conjunctive
key-use boundary contracts are unproved. Severity: **Advisory**. Risk: an
operational attempt could widen identity authority, prompt, outlive
cancellation, leak metadata, or be overstated as product signing. Effort:
**Large**. Milestone: before any P3-3 through P3-5, P4, signing, or V0-3 work.
It does not block this truthful documentation closeout; it blocks every
operational successor.

## Roadmap findings

**Blocked.** D-107 admits no candidate. D-101, D-102, P3-3 through P3-5, P4,
operational signing, V0-3, and every product/external successor remain Blocked.
The smallest future action is an owner-selected, separately approved
documentation-only decision addressing one exact unresolved constraint or one
new specifically named candidate. No branch, gate, edit, or operation starts
automatically.

## Completion decision

**PASS WITH ADVISORIES.** The bounded negative classification is complete,
truthful, internally consistent, and documentation-only. The advisory blocks a
successor but not this closeout.

## Next-increment readiness

**Blocked.** No next increment is admitted or Ready.

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
13. `docs/increments/personal-assistant-v0-key-use-containment-classification.md`
14. `docs/plans/2026-09-02-personal-assistant-v0-key-use-containment-classification.md`
15. `docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md`

## Exact commands executed

The gate schema requires every verification command identifier in its
`commands_executed` inventory, including checks explicitly classified `Not
run`. The per-command statuses below are authoritative; inventory membership
does not assert that a Not-run check executed.

- `git status --short --branch` — Passed.
- `git rev-parse HEAD` — Passed; baseline
  `2287c1bfe999d733495ee78728f4dc7a653f393f`.
- `git rev-parse origin/main` — Passed; matched the baseline.
- `git merge-base --is-ancestor HEAD origin/main` — Passed.
- `python3 .codex/hooks/post_increment_gate.py status` — Passed.
- `git switch -c codex/p3-in-process-key-use-containment-classification` —
  Passed.
- `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-in-process-key-use-containment-classification`
  — Failed closed because the identifier was 67 characters; no state changed.
- `python3 .codex/hooks/post_increment_gate.py begin --increment personal-assistant-v0-key-use-containment-classification`
  — first sandboxed attempt Failed to write ignored state; the exact
  owner-authorized rerun Passed.
- Read-only `grep` and `sed` inspections of the hook, current decisions,
  project memory, plan templates, pinned manifest/lockfile, and checksum-
  resolved crate source — Passed.
- Read-only retrieval of the frozen Apple public source register — Passed.
- `npm exec prettier -- --write ...` over the exact documentation allowlist —
  Passed.
- `npm run docs:check` — Passed.
- `npm run repository:check` — Passed.
- `npm run security:scan` — Passed.
- `git diff --check` — Passed.
- Protected-path diff command from the manifest — Passed.
- `python3 .codex/hooks/session_end_gate.py` — Passed.
- Final pre-finalization gate status — Passed.
- `python3 .codex/hooks/post_increment_gate.py finalize --increment personal-assistant-v0-key-use-containment-classification --report docs/reviews/2026-09-02-personal-assistant-v0-key-use-containment-classification-post-increment-review.md`
  — first attempt Failed closed on the unsupported finding category; the second
  attempt Failed closed on a duplicate command-manifest entry; the third attempt
  Failed closed because Not-run verification identifiers were absent from that
  inventory; the corrected exact rerun Passed.
- `npm run verify` — Not run; documentation-only scope.
- `npm audit --audit-level=low` — Not run; no dependency change.
- `npm run tauri -- build --no-bundle` — Not run; no build authorized.
