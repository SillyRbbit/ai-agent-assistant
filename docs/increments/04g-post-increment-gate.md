# Repository Workflow Increment 4G - Post-increment gate

Last updated: 2026-07-14

Status: **Verified complete**

## Goal

Create a repository-local, trusted, deterministic post-increment review system without changing Cortexa product behavior.

## Baseline

```text
branch: main
commit: a4ab51f
working tree: clean
main relative to origin/main: synchronized
repository .codex directory: absent
Codex CLI: 0.144.2
Codex hooks feature: stable, enabled
Python: 3.12.1
npm run format:check: passed
```

The approved implementation branch is `codex/post-increment-gate`. The validator was installed first and then recorded active Increment `04g` in ignored state so this workflow can bootstrap its own report.

## Implemented scope

- Added one repository-local Stop command hook with no matcher and a 30-second timeout.
- Added one Python standard-library validator with explicit exit codes and bounded begin/finalize/status/Stop modes.
- Added deterministic report-schema, exact changed-file, command-evidence, conflict, suspicious-path, safe-path, report-hash, and workspace-fingerprint validation.
- Added an ignored atomic active/completed state whose completion marker is written only after a passing report validates.
- Added a report template, report directory guidance, focused tests, and `npm run verify` integration.
- Updated the verified-increment, review, security, and end-session contracts.

## Explicit non-goals

- No application behavior or source change.
- No dependency, network, transcript, arbitrary shell, database, credential, permission, Tauri, IPC, persistence, model, gateway, policy, approval, audit, or execution change.
- No automatic commit, push, advisory fix, or roadmap reorder.

## Initial focused evidence

Passed:

```text
python3 .codex/hooks/post_increment_gate.py --help
python3 .codex/hooks/post_increment_gate.py begin --increment 04g
python3 -m json.tool .codex/hooks.json
python3 -m unittest discover -s .codex/hooks/tests -p 'test_*.py' -v
  15 passed
npm run test:hooks
  15 passed
```

Resolved failure:

- The first direct unittest discovery run failed because the dynamically loaded module was absent from `sys.modules`, which Python 3.12 dataclasses require. The test harness now registers the module before execution; the exact command passes with all 15 tests.
- Security review found that report and state reads checked only the final path component for a symlink. Resolved report-directory and state-file paths now must remain inside the Git root; focused parent-symlink escape tests pass.
- The first finalization attempt could not write ignored state under the sandbox-protected `.codex` tree. The approved exact retry completed successfully, and final status is complete and valid.

## Completion gates

- [x] Approved exact file plan preserved.
- [x] Active state bootstrapped.
- [x] Focused required validator cases pass.
- [x] Full repository verification passes.
- [x] Complete diff, code, security, scope, secret, and generated-output reviews pass.
- [x] Normal `/hooks` trust review and live Stop behavior pass.
- [x] Project memory and the passing consolidated report are synchronized.
- [x] Deterministic completion marker is valid.

## Exact next task

Workflow Increment 4G is published and merged into `main`. Wait for the project owner to select and approve one bounded next plan; do not begin another increment automatically.
