# ARB-002A gateway threat model and closed configuration review

<!-- post-increment-gate-manifest
{
  "commands_executed": [
    "git status --short --branch",
    "git rev-parse HEAD",
    "git rev-parse origin/main",
    "git log -5 --oneline --decorate",
    "node --version",
    "npm --version",
    "rustc --version",
    "cargo --version",
    "rustfmt --version",
    "cargo clippy --version",
    "uname -m",
    "sw_vers",
    "xcode-select -p",
    "python3 .codex/hooks/post_increment_gate.py status",
    "npm run docs:check",
    "python3 .codex/hooks/post_increment_gate.py begin --increment arb-002a-gateway-threat-model-and-configuration",
    "npx prettier --write docs/plans/arb-002a-gateway-threat-model-and-configuration.md docs/security/phase4-gateway-configuration-spec.md docs/security/phase4-gateway-threat-model.md ROADMAP.md",
    "npm run repository:check",
    "npm run security:scan",
    "git diff --check",
    "test \"$(git status --porcelain | wc -l | tr -d ' ')\" = \"19\"",
    "test \"$(git status --porcelain=v1 --untracked-files=all | wc -l | tr -d ' ')\" = \"19\"",
    "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts",
    "rg -n \"D-064|Stage A|Stage B|Stage C|Stage D|gateway.access|oauth/callback|ContentLogging=false|ARB-002\" AGENTS.md ARCHITECTURE.md PRODUCT_REQUIREMENTS.md SECURITY.md SECURITY_CHECKLIST.md DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md ROADMAP.md docs/security docs/plans/arb-002a-gateway-threat-model-and-configuration.md docs/increments/arb-002a-gateway-threat-model-and-configuration.md docs/reviews/2026-07-16-advisory-remediation-backlog.md",
    "npx prettier --write docs/security/phase4-gateway-configuration-spec.md docs/security/phase4-gateway-threat-model.md",
    "npx prettier --write ROADMAP.md",
    "if rg -n -i \"oauth|openid|pkce|keychain|agentprovider|api\\.cortexaai\\.io|azure openai|authorization: bearer\" src-tauri/src src-tauri/tests; then exit 1; fi",
    "if rg -n -i \"reqwest|oauth|openid|pkce|msal|azure_identity\" package.json src-tauri/Cargo.toml; then exit 1; fi",
    "git status --short --untracked-files=all",
    "git diff --stat",
    "python3 .codex/hooks/session_end_gate.py",
    "python3 .codex/hooks/post_increment_gate.py finalize --increment arb-002a-gateway-threat-model-and-configuration --report docs/reviews/2026-07-19-arb-002a-gateway-threat-model-and-configuration-post-increment-review.md"
  ],
  "files_changed": [
    "AGENTS.md",
    "ARCHITECTURE.md",
    "CHANGELOG.md",
    "DECISIONS.md",
    "HANDOFF.md",
    "NEXT_STEPS.md",
    "PLANS.md",
    "PRODUCT_REQUIREMENTS.md",
    "PROJECT_STATUS.md",
    "ROADMAP.md",
    "SECURITY.md",
    "SECURITY_CHECKLIST.md",
    "docs/increments/arb-002a-gateway-threat-model-and-configuration.md",
    "docs/plans/README.md",
    "docs/plans/arb-002a-gateway-threat-model-and-configuration.md",
    "docs/reviews/2026-07-16-advisory-remediation-backlog.md",
    "docs/reviews/2026-07-19-arb-002a-gateway-threat-model-and-configuration-post-increment-review.md",
    "docs/security/phase4-gateway-configuration-spec.md",
    "docs/security/phase4-gateway-threat-model.md"
  ],
  "findings": [
    {
      "blocks_completion": false,
      "blocks_next_increment": true,
      "category": "Security",
      "effort": "Large",
      "milestone": "ARB-002 staged gateway remediation",
      "risk": "No registration, Azure deployment, provider-approved ZDR, transport, or operational evidence exists. Microsoft's documented default token lifetime exceeds the accepted 15-minute maximum, and the approved IP-literal ephemeral callback remains unproven.",
      "severity": "High",
      "summary": "ARB-002 remains unresolved and blocks every later gateway stage."
    }
  ],
  "increment_id": "arb-002a-gateway-threat-model-and-configuration",
  "manual_verification": [
    {
      "check": "Project owner approved the four-stage model, closed defaults, exact 19-path scope, risks, non-goals, verification, and rollback before edits.",
      "required": true,
      "status": "Passed"
    },
    {
      "check": "No product manual verification applies to this documentation-only increment.",
      "required": false,
      "status": "Passed"
    }
  ],
  "next_increment_readiness": "Blocked",
  "quality_gate": "PASS WITH ADVISORIES",
  "schema_version": 1,
  "verification": [
    {"command": "npm run docs:check", "required": true, "status": "Passed"},
    {"command": "npm run repository:check", "required": true, "status": "Passed"},
    {"command": "npm run security:scan", "required": true, "status": "Passed"},
    {"command": "git diff --check", "required": true, "status": "Passed"},
    {"command": "test \"$(git status --porcelain=v1 --untracked-files=all | wc -l | tr -d ' ')\" = \"19\"", "required": true, "status": "Passed"},
    {"command": "git diff --exit-code -- src src-tauri tests package.json package-lock.json Cargo.toml Cargo.lock .github .codex .agents scripts", "required": true, "status": "Passed"},
    {"command": "rg -n \"D-064|Stage A|Stage B|Stage C|Stage D|gateway.access|oauth/callback|ContentLogging=false|ARB-002\" AGENTS.md ARCHITECTURE.md PRODUCT_REQUIREMENTS.md SECURITY.md SECURITY_CHECKLIST.md DECISIONS.md HANDOFF.md PROJECT_STATUS.md NEXT_STEPS.md PLANS.md ROADMAP.md docs/security docs/plans/arb-002a-gateway-threat-model-and-configuration.md docs/increments/arb-002a-gateway-threat-model-and-configuration.md docs/reviews/2026-07-16-advisory-remediation-backlog.md", "required": true, "status": "Passed"},
    {"command": "if rg -n -i \"oauth|openid|pkce|keychain|agentprovider|api\\.cortexaai\\.io|azure openai|authorization: bearer\" src-tauri/src src-tauri/tests; then exit 1; fi", "required": true, "status": "Passed"},
    {"command": "if rg -n -i \"reqwest|oauth|openid|pkce|msal|azure_identity\" package.json src-tauri/Cargo.toml; then exit 1; fi", "required": true, "status": "Passed"},
    {"command": "python3 .codex/hooks/session_end_gate.py", "required": true, "status": "Passed"}
  ]
}
-->

