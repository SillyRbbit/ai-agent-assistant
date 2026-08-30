# V0 terminal-failed successor disposition recovery

Status: Tracked-evidence freeze for exceptional same-terminal-record recovery;
no normal gate
Owner: Henry Dang
Date: 2026-08-29
Baseline: `a417e5f1c1c602b917ca27c65af71480e3db6a45`
Decision: D-098

## Goal

Attach one passing, exact-target cumulative-evidence disposition to the
published D-097 `failed` / `FAIL` / `Blocked` record without changing its
historical report, result, readiness, or missing completion marker.

## Scope

Implement state schema v3, one argument-free source-allowlisted
`record-failed-disposition` command, exact lineage validation, one-target clean
successor admission, focused tests, and the exact 22-path documentation sync in
the linked ExecPlan.

## Non-goals

No normal gate; no `begin`, `finalize`, or `close-failed`; no completion marker;
no generic override or caller-selected successor; no original-report edit; no
product, dependency, workflow, Apple, Xcode, Keychain, signing, credential,
external, commit, push, merge, publication, or successor-start action.

## Exact successor

Only `personal-assistant-v0-signing-security-prerequisite-planning` may later
become technically admissible. It does not start automatically and still
requires a clean workspace plus separate owner-controlled `begin`.

## Result

At tracked-evidence freeze, the argument-free disposition command is
necessarily Not run and no completion marker, publication, or successor start
exists. The frozen recovery report records pre-transition verification. Only
the ignored schema-v3 state and redacted `post_increment_gate.py status` output
can establish the later transition result; no tracked closeout edit follows it.
