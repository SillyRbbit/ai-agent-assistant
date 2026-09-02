# Personal Assistant v0 evidence-privacy protocol planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Increment: `personal-assistant-v0-evidence-privacy-protocol-planning`
Predecessor: completed D-099 signing-security prerequisite planning

## Goal

Define the smallest privacy-safe evidence protocol for any separately approved
future Personal Assistant v0 security or signing plan. The protocol constrains
evidence before it can cross into chat, Git, reports, logs, or other ordinary
project records. This increment documents the protocol only; it does not run,
implement, or authorize an evidence-producing operation.

## User-visible outcome

Future plans can refer to one closed evidence vocabulary instead of asking the
owner to send screenshots, copied output, identifiers, or free-text
descriptions. A result is useful only when its check identifier and outcome
were declared before the operation and the underlying data was minimized at
the source.

## Scope

1. Define `evidence_privacy_v1`, a documentation contract with exactly three
   fields:
   - `protocol_version`: fixed `evidence_privacy_v1`;
   - `check_id`: one fixed identifier declared by the separately approved plan;
   - `outcome`: one closed value from that check's predeclared allowlist.
2. Prohibit free-text values, arbitrary keys, dynamic or target-derived
   identifiers, raw counts, private/target-derived paths, diagnostics, excerpts,
   and caller-selected metadata.
3. Define owner-observation checks as prewritten questions with closed outcomes
   such as `observed`, `not_observed`, `not_run`, `pending`, `unavailable`, or
   `boundary_failed`. Every operational check must include `boundary_failed`;
   each future plan selects the smallest other applicable outcomes before
   execution and may define narrower check-specific outcomes.
4. Require source minimization: raw data must not cross the approved local
   inspection boundary and then be redacted. If the operation cannot emit only
   the predeclared category, it remains Not run.
5. Define categorical incident handling for accidental exposure without
   copying, retransmitting, or rewriting historical evidence.
6. Reconcile only the exact fifteen documentation paths listed below.

## Explicit non-goals

- No Apple, Xcode, Keychain, certificate, private-key, signing, build,
  notarization, credential, provider, model, network, product, target-Mac
  security/signing/product, or external-system operational work.
- No evidence sanitizer, wrapper, command, script, hook, runtime, IPC,
  filesystem access, log sink, attachment flow, clipboard flow, screenshot, or
  recording implementation.
- No product source, dependency, lockfile, capability, CSP, permission,
  workflow, runner, configuration, or test-code change.
- No rerun of the consumed identity query and no acceptance of the Pending Open
  Directory boundary.
- No branch, commit, push, merge, pull request, release, or publication.

## Existing behavior and constraints

- D-097 remains `failed` / `FAIL` / `Blocked` with no completion marker.
- D-097's original report, report digest, reconstructed state digest, and
  historical finding set remain unchanged.
- The historical screenshot/privacy finding remains Failed.
- The `getpwuid`/`opendirectoryd` boundary remains Manual verification pending.
- Signing and every target-Mac security/signing/product operational check remain
  Not run.
- D-098 lineage and the completed D-099 record remain historical evidence and
  are not modified by this plan.
- P2 account-directory, P3 executable-build-child containment, and P4 immutable
  signer binding remain Proposed/Blocked.

## Current-state evidence

- Baseline `HEAD` and `origin/main` are both
  `096cdbe0139832010749039149fb2e335cfa19fd`, with ahead/behind `0/0` and a clean
  working tree.
- `git fsck --full --no-dangling` completed successfully.
- The ignored prior gate reports the completed
  `personal-assistant-v0-signing-security-prerequisite-planning` increment as
  valid `PASS WITH ADVISORIES`.
- Repository-pinned toolchains are Node `26.3.0`, npm `11.16.0`, Cargo/Rust
  `1.90.0`; the read-only platform prerequisite check completed, but no host
  detail is retained as protocol evidence.
- Baseline `npm run docs:check` passed.

