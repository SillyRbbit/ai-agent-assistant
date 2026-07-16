# Cortexa

<img src="assets/branding/logo-primary.png" alt="Cortexa logo" width="144" />

Cortexa is a local-first Tauri 2 desktop application for a secure personal
executive assistant. The repository contains a verified deterministic desktop
shell and transport-free trusted Rust boundaries; it does not yet contain a live
model, gateway transport, or tool executor.

The current repository also contains the engineering operating system used to
continue development safely across maintainers and coding-assistant sessions.

## Current application proof

The application and trusted core currently prove:

1. Vite can build the React frontend.
2. Strict TypeScript passes.
3. The WebView can invoke only the typed Rust `get_app_info` command and consume
   one closed native menu-route event.
4. Rust startup errors propagate through typed errors and SQLite startup applies
   two idempotent bootstrap migrations.
5. The Rust core can serialize and validate a bounded initial gateway request,
   normalize terminal events, validate two strict local tool schemas, derive
   deterministic policy, and bind exact approval lifecycle transitions without
   authorizing or executing a tool.
6. Formatting, linting, hook tests, frontend tests, Rust unit and integration
   tests, frontend builds, and native no-bundle builds have repeatable commands.

The React assistant interaction remains a deterministic in-memory mock. No API
key, shell plugin, live gateway or provider, dispatcher, executor, product
memory, durable audit, Accessibility permission, screen-capture permission, or
Apple Events integration is present. SQLite currently stores only bootstrap
metadata in debug startup; release startup uses in-memory storage.

## Repository creation provenance

The initial directory was generated with:

```bash
npm create tauri-app@4.6.2 -- ai-agent-assistant \
  --manager npm \
  --template react-ts \
  --identifier com.aiagentassistant.desktop \
  --tauri-version 2 \
  --yes
```

Do not run this command over an existing checkout.

## macOS prerequisites

```bash
xcode-select --install
brew install rustup
export PATH="$(brew --prefix rustup)/bin:$PATH"
rustup toolchain install 1.90.0 --profile minimal --component clippy,rustfmt
rustup default 1.90.0
```

The preferred JavaScript environment is Node.js 26.3.0 with npm 11.16.0. The repository also accepts maintained Node.js 22 and 24 environments with npm 10 or 11.

Verify:

```bash
node --version
npm --version
cargo --version
rustc --version
```

## Install and run

```bash
npm ci
npm run tauri -- dev
```

Expected result: a native window titled **Cortexa** opens and displays **Rust core connected** with application and platform information.

## Verification

```bash
npm run format:check
npm run lint
npm run typecheck
npm run test:unit
npm run test:integration
npm run build
npm run tauri -- build --no-bundle
```

Complete sequence:

```bash
npm run verify
```

## Continue development with a coding assistant

Start with `ASSISTANT_USAGE.md`.

Persistent project memory:

- `AGENTS.md` — durable assistant instructions.
- `HANDOFF.md` — current working-session state and exact resume prompt.
- `PROJECT_STATUS.md` — capability and architecture status.
- `NEXT_STEPS.md` — ordered implementation queue.
- `DECISIONS.md` — durable technical decisions.
- `CHANGELOG.md` — verified repository changes.
- `TROUBLESHOOTING_LOG.md` — known failures and resolutions.

Authoritative engineering references:

- `ENGINEERING_GUIDE.md` — development workflow and documentation authority.
- `ARCHITECTURE.md` — current, mocked, planned, and prohibited architecture.
- `PRODUCT_REQUIREMENTS.md` — normalized product requirements and MVP boundary.
- `ROADMAP.md` — product and meta milestone status.
- `TESTING_GUIDE.md` — test layers, commands, mocks, and completion evidence.
- `SECURITY_CHECKLIST.md` — security review procedure.
- `RELEASE_CHECKLIST.md` — production release gate.

Reusable workflows:

- `.agents/skills/` — repository-scoped assistant skills.
- `prompts/` — copy-paste prompts.
- `docs/workflows/` — start, resume, end, and troubleshoot runbooks.
- `docs/templates/` — handoff, increment, decision, and troubleshooting templates.

A recommended first assistant prompt is:

```text
Use $session-start for this repository. Read the required project-memory files, verify the actual Git and toolchain state, and propose the smallest ready increment before changing files.
```

## Product and security documentation

- Normalized requirements: `PRODUCT_REQUIREMENTS.md`
- Current architecture: `ARCHITECTURE.md`
- Milestone roadmap: `ROADMAP.md`
- Product brief: `docs/product/PRODUCT_BRIEF.md`
- Target architecture baseline: `docs/product/ARCHITECTURE_BASELINE.md`
- Brand guidelines: `docs/branding/BRAND_GUIDELINES.md`
- Brand asset usage: `docs/branding/BRAND_USAGE.md`
- Security rules: `SECURITY.md`
- Code-review guide: `CODE_REVIEW.md`

## Current security boundary

The frontend has access only to Tauri core defaults and the explicitly
registered `get_app_info` command. The native backend emits one closed
`assistant-menu-route` navigation event. Transport-free gateway, schema, policy,
approval, cancellation, and approval-audit primitives are not wired to the
WebView or live networking and remain non-authorizing. No secret is read or
stored. Privileged macOS integrations, production model networking, restricted
execution, and product persistence remain deferred.
