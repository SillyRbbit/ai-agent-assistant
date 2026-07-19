# Self-hosted GitHub Actions runners

Status: Active under D-058

Active CI and Documentation workflows use two repository-scoped persistent
runners. Runner 21 is Linux x64 and owns classification, documentation,
frontend, Linux Rust, and dependency audits. Runner 22 is macOS x64 and adds
target-Mac Rust validation whenever the classifier selects Rust.

Neither runner is an isolated security boundary. The workflows therefore do
not subscribe to `pull_request` or `pull_request_target`. D-058 is the active
routing decision; D-054 and D-057 remain historical and rollback evidence.

## Active configuration

## Required labels

The Linux runner requires:

```yaml
runs-on: [self-hosted, Linux, X64, cortexa-ci]
```

The target-Mac runner requires:

```yaml
runs-on: [self-hosted, macOS, X64, cortexa-ci]
```

`self-hosted`, the operating-system label, and `X64` are GitHub-managed labels.
`cortexa-ci` is repository-specific. Reapply it when replacing or
reregistering either runner.

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

As verified through the GitHub API on 2026-07-18, runner 21
`henry-dang-HP-Elite-Slice` and runner 22 `Henrys-MacBook-Pro` were online,
idle, and carried their exact selectors. Remote status is transient and must be
checked again before relying on a run.

## Verified PR #30 execution

D-058 implementation commit `9a2c75d` produced two successful push-triggered
workflows on 2026-07-18:

- CI run `29670565671`: classification job `88148646821`, frontend job
  `88148677830`, Linux Rust job `88148677826`, and dependency-audit job
  `88148677832` ran on Linux runner 21 `henry-dang-HP-Elite-Slice` with labels
  `[self-hosted, Linux, X64, cortexa-ci]`.
- Documentation run `29670565657`: documentation job `88148646740` ran on
  Linux runner 21 with the same exact selector.
- CI job `88148677829`: target-Mac Rust ran on macOS runner 22
  `Henrys-MacBook-Pro` with labels
  `[self-hosted, macOS, X64, cortexa-ci]`.

The GitHub run listing for `9a2c75d` contains only those two `push` events. No
`pull_request` execution is claimed or authorized. This proves the reviewed
D-058 assignment for that commit, not future runner health or release
readiness.

## Trust policy

The runners may execute only:

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
unrelated sensitive material on either runner host. Do not add `pull_request`
while persistent runners are selected.

## Host baseline

Run the service under a dedicated unprivileged operating-system account. The
account must not have interactive `sudo`, production credentials, SSH keys,
cloud metadata access, personal files, mounted production data, or access to
other trusted services.

Both hosts must provide Git, Bash, Python 3, outbound HTTPS, and the GitHub
runner service. The Linux host must also provide:

- `curl` and `file`;
- Rustup with Cargo available to the runner service account;
- `pkg-config` and the Tauri v2 Linux development packages; and
- access required by npm, Rustup, Cargo, and the explicit dependency-audit
  jobs.

The macOS host must provide Xcode Command Line Tools and Rustup with Cargo
available to the runner service account. Native UI, signing, notarization, and
installer checks remain manual or release-specific; the target-Mac workflow
does not claim them.

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

The workflows install the pinned Node.js version where needed, the pinned Rust
toolchain, locked npm dependencies, and the pinned Cargo audit tool. They do not
mutate system packages or require `sudo`.

## Active verification

For each later workflow or runner change:

1. Confirm both runners are `online`, idle, and have their exact labels.
2. Push the reviewed workflow branch only after local checks pass.
3. Confirm Linux jobs name runner 21 and target-Mac Rust names runner 22.
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

After Actions minute or billing availability is restored, a separately
reviewed decision may restore hosted `runs-on` values and pull-request triggers.
Only after active workflows no longer select the persistent hosts, remove the
`cortexa-ci` label from each runner:

```bash
gh api --method DELETE \
  repos/SillyRbbit/ai-agent-assistant/actions/runners/RUNNER_ID/labels/cortexa-ci
```

Run local repository checks and actual hosted workflow checks before treating
the transition as complete.