## Files expected to change

Exactly:

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
13. `docs/increments/personal-assistant-v0-evidence-privacy-protocol-planning.md`
14. this plan
15. `docs/reviews/2026-09-01-personal-assistant-v0-evidence-privacy-protocol-planning-post-increment-review.md`

## Affected components

| Component                   | Effect                                                        |
| --------------------------- | ------------------------------------------------------------- |
| Repository governance       | Adds one closed evidence vocabulary for future approved work. |
| Security/privacy planning   | Requires source minimization and categorical results.         |
| Product runtime and IPC     | None.                                                         |
| Target Mac/external systems | None.                                                         |

## Interfaces and invariants

### Closed record

```text
EvidencePrivacyV1 {
  protocol_version: "evidence_privacy_v1",
  check_id: <exact plan-owned enum member>,
  outcome: <exact check-owned enum member>
}
```

- The separately approved future plan owns the check and outcome allowlists;
  the caller, owner response, UI, model, command output, or environment cannot
  extend them at runtime.
- `check_id` and `outcome` are repository-reviewed static lowercase ASCII
  tokens matching `[a-z][a-z0-9_]*`; `check_id` is at most 64 bytes and
  `outcome` at most 32 bytes. Their spellings and allowlists are never derived
  from the inspected target; `check_id` is fixed before inspection, and only
  selection of one already allowed outcome may depend on the bounded predicate.
- Every operational check's predeclared outcome allowlist includes
  `boundary_failed`, so an incident never requires widening the contract after
  execution begins.
- The record contains no optional extension map, free-text note, timestamp,
  hostname, user/account name, path, identity, certificate metadata,
  fingerprint, serial, label, value, excerpt, count, diagnostic, or raw error.
- Human observation uses one request for one prewritten question and accepts
  one closed outcome only. No screenshot, attachment, pasted output, recording,
  or explanatory free text is evidence.
- Automated evidence is permitted only when a separately reviewed local
  boundary discards raw bytes before returning the closed record. Capturing
  first and redacting later is not compliant.
- `boundary_failed` records only that the evidence boundary failed. It does not
  carry the exposed content, clear the failure, or authorize retry.
- Each check describes only one directly observable or deterministically
  established bounded predicate. Check or outcome semantics cannot claim
  approval, authorization, safety, readiness, broad verification, exclusivity,
  historical absence, or a permission to proceed.
- The three-field record has no standalone provenance, freshness,
  authentication, authorization, audit, or readiness meaning. A future trusted
  consumer must bind it in private application-owned state to one separately
  approved plan, check, and attempt; accept it once and only during that
  attempt's fixed window; and reject duplicate, replayed, cross-plan,
  pre-admission, or late records.
- Any future serialization accepts exactly one record with exactly the three
  unique keys in declared order, string values only, compact ASCII JSON, no
  leading/trailing whitespace or bytes, and at most 256 bytes total. Token
  comparison is exact; trimming, case folding, Unicode normalization,
  confusable mapping, and aliasing are prohibited.
- Unknown check IDs, outcomes, keys, versions, malformed records, prompts,
  ambiguity, unexpected output, or unapproved side effects fail closed and stop
  the active operation without retry.

### Prohibited evidence

For a future separately approved security or signing check, the following
inspection-target or operation-derived payload cannot enter chat, Git, issue/PR
text, reports, logs, test fixtures, screenshots, recordings, clipboard
transfers, or ordinary CI:

- GUI or terminal captures, transcripts, stdout/stderr, raw logs, traces,
  diagnostics, exception text, or copied command output;
- raw, sensitive, dynamic, or target-derived account, certificate, key, signer,
  device, host, team, developer, provider, operational, or credential
  identifiers and metadata;
- fingerprints, serials, labels, email addresses, personal names, private
  paths, tokens, secrets, passwords, key bytes, authorization material, or
  sensitive prompt/content values.

