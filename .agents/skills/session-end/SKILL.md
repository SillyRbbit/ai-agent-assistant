---
name: session-end
description: End a Cortexa coding session by running final checks, reviewing changes, updating all repository memory, and writing an exact resume prompt.
---

# Session end

1. Follow `docs/workflows/END_SESSION.md`.
2. Stop active dev processes and inspect the full Git diff.
3. Run the relevant final checks and record passed, failed, and not-run commands.
4. Review security, correctness, tests, and documentation drift.
5. Update `HANDOFF.md` and every affected project-memory file.
6. Include an exact copy-paste resume prompt.
7. Confirm no secrets, personal data, logs, or build artifacts were added.
8. Report final Git status and the next recommended task.

A session is not complete when the code changed but the handoff is stale.
