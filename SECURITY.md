# Security policy and development guardrails

## Security model

AI Agent Assistant treats the model as an untrusted planner. The trusted Rust core validates requests, applies deterministic policy, obtains approval where required, executes only registered tools, and records redacted audit events.

## Non-negotiable invariants

- The WebView cannot execute a generic local action.
- Unknown tools and invalid arguments fail closed.
- Tool schemas use strict validation and reject additional properties.
- Class 3 personal-data modifications always require an exact trusted preview and explicit approval.
- Class 4 actions are not registered in the MVP.
- Class 5 behavior is prohibited.
- Approval is bound to canonical arguments, tool identity, expiry, and one-time consumption.
- Untrusted content cannot grant permission or change policy.
- Credentials, authentication codes, private keys, and production API keys are never logged or stored in SQLite.
- Privileged macOS permissions are requested only from a user-initiated feature flow.

## Phase 2 prohibited changes

Do not add:

- Accessibility APIs.
- ScreenCaptureKit.
- Apple Events.
- Microphone permissions.
- Unrestricted shell execution.
- Generic filesystem access.
- Production model API credentials.
- Automatic external communication or destructive actions.

Any request to introduce one of these requires a new architecture decision and a later-phase threat-model review.

## Dependency review

Before adding a production dependency:

1. Explain the exact capability it provides.
2. Confirm the standard library or an existing dependency cannot satisfy the need.
3. Review maintenance, licensing, native build impact, and transitive dependencies.
4. Record the decision in `DECISIONS.md`.
5. Pin the version and update lockfiles.
6. Run the full relevant verification suite.

## Data handling

- Keep local data local unless the user explicitly sends it.
- Minimize data before passing it across IPC or network boundaries.
- Redact content from logs and audit details unless the exact field is required for an approved action.
- Use opaque references rather than arbitrary paths where possible.
- Reject symlink escapes and unsupported executable content when file tooling is implemented.

## Reporting a security concern

Do not place secrets, personal files, tokens, or exploitable details in a public issue. Record a sanitized summary in the project handoff and notify the repository owner through a private channel.

A security fix is not complete until regression tests and the relevant threat-model documentation are updated.
