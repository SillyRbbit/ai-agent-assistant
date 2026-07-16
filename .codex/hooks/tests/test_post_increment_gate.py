from __future__ import annotations

import importlib.util
import io
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HOOKS_DIRECTORY = Path(__file__).resolve().parents[1]
if str(HOOKS_DIRECTORY) not in sys.path:
    sys.path.insert(0, str(HOOKS_DIRECTORY))

MODULE_PATH = HOOKS_DIRECTORY / "post_increment_gate.py"
SPEC = importlib.util.spec_from_file_location("post_increment_gate", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("post-increment gate module could not be loaded")
gate = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = gate
SPEC.loader.exec_module(gate)


class PostIncrementGateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary_directory.name).resolve()
        self._git("init", "--quiet")
        self._git("config", "user.email", "tests@example.invalid")
        self._git("config", "user.name", "Post Increment Gate Tests")
        (self.root / ".gitignore").write_text(
            ".codex/state/\n__pycache__/\n*.py[cod]\n", encoding="utf-8"
        )
        (self.root / "README.md").write_text("fixture repository\n", encoding="utf-8")
        (self.root / "tracked").mkdir()
        (self.root / "tracked/deleted.txt").write_text(
            "tracked deletion fixture\n", encoding="utf-8"
        )
        (self.root / "docs/reviews").mkdir(parents=True)
        self._git("add", ".gitignore", "README.md", "tracked/deleted.txt")
        self._git("commit", "--quiet", "-m", "Create fixture")
        gate.begin_gate(self.root, "04g")

    def tearDown(self) -> None:
        self.temporary_directory.cleanup()

    def _git(self, *arguments: str) -> None:
        result = subprocess.run(
            ["git", *arguments],
            cwd=self.root,
            check=False,
            capture_output=True,
        )
        if result.returncode != 0:
            self.fail("fixture Git command failed")

    def _stop_payload(self, *, stop_hook_active: bool = False) -> dict[str, object]:
        return {
            "cwd": str(self.root),
            "hook_event_name": "Stop",
            "stop_hook_active": stop_hook_active,
        }

    def _finding(
        self,
        *,
        severity: str = "High",
        blocks_completion: bool = True,
    ) -> dict[str, object]:
        return {
            "blocks_completion": blocks_completion,
            "blocks_next_increment": True,
            "category": "Security",
            "effort": "Small",
            "milestone": "Before the next increment",
            "risk": "The gate could accept incomplete evidence.",
            "severity": severity,
            "summary": "Fixture finding",
        }

    def _write_report(
        self,
        *,
        verification_status: str = "Passed",
        manual_status: str = "Passed",
        quality_gate: str = "PASS",
        readiness: str = "Ready",
        findings: list[dict[str, object]] | None = None,
    ) -> str:
        (self.root / "change.txt").write_text("reviewed change\n", encoding="utf-8")
        relative_report = "docs/reviews/2026-07-14-04g-post-increment-review.md"
        report_path = self.root / relative_report
        report_path.write_text("placeholder\n", encoding="utf-8")
        files_changed = list(gate.changed_paths(self.root))
        manifest = {
            "commands_executed": ["test command"],
            "files_changed": files_changed,
            "findings": findings or [],
            "increment_id": "04g",
            "manual_verification": [
                {
                    "check": "Review and trust the Stop hook",
                    "required": True,
                    "status": manual_status,
                }
            ],
            "next_increment_readiness": readiness,
            "quality_gate": quality_gate,
            "schema_version": 1,
            "verification": [
                {
                    "command": "test command",
                    "required": True,
                    "status": verification_status,
                }
            ],
        }
        report = "\n".join(
            [
                "# Post-increment review",
                "",
                gate.MANIFEST_START.rstrip("\n"),
                json.dumps(manifest, indent=2, sort_keys=True),
                gate.MANIFEST_END.lstrip("\n"),
                "",
                "## Executive summary",
                "Fixture summary.",
                "",
                "## Verification results",
                "Fixture verification.",
                "",
                "## Architecture findings",
                "None.",
                "",
                "## Security findings",
                "None.",
                "",
                "## Code-health findings",
                "None.",
                "",
                "## Technical debt",
                "None.",
                "",
                "## Roadmap findings",
                "None.",
                "",
                "## Completion decision",
                quality_gate,
                "",
                "## Next-increment readiness",
                readiness,
                "",
                "## Exact files changed",
                "See manifest.",
                "",
                "## Exact commands executed",
                "See manifest.",
                "",
            ]
        )
        report_path.write_text(report, encoding="utf-8")
        return relative_report

    def test_active_increment_without_report_requests_continuation(self) -> None:
        decision = gate.evaluate_stop_payload(self._stop_payload())

        self.assertTrue(decision.should_continue)

    def test_missing_report_prevents_completion(self) -> None:
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(
                self.root,
                "04g",
                "docs/reviews/2026-07-14-04g-post-increment-review.md",
            )

    def test_failed_required_verification_prevents_completion(self) -> None:
        report = self._write_report(
            verification_status="Failed", quality_gate="FAIL", readiness="Blocked"
        )

        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "04g", report)
        self.assertEqual(gate.read_state(self.root)["status"], "active")

    def test_pending_required_manual_check_prevents_completion(self) -> None:
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "04g", report)

    def test_high_blocking_finding_prevents_completion(self) -> None:
        report = self._write_report(
            quality_gate="FAIL",
            readiness="Blocked",
            findings=[self._finding()],
        )

        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "04g", report)

    def test_passing_report_writes_valid_completion_marker(self) -> None:
        report = self._write_report()

        gate.finalize_gate(self.root, "04g", report)

        state = gate.read_state(self.root)
        self.assertEqual(state["completion_marker"], gate.COMPLETION_MARKER)
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_passing_report_with_advisories_writes_valid_completion_marker(
        self,
    ) -> None:
        report = self._write_report(
            quality_gate="PASS WITH ADVISORIES",
            readiness="Ready with advisories",
            findings=[
                self._finding(severity="Advisory", blocks_completion=False)
            ],
        )

        gate.finalize_gate(self.root, "04g", report)

        state = gate.read_state(self.root)
        self.assertEqual(state["quality_gate"], "PASS WITH ADVISORIES")
        self.assertTrue(gate.redacted_status(self.root)["valid"])

    def test_completion_marker_remains_valid_after_commit(self) -> None:
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)

        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Complete fixture")

        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_completion_marker_remains_valid_after_commit_with_tracked_deletion(
        self,
    ) -> None:
        (self.root / "tracked/deleted.txt").unlink()
        (self.root / "tracked").rmdir()
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)

        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Complete deletion fixture")

        self.assertTrue(gate.redacted_status(self.root)["valid"])
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_completed_increment_can_be_refinalized_after_report_correction(self) -> None:
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)
        corrected_report = self._write_report(
            quality_gate="PASS WITH ADVISORIES",
            readiness="Ready with advisories",
        )

        gate.finalize_gate(self.root, "04g", corrected_report)

        state = gate.read_state(self.root)
        self.assertEqual(state["quality_gate"], "PASS WITH ADVISORIES")
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_changed_workspace_invalidates_completion_marker(self) -> None:
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)
        (self.root / "change.txt").write_text("changed after review\n", encoding="utf-8")

        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_tracked_deletion_after_finalization_invalidates_completion_marker(
        self,
    ) -> None:
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)

        (self.root / "tracked/deleted.txt").unlink()
        (self.root / "tracked").rmdir()

        self.assertFalse(gate.redacted_status(self.root)["valid"])
        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_stop_hook_active_prevents_continuation_loop(self) -> None:
        decision = gate.evaluate_stop_payload(
            self._stop_payload(stop_hook_active=True)
        )

        self.assertFalse(decision.should_continue)

    def test_suspicious_changed_path_prevents_completion(self) -> None:
        report = self._write_report()
        (self.root / ".env").write_text("SECRET=value\n", encoding="utf-8")

        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "04g", report)

    def test_unsafe_report_path_is_rejected(self) -> None:
        (self.root / "outside.md").write_text("not a report\n", encoding="utf-8")

        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "04g", "outside.md")

    def test_report_directory_symlink_escape_is_rejected(self) -> None:
        (self.root / "docs/reviews").rmdir()
        with tempfile.TemporaryDirectory() as external_directory:
            external_root = Path(external_directory)
            report_name = "2026-07-14-04g-post-increment-review.md"
            (external_root / report_name).write_text("external\n", encoding="utf-8")
            (self.root / "docs/reviews").symlink_to(
                external_root, target_is_directory=True
            )

            with self.assertRaises(gate.GateError):
                gate._safe_report_path(
                    self.root, f"docs/reviews/{report_name}", "04g"
                )

    def test_state_directory_symlink_escape_is_rejected(self) -> None:
        gate.state_path(self.root).unlink()
        (self.root / ".codex/state").rmdir()
        with tempfile.TemporaryDirectory() as external_directory:
            external_root = Path(external_directory)
            (external_root / "post_increment_gate.json").write_text(
                "{}\n", encoding="utf-8"
            )
            (self.root / ".codex/state").symlink_to(
                external_root, target_is_directory=True
            )

            with self.assertRaises(gate.GateError):
                gate.read_state(self.root)

    def test_merge_conflict_prevents_completion(self) -> None:
        report = self._write_report()

        with mock.patch.object(gate, "has_merge_conflicts", return_value=True):
            with self.assertRaises(gate.GateError):
                gate.finalize_gate(self.root, "04g", report)

    def test_malformed_hook_input_fails_closed_with_exact_prompt(self) -> None:
        output = io.StringIO()

        result = gate.run_stop_hook(io.BytesIO(b"{"), output)

        self.assertEqual(result, gate.ExitCode.OK)
        self.assertEqual(
            json.loads(output.getvalue()),
            {"decision": "block", "reason": gate.CONTINUATION_PROMPT},
        )


if __name__ == "__main__":
    unittest.main()
