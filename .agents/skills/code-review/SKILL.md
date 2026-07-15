---
name: code-review
description: Review Cortexa changes for concrete correctness, security, privacy, testing, portability, and documentation defects without automatically rewriting the code.
---

# Code review

1. Read `CODE_REVIEW.md`, `SECURITY.md`, `AGENTS.md`, and the active increment acceptance criteria.
2. Inspect the full diff, related source, and tests.
3. Trace changed behavior through IPC, Rust, persistence, platform, and UI boundaries as applicable.
4. Report real findings first in severity order, with location, impact, evidence, and smallest fix.
5. Identify missing tests and verification commands.
6. Check project-memory drift.
7. State whether acceptance criteria are met.
8. Do not modify files unless the user asks for fixes after reviewing findings.
