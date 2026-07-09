---
name: resume-session
description: Resume AI Agent Assistant work from HANDOFF.md, reconcile it with the actual repository, and continue only the recorded next task.
---

# Resume session

1. Read `HANDOFF.md`, then the full instruction and project-memory chain in `AGENTS.md`.
2. Compare the handoff with Git status, diff, branch, recent commits, and toolchain output.
3. Re-run the smallest command that proves the recorded baseline.
4. Identify stale documentation or uncommitted work before changing files.
5. Restate the exact next task and acceptance criteria.
6. Continue only that coherent task unless a blocker requires troubleshooting.
7. Update repository memory before ending.

Use `docs/workflows/RESUME_SESSION.md` for conflict handling and exit criteria.
