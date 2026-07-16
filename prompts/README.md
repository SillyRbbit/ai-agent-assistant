# Prompt library

Use these prompts when the coding assistant does not automatically discover repository skills or when a copy-paste workflow is more convenient.

| Prompt                        | Purpose                                       |
| ----------------------------- | --------------------------------------------- |
| `start-work.md`               | Orient at the beginning of a thread           |
| `resume-work.md`              | Resume from the repository handoff            |
| `implement-next-increment.md` | Implement the first ready increment           |
| `troubleshooting.md`          | Diagnose a concrete failure                   |
| `review-change.md`            | Review a diff before acceptance               |
| `architecture-review.md`      | Review architecture boundaries and drift      |
| `security-review.md`          | Review a trust-boundary or data-flow change   |
| `technical-debt-review.md`    | Classify evidence-backed technical debt       |
| `readiness-review.md`         | Decide whether one increment is Ready         |
| `quality-gate.md`             | Run the consolidated quality assessment       |
| `post-increment-gate.md`      | Finalize evidence, memory, report, and marker |
| `executive-review.md`         | Summarize verified status for leadership      |
| `release-review.md`           | Review production release readiness           |
| `update-project-memory.md`    | Synchronize documentation only                |
| `end-of-session-handoff.md`   | Close the session cleanly                     |

Replace bracketed placeholders before use. Keep one prompt focused on one coherent task.
