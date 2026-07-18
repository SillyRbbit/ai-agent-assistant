# Self-hosted GitHub Actions runner

Status: Retired fallback; superseded by D-057

Active CI and Documentation workflows use ephemeral GitHub-hosted
`ubuntu-latest` runners. They do not select the repository-scoped Linux x64
runner described below. The persistent runner remains registered only as a
rollback option and is not an isolated security boundary or target-macOS
evidence.

Do not restore a self-hosted `runs-on` selector without a separate approved
security and repository-governance increment. That review must account for
untrusted pull-request code, host persistence, secrets, network reachability,
branch protection, and cleanup between jobs. D-054 and the operating record
below remain historical evidence; D-057 is the current routing decision.

## Preserved fallback configuration

## Required labels

The retired runner was registered with all four labels:

```yaml
runs-on: [self-hosted, Linux, X64, cortexa-ci]
```

`self-hosted`, `Linux`, and `X64` are GitHub-managed labels. `cortexa-ci` is the
repository-specific custom label. Reapply the custom label when replacing or
reregistering the runner.

List the registered runner and its labels:

```bash
gh api repos/SillyRbbit/ai-agent-assistant/actions/runners
```

Add the custom label to a known runner ID:

```bash
gh api --method POST \
  repos/SillyRbbit/ai-agent-assistant/actions/runners/RUNNER_ID/labels \
  -f 'labels[]=cortexa-ci'
```

## Historical trust policy

The runner may execute only:

- pushes to `main`, `codex/**`, `feature/**`, `fix/**`, `refactor/**`,
  `meta/**`, or `phase*/**`;
- scheduled security checks;
- manually dispatched workflows.

The workflows do not subscribe to `pull_request`. Fork, external-contributor,
and dependency-bot pull requests therefore do not receive the persistent
runner. Their missing self-hosted checks are not approval to merge. Review the
changes, reproduce them on a maintainer-controlled allowlisted repository
branch, and run the complete checks there.

The trigger allowlist is a repository guardrail, not isolation from a trusted
writer who can change and push the workflow itself. Limit write access, require
review of `.github/workflows/`, and do not place production credentials or
unrelated sensitive material on the runner host. Do not add `pull_request` back
while this persistent runner is selected.

## Host baseline

Run the service under a dedicated unprivileged operating-system account. The
account must not have interactive `sudo`, production credentials, SSH keys,
cloud metadata access, personal files, mounted production data, or access to
other trusted services.

The host must provide:

- Git, Bash, Python 3, `curl`, and `file`;
- Rustup with Cargo available to the runner service account;
- `pkg-config` and the Tauri v2 Linux development packages; and
- outbound HTTPS access required by GitHub Actions, npm, Rustup, Cargo, and the
  explicit dependency-audit jobs.

For Debian or Ubuntu, install the Tauri prerequisites outside workflow
execution:

```bash
sudo apt update
sudo apt install \
  build-essential \
  curl \
  file \
  libayatana-appindicator3-dev \
  librsvg2-dev \
  libssl-dev \
  libwebkit2gtk-4.1-dev \
  libxdo-dev \
  pkg-config \
  wget
```

The workflows install the pinned Node.js version, the pinned Rust toolchain,
locked npm dependencies, and the pinned Cargo audit tool. They do not mutate
system packages or require `sudo`.

## Fallback verification

Before relying on the runner:

1. Confirm the runner is `online`, idle, and has all four labels.
2. Push the reviewed workflow branch only after local checks pass.
3. Confirm only the separately approved fallback jobs name the intended runner
   and complete successfully.
4. Confirm fork and dependency-bot pull requests do not receive the runner.
5. Continue to run native menus, windows, dialogs, icons, signing, notarization,
   and installer checks on a target Mac.

## Maintenance and incident response

- Keep the runner application current. GitHub can stop assigning jobs to stale
  or critically vulnerable runner versions.
- Patch the operating system and build tools on a documented schedule.
- Monitor free disk space and remove only known build caches and workspaces.
- If compromise is suspected, stop the service, remove the runner registration,
  rotate any credentials that could have been exposed, rebuild the host from a
  trusted baseline, and register a new runner.
- Prefer an ephemeral runner with a clean host lifecycle when the repository
  begins accepting untrusted pull requests.

## Return to hosted runners

Restore the previous hosted `runs-on` values, remove the self-hosted trust
conditions and preflight steps, and remove the `cortexa-ci` label:

```bash
gh api --method DELETE \
  repos/SillyRbbit/ai-agent-assistant/actions/runners/RUNNER_ID/labels/cortexa-ci
```

Run local repository checks and hosted workflow checks before treating rollback
as complete.
