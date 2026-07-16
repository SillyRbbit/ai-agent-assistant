# GitHub milestone policy

Status: Authoritative mapping guidance; remote application is not verified
Last updated: 2026-07-16

GitHub milestones are reporting views over accepted work. They do not replace
`ROADMAP.md`, `NEXT_STEPS.md`, an approved plan, or completion evidence.

## Milestone model

- Product milestones map to the numbered phases in `ROADMAP.md`.
- Meta milestones map to repository, brand, documentation, release, or workflow
  capability and remain separate from product capability.
- One issue or pull request belongs to at most one delivery milestone.
- A milestone closes only after every included increment has published evidence
  or is explicitly moved with a recorded reason.
- Dates are targets, not completion claims. Do not invent delivery dates when
  the project owner has not approved them.

## Current mapping

| Milestone                         | Repository state                                             |
| --------------------------------- | ------------------------------------------------------------ |
| Phase 1 - runnable foundation     | Completed                                                    |
| Phase 2 - local application       | Completed                                                    |
| Phase 3 - bounded mock loop       | Completed                                                    |
| Phase 4 - trusted boundaries      | Completed through 4U; 4V remains separately proposed         |
| Meta 1-3 - repository foundation  | Completed through branding, operating model, and Codex gates |
| Meta 5 - repository health        | Verified complete; uncommitted and unpublished               |
| Meta 6 - application icon rollout | Ready only after Meta 5 closeout and separate owner approval |

No GitHub milestone is claimed to exist remotely until an authenticated
repository-administration check confirms it. Remote creation, due dates, branch
protection, and issue assignment are outside Meta Increment 5.
