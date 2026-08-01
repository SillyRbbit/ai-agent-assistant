# TS-017 signed macOS identity path decision post-increment review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "npm run docs:check",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock",
    "python3 .codex/hooks/session_end_gate.py"
  ],
  "files_changed": [
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PROJECT_STATUS.md",
    "TROUBLESHOOTING_LOG.md",
    "docs/reviews/2026-08-01-ts-017-signed-identity-path-decision-post-increment-review.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Roadmap",
      "effort": "separate evidence-based plan and owner approval before any future support or alternate-CSR path",
      "milestone": "before diagnostic repetition, CSR retry, certificate creation, signing, or credential ingestion",
      "risk": "TS-017 remains unresolved and D-076 deliberately defers every recovery option",
      "severity": "Advisory",
      "summary": "No operational follow-up is Ready; the signed macOS identity path remains deferred."
    }
  ],
  "increment_id": "ts-017-signed-identity-path-decision",
  "manual_verification": [
    {
      "check": "Owner selected defer after comparing deferral, future Apple Support assistance, and a future alternate CSR workflow.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-076 preserves the TS-017 no-asset baseline and cause: not determined while prohibiting every operational action.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "D-064's 15-minute production maximum and D-068's 30-day owner-only demo exception remain unchanged.",
      "required": true,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-08-01
Increment: ts-017-signed-identity-path-decision
Branch: main

## Executive summary

D-076 records the owner's documentation-only decision to defer the signed macOS
identity path after TS-017. The owner considered deferral, future Apple Support
assistance, and a future alternate CSR workflow, then selected deferral. The
decision preserves the no-asset baseline and `not determined` cause. It creates
no Apple, Keychain, signing, credential, Cloudflare, provider, deployment,
traffic, code, dependency, or runtime capability. Quality-gate result:
**PASS WITH ADVISORIES**.

## Scope and boundaries

The approved scope was exactly eight documentation files. D-076 does not
diagnose TS-017 or authorize a recovery path. Apple Support and alternate CSR
workflows remain separate future options requiring their own owner-approved
plans, privacy controls, stop conditions, and proof that D-072's target-Mac,
owner-controlled, non-exported private-key boundary is preserved.

## Verification results

- `npm run docs:check`: **Passed** — formatting and internal-link validation
  passed.
- `npm run repository:check`: **Passed** — repository-health checks passed.
- `npm run security:scan`: **Passed** — secret scan passed.
- `git diff --check`: **Passed** — no whitespace errors.
- `git diff --exit-code -- src src-tauri package.json package-lock.json
src-tauri/Cargo.toml src-tauri/Cargo.lock`: **Passed** — no product or
  dependency path changed.
- `python3 .codex/hooks/session_end_gate.py`: **Passed** — no conflicts;
  inventory contained only the approved documentation paths.
- Manual owner decision: **Passed** — the owner selected deferral after the
  three options were compared.
- Manual policy review: **Passed** — D-064 and D-068 remain unchanged.

## Architecture findings

None. No application, adapter, IPC, permission, storage, dependency, network,
or runtime path changed. The decision does not represent planned recovery as a
current capability.

## Security findings

None. D-076 preserves D-072's owner-controlled non-exported-private-key
boundary, the TS-017 no-asset baseline, and the unknown cause. It gives no
authority to Apple Support, alternate tools, Keychain changes, signing, or
credentials.

## Code-health findings

None. No source, test, configuration, dependency, generated, or build path
changed.

## Technical debt

One advisory operational gap: the native Certificate Assistant failure remains
undetermined and the signed macOS identity path is deferred. This does not block
the decision record but blocks every recovery, signing, fake-only proof, and
credential increment.

## Roadmap findings

`$readiness-review` result: **Blocked**. No support, alternate CSR, repeat
diagnostic, retry, signing, or credential increment is Ready. The smallest
future task, if the owner reopens the path, is a separate documentation-only
plan for one selected evidence route.

## Completion decision

**PASS WITH ADVISORIES**

## Next-increment readiness

**Blocked** — D-076 deliberately defers every operational path.

## Exact files changed

- `CHANGELOG.md`
- `DECISIONS.md`
- `HANDOFF.md`
- `NEXT_STEPS.md`
- `PLANS.md`
- `PROJECT_STATUS.md`
- `TROUBLESHOOTING_LOG.md`
- `docs/reviews/2026-08-01-ts-017-signed-identity-path-decision-post-increment-review.md`

## Exact commands executed

- `npm run docs:check` — passed.
- `npm run repository:check` — passed.
- `npm run security:scan` — passed.
- `git diff --check` — passed.
- `git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock` — passed.
- `python3 .codex/hooks/session_end_gate.py` — passed.
