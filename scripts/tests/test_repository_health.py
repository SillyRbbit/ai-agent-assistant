from __future__ import annotations

import importlib.util
import json
import sys
import tempfile
import unittest
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "repository_health.py"
    spec = importlib.util.spec_from_file_location("repository_health", module_path)
    if spec is None or spec.loader is None:
        raise RuntimeError("repository_health module could not be loaded")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


health = load_module()


def write_valid_workflows(root: Path) -> None:
    workflow_root = root / ".github" / "workflows"
    workflow_root.mkdir(parents=True)
    shared = (
        "on:\n"
        "  push:\n"
        "    branches:\n"
        "      - main\n"
        '      - "codex/**"\n'
        '      - "feature/**"\n'
        '      - "fix/**"\n'
        '      - "refactor/**"\n'
        '      - "meta/**"\n'
        '      - "phase*/**"\n'
        "    paths:\n"
        '      - "src/**"\n'
        "  workflow_dispatch:\n"
        "permissions:\n"
        "  contents: read\n"
        "concurrency:\n"
        "  group: fixture\n"
        "  cancel-in-progress: true\n"
    )
    (workflow_root / "ci.yml").write_text(
        shared
        + "  schedule:\n"
        + '    - cron: "23 13 * * 1"\n'
        + "jobs:\n"
        + "  classify:\n"
        + "    name: Classify change\n"
        + "    runs-on: [self-hosted, Linux, X64, cortexa-ci]\n"
        + "    steps:\n"
        + "      - uses: actions/checkout@"
        + ("a" * 40)
        + " # v7\n"
        + "        with:\n          persist-credentials: false\n"
        + "      - run: python3 scripts/ci_change_scope.py classify\n"
        + "      - run: python3 -m unittest discover -s scripts/tests -v\n"
        + "      - run: python3 -m unittest discover -s .codex/hooks/tests -v\n"
        + "  frontend:\n    name: Frontend validation\n    runs-on: [self-hosted, Linux, X64, cortexa-ci]\n"
        + "  rust:\n    name: Linux Rust validation\n    runs-on: [self-hosted, Linux, X64, cortexa-ci]\n"
        + "  rust-macos:\n    name: Target-Mac Rust validation\n    runs-on: [self-hosted, macOS, X64, cortexa-ci]\n"
        + "  audit:\n    name: Dependency and secret audit\n    runs-on: [self-hosted, Linux, X64, cortexa-ci]\n"
        + '  # "src-tauri/**" "assets/branding/**" "package-lock.json" ".prettierrc*" ".codex/**" ".github/workflows/ci.yml" ".github/workflows/documentation.yml"\n',
        encoding="utf-8",
    )
    (workflow_root / "documentation.yml").write_text(
        shared.replace('      - "src/**"', '      - "**/*.md"')
        + "jobs:\n"
        + "  documentation:\n"
        + "    runs-on: [self-hosted, Linux, X64, cortexa-ci]\n"
        + "    steps:\n"
        + '      - run: npm run docs:check && npm run repository:check # "prompts/**" ".agents/**" "LICENSE*"\n',
        encoding="utf-8",
    )