Static repository-owned protocol literals and necessary non-sensitive
repository validation/governance metadata that are not derived from the
inspected security target are not operational evidence and remain permitted in
documentation.

### Evidence-boundary incident

If prohibited evidence appears, stop the operation; do not copy, quote,
summarize, retransmit, attach, or rerun it. Preserve existing historical facts
without claiming that an immutable system record was removed. Record only the
applicable closed `boundary_failed` outcome and require a separately approved
incident/disposition plan before related operational work can resume.

## Implementation milestones

- [x] Confirm clean synchronized baseline, toolchains, prior gate, and
      documentation baseline.
- [x] Add D-100 and the closed `evidence_privacy_v1` documentation contract.
- [x] Reconcile project-memory, security, testing, and troubleshooting records.
- [x] Run documentation-tier validation and independent reviews.
- [x] Record the post-increment review; gate finalization is the immediate final
      action after the frozen passing evidence.

## Security and privacy considerations

| Threat                                    | Required control                                                    |
| ----------------------------------------- | ------------------------------------------------------------------- |
| Sensitive data is captured then redacted  | Minimize at source; raw data never crosses the evidence boundary.   |
| Free text smuggles identifiers or content | Exact check/outcome enums; reject extra keys and explanations.      |
| Screenshot repeats the historical failure | Screenshots and recordings are prohibited evidence.                 |
| Passing docs are mistaken for enforcement | State that no sanitizer, operation, or runtime changed.             |
| Failure is hidden or retried              | Closed `boundary_failed`, stop, preserve history, no retry.         |
| Owner observation becomes broad authority | One prewritten question, one closed answer, one approved operation. |
| Valid category is replayed or substituted | Private plan/check/attempt binding; once-only in-window acceptance. |
| Enum name smuggles authority or safety    | One bounded predicate; categories remain strictly non-authorizing.  |
| Parser variants bypass a closed enum      | Exact bounded ASCII tokens and canonical one-record serialization.  |

## Test plan

- Inspect the complete diff and confirm it matches the exact fifteen paths.
- Confirm D-097/D-098/D-099 and all historical Failed/Pending/Not-run evidence
  remain unchanged.
- Table-review accepted examples:
  - fixed plan-owned check + `observed`;
  - fixed plan-owned check + `not_observed`;
  - fixed plan-owned check + `not_run`;
  - fixed plan-owned check + `pending`;
  - fixed plan-owned check + `unavailable`;
  - fixed plan-owned check + `boundary_failed`.
- Table-review rejected examples: unknown check/outcome/version; extra key;
  missing required key; wrong key order; non-string field; an operational check
  table that omits `boundary_failed`; free-text explanation; target-derived
  identifier or private path; numeric/raw output; duplicate key; multiple
  record; non-ASCII/confusable/case/whitespace variant; overlength/over-256-byte
  record; duplicate, replayed, cross-plan, pre-admission, or late record;
  outcome semantics that claim `approved`, `authorized`, `safe`, `ready`,
  `verified`, `exclusive`, `non_exported`, or another broader property;
  screenshot, attachment, transcript, recording, log, or copied GUI/terminal
  text.
- Run documentation formatting/link, repository-health, secret-scan,
  protected-path, whitespace, session-end, and post-increment checks.
- Target-Mac Apple, Xcode, Keychain, certificate, signing, build, credential,
  provider, network, and product-runtime operational checks are Not run by
  design.

## Verification commands

```bash
npm run docs:check
npm run repository:check
npm run security:scan
git diff --check
git diff --exit-code -- src src-tauri package.json package-lock.json src-tauri/Cargo.toml src-tauri/Cargo.lock .github .codex/hooks scripts
python3 .codex/hooks/session_end_gate.py
python3 .codex/hooks/post_increment_gate.py status
```

## Manual gates

- Confirm the record shape and both accepted/rejected tables are closed and
  contain no target-derived identifier or other prohibited evidence.
