# Resume work

```text
Resume Cortexa from the repository state rather than prior chat history.

Read AGENTS.md, HANDOFF.md, PROJECT_STATUS.md, NEXT_STEPS.md, DECISIONS.md, TROUBLESHOOTING_LOG.md, SECURITY.md, and the relevant increment or plan documents. Compare the handoff with the actual Git status, diff, branch, and toolchain. Re-run the smallest check that proves the recorded baseline.

Continue only the exact next recommended task in HANDOFF.md and NEXT_STEPS.md. Preserve all security boundaries and current working behavior. Use strict TypeScript, typed Rust errors, and no unwrap or panic in production paths. Work in a small verified increment, add focused tests, and stop broad implementation if the baseline fails or required information is genuinely missing.

At completion, run the documented verification, review the diff, and update all applicable project-memory files with actual results and an exact next resume prompt.
```
