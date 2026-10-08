---
name: session-end
description: End a Cortexa coding session by running final checks, reviewing changes, updating all repository memory, and writing an exact resume prompt.
---

# Session end

1. Follow `docs/workflows/END_SESSION.md`.
2. Stop only task-owned dev processes when no longer needed and inspect the full Git diff; preserve other active tasks.
3. Run the relevant final checks and record passed, failed, and not-run commands.
4. Review security, correctness, tests, and documentation drift.
5. Update `HANDOFF.md` and every affected project-memory file.
6. Include an exact copy-paste resume prompt.
7. Confirm no secrets, personal data, logs, or build artifacts were added.
8. Report final Git status and the next recommended task.

A session is not complete when the code changed but the handoff is stale.

## D-134 milestone operation

Follow AGENTS.md and ENGINEERING_GUIDE.md: objective, exclusions and acceptance
are the scope boundary. Path counts are informational; preserve protected paths,
attribution and unrelated work. Continue authorized routine work through review,
documentation and truthful finalization. Reuse valid unchanged-input verification
with provenance; distinguish implemented, automatically verified, live verified,
deferred and blocked criteria. Historical FAIL evidence and D-133's separate
acceptance route remain immutable. Request approval only at AGENTS.md boundaries.
