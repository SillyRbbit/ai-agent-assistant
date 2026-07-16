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

    def test_workflow_check_accepts_immutable_read_only_action(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / ".github" / "workflows" / "ci.yml"
            path.parent.mkdir(parents=True)
            path.write_text(
                "permissions:\n  contents: read\njobs:\n  check:\n    steps:\n"
                "      - uses: actions/checkout@" + ("a" * 40) + " # v6\n"
                "        with:\n          persist-credentials: false\n",
                encoding="utf-8",
            )

            self.assertEqual(health.workflow_findings(root), ())

    def test_workflow_check_rejects_mutable_action_and_write_permission(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            path = root / ".github" / "workflows" / "ci.yml"
            path.parent.mkdir(parents=True)
            path.write_text(
                "permissions:\n  contents: write\njobs:\n  check:\n    steps:\n"
                "      - uses: actions/checkout@v6\n",
                encoding="utf-8",
            )

            findings = health.workflow_findings(root)

            self.assertEqual(len(findings), 2)


if __name__ == "__main__":
    unittest.main()
