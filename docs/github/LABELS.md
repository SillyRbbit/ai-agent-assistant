# GitHub label policy

Status: Authoritative proposed taxonomy; remote application is not verified
Last updated: 2026-07-16

Labels support triage and reporting. They do not establish increment readiness,
security approval, merge authority, or completion evidence. `NEXT_STEPS.md` and
the mandatory post-increment gate remain authoritative.

## Type labels

| Label                   | Purpose                                    |
| ----------------------- | ------------------------------------------ |
| `type: bug`             | Reproducible incorrect behavior            |
| `type: feature`         | New bounded user or engineering capability |
| `type: documentation`   | Documentation-only correction or addition  |
| `type: security-review` | Sanitized design or implementation review  |
| `type: maintenance`     | Repository, dependency, or tooling upkeep  |

## Area labels

| Label                 | Area                                       |
| --------------------- | ------------------------------------------ |
| `area: frontend`      | React presentation and volatile state      |
| `area: rust-core`     | Trusted Rust domain boundaries             |
| `area: storage`       | SQLite bootstrap and future repositories   |
| `area: native-macos`  | Menu, window, dialog, icon, or OS behavior |
| `area: security`      | Security and privacy controls              |
| `area: documentation` | Product and engineering documentation      |
| `area: repository`    | GitHub, hooks, scripts, and workflows      |

## Status and priority labels

| Label                  | Meaning                                                         |
| ---------------------- | --------------------------------------------------------------- |
| `status: needs-triage` | Not yet classified or accepted                                  |
| `status: ready`        | Has a bounded accepted plan; repository Ready rules still apply |
| `status: blocked`      | Named prerequisite prevents progress                            |
| `priority: critical`   | Immediate security, data, or release blocker                    |
| `priority: high`       | Significant bounded risk or user impact                         |
| `priority: normal`     | Normal queue priority                                           |
| `priority: low`        | Non-urgent maintenance or advisory                              |

Use one type label, at least one area label, at most one status label, and at
most one priority label. Do not encode secrets, people, customers, or incident
details in labels.

The repository-local files do not create remote labels. A separately authorized
GitHub administration step must reconcile this taxonomy without deleting labels
used by open work.
