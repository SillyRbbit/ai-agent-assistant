# Meta Increment 2 - engineering operating system

Status: Complete; verified 2026-07-15
Owner: Project maintainer
Last updated: 2026-07-15

## Goal

Create one authoritative engineering operating system for Cortexa covering
engineering practice, current architecture, normalized product requirements,
roadmap status, testing, security review, and release preparation without
changing product behavior.

## User-visible outcome

No application behavior changes. Maintainers, reviewers, and coding assistants
gain a consistent source for what the repository implements, how work becomes
Ready and Done, how it is tested and reviewed, and what blocks production
release.

## Scope

- Create seven root-level authoritative guides.
- Reconcile root project memory with merged Meta Increment 1 at `5edbf4d`.
- Define documentation ownership and precedence while preserving historical
  evidence.
- Correct README claims about phase, SQLite, and current Rust boundaries.
- Distinguish current, mocked, planned, and prohibited architecture.
- Record the owner-directed reassignment of Meta Increment 2 and renumber the
  unimplemented application-icon rollout to Meta Increment 3.
- Mark Meta Increment 3 Ready only after Meta Increment 2 passes every gate.

## Explicit non-goals

- Application source, test, runtime, behavior, UI, branding asset, or icon
  changes.
- Dependencies, manifests, lockfiles, package, crate, executable, bundle,
  database, GitHub, command, event, or storage identifier changes.
- Tauri commands, events, capabilities, CSP, plugins, entitlements, permissions,
  configuration, or generated schema changes.
- SQLite schema, migration, database, persistence, or startup changes.
- Gateway, provider, credential, model, tool, policy, approval, audit, memory,
  platform adapter, dispatch, executor, or release implementation.
- Meta Increment 3, Increment 4V, or any later product increment implementation.
- Rewriting completed review reports or historical decisions to hide their
  original checkpoint state.

## Existing behavior and conflicts

- `main`, `origin/main`, and this branch began at `5edbf4d`; the tree was clean.
- The `meta-01` marker was complete and valid with `PASS WITH ADVISORIES`.
- `README.md` still described Phase 2, said SQLite was absent, and understated
  the transport-free Phase 4 boundaries.
- `ARCHITECTURE_BASELINE.md` mixed target ownership with current implementation.
- `PRODUCT_BRIEF.md` identified inception intent but did not distinguish current
  implementation status.
- Root memory described merged Meta Increment 1 as uncommitted.
- Meta Increment 2 was already assigned to the unimplemented icon plan, while
  the project owner explicitly assigned Meta Increment 2 to this documentation
  increment and requested Meta Increment 3 become Ready.
- Engineering, review, testing, security, release, session, and completion rules
  were duplicated across root documents, workflows, prompts, and skills without
  an explicit authority order.

## Exact files

Created:

```text
ENGINEERING_GUIDE.md
ARCHITECTURE.md
PRODUCT_REQUIREMENTS.md
ROADMAP.md
TESTING_GUIDE.md
RELEASE_CHECKLIST.md
SECURITY_CHECKLIST.md
docs/increments/meta-02-engineering-operating-system.md
docs/plans/meta-02-engineering-operating-system.md
docs/reviews/2026-07-15-meta-02-post-increment-review.md
```

Renamed and updated:

```text
docs/plans/meta-02-verified-application-icon-rollout.md
docs/plans/meta-03-verified-application-icon-rollout.md
```

Modified:

```text
AGENTS.md
ASSISTANT_USAGE.md
CHANGELOG.md
CODE_REVIEW.md
CONTRIBUTING.md
DECISIONS.md
HANDOFF.md
NEXT_STEPS.md
PLANS.md
PROJECT_STATUS.md
README.md
SECURITY.md
TROUBLESHOOTING_LOG.md
docs/increments/meta-01-branding-foundation.md
docs/plans/README.md
docs/product/ARCHITECTURE_BASELINE.md
docs/product/PRODUCT_BRIEF.md
```

No other path may change without renewed project-owner approval.

## Documentation ownership

- `ENGINEERING_GUIDE.md`: engineering operating model and document precedence.
- `ARCHITECTURE.md`: current implementation and approved target boundaries.
- `PRODUCT_REQUIREMENTS.md`: normalized current product requirements.
- `ROADMAP.md`: milestone status and acceptance gates.
- `TESTING_GUIDE.md`: test layers, commands, mocks, and completion evidence.
- `SECURITY.md`: normative security policy.
- `SECURITY_CHECKLIST.md`: change and release security review procedure.
- `CODE_REVIEW.md`: finding order, severity, and evidence expectations.
- `RELEASE_CHECKLIST.md`: production release gate.
- `NEXT_STEPS.md`: ordered immediate implementation queue.
- Historical plans, records, reviews, decisions, changelog, and troubleshooting
  entries: checkpoint evidence, preserved or explicitly superseded.

