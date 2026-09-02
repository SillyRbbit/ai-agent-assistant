# Personal Assistant v0 build-child-containment planning

Status: Complete — `PASS WITH ADVISORIES`
Owner: Henry Dang
Last updated: 2026-09-01
Decision: D-102

## Goal

Define the smallest fail-closed policy and review surface required before a
future disposable build can be described as containing its complete child
process graph. This increment documents gates only; it neither selects a macOS
primitive nor runs a build, a child process, or a containment probe.

## Scope

- Add the linked ExecPlan and D-102.
- Define fixed graph, executable, argument, working-directory, environment,
  descriptor, read/write, network, membership, terminal, cleanup, and evidence
  requirements for a future application-owned build attempt.
- Record one closed D-100-compatible source/contract review table.
- Preserve D-097 failed/FAIL/Blocked evidence, its historical privacy failure,
  the Pending Open Directory boundary, and every Not-run signing result.
- Reconcile the exact current documentation state.

## Explicit non-goals

No containment primitive selection or implementation; build; child, process,
network, filesystem-effect, cache, log, socket, or probe activity; Apple,
Xcode, Keychain, certificate, signing, credential, provider, product,
dependency, configuration, source, branch, commit, push, merge, release,
publication, or external-system work.

## Result

`PASS WITH ADVISORIES`. The exact documentation policy, closed review table,
and blocked dependency sequence passed the required documentation checks and
reviews. All operational P3 predicates are Not run. The policy cannot make P3
operational, P4 signer binding, signing, V0-3, or another successor Ready.
