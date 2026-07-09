# Troubleshoot a failure

```text
Use the repository troubleshooting workflow to diagnose this exact failure:

[PASTE SANITIZED COMMAND AND ERROR]

Read AGENTS.md, HANDOFF.md, TROUBLESHOOTING_LOG.md, and docs/workflows/TROUBLESHOOTING.md. Capture the environment and Git state, classify the failing layer, and reproduce the narrowest failing command. Form one evidence-based hypothesis at a time and test the smallest correction.

Do not delete lockfiles, upgrade unrelated dependencies, disable engine-strict, weaken lint or security controls, or make broad source changes without evidence. Do not request or expose secrets.

Verify the original command and the nearest regression check. Append the symptom, root cause, resolution, verification, and prevention to TROUBLESHOOTING_LOG.md. Update setup documentation and HANDOFF.md when the failure affects future sessions.
```
