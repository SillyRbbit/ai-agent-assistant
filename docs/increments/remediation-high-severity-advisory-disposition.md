# Remediation - High-severity advisory disposition

Status: Verified complete with advisories in the current workspace; publication
pending
Date: 2026-07-19
Owner: Project maintainer
Baseline: clean synchronized `main` at `6ce9fce`
Gate ID: `remediation-high-severity-advisory-disposition`

## Goal

Record an evidence-based disposition for every canonical High-severity finding
without feature implementation, false resolution, severity reduction, hidden
risk acceptance, or owner decisions that were not explicitly made.

## Implemented disposition

- ARB-001 remains `RESOLVED` with published Increment 4V evidence.
- ARB-002 is `DECISION REQUIRED`; O-006 and O-007 block live model networking.
- ARB-003, ARB-004, ARB-005, and ARB-008 are
  `BLOCKED - FUTURE CAPABILITY` and remain outside this remediation.
- ARB-006 and ARB-007 are `DEFERRED - NON-BLOCKING` only until their explicit
  public-distribution and release triggers; both retain High severity.
- ARB-044 remains `SUPERSEDED` by its canonical split findings.
- No `REMEDIATE NOW` or `ACCEPTED TEMPORARY RISK` item applies.

## Owner direction recorded

- Use reversible secure defaults without selecting vendors, identity
  providers, token issuers, licenses, credential owners, or release authorities.
- Keep O-006 and O-007 open and prohibit live model traffic until both close.
- Preserve proprietary/all-rights-reserved as the temporary licensing posture.
- Preserve macOS 14+ on Apple Silicon as the provisional tested baseline and
  make no Intel claim.
- Treat signing and notarization as public-release blockers, not local
  development blockers.
- Do not implement Executor, complete Workflow, durable product data, or
  enterprise controls in this remediation.

## Scope and boundaries

The exact scope is the nine modified and three created documentation paths in
`docs/plans/remediation-high-severity-advisory-disposition.md`. No product
source, test, dependency, lockfile, workflow, hook, skill, Tauri, IPC, storage,
permission, capability, CSP, credential, network, signing, release, or product
behavior changes.

## Verification

Passed:

- `npm run docs:check` after formatting-only corrections.
- `npm run repository:check`.
- `npm run security:scan`.
- `git diff --check`.
- The exact protected-path diff covering product source, tests, dependencies,
  lockfiles, workflows, hooks, and skills.
- Complete 12-path scope and diff review.
- Architecture, security, code-health, technical-debt, and readiness reviews.
- `python3 .codex/hooks/session_end_gate.py`.
- Mandatory post-increment finalization and valid completion marker.

Failed and corrected: intermediate documentation checks reported only Prettier
formatting in approved-scope files. The first marker-finalization attempt also
rejected three report categories outside the hook's closed vocabulary; mapping
them to allowed categories changed no severity, disposition, risk, or trigger.
Formatting those exact files and rerunning the documentation tier and marker
finalization passed.

Not run: frontend tests, Rust tests, application builds, native launch,
networked dependency audit, and product manual testing. They are not required
for this documentation-only change.

Manual verification pending: none.

## Risks and rollback

The principal risks are false resolution, hidden risk acceptance, premature
vendor or legal selection, and loss of explicit revisit triggers. D-059, the
canonical backlog table, protected-path validation, and complete diff review
control those risks.

Before publication, restore the nine modified files and remove the three new
records. After publication, revert only the bounded documentation commit. No
runtime, data, dependency, configuration, permission, or release rollback
applies.

## Next task

None is Ready. Obtain explicit owners and decisions for O-006 and O-007 before
any live model-networking plan. Do not start another remediation automatically.