## Implementation steps

- [x] Read all root documentation, project memory, workflows, prompts, skills,
      plans, records, Rust modules, React structure, SQLite design, Tauri config,
      and branding guidance.
- [x] Confirm clean `meta/engineering-operating-system` at `5edbf4d`.
- [x] Pass the smallest formatting baseline.
- [x] State and receive approval for the exact 29-path Git scope.
- [x] Begin mandatory `meta-02` gate state before edits.
- [x] Create and cross-link the seven authoritative guides.
- [x] Reconcile active memory, authority, and Meta 2/3 numbering.
- [x] Verify formatting, links, protected paths, and the complete repository.
- [x] Review the full diff for scope, security, contradictions, and stale claims.
- [x] Synchronize completion evidence and run the mandatory post-increment gate.

## Security and privacy considerations

This increment introduces no product trust boundary or data flow. Documentation
must not include credentials, private keys, tokens, personal data, local database
content, or unnecessary private paths. It must preserve the model-as-untrusted-
planner rule, least-privilege Tauri boundary, credential ownership, exact
approval, non-authorizing receipt, SQLite, redaction, and release controls.

## Risks

- A consolidated guide could conflict with an accepted historical decision.
- Target architecture could be presented as current capability.
- Renumbering could leave a live broken link or ambiguous queue entry.
- Release checklists could imply signing or notarization is configured.
- Broad documentation edits could accidentally touch source or generated output.
- Historical evidence could be rewritten instead of explicitly superseded.

Mitigations are explicit document precedence, status labels, a protected-path
diff, link/path validation, preserved historical reports, D-044, full
verification, and the mandatory gate.

## Verification commands

```bash
npm run format:check
npm run verify
git diff --check
git status --short
python3 .codex/hooks/post_increment_gate.py status
```

Also run a Python standard-library audit of relative Markdown links and local
image references, scan current-state files for stale Meta 1 publication and
Meta 2 icon-plan claims, verify the old icon-plan path has no live link, confirm
every declared new document is discoverable, and prove application source,
tests, manifests, lockfiles, Tauri config, capabilities, CSP, migrations, and
branding assets are unchanged.

No manual application check is required because no product or rendered UI file
changes.

## Rollback

Before commit, remove the ten created files, restore the seventeen modified
files, and rename the Meta 3 icon plan back to its original Meta 2 path. After a
commit, revert the one bounded documentation commit. No migration, dependency,
identifier, credential, external service, or user data requires rollback.

## Acceptance criteria

- [x] The exact approved path scope is preserved.
- [x] All seven requested authoritative documents exist and contain the required
      subjects.
- [x] Current architecture and product requirements match actual source and
      tests without overstating planned capability.
- [x] Documentation authority and supersession rules are explicit.
- [x] Meta Increment 1 publication state is current.
- [x] The icon rollout is consistently Meta Increment 3 in live queue and plan
      documents, while historical evidence remains explainable.
- [x] Meta Increment 3 is Ready but unimplemented after Meta Increment 2 passes.
- [x] Markdown formatting, links, paths, full repository verification, scope,
      code review, security review, and mandatory gate pass.
- [x] No application behavior, source, config, dependency, schema, capability,
      permission, icon, or identifier changes.

## Actual results

The exact approved documentation-only scope establishes the seven root
authorities, reconciles current implementation and publication facts, records
D-044, resolves TS-010, and renumbers the unchanged icon plan to Meta Increment 3. Historical reports and decisions retain their checkpoint wording.

The final rendered-link audit resolves every live Markdown link and local image
reference. `npm run verify` passes formatting, lint, strict Clippy, 17 hook
tests, 124 frontend tests, 95 Rust library tests, 21 Rust integration tests,
both Vite builds, and the Tauri release no-bundle build. Protected application,
test, config, workflow-code, branding, schema, capability, permission, manifest,
and lockfile paths are unchanged. Exact-scope, conflict, secret, generated-
output, architecture, code-health, security, and documentation reviews find no
blocking issue. No manual application check applies.

The mandatory report result is `PASS WITH ADVISORIES`. The advisory is that
historical checkpoint documents intentionally retain their original Meta 2 icon
wording and must be read with D-044; no live queue or plan reference is stale.
The `meta-02` marker is complete and valid.