class RepositoryHealthTests(unittest.TestCase):
    def test_links_accept_existing_targets_and_ignore_fenced_examples(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "docs").mkdir()
            (root / "README.md").write_text(
                "[guide](docs/guide.md)\n```markdown\n[fixture](missing.md)\n```\n",
                encoding="utf-8",
            )
            (root / "docs" / "guide.md").write_text("# Guide\n", encoding="utf-8")

            findings = health.link_findings(root, ("README.md", "docs/guide.md"))

            self.assertEqual(findings, ())

    def test_links_reject_missing_and_outside_targets(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "README.md").write_text(
                "[missing](docs/missing.md)\n[outside](../outside.md)\n",
                encoding="utf-8",
            )

            findings = health.link_findings(root, ("README.md",))

            self.assertEqual(len(findings), 2)

    def test_secret_scan_reports_pattern_name_without_secret_value(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            secret = "sk-" + ("a" * 40)
            (root / "fixture.txt").write_text(secret, encoding="utf-8")

            findings = health.secret_findings(root, ("fixture.txt",))

            self.assertEqual(len(findings), 1)
            self.assertIn("openai-key", findings[0].detail)
            self.assertNotIn(secret, findings[0].render())

    def test_generated_scan_rejects_outputs_and_allows_historical_docs(self) -> None:
        paths = (
            "dist/index.html",
            "src-tauri/target/debug/app",
            "local.sqlite-wal",
            "docs/backups/checkpoint/HANDOFF.md",
        )

        findings = health.generated_path_findings(paths)

        self.assertEqual({finding.path for finding in findings}, set(paths[:3]))

    def test_license_check_accepts_explicit_decision_record(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "docs" / "github" / "LICENSING.md"
            path.parent.mkdir(parents=True)
            path.write_text("No license has been selected.\n", encoding="utf-8")

            self.assertEqual(health.license_findings(root), ())

    def test_command_check_rejects_undocumented_script_name(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            scripts = {name: "true" for name in health.REQUIRED_SCRIPTS}
            (root / "package.json").write_text(
                json.dumps({"scripts": scripts}), encoding="utf-8"
            )
            for relative_path in health.AUTHORITATIVE_COMMAND_DOCUMENTS:
                path = root / relative_path
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text("npm run not-present\n", encoding="utf-8")

            findings = health.command_findings(root)

            self.assertTrue(any("not-present" in finding.detail for finding in findings))

    def test_prompt_check_accepts_metadata_and_declared_placeholders(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "prompts" / "increments" / "fixture.md"
            path.parent.mkdir(parents=True)
            path.write_text(
                "# Fixture\n\n"
                "- **Category:** Increment\n"
                "- **Purpose:** Test.\n"
                "- **Use when:** Testing.\n"
                "- **Do not use when:** Not testing.\n"
                "- **Required inputs:** `{{VALUE}}`.\n"
                "- **Expected outputs:** Result.\n"
                "- **Related skills:** None.\n"
                "- **Related prompts:** None.\n"
                "- **Last reviewed:** 2026-07-18\n\n"
                "## Prompt\n\n```text\nUse {{VALUE}}.\n```\n",
                encoding="utf-8",
            )

            self.assertEqual(health.prompt_findings(root), ())

    def test_prompt_check_rejects_missing_metadata_and_undeclared_placeholder(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / "prompts" / "fixture.md"
            path.parent.mkdir(parents=True)
            path.write_text(
                "# Fixture\n\n- **Required inputs:** None.\n\n"
                "## Prompt\n\n```text\nUse {{VALUE}}.\n```\n",
                encoding="utf-8",
            )

            findings = health.prompt_findings(root)

            self.assertTrue(any("required metadata" in finding.detail for finding in findings))
            self.assertTrue(any("VALUE" in finding.detail for finding in findings))

    def test_prompt_check_rejects_stale_active_prompt_path(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "prompts").mkdir()
            (root / "prompts" / "README.md").write_text(
                "Use `prompts/removed.md`.\n", encoding="utf-8"
            )

            findings = health.prompt_findings(root)

            self.assertTrue(any("stale prompt path" in finding.detail for finding in findings))

    def test_workflow_check_accepts_risk_based_dual_runner_workflows(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_workflows(root)

            self.assertEqual(health.workflow_findings(root), ())

    def test_workflow_check_rejects_mutable_action_and_write_permission(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_workflows(root)
            path = root / ".github" / "workflows" / "ci.yml"
            path.write_text(
                path.read_text(encoding="utf-8")
                .replace("contents: read", "contents: write")
                .replace("actions/checkout@" + ("a" * 40), "actions/checkout@v7"),
                encoding="utf-8",
            )

            details = {finding.detail for finding in health.workflow_findings(root)}

            self.assertIn("write workflow permission is prohibited", details)
            self.assertIn("action reference is not pinned to an immutable digest", details)

    def test_workflow_check_rejects_hosted_runner_and_pull_request_trigger(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_workflows(root)
            path = root / ".github" / "workflows" / "ci.yml"
            path.write_text(
                path.read_text(encoding="utf-8")
                .replace("  push:\n", "  pull_request:\n", 1)
                .replace(
                    "runs-on: [self-hosted, Linux, X64, cortexa-ci]",
                    "runs-on: ubuntu-latest",
                    1,
                ),
                encoding="utf-8",
            )

            details = {finding.detail for finding in health.workflow_findings(root)}

            self.assertTrue(
                any("must not subscribe to pull_request" in detail for detail in details),
            )
            self.assertIn(
                "runner selector is not an approved dedicated Cortexa runner",
                details,
            )

    def test_workflow_check_rejects_unexpected_workflow(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_workflows(root)
            (root / ".github" / "workflows" / "extra.yml").write_text(
                "permissions:\n  contents: read\n", encoding="utf-8"
            )

            findings = health.workflow_findings(root)

            self.assertTrue(any("unexpected workflow" in finding.detail for finding in findings))


if __name__ == "__main__":
    unittest.main()