Date: 2026-07-19
Increment: ARB-002A gateway threat model and closed configuration
Branch: `main`

## Executive summary

D-064, the Phase 4 gateway threat model, and the closed configuration
specification resolve the pre-implementation design stage without inventing
registrations, resources, or operational evidence. The exact 19-path
documentation-only scope meets its acceptance criteria. Result: `PASS WITH
ADVISORIES`.

ARB-002 remains High and unresolved. No later gateway stage or runtime
increment is Ready.

## Scope and boundaries

The approved scope is fourteen modified documentation paths and five created
documentation paths. No application source, test, dependency, lockfile,
workflow, hook, skill, Tauri, IPC, CSP, capability, permission, SQLite, or
runtime path changed.

D-064 separates Stage A design, Stage B no-traffic provisioning, Stage C
synthetic-only transport, and Stage D real-content activation. No stage grants
or automatically starts the next. D-060 through D-063 and dated historical
evidence remain authoritative and unchanged except for additive live-memory
references.

## Verification results

Passed: Markdown formatting, local links, repository policy, secret scan,
whitespace, exact 19-path scope, protected paths, source and direct-dependency
absence scans, D-064/stage consistency, official-reference review, complete
diff review, and session-end inspection.

Failed and corrected: the first documentation check found only Prettier issues
in the new plan, both security documents, and `ROADMAP.md`. After later evidence
language, the two security documents required one more formatting pass. The
first exact-scope command used default porcelain output, which collapsed the
new `docs/security/` directory and undercounted it; the corrected command uses
`--untracked-files=all`. The first final pass found one formatting-only wrap in
`ROADMAP.md`. No correction changed scope, disposition, or security policy.

Not run: frontend tests, Rust tests, application builds, native launch,
Microsoft registration, identity, Azure, DNS/TLS, managed identity, RBAC,
gateway, provider, disclosure UI, ZDR, or operational verification. They are
outside the documentation tier and remain future stage evidence. Manual
verification pending: none.

## Architecture findings

The change keeps current, planned, and prohibited behavior distinct. Detailed
authority is concentrated in one threat model and one closed configuration
specification referenced by D-064. Provider-neutral normalized desktop
contracts remain separate from Microsoft identity and Azure provider details.
No module, dependency, permission, performance, portability, or runtime
boundary changed.

## Security findings

The design is secure by default: separate registrations, fixed authority and
gateway origin, minimal scope, fail-closed token validation, dedicated managed
identity, exact-resource RBAC, private provider access, restricted egress,
content-free logs, versioned disclosure acknowledgement, and exact activation
evidence. Credentials remain absent from the desktop persistence and WebView
boundaries.

The preserved High advisory is explicit. Microsoft's documented default access
token lifetime exceeds Cortexa's accepted maximum 15 minutes, and the
manifest-based `127.0.0.1` ephemeral callback still needs operational proof.
Neither is treated as resolved, and Stage C is blocked without compatible
evidence or an additive decision.

## Code-health findings

No application code changed. The documents use one decision record plus two
focused authoritative artifacts rather than duplicating implementation
instructions across product modules. Internal links and path references pass.
No generated output, database, log, credential, certificate, key, or build
artifact is present.

## Technical debt

No new technical debt. Existing ARB-002 remains High, effort Large, and blocks
later gateway milestones but not this documentation-only Stage A completion.

## Roadmap findings

ARB-002A is complete locally and awaiting publication. Stage B, Stage C, and
Stage D are `Blocked`; each requires its own exact plan, owner approval,
evidence, verification, and rollback. ARB-003 through ARB-008 remain unchanged.

## Completion decision

`PASS WITH ADVISORIES`.

## Next-increment readiness

`Blocked`. The next possible work is a separately planned no-traffic Stage B
registration and Azure provisioning increment. It cannot begin from this
report, and Stage C remains blocked by token-lifetime, callback, deployment,
disclosure, and security evidence.

## Exact files changed

The machine manifest records all fourteen modified and five created
documentation paths. No deletion or path outside the approved scope exists.

## Exact commands executed

The machine manifest records baseline inspection, formatting corrections,
required verification, source/protected-path checks, session-end inspection,
and finalization commands with their actual outcomes.
