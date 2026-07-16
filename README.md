# Cortexa

<img src="assets/branding/logo-primary.png" alt="Cortexa logo" width="144" />

Phase 2: a local-first Tauri 2 desktop application for a secure personal executive assistant.

The current repository contains the smallest runnable macOS application plus a repository-based working system for continuing development safely across coding-assistant sessions.

## Current application proof

The application currently proves:

1. Vite can build the React frontend.
2. Strict TypeScript passes.
3. The Tauri WebView can invoke the typed Rust `get_app_info` command.
4. Rust startup errors propagate through typed errors.
5. Formatting, linting, unit tests, integration tests, and native builds have repeatable commands.

No API keys, shell plugin, SQLite database, Accessibility permission, screen-capture permission, or Apple Events integration are present yet.

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

- Product brief: `docs/product/PRODUCT_BRIEF.md`
- Architecture baseline: `docs/product/ARCHITECTURE_BASELINE.md`
- Brand guidelines: `docs/branding/BRAND_GUIDELINES.md`
- Brand asset usage: `docs/branding/BRAND_USAGE.md`
- Security rules: `SECURITY.md`
- Code-review guide: `CODE_REVIEW.md`

## Current security boundary

The frontend has access only to Tauri core defaults and the explicitly registered `get_app_info` command. No secret is read or stored. Privileged macOS integrations and production model networking remain deferred.
