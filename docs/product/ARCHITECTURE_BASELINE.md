# Architecture baseline

## Product shape

AI Agent Assistant is a standalone desktop executive assistant. The initial target is macOS, with portable domain logic intended for Windows, Linux, and iOS where platform capabilities permit.

## Layer model

### React presentation layer

The WebView renders conversations, navigation, settings, permission status, action previews, tool activity, and tasks. It receives typed application events and sends narrow commands. It is not trusted to authorize or execute local actions.

### Tauri IPC boundary

Only explicitly registered, typed commands cross into Rust. Capabilities are scoped per window. Generic shell, filesystem, database, tool-execution, and model-request commands are prohibited.

### Rust application core

The trusted core owns:

- Run lifecycle.
- Context provenance.
- Provider abstraction.
- Tool registry and schema validation.
- Deterministic policy.
- Approval transactions.
- Restricted dispatch.
- Persistence.
- Audit and redaction.
- Cancellation and limits.

### Provider boundary

Phase 2 uses a deterministic mock provider. A later provider will call an authenticated gateway that owns production model credentials and streams OpenAI Responses events. The gateway will not execute local tools.

### Platform adapters

Operating-system access remains behind interfaces such as application, calendar, reminders, contacts, files, clipboard, notifications, system information, secret storage, and local authentication. Phase 2 does not enable privileged adapters.

## Trust boundaries

1. Human authority.
2. Untrusted WebView and rendered content.
3. Trusted Rust core and deterministic policy.
4. OS-protected resources and local encrypted storage.
5. Product gateway.
6. Third-party model and OAuth services.

The model, gateway, WebView, and untrusted content have no direct executor path.

## Action risk classes

- **Class 0:** information only; no approval.
- **Class 1:** read-only device access within existing permission and scope.
- **Class 2:** reversible local action; direct only for narrow explicit intent, otherwise confirmation.
- **Class 3:** personal-data modification; trusted preview and approval every time.
- **Class 4:** external or high-impact action; not registered in the MVP.
- **Class 5:** prohibited autonomy; always denied.

A tool's target can escalate its effective risk. Shared calendars, attendees, executable files, unsupported URL schemes, or out-of-scope resources must be blocked.

## Data and secrets

- SQLite stores local product data through versioned migrations.
- Database encryption and Keychain-held key material are required before sensitive persistence.
- OAuth tokens and production credentials never enter SQLite.
- Raw sensitive tool output remains ephemeral where possible.
- Audit records retain normalized and redacted evidence, not duplicate personal content.

## Agent loop

1. Receive a user request.
2. Select only approved context.
3. Ask the provider for text or a typed function proposal.
4. Validate the tool and arguments.
5. Apply deterministic policy.
6. Execute an allowed read or request approval.
7. Execute the exact approved proposal.
8. Record a redacted audit event.
9. Return a verified tool result to the provider.
10. Continue until final text or a conservative limit is reached.

## Phase boundaries

- Phase 2: repository and application shell, interfaces, mocks, local UI scaffolding.
- Phase 3: complete conversation UI and mocked loop.
- Phase 4: gateway and Responses integration.
- Phase 5: policy, approval, and audit enforcement.
- Phase 6: basic macOS tools.
- Phase 7: permissions and onboarding.
- Phase 8: memory and tasks.
- Phase 9: adversarial security testing.
- Phase 10: signing, notarization, packaging, and release.

The full product brief is in `PRODUCT_BRIEF.md`.
