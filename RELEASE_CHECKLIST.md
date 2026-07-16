# Cortexa release checklist

Status: Authoritative release gate; production release process not yet enabled
Last updated: 2026-07-15

Use this checklist only in a separately approved release increment. A successful
development or no-bundle build is not a production release.

## Release identity

- [ ] The release owner, version, target commit, target platforms, channel, and
      date are recorded.
- [ ] The intended version follows the repository's approved versioning policy.
- [ ] `package.json`, `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, and
      user-visible release notes agree where version fields apply.
- [ ] Compatibility identifiers, bundle identifier, executable, database, and
      update channel change only under an accepted migration decision.

## Git and scope

- [ ] The release starts from clean synchronized `main`.
- [ ] `git status --short --branch` is clean.
- [ ] The target commit is reviewed, published, and reproducible from the remote.
- [ ] Every included increment has valid completion evidence.
- [ ] No untracked database, log, environment, credential, certificate, key,
      build output, or personal file is present.
- [ ] The release tag is created only after all required gates pass.

## Dependencies and supply chain

- [ ] `npm ci` succeeds from the lockfile in a clean environment.
- [ ] Rust builds use `--locked`.
- [ ] JavaScript and Rust dependency inventories and licenses are reviewed.
- [ ] `npm audit --audit-level=low` is reviewed with actual network evidence.
- [ ] A Rust advisory scan is run and every exception is tied to an accepted,
      still-valid decision with bounded exposure.
- [ ] Build scripts, native libraries, generated code, and transitive changes
      since the prior release are reviewed.
- [ ] Toolchain versions match the documented supported matrix.

## Secrets and privacy

- [ ] Tracked files and release artifacts are scanned for API keys, OAuth
      tokens, passwords, private keys, certificates, authentication codes,
      `.env` content, personal data, and local paths.
- [ ] The production OpenAI credential is absent from the desktop application.
- [ ] Logs, source maps, crash symbols, diagnostics, and audit examples contain
      no secret or unnecessary personal content.
- [ ] External processing, provider retention, telemetry, and privacy disclosures
      match accepted decisions and actual release behavior.

## Automated verification

- [ ] `npm run format:check`
- [ ] `npm run lint`
- [ ] `npm run typecheck`
- [ ] `npm run test`
- [ ] `npm run build`
- [ ] `npm run verify`
- [ ] Any release-specific, migration, update, gateway, integration, or
      adversarial suites required by the included changes.
- [ ] `git diff --check` and a complete release diff review.

Record every command, environment, duration, and actual result. A failed or
not-run required command blocks release.

## Security review

- [ ] `SECURITY_CHECKLIST.md` is complete for the release diff.
- [ ] Tauri commands, events, capabilities, CSP, plugins, entitlements, and
      operating-system permissions match the approved release design.
- [ ] Credential ownership, gateway origin, identity, provider retention,
      policy, approval, audit, and execution boundaries are verified where live.
- [ ] SQLite migrations, encryption, key storage, backup, upgrade, downgrade,
      and corruption recovery are verified where product data is persisted.
- [ ] No unresolved Critical or High finding remains.

## Signing and notarization

These controls are placeholders until a separately approved release-system
increment configures them. Their absence currently blocks production release.

- [ ] O-003 macOS minimum deployment target is resolved on supported hardware.
- [ ] Apple Developer identity ownership and access are documented.
- [ ] Signing certificates and private keys are stored outside the repository
      with least-privilege access and rotation/revocation procedures.
- [ ] Hardened Runtime, entitlements, and signing order are reviewed.
- [ ] The `.app` and installer are signed and signature verification passes.
- [ ] Notarization submission succeeds and the ticket is stapled.
- [ ] Gatekeeper assessment passes on a clean supported Mac.
- [ ] Signing and notarization logs are retained without secrets.

## Artifact and installer validation

- [ ] Release `.app` and `.dmg` are produced from the target commit.
- [ ] Artifact names, versions, hashes, sizes, architectures, and provenance are
      recorded.
- [ ] Install, first launch, normal launch, menu access, window lifecycle, and
      clean quit pass on every supported macOS version and architecture.
- [ ] Upgrade from the prior supported version preserves compatible data.
- [ ] Fresh install creates only documented files and requests no unapproved
      permission.
- [ ] Uninstall and rollback behavior are documented and tested.
- [ ] Offline startup and closed failure behavior are tested where supported.
- [ ] Icons, title, About/version surfaces, dark/light appearance, and
      accessibility smoke checks pass.

## Release notes and support

- [ ] Release notes identify user-visible changes, security impact, migrations,
      known limitations, blocked capabilities, and rollback instructions.
- [ ] No mocked, planned, or unavailable feature is described as shipping.
- [ ] Support diagnostics are bounded, redacted, and documented.
- [ ] The incident owner and private security-reporting route are confirmed.
- [ ] `CHANGELOG.md`, `PROJECT_STATUS.md`, `ROADMAP.md`, `HANDOFF.md`, and the
      release record match the published artifact.

## Publication

- [ ] Final project-owner approval is recorded after all prior checks pass.
- [ ] The reviewed commit is tagged without rewriting history.
- [ ] Artifacts and checksums are uploaded only to the approved distribution
      location.
- [ ] Published artifacts are downloaded independently and reverified.
- [ ] Repository and distribution metadata point to the same version and commit.
- [ ] Post-release launch and health checks pass.

## Rollback plan

Before publication, define:

- the previous known-good version and artifact hashes;
- conditions that trigger withdrawal or rollback;
- artifact removal and user-notification ownership;
- data and schema compatibility in both directions;
- credential, certificate, token, and integration revocation steps;
- support instructions and evidence-retention requirements.

Rollback must not use destructive Git history rewriting or silently discard user
data. If downgrade is unsafe, stop rollout and publish a forward repair plan.

## Completion record

The release record must include exact files, commands, results, manual evidence,
artifact hashes, signatures, notarization identifiers, open advisories, release
notes, rollback decision, and the post-increment gate result.
