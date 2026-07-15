---
name: troubleshoot
description: Diagnose Cortexa installation, build, test, Tauri, Rust, TypeScript, or macOS runtime failures using one evidence-based hypothesis at a time.
---

# Troubleshoot

1. Capture the exact sanitized command, error, working directory, environment, and Git state.
2. Read `TROUBLESHOOTING_LOG.md` and `docs/workflows/TROUBLESHOOTING.md`.
3. Classify the failing layer and reproduce the narrowest failing command.
4. Inspect PATH and toolchain before editing source for command-not-found or metadata failures.
5. State one hypothesis and the smallest test.
6. Change one variable at a time.
7. Re-run the original failure and the nearest regression check.
8. Record symptom, cause, resolution, verification, and prevention in `TROUBLESHOOTING_LOG.md`.
9. Update setup or handoff documentation when the issue can recur.

Never disable security or quality gates merely to make the command pass.