- Confirm no prohibited security/signing evidence was viewed, requested,
  captured, copied, or transmitted during this increment.
- Confirm target-Mac security/signing/product and other operational checks
  remain Not run.
- Confirm no later P2–P4 or operational increment is made Ready.

## Dependencies

- D-099 completed prerequisite map.
- Existing repository documentation/gate workflows only.
- No new dependency, external account, service, credential, or tool.

## Risks

- A future plan might cite this protocol without implementing source
  minimization at its actual boundary.
- A closed category can prove only the declared observation, not an unstated
  security property.
- Documentation cannot erase the historical privacy failure or establish
  target-Mac behavior.

## Rollback or failure strategy

If validation fails, correct only these fifteen documentation paths. If a
truthful passing closeout is impossible, record a normal terminal failure; do
not change predecessor evidence or broaden scope. A future rollback reverts
only this documentation increment and leaves D-097/D-098/D-099 intact.

## Stop conditions

Stop on any request for or appearance of raw evidence; any file outside the
declared scope; any target-Mac security/signing/product, Apple, Keychain,
certificate, private-key, signing, build, credential, provider, network,
product, or external-system operational action; any need for a new dependency
or executable enforcement; any dirty or divergent Git state not caused by this
increment; or any failed security/gate check that cannot be resolved inside the
exact documentation scope.

## Decisions made

- D-100 defines the documentation-only `evidence_privacy_v1` protocol.

## Discoveries

- P1 can define a safe vocabulary without collecting new operational evidence.
- After-the-fact redaction is weaker than emitting a category at the source.

## Progress

- 2026-09-01: Owner approved the P1 documentation-only planning increment.
- 2026-09-01: Began from clean synchronized `main` at `096cdbe` after the
  documentation baseline passed.
- 2026-09-01: Architecture review passed; security and code-health review
  passed with only the intentional documentation-not-enforcement advisory.
- 2026-09-01: Required documentation, repository, secret, protected-path,
  whitespace, and session-end checks passed.

## Acceptance criteria

- [x] The protocol is exact, categorical, bounded, and contains no free text.
- [x] Static tokens and serialization have exact lexical, count, uniqueness,
      ordering, and size bounds.
- [x] A future consumer privately binds one record to one plan/check/attempt
      and rejects duplicate, replayed, substituted, pre-admission, or late data.
- [x] Check/outcome semantics are bounded observations and never authorize,
      approve, declare safety/readiness, or claim a broader security property.
- [x] Source minimization is mandatory and after-the-fact redaction is denied.
- [x] Screenshots, recordings, raw output, target-derived sensitive
      identifiers/metadata, private paths, credentials, and content are
      prohibited evidence.
- [x] Boundary failure stops without retry and preserves historical truth.
- [x] D-097/D-098/D-099 and Failed/Pending/Not-run evidence remain unchanged.
- [x] P2–P4 and all operational successors remain Proposed/Blocked.
- [x] The exact documentation checks and frozen post-increment review pass;
      marker finalization is the immediate final action.

## Final results

`PASS WITH ADVISORIES`. The documentation establishes a closed P1 policy but no
parser, sanitizer, trusted local minimization boundary, consumer, or operational
evidence path. D-097/D-098/D-099 and all historical Failed/Pending/Not-run
evidence remain unchanged. P2–P4 and every operational successor remain
Blocked.

## Documentation updates

- [x] `ARCHITECTURE.md`
- [x] `CHANGELOG.md`
- [x] `DECISIONS.md`
- [x] `HANDOFF.md`
- [x] `NEXT_STEPS.md`
- [x] `PLANS.md`
- [x] `PROJECT_STATUS.md`
- [x] `ROADMAP.md`
- [x] `SECURITY.md`
- [x] `SECURITY_CHECKLIST.md`
- [x] `TESTING_GUIDE.md`
- [x] `TROUBLESHOOTING_LOG.md`
- [x] Increment record and post-increment review
