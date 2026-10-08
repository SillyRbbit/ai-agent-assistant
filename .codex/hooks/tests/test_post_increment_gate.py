from __future__ import annotations

import importlib.util
import hashlib
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

    def test_acceptance_schema_requires_exact_lineage(self) -> None:
        state = gate.read_state(self.root)
        state["schema_version"] = 4
        with self.assertRaises(gate.GateError):
            gate.validate_state(state)
        state["acceptance_lineage"] = {"maintenance": "0" * 64, "admission": "1" * 64, "force": True}
        with self.assertRaises(gate.GateError):
            gate.validate_state(state)

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
        blocks_next_increment: bool = True,
    ) -> dict[str, object]:
        return {
            "blocks_completion": blocks_completion,
            "blocks_next_increment": blocks_next_increment,
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
                "## Scope and boundaries",
                "Fixture scope.",
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

    def _write_named_report(
        self,
        *,
        increment_id: str,
        relative_report: str,
        verification_status: str = "Passed",
        manual_status: str = "Passed",
        quality_gate: str = "PASS",
        readiness: str = "Ready",
        findings: list[dict[str, object]] | None = None,
    ) -> str:
        report_path = self.root / relative_report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text("placeholder\n", encoding="utf-8")
        manifest = {
            "commands_executed": ["test command"],
            "files_changed": list(gate.changed_paths(self.root)),
            "findings": findings or [],
            "increment_id": increment_id,
            "manual_verification": [
                {
                    "check": "Review the exact bounded recovery",
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
                "## Scope and boundaries",
                "Fixture scope.",
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

    @staticmethod
    def _finding_sha256(finding: dict[str, object]) -> str:
        payload = json.dumps(
            finding, ensure_ascii=False, separators=(",", ":"), sort_keys=True
        ).encode("utf-8")
        return hashlib.sha256(payload).hexdigest()

    def _failed_disposition_blockers(self) -> list[dict[str, object]]:
        summaries = (
            "Post-commit readiness disposition is not yet implemented.",
            "Historical screenshot handling remains failed evidence.",
            "Historical Open Directory acceptance remains pending.",
        )
        blockers: list[dict[str, object]] = []
        for summary in summaries:
            finding = self._finding(
                severity="Medium",
                blocks_completion=False,
                blocks_next_increment=True,
            )
            finding["summary"] = summary
            blockers.append(finding)
        return blockers

    def _prepare_failed_disposition_fixture(
        self,
        *,
        blockers: list[dict[str, object]] | None = None,
        recovery_verification: str = "Passed",
        recovery_manual: str = "Passed",
        recovery_quality: str = "PASS WITH ADVISORIES",
        recovery_readiness: str = "Ready with advisories",
        recovery_findings: list[dict[str, object]] | None = None,
    ) -> tuple[str, frozenset[str]]:
        failed_increment = gate.FAILED_DISPOSITION_INCREMENT_ID
        active_state = dict(gate.read_state(self.root))
        active_state["increment_id"] = failed_increment
        gate.write_state(self.root, active_state)

        predecessor_blockers = blockers or self._failed_disposition_blockers()
        (self.root / "change.txt").write_text(
            "reviewed failed recovery\n", encoding="utf-8"
        )
        predecessor_report = self._write_named_report(
            increment_id=failed_increment,
            relative_report=gate.FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH,
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
            findings=predecessor_blockers,
        )
        gate.close_failed_gate(self.root, failed_increment, predecessor_report)

        predecessor_state = dict(gate.read_state(self.root))
        predecessor_state["schema_version"] = gate.LEGACY_FAILED_STATE_SCHEMA_VERSION
        predecessor_state.pop("successor_disposition", None)
        predecessor_state.pop("predecessor_disposition", None)
        gate.write_state(self.root, predecessor_state)
        self.failed_disposition_predecessor_report_sha256 = hashlib.sha256(
            (self.root / predecessor_report).read_bytes()
        ).hexdigest()
        self.failed_disposition_predecessor_state_sha256 = hashlib.sha256(
            gate.state_path(self.root).read_bytes()
        ).hexdigest()
        self.failed_disposition_predecessor_head_commit = predecessor_state[
            "head_commit"
        ]
        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Publish failed fixture")
        base_commit = (
            subprocess.run(
                ["git", "rev-parse", "HEAD"],
                cwd=self.root,
                check=True,
                capture_output=True,
                text=True,
            )
            .stdout.strip()
        )

        for relative_path in gate.FAILED_DISPOSITION_RECOVERY_ALLOWED_PATHS:
            if relative_path == gate.FAILED_DISPOSITION_RECOVERY_REPORT_PATH:
                continue
            path = self.root / relative_path
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("bounded governance recovery\n", encoding="utf-8")
        self._write_named_report(
            increment_id=gate.FAILED_DISPOSITION_RECOVERY_ID,
            relative_report=gate.FAILED_DISPOSITION_RECOVERY_REPORT_PATH,
            verification_status=recovery_verification,
            manual_status=recovery_manual,
            quality_gate=recovery_quality,
            readiness=recovery_readiness,
            findings=recovery_findings,
        )
        blocker_hashes = frozenset(
            self._finding_sha256(finding) for finding in predecessor_blockers
        )
        fixture_constants = {
            "FAILED_DISPOSITION_BASE_COMMIT": base_commit,
            "FAILED_DISPOSITION_PREDECESSOR_REPORT_SHA256": self.failed_disposition_predecessor_report_sha256,
            "FAILED_DISPOSITION_PREDECESSOR_STATE_SHA256": self.failed_disposition_predecessor_state_sha256,
            "FAILED_DISPOSITION_PREDECESSOR_HEAD_COMMIT": self.failed_disposition_predecessor_head_commit,
            "FAILED_DISPOSITION_BLOCKING_FINDING_SHA256": blocker_hashes,
        }
        for name, value in fixture_constants.items():
            patcher = mock.patch.object(gate, name, value)
            patcher.start()
            self.addCleanup(patcher.stop)
        return base_commit, blocker_hashes

    def _record_failed_disposition(
        self, base_commit: str, blocker_hashes: frozenset[str]
    ) -> None:
        self.assertEqual(gate.FAILED_DISPOSITION_BASE_COMMIT, base_commit)
        if gate.FAILED_DISPOSITION_BLOCKING_FINDING_SHA256 == blocker_hashes:
            gate.record_failed_disposition(self.root)
            return
        with mock.patch.object(
            gate, "FAILED_DISPOSITION_BLOCKING_FINDING_SHA256", blocker_hashes
        ):
            gate.record_failed_disposition(self.root)

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

    def test_failed_report_writes_valid_terminal_record_without_completion(
        self,
    ) -> None:
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        gate.close_failed_gate(self.root, "04g", report)

        state = gate.read_state(self.root)
        self.assertEqual(state["status"], "failed")
        self.assertEqual(state["quality_gate"], "FAIL")
        self.assertEqual(state["next_increment_readiness"], "Blocked")
        self.assertEqual(state["schema_version"], gate.STATE_SCHEMA_VERSION)
        self.assertNotIn("completion_marker", state)
        self.assertTrue(gate.validate_failed_state(self.root, state))
        self.assertEqual(
            gate.redacted_status(self.root),
            {
                "increment_id": "04g",
                "next_increment_readiness": "Blocked",
                "quality_gate": "FAIL",
                "report_path": report,
                "status": "failed",
                "valid": True,
            },
        )
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_failed_verification_can_close_only_as_terminal_failure(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        gate.close_failed_gate(self.root, "04g", report)

        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))

    def test_high_blocking_finding_can_close_only_as_terminal_failure(self) -> None:
        report = self._write_report(
            quality_gate="FAIL",
            readiness="Blocked",
            findings=[self._finding()],
        )

        gate.close_failed_gate(self.root, "04g", report)

        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))

    def test_passing_report_cannot_record_terminal_failure(self) -> None:
        report = self._write_report()

        with self.assertRaisesRegex(
            gate.GateError, "terminal failure requires a FAIL report"
        ):
            gate.close_failed_gate(self.root, "04g", report)

        self.assertEqual(gate.read_state(self.root)["status"], "active")

    def test_terminal_failure_cannot_be_promoted_to_completion(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)

        with self.assertRaisesRegex(
            gate.GateError, "terminally failed increment cannot be completed"
        ):
            gate.finalize_gate(self.root, "04g", report)

        state = gate.read_state(self.root)
        self.assertEqual(state["status"], "failed")
        self.assertNotIn("completion_marker", state)

    def test_terminal_failure_report_tamper_reactivates_stop_block(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        report_path = self.root / report
        report_path.write_text(
            report_path.read_text(encoding="utf-8") + "tampered\n",
            encoding="utf-8",
        )

        state = gate.read_state(self.root)
        self.assertFalse(gate.validate_failed_state(self.root, state))
        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_terminal_failure_workspace_drift_reactivates_stop_block(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        (self.root / "change.txt").write_text("changed after failure\n", encoding="utf-8")

        state = gate.read_state(self.root)
        self.assertFalse(gate.validate_failed_state(self.root, state))
        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_blocked_terminal_failure_rejects_successor_increment(self) -> None:
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)

        with self.assertRaisesRegex(
            gate.GateError, "terminal failed gate evidence blocks the next increment"
        ):
            gate.begin_gate(self.root, "05g")

        self.assertEqual(gate.read_state(self.root)["increment_id"], "04g")

    def test_nonblocked_terminal_failure_allows_distinct_successor(self) -> None:
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Ready with advisories",
        )
        gate.close_failed_gate(self.root, "04g", report)

        with self.assertRaisesRegex(
            gate.GateError, "terminal failed increment must have a clean workspace"
        ):
            gate.begin_gate(self.root, "05g")

        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record failed fixture")
        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))

        gate.begin_gate(self.root, "05g")

        state = gate.read_state(self.root)
        self.assertEqual(state["increment_id"], "05g")
        self.assertEqual(state["status"], "active")

    def test_terminal_failure_state_rejects_completion_marker_field(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        state = dict(gate.read_state(self.root))
        state["completion_marker"] = gate.COMPLETION_MARKER

        with self.assertRaises(gate.GateError):
            gate.validate_state(state)

    def test_terminal_failure_state_requires_exact_bounded_evidence_fields(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        valid_state = dict(gate.read_state(self.root))

        mutations = {
            "missing readiness": lambda value: value.pop(
                "next_increment_readiness"
            ),
            "passing quality": lambda value: value.update(quality_gate="PASS"),
            "invalid readiness": lambda value: value.update(
                next_increment_readiness="Unknown"
            ),
            "invalid report hash": lambda value: value.update(report_sha256="0"),
            "invalid workspace hash": lambda value: value.update(
                workspace_fingerprint="0"
            ),
            "invalid head": lambda value: value.update(head_commit="0"),
        }
        for label, mutate in mutations.items():
            with self.subTest(label=label):
                state = dict(valid_state)
                mutate(state)
                with self.assertRaises(gate.GateError):
                    gate.validate_state(state)

    def test_blocking_next_finding_requires_blocked_readiness(self) -> None:
        report = self._write_report(
            quality_gate="FAIL",
            readiness="Ready with advisories",
            findings=[self._finding()],
        )

        with self.assertRaisesRegex(
            gate.GateError, "readiness contradicts a blocking finding"
        ):
            gate.close_failed_gate(self.root, "04g", report)

    def test_terminal_failure_can_be_reclosed_before_head_changes(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        original_state = gate.read_state(self.root)
        original_hash = original_state["report_sha256"]
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        gate.close_failed_gate(self.root, "04g", report)

        reclosed_state = gate.read_state(self.root)
        self.assertEqual(
            reclosed_state["baseline_fingerprint"],
            original_state["baseline_fingerprint"],
        )
        self.assertEqual(reclosed_state["report_path"], original_state["report_path"])
        self.assertNotEqual(reclosed_state["report_sha256"], original_hash)
        self.assertTrue(gate.validate_failed_state(self.root, reclosed_state))

    def test_reclosed_nonblocked_failure_allows_successor_after_commit(self) -> None:
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        report = self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Ready with advisories",
        )

        gate.close_failed_gate(self.root, "04g", report)
        reclosed_state = gate.read_state(self.root)
        self.assertEqual(
            reclosed_state["next_increment_readiness"], "Ready with advisories"
        )
        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record reclosed failed fixture")
        self.assertTrue(gate.validate_failed_state(self.root, reclosed_state))

        gate.begin_gate(self.root, "05g")

        successor_state = gate.read_state(self.root)
        self.assertEqual(successor_state["increment_id"], "05g")
        self.assertEqual(successor_state["status"], "active")

    def test_terminal_failure_reclose_rejects_another_report_path(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)

        with self.assertRaisesRegex(gate.GateError, "same report path"):
            gate.close_failed_gate(
                self.root,
                "04g",
                "docs/reviews/2026-07-15-04g-post-increment-review.md",
            )

    def test_terminal_failure_reclose_is_rejected_after_commit(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)
        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record failed fixture")
        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))
        self._write_report(
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        with self.assertRaisesRegex(gate.GateError, "after HEAD changes"):
            gate.close_failed_gate(self.root, "04g", report)

    def test_close_failed_rejects_merge_conflict_and_suspicious_path(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        with mock.patch.object(gate, "has_merge_conflicts", return_value=True):
            with self.assertRaisesRegex(gate.GateError, "merge conflicts"):
                gate.close_failed_gate(self.root, "04g", report)

        (self.root / ".env").write_text("SECRET=value\n", encoding="utf-8")
        with self.assertRaisesRegex(gate.GateError, "suspicious"):
            gate.close_failed_gate(self.root, "04g", report)

    def test_completed_increment_cannot_be_changed_to_failed(self) -> None:
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)

        with self.assertRaisesRegex(gate.GateError, "cannot be changed to failed"):
            gate.close_failed_gate(self.root, "04g", report)

    def test_same_terminally_failed_increment_cannot_restart(self) -> None:
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )
        gate.close_failed_gate(self.root, "04g", report)

        with self.assertRaisesRegex(gate.GateError, "terminal failure record"):
            gate.begin_gate(self.root, "04g")

    def test_legacy_active_state_can_close_to_current_failed_schema(self) -> None:
        legacy_state = dict(gate.read_state(self.root))
        legacy_state["schema_version"] = gate.LEGACY_STATE_SCHEMA_VERSION
        gate.write_state(self.root, legacy_state)
        report = self._write_report(
            verification_status="Failed",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        gate.close_failed_gate(self.root, "04g", report)

        self.assertEqual(
            gate.read_state(self.root)["schema_version"], gate.STATE_SCHEMA_VERSION
        )

    def test_legacy_completed_state_remains_valid(self) -> None:
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)
        legacy_state = dict(gate.read_state(self.root))
        legacy_state["schema_version"] = gate.LEGACY_STATE_SCHEMA_VERSION
        gate.write_state(self.root, legacy_state)

        self.assertTrue(gate.validate_completed_state(self.root, legacy_state))

    def test_report_requires_template_scope_section(self) -> None:
        report = self._write_report()
        report_path = self.root / report
        report_path.write_text(
            report_path.read_text(encoding="utf-8").replace(
                "## Scope and boundaries\nFixture scope.\n\n", ""
            ),
            encoding="utf-8",
        )

        with self.assertRaisesRegex(
            gate.GateError, "exactly one ## Scope and boundaries section"
        ):
            gate.finalize_gate(self.root, "04g", report)

    def test_report_rejects_duplicate_template_scope_section(self) -> None:
        report = self._write_report()
        report_path = self.root / report
        report_path.write_text(
            report_path.read_text(encoding="utf-8")
            + "## Scope and boundaries\nDuplicate scope.\n",
            encoding="utf-8",
        )

        with self.assertRaisesRegex(
            gate.GateError, "exactly one ## Scope and boundaries section"
        ):
            gate.finalize_gate(self.root, "04g", report)

    def test_report_section_name_in_prose_is_not_a_duplicate_heading(self) -> None:
        report = self._write_report()
        report_path = self.root / report
        report_path.write_text(
            report_path.read_text(encoding="utf-8")
            + "The `## Scope and boundaries` contract is enforced.\n",
            encoding="utf-8",
        )

        gate.finalize_gate(self.root, "04g", report)

        self.assertTrue(gate.validate_completed_state(self.root, gate.read_state(self.root)))

    def test_report_finding_categories_remain_closed(self) -> None:
        finding = self._finding()
        finding["category"] = "Governance tooling"
        report = self._write_report(
            quality_gate="FAIL", readiness="Blocked", findings=[finding]
        )

        with self.assertRaisesRegex(gate.GateError, "invalid category"):
            gate.close_failed_gate(self.root, "04g", report)

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
                self._finding(
                    severity="Advisory",
                    blocks_completion=False,
                    blocks_next_increment=False,
                )
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

    def test_failed_disposition_contract_is_exact_and_argument_free(self) -> None:
        self.assertEqual(
            gate.FAILED_DISPOSITION_INCREMENT_ID,
            "v0-xcode-developer-id-recovery-execution",
        )
        self.assertEqual(
            gate.FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH,
            "docs/reviews/2026-08-28-v0-xcode-developer-id-recovery-execution-post-increment-review.md",
        )
        self.assertEqual(
            gate.FAILED_DISPOSITION_BASE_COMMIT,
            "a417e5f1c1c602b917ca27c65af71480e3db6a45",
        )
        self.assertEqual(
            gate.FAILED_DISPOSITION_RECOVERY_ID,
            "v0-terminal-failed-successor-disposition-recovery",
        )
        self.assertEqual(
            gate.FAILED_DISPOSITION_RECOVERY_REPORT_PATH,
            "docs/reviews/2026-08-29-v0-terminal-failed-successor-disposition-recovery-post-increment-review.md",
        )
        self.assertEqual(
            gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
            "personal-assistant-v0-signing-security-prerequisite-planning",
        )
        parsed = gate.build_parser().parse_args(["record-failed-disposition"])
        self.assertEqual(parsed.command, "record-failed-disposition")
        with (
            mock.patch("sys.stderr", new=io.StringIO()),
            self.assertRaises(SystemExit),
        ):
            gate.build_parser().parse_args(
                ["record-failed-disposition", "--increment", "caller-selected"]
            )

    def test_record_failed_disposition_preserves_failure_and_writes_exact_v3(
        self,
    ) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        predecessor_state = dict(gate.read_state(self.root))

        self._record_failed_disposition(base_commit, blocker_hashes)

        state = gate.read_state(self.root)
        self.assertEqual(
            set(state),
            {
                "baseline_fingerprint",
                "head_commit",
                "increment_id",
                "next_increment_readiness",
                "quality_gate",
                "report_path",
                "report_sha256",
                "schema_version",
                "status",
                "successor_disposition",
                "workspace_fingerprint",
            },
        )
        for key, value in predecessor_state.items():
            if key == "schema_version":
                continue
            self.assertEqual(state[key], value)
        self.assertEqual(state["schema_version"], gate.STATE_SCHEMA_VERSION)
        self.assertEqual(state["status"], "failed")
        self.assertEqual(state["quality_gate"], "FAIL")
        self.assertEqual(state["next_increment_readiness"], "Blocked")
        self.assertNotIn("completion_marker", state)
        disposition = state["successor_disposition"]
        self.assertEqual(
            set(disposition),
            {
                "allowed_paths",
                "baseline_commit",
                "disposition_id",
                "next_increment_readiness",
                "predecessor_state_sha256",
                "quality_gate",
                "report_path",
                "report_sha256",
                "successor_increment_id",
                "workspace_fingerprint",
            },
        )
        self.assertEqual(disposition["baseline_commit"], base_commit)
        self.assertEqual(
            disposition["disposition_id"], gate.FAILED_DISPOSITION_RECOVERY_ID
        )
        self.assertEqual(disposition["quality_gate"], "PASS WITH ADVISORIES")
        self.assertEqual(
            disposition["next_increment_readiness"], "Ready with advisories"
        )
        self.assertEqual(
            disposition["successor_increment_id"],
            gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
        )
        self.assertEqual(
            disposition["allowed_paths"],
            list(gate.FAILED_DISPOSITION_SUCCESSOR_ALLOWED_PATHS),
        )
        self.assertTrue(gate.validate_failed_state(self.root, state))

    def test_v3_failed_disposition_requires_exact_state_and_nested_keys(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)
        valid_state = dict(gate.read_state(self.root))

        state = dict(valid_state)
        state["unexpected"] = True
        with self.assertRaises(gate.GateError):
            gate.validate_state(state)

        for missing_key in valid_state["successor_disposition"]:
            with self.subTest(missing_key=missing_key):
                state = dict(valid_state)
                disposition = dict(valid_state["successor_disposition"])
                disposition.pop(missing_key)
                state["successor_disposition"] = disposition
                with self.assertRaises(gate.GateError):
                    gate.validate_state(state)

        preserved_field_mutations: dict[str, object] = {
            "baseline_fingerprint": "f" * 64,
            "head_commit": "f" * 40,
            "increment_id": "different-increment",
            "next_increment_readiness": "Ready",
            "quality_gate": "PASS",
            "report_path": gate.FAILED_DISPOSITION_RECOVERY_REPORT_PATH,
            "report_sha256": "f" * 64,
            "schema_version": gate.LEGACY_FAILED_STATE_SCHEMA_VERSION,
            "status": "complete",
            "workspace_fingerprint": "f" * 64,
        }
        for preserved_key, mutation in preserved_field_mutations.items():
            with self.subTest(preserved_key=preserved_key):
                state = dict(valid_state)
                state[preserved_key] = mutation
                self.assertFalse(gate.validate_failed_state(self.root, state))

        mutations = {
            "alternate disposition": lambda value: value.update(
                disposition_id="alternate-recovery"
            ),
            "alternate report": lambda value: value.update(
                report_path="docs/reviews/2026-08-29-alternate-post-increment-review.md"
            ),
            "alternate base": lambda value: value.update(baseline_commit="0" * 40),
            "alternate successor": lambda value: value.update(
                successor_increment_id="alternate-successor"
            ),
            "failed recovery": lambda value: value.update(quality_gate="FAIL"),
            "blocked recovery": lambda value: value.update(
                next_increment_readiness="Blocked"
            ),
            "widened paths": lambda value: value.update(
                allowed_paths=[*value["allowed_paths"], "src/unauthorized.ts"]
            ),
        }
        for label, mutate in mutations.items():
            with self.subTest(label=label):
                state = dict(valid_state)
                disposition = dict(valid_state["successor_disposition"])
                mutate(disposition)
                state["successor_disposition"] = disposition
                with self.assertRaises(gate.GateError):
                    gate.validate_state(state)

    def test_legacy_v1_and_v2_states_remain_readable_after_v3(self) -> None:
        active = dict(gate.read_state(self.root))
        active["schema_version"] = gate.LEGACY_STATE_SCHEMA_VERSION
        gate.validate_state(active)

        base_commit, _ = self._prepare_failed_disposition_fixture()
        failed = gate.read_state(self.root)
        self.assertEqual(
            failed["schema_version"], gate.LEGACY_FAILED_STATE_SCHEMA_VERSION
        )
        gate.validate_state(failed)
        self.assertTrue(base_commit)

    def test_recovery_report_failed_verification_is_rejected(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture(
            recovery_verification="Failed",
            recovery_quality="FAIL",
            recovery_readiness="Blocked",
        )
        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)
        self.assertEqual(gate.read_state(self.root)["schema_version"], 2)

    def test_recovery_report_pending_manual_evidence_is_rejected(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture(
            recovery_manual="Manual verification pending",
            recovery_quality="FAIL",
            recovery_readiness="Blocked",
        )
        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)

    def test_recovery_report_blocked_readiness_is_rejected(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture(
            recovery_quality="PASS WITH ADVISORIES",
            recovery_readiness="Blocked",
        )
        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)

    def test_recovery_report_next_blocking_finding_is_rejected(self) -> None:
        finding = self._finding(
            severity="Medium", blocks_completion=False, blocks_next_increment=True
        )
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture(
            recovery_quality="PASS WITH ADVISORIES",
            recovery_readiness="Blocked",
            recovery_findings=[finding],
        )
        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)

    def test_predecessor_requires_exact_three_blocker_digests(self) -> None:
        expected = self._failed_disposition_blockers()
        expected_hashes = frozenset(self._finding_sha256(item) for item in expected)
        base_commit, _ = self._prepare_failed_disposition_fixture(
            blockers=expected[:2]
        )

        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, expected_hashes)

    def test_predecessor_changed_or_extra_blocker_digest_is_rejected(self) -> None:
        expected = self._failed_disposition_blockers()
        expected_hashes = frozenset(self._finding_sha256(item) for item in expected)
        changed = [dict(item) for item in expected]
        changed[0]["summary"] = "Changed blocker identity."
        changed.append(
            self._finding(
                severity="Medium",
                blocks_completion=False,
                blocks_next_increment=True,
            )
        )
        base_commit, _ = self._prepare_failed_disposition_fixture(blockers=changed)

        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, expected_hashes)

    def test_failed_disposition_rejects_original_or_recovery_report_tamper(
        self,
    ) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        predecessor = self.root / gate.FAILED_DISPOSITION_PREDECESSOR_REPORT_PATH
        predecessor.write_text(
            predecessor.read_text(encoding="utf-8") + "tampered\n", encoding="utf-8"
        )

        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)

    def test_failed_disposition_rejects_conflict_suspicious_and_inventory_drift(
        self,
    ) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        (self.root / ".env").write_text("SECRET=value\n", encoding="utf-8")

        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)
        with mock.patch.object(gate, "has_merge_conflicts", return_value=True):
            with self.assertRaises(gate.GateError):
                self._record_failed_disposition(base_commit, blocker_hashes)
        (self.root / ".env").unlink()
        (self.root / "unexpected-recovery-file.md").write_text(
            "inventory drift\n", encoding="utf-8"
        )
        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)

    def test_failed_disposition_is_one_shot_and_identical_retry_is_idempotent(
        self,
    ) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)
        first_state = gate.read_state(self.root)

        self._record_failed_disposition(base_commit, blocker_hashes)

        self.assertEqual(gate.read_state(self.root), first_state)
        report = self.root / gate.FAILED_DISPOSITION_RECOVERY_REPORT_PATH
        report.write_text(
            report.read_text(encoding="utf-8") + "changed retry\n", encoding="utf-8"
        )
        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)
        self.assertEqual(gate.read_state(self.root), first_state)

    def test_failed_disposition_atomic_write_failure_preserves_v2_state(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        original_state = gate.read_state(self.root)

        with mock.patch.object(
            gate, "write_state", side_effect=gate.GateError("fixture write failure")
        ):
            with self.assertRaisesRegex(gate.GateError, "fixture write failure"):
                self._record_failed_disposition(base_commit, blocker_hashes)

        self.assertEqual(gate.read_state(self.root), original_state)

    def test_failed_disposition_stop_and_status_fail_closed_on_drift(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)

        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)
        status = gate.redacted_status(self.root)
        self.assertEqual(status["status"], "failed")
        self.assertEqual(status["quality_gate"], "FAIL")
        self.assertEqual(status["next_increment_readiness"], "Blocked")
        self.assertTrue(status["valid"])
        redacted_disposition = status["successor_disposition"]
        self.assertEqual(
            redacted_disposition["successor_increment_id"],
            gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
        )
        self.assertNotIn("report_sha256", redacted_disposition)
        self.assertNotIn("workspace_fingerprint", redacted_disposition)
        self.assertNotIn("predecessor_state_sha256", redacted_disposition)

        changed_path = self.root / gate.FAILED_DISPOSITION_RECOVERY_REPORT_PATH
        changed_path.write_text(
            changed_path.read_text(encoding="utf-8") + "drift\n", encoding="utf-8"
        )
        self.assertFalse(gate.redacted_status(self.root)["valid"])
        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_only_clean_exact_successor_can_consume_failed_disposition(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)

        with mock.patch.object(gate, "FAILED_DISPOSITION_BASE_COMMIT", base_commit):
            with self.assertRaises(gate.GateError):
                gate.begin_gate(
                    self.root, gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
                )
            with self.assertRaises(gate.GateError):
                gate.begin_gate(self.root, "unrelated-increment")
            with self.assertRaises(gate.GateError):
                gate.begin_gate(self.root, gate.FAILED_DISPOSITION_INCREMENT_ID)

        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record exact disposition")
        with mock.patch.object(gate, "FAILED_DISPOSITION_BASE_COMMIT", base_commit):
            gate.begin_gate(
                self.root, gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
            )

        state = gate.read_state(self.root)
        self.assertEqual(state["status"], "active")
        self.assertEqual(
            state["increment_id"], gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
        )
        self.assertEqual(
            state["predecessor_disposition"]["allowed_paths"],
            list(gate.FAILED_DISPOSITION_SUCCESSOR_ALLOWED_PATHS),
        )

    def test_admitted_successor_enforces_nested_exact_path_allowlist(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)
        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record exact disposition")
        with mock.patch.object(gate, "FAILED_DISPOSITION_BASE_COMMIT", base_commit):
            gate.begin_gate(
                self.root, gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
            )
        (self.root / "outside-allowlist.md").write_text(
            "unapproved scope\n", encoding="utf-8"
        )

        with self.assertRaises(gate.GateError):
            gate.finalize_gate(
                self.root,
                gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
                "docs/reviews/2026-08-29-personal-assistant-v0-signing-security-prerequisite-planning-post-increment-review.md",
            )

    def test_disposition_rejects_wrong_baseline_and_nonancestor_predecessor(
        self,
    ) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        with mock.patch.object(
            gate, "FAILED_DISPOSITION_BASE_COMMIT", "f" * 40
        ):
            with self.assertRaises(gate.GateError):
                gate.record_failed_disposition(self.root)
        with mock.patch.object(gate, "_is_ancestor", return_value=False):
            with self.assertRaises(gate.GateError):
                self._record_failed_disposition(base_commit, blocker_hashes)

    def test_disposition_rejects_raw_predecessor_state_digest_drift(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        state = dict(gate.read_state(self.root))
        state["baseline_fingerprint"] = "f" * 64
        gate.write_state(self.root, state)

        with self.assertRaises(gate.GateError):
            self._record_failed_disposition(base_commit, blocker_hashes)

    def test_exact_successor_completion_preserves_disposition_lineage(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)
        disposition = dict(gate.read_state(self.root)["successor_disposition"])
        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record exact disposition")
        gate.begin_gate(
            self.root, gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
        )
        (self.root / "ARCHITECTURE.md").write_text(
            "bounded successor planning\n", encoding="utf-8"
        )
        report = self._write_named_report(
            increment_id=gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
            relative_report=(
                "docs/reviews/2026-08-29-personal-assistant-v0-signing-security-"
                "prerequisite-planning-post-increment-review.md"
            ),
        )

        gate.finalize_gate(
            self.root,
            gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
            report,
        )

        state = gate.read_state(self.root)
        self.assertEqual(state["status"], "complete")
        self.assertEqual(state["predecessor_disposition"], disposition)
        self.assertTrue(gate.validate_completed_state(self.root, state))

    def test_exact_successor_failure_preserves_disposition_lineage(self) -> None:
        base_commit, blocker_hashes = self._prepare_failed_disposition_fixture()
        self._record_failed_disposition(base_commit, blocker_hashes)
        disposition = dict(gate.read_state(self.root)["successor_disposition"])
        self._git("add", "--all")
        self._git("commit", "--quiet", "-m", "Record exact disposition")
        gate.begin_gate(
            self.root, gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID
        )
        (self.root / "SECURITY.md").write_text(
            "bounded successor failure\n", encoding="utf-8"
        )
        report = self._write_named_report(
            increment_id=gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
            relative_report=(
                "docs/reviews/2026-08-29-personal-assistant-v0-signing-security-"
                "prerequisite-planning-post-increment-review.md"
            ),
            manual_status="Manual verification pending",
            quality_gate="FAIL",
            readiness="Blocked",
        )

        gate.close_failed_gate(
            self.root,
            gate.FAILED_DISPOSITION_SUCCESSOR_INCREMENT_ID,
            report,
        )

        state = gate.read_state(self.root)
        self.assertEqual(state["status"], "failed")
        self.assertEqual(state["predecessor_disposition"], disposition)
        self.assertTrue(gate.validate_failed_state(self.root, state))

    def test_malformed_hook_input_fails_closed_with_exact_prompt(self) -> None:
        output = io.StringIO()

        result = gate.run_stop_hook(io.BytesIO(b"{"), output)

        self.assertEqual(result, gate.ExitCode.OK)
        self.assertEqual(
            json.loads(output.getvalue()),
            {"decision": "block", "reason": gate.CONTINUATION_PROMPT},
        )


class ClosureWithoutCompletionTests(unittest.TestCase):
    setUp = PostIncrementGateTests.setUp
    tearDown = PostIncrementGateTests.tearDown
    _git = PostIncrementGateTests._git
    _stop_payload = PostIncrementGateTests._stop_payload
    _write_report = PostIncrementGateTests._write_report

    def _closure_review(self, increment="closure-maintenance", *, passing=True):
        closure = gate._closure()
        report = f"docs/reviews/2026-10-01-{increment}-post-increment-review.md"
        path = self.root / report
        path.write_text("pending\n")
        m = {
            "schema_version": 1, "increment_id": increment,
            "quality_gate": "PASS" if passing else "FAIL", "next_increment_readiness": "Ready",
            "commands_executed": sorted(closure.REQUIRED_COMMANDS),
            "files_changed": list(gate.changed_paths(self.root)), "findings": [],
            "verification": [{"command": c, "required": True,
                              "status": "Passed" if passing else "Failed"}
                             for c in sorted(closure.REQUIRED_COMMANDS)],
            "manual_verification": [{"check": c, "required": True, "status": "Passed"}
                                    for c in sorted(closure.REQUIRED_REVIEWS)],
        }
        path.write_text(gate.MANIFEST_START + json.dumps(m) + gate.MANIFEST_END + "\n" +
                        "\n".join(section + "\nFixture evidence.\n" for section in gate.REQUIRED_REPORT_SECTIONS))
        return report

    def _closure_fixture(self):
        import shutil
        closure = gate._closure()
        (self.root / "DECISIONS.md").write_text("Owner approves fixture closure and separately ready fixture work.\n")
        failed_report = self._write_report(verification_status="Failed", quality_gate="FAIL", readiness="Blocked")
        gate.close_failed_gate(self.root, "04g", failed_report)
        raw = gate.state_path(self.root).read_bytes()
        before = closure.manifest(self.root)
        backup = self.root / ".codex/state/backup"
        for name, item in before.items():
            if item["exists"]:
                p = backup / "before" / name
                p.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(self.root / name, p)
        for name in (".git/index", str(gate.STATE_RELATIVE_PATH)):
            p = backup / "before" / name
            p.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(closure.git_index_path(self.root) if name == ".git/index" else self.root / name, p)
        (backup / "workspace-before.json").write_bytes(closure.encoded(before))
        (self.root / "maintenance.md").write_text("Maintenance implementation fixture.\n")
        review = self._closure_review()
        request = {
            "schema_version": 1,
            "owner_authorization": {"approved": True, "source": "Explicit fixture owner instruction",
                                    "decision_path": "DECISIONS.md",
                                    "decision_sha256": closure.digest((self.root / "DECISIONS.md").read_bytes())},
            "failed_state_sha256": closure.digest(raw), "backup_directory": str(backup),
            "manifest_sha256": closure.digest((backup / "workspace-before.json").read_bytes()),
            "allowed_paths": ["maintenance.md", review],
            "disposition": {"retained": ["existing work"], "deferred": ["unfinished acceptance"],
                            "restored": [], "unresolved": closure.unresolved((self.root / failed_report).read_text())},
            "verification": {"increment_id": "closure-maintenance", "report_path": review},
        }
        path = ".codex/state/closure-request.json"
        (self.root / path).write_bytes(closure.encoded(request))
        return closure, request, path, raw

    def _closure_admission(self, closure, request, *, relationship="independent", increment="independent-task"):
        readiness = self._closure_review(increment + "-readiness")
        body = {
            "schema_version": 1, "owner_authorization": request["owner_authorization"],
            "increment_id": increment, "workspace_fingerprint": gate.workspace_fingerprint(self.root),
            "allowed_paths": ["next.txt", f"docs/reviews/2026-10-01-{increment}-post-increment-review.md"],
            "dependencies": {request["failed_state_sha256"] + "/" + issue:
                             {"relationship": relationship, "rationale": "Fixture task requires no failed capability."}
                             for issue in request["disposition"]["unresolved"]},
            "readiness_report": readiness, "readiness_increment": increment + "-readiness",
        }
        path = ".codex/state/admission-request.json"
        (self.root / path).write_bytes(closure.encoded(body))
        return body, path

    def _milestone_admission(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request)
        body.pop("allowed_paths")
        body["schema_version"] = 2
        body["milestone"] = {
            "objective": "Implement the fixture documentation milestone",
            "exclusions": ["Product behavior and external actions"],
            "acceptance": {"documentation": "Document the fixture with verified evidence"},
            "protected_paths": ["src", "DECISIONS.md"],
            "authorized_destructive_paths": [],
        }
        (self.root / admission).write_bytes(closure.encoded(body))
        gate.begin_gate(self.root, "independent-task", admission=admission)
        return closure, request, raw

    def _milestone_review(self, *, status="automatically_verified", quality="PASS"):
        closure = gate._closure()
        report = self._closure_review("independent-task", passing=quality != "FAIL")
        path = self.root / report
        text = path.read_text()
        m = gate._extract_manifest(text)
        state = gate.read_state(self.root)
        admission = closure.read_json(self.root / closure.DIRECTORY /
                                     ("admission-" + state["closure_lineage"]["admission"] + ".json"))
        delta = closure.changed(admission["baseline_manifest"], closure.manifest(self.root))
        m["milestone"] = {
            "criteria": {"documentation": {"status": status, "evidence": "Fixture checks observed passing."}},
            "paths": {p: {"criterion": "documentation", "rationale": "Supports the fixture checklist.",
                          "within_objective": True, "preserves_existing": True} for p in delta},
        }
        self._replace_milestone_report(path, text, m)
        return report

    def _replace_milestone_report(self, path, text, m):
        start = text.index(gate.MANIFEST_START) + len(gate.MANIFEST_START)
        end = text.index(gate.MANIFEST_END, start)
        path.write_text(text[:start] + json.dumps(m) + text[end:])

    def _assert_absent_path_checkpoint(self, version):
        deleted = self.root / "tracked/deleted.txt"
        deleted.unlink()  # Inherited physical deletion, before closure/admission.
        if version == 2:
            closure, request, raw = self._milestone_admission()
            report = self._milestone_review()
        else:
            closure, request, path, raw = self._closure_fixture()
            closure.close(self.root, path)
            _, path = self._closure_admission(closure, request)
            gate.begin_gate(self.root, "independent-task", admission=path)
            report = self._closure_review("independent-task")
        gate.finalize_gate(self.root, "independent-task", report)
        state = gate.read_state(self.root)
        admission = self.root / closure.DIRECTORY / (
            "admission-" + state["closure_lineage"]["admission"] + ".json")
        frozen = admission.read_bytes()
        self.assertEqual(json.loads(frozen)["baseline_manifest"]["tracked/deleted.txt"],
                         {"exists": False})
        before = gate.workspace_fingerprint(self.root)
        self.assertTrue(gate.validate_completed_state(self.root, state))
        self._git("add", "--all")
        self.assertNotIn("tracked/deleted.txt", closure.manifest(self.root))
        self.assertEqual(gate.workspace_fingerprint(self.root), before)
        self.assertTrue(gate.validate_completed_state(self.root, state))
        self._git("commit", "--quiet", "-m", "Checkpoint inherited absence")
        self.assertTrue(gate.validate_completed_state(self.root, state))
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)
        self.assertEqual(admission.read_bytes(), frozen)
        self.assertEqual(closure.historical(self.root, request["failed_state_sha256"])["raw_state"].encode(), raw)
        # Canonical comparison must not relax the byte-bound evidence check.
        admission.write_bytes(frozen + b"\n")
        self.assertFalse(gate.validate_completed_state(self.root, state))
        admission.write_bytes(frozen)
        historical = self.root / json.loads(raw)["report_path"]
        original = historical.read_bytes()
        historical.write_bytes(original + b"\n")
        self.assertFalse(gate.validate_completed_state(self.root, state))
        historical.write_bytes(original)
        # Reappearance is a real content change, not index bookkeeping.
        deleted.write_text("Unapproved recreation.\n")
        self.assertFalse(gate.validate_completed_state(self.root, state))
        if version == 1:
            with self.assertRaisesRegex(gate.GateError, "authorized scope"):
                closure.validate_lineage(self.root, state)
        else:
            review = self._milestone_review()
            text = (self.root / review).read_text()
            m = gate._extract_manifest(text)
            m["milestone"]["paths"]["tracked/deleted.txt"]["within_objective"] = False
            self._replace_milestone_report(self.root / review, text, m)
            with self.assertRaisesRegex(gate.GateError, "unrelated scope"):
                gate.finalize_gate(self.root, "independent-task", review)

    def test_absent_path_legacy_staging_commit_and_evidence_binding(self):
        self._assert_absent_path_checkpoint(1)

    def test_absent_path_milestone_staging_commit_and_evidence_binding(self):
        self._assert_absent_path_checkpoint(2)

    def test_absent_path_comparison_is_symmetric_and_keeps_real_changes(self):
        closure = gate._closure()
        absent = {"deleted": {"exists": False}}
        frozen = json.dumps(absent)
        self.assertEqual(closure.changed(absent, {}), set())
        self.assertEqual(closure.changed({}, absent), set())
        self.assertEqual(json.dumps(absent), frozen)
        for value in ({"exists": True, "mode": 420, "sha256": "a" * 64},
                      {"exists": False, "unexpected": True}, None):
            with self.subTest(value=value):
                self.assertEqual(closure.changed(absent, {"deleted": value}), {"deleted"})
                self.assertEqual(closure.changed({"deleted": value}, absent), {"deleted"})

    def test_absent_path_fix_rejects_new_deletion_before_and_after_staging(self):
        closure, _, _ = self._milestone_admission()
        state = gate.read_state(self.root)
        (self.root / "README.md").unlink()
        for stage in (False, True):
            if stage:
                self._git("add", "--", "README.md")
            with self.subTest(staged=stage), self.assertRaisesRegex(gate.GateError, "destructive path"):
                closure.validate_lineage(self.root, state)

    def test_milestone_without_failure_history_preserves_contract_on_next_begin(self):
        closure = gate._closure()
        report = self._write_report()
        gate.finalize_gate(self.root, "04g", report)
        (self.root / "DECISIONS.md").write_text("Explicit owner approval for fixture milestone.\n")
        ready = self._closure_review("independent-task-readiness")
        request = {
            "schema_version": 2,
            "owner_authorization": {"approved": True, "source": "Fixture owner instruction",
                                    "decision_path": "DECISIONS.md",
                                    "decision_sha256": closure.digest((self.root / "DECISIONS.md").read_bytes())},
            "increment_id": "independent-task", "workspace_fingerprint": gate.workspace_fingerprint(self.root),
            "dependencies": {}, "readiness_report": ready, "readiness_increment": "independent-task-readiness",
            "milestone": {"objective": "Fixture documentation", "exclusions": ["Product changes"],
                          "acceptance": {"documentation": "Document verified behavior"},
                          "protected_paths": ["src"], "authorized_destructive_paths": []},
        }
        path = ".codex/state/new-milestone.json"
        (self.root / path).write_bytes(closure.encoded(request))
        gate.begin_gate(self.root, "independent-task", admission=path)
        final = self._milestone_review()
        gate.finalize_gate(self.root, "independent-task", final)
        self.assertTrue(gate.validate_completed_state(self.root, gate.read_state(self.root)))
        with self.assertRaisesRegex(gate.GateError, "admission required"):
            gate.begin_gate(self.root, "unapproved-successor")

    def test_milestone_subtree_and_mode_changes_are_protected(self):
        self._milestone_admission()
        (self.root / "src").mkdir(exist_ok=True)
        path = self.root / "src/unrelated.txt"; path.write_text("Unrelated implementation.\n")
        report = self._milestone_review()
        with self.assertRaisesRegex(gate.GateError, "protected path"):
            gate.finalize_gate(self.root, "independent-task", report)
        path.unlink()
        (self.root / "maintenance.md").chmod(0o755)
        report = self._milestone_review()
        with self.assertRaisesRegex(gate.GateError, "destructive path"):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_milestone_relevant_growth_exceeds_old_ceilings_and_completes(self):
        closure, request, raw = self._milestone_admission()
        for i in range(48):
            (self.root / f"relevant-{i}.md").write_text("Relevant documentation fixture.\n")
        report = self._milestone_review()
        gate.finalize_gate(self.root, "independent-task", report)
        self.assertTrue(gate.validate_completed_state(self.root, gate.read_state(self.root)))
        self.assertEqual(closure.historical(self.root, request["failed_state_sha256"])["raw_state"].encode(), raw)
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_milestone_rejects_unrelated_scope_or_missing_attribution(self):
        self._milestone_admission()
        (self.root / "new.md").write_text("Fixture content.\n")
        report = self._milestone_review()
        p = self.root / report; original = p.read_text()
        for field, value in (("within_objective", False), ("preserves_existing", False),
                             ("criterion", "unapproved-objective"), ("rationale", "")):
            with self.subTest(field=field):
                m = gate._extract_manifest(original)
                m["milestone"]["paths"]["new.md"][field] = value
                self._replace_milestone_report(p, original, m)
                with self.assertRaises(gate.GateError):
                    gate.finalize_gate(self.root, "independent-task", report)
        m = gate._extract_manifest(original); del m["milestone"]["paths"]["new.md"]
        self._replace_milestone_report(p, original, m)
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_milestone_protected_paths_and_unauthorized_deletion_are_blocked(self):
        closure, request, raw = self._milestone_admission()
        protected = self.root / "DECISIONS.md"; before = protected.read_bytes()
        protected.write_bytes(before + b"Unauthorized scope expansion\n")
        report = self._milestone_review()
        with self.assertRaisesRegex(gate.GateError, "protected path"):
            gate.finalize_gate(self.root, "independent-task", report)
        protected.write_bytes(before)
        (self.root / "maintenance.md").unlink()
        report = self._milestone_review()
        with self.assertRaisesRegex(gate.GateError, "destructive path"):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_milestone_checklist_and_false_completion_are_blocked(self):
        self._milestone_admission()
        for status in ("implemented", "deferred", "blocked"):
            with self.subTest(status=status):
                report = self._milestone_review(status=status)
                with self.assertRaises(gate.GateError):
                    gate.finalize_gate(self.root, "independent-task", report)
        report = self._milestone_review(); p = self.root / report; original = p.read_text()
        m = gate._extract_manifest(original); m["milestone"]["criteria"] = {}
        self._replace_milestone_report(p, original, m)
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)
        m = gate._extract_manifest(original); del m["milestone"]
        self._replace_milestone_report(p, original, m)
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_milestone_failure_is_truthful_and_retains_history(self):
        closure, request, raw = self._milestone_admission()
        report = self._milestone_review(status="blocked", quality="FAIL")
        gate.close_failed_gate(self.root, "independent-task", report)
        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))
        self.assertNotIn("completion_marker", gate.read_state(self.root))
        self.assertEqual(closure.historical(self.root, request["failed_state_sha256"])["raw_state"].encode(), raw)

    def test_milestone_checklist_only_failure_is_retained_for_successor_assessment(self):
        closure, request, raw = self._milestone_admission()
        report = self._milestone_review(status="deferred")
        p = self.root / report; text = p.read_text(); m = gate._extract_manifest(text)
        m["quality_gate"] = "FAIL"
        self._replace_milestone_report(p, text, m)
        gate.close_failed_gate(self.root, "independent-task", report)
        issues = closure.unresolved(p.read_text())
        self.assertEqual(len(issues), 1)
        self.assertTrue(issues[0].startswith("criterion:"))
        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))
        # No ordinary independent begin can clear the new failure; its own closure
        # must now carry the checklist criterion into the existing dependency map.
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, "another-task")
        self.assertEqual(closure.historical(self.root, request["failed_state_sha256"])["raw_state"].encode(), raw)

    def test_milestone_tampered_admission_or_history_cannot_complete(self):
        closure, request, raw = self._milestone_admission()
        report = self._milestone_review()
        state = gate.read_state(self.root)
        p = self.root / closure.DIRECTORY / ("admission-" + state["closure_lineage"]["admission"] + ".json")
        original = p.read_bytes(); p.write_bytes(original + b" ")
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)
        p.write_bytes(original)
        failed = json.loads(raw)["report_path"]
        (self.root / failed).write_text("Tampered historical failure")
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_milestone_rejects_dependent_or_unassessed_successor(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request)
        body.pop("allowed_paths"); body["schema_version"] = 2
        body["milestone"] = {"objective": "Fixture milestone", "exclusions": ["External effects"],
                             "acceptance": {"check": "Required result"}, "protected_paths": [],
                             "authorized_destructive_paths": []}
        for assessment in ({}, {k: {"relationship": "dependent", "rationale": "Requires failed capability"}
                                for k in body["dependencies"]}):
            body["dependencies"] = assessment
            (self.root / admission).write_bytes(closure.encoded(body))
            with self.assertRaises(gate.GateError):
                gate.begin_gate(self.root, "independent-task", admission=admission)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_closure_keeps_failure_raw_and_creates_no_completion_marker(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        status = gate.redacted_status(self.root)
        self.assertEqual(status["status"], "failed")
        self.assertEqual(status["quality_gate"], "FAIL")
        self.assertEqual(status["next_increment_readiness"], "Blocked")
        self.assertTrue(status["valid"])
        self.assertTrue(status["workspace_valid"])
        self.assertNotIn("completion_marker", gate.read_state(self.root))
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_closure_requires_recorded_owner_approval_and_decision_binding(self):
        closure, request, path, raw = self._closure_fixture()
        for value in (False, "true", 1):
            request["owner_authorization"]["approved"] = value
            (self.root / path).write_bytes(closure.encoded(request))
            with self.assertRaises(gate.GateError):
                closure.close(self.root, path)
        request["owner_authorization"]["approved"] = True
        request["owner_authorization"]["decision_sha256"] = "0" * 64
        (self.root / path).write_bytes(closure.encoded(request))
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_closure_rejects_missing_and_tampered_backup_payloads(self):
        closure, request, path, raw = self._closure_fixture()
        p = Path(request["backup_directory"]) / "before/README.md"
        original = p.read_bytes()
        for content in (None, b"tamper"):
            if content is None:
                p.unlink()
            else:
                p.write_bytes(content)
            with self.assertRaises(gate.GateError):
                closure.close(self.root, path)
        p.write_bytes(original)
        m = Path(request["backup_directory"]) / "workspace-before.json"
        m.write_bytes(m.read_bytes() + b"\n")
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_closure_rejects_state_report_head_and_index_drift(self):
        closure, request, path, raw = self._closure_fixture()
        state = gate.read_state(self.root)
        for name in (str(gate.STATE_RELATIVE_PATH), state["report_path"], ".git/index"):
            p = self.root / name
            original = p.read_bytes()
            p.write_bytes(original + b"\n")
            with self.assertRaises(gate.GateError):
                closure.close(self.root, path)
            p.write_bytes(original)
        with mock.patch.object(gate, "current_head_commit", return_value="0" * 40):
            with self.assertRaises(gate.GateError):
                closure.close(self.root, path)

    def test_closure_rejects_workspace_scope_escape_and_incomplete_criteria(self):
        closure, request, path, raw = self._closure_fixture()
        p = self.root / "README.md"
        original = p.read_bytes()
        p.write_bytes(b"unrelated modification")
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)
        p.write_bytes(original)
        request["disposition"]["unresolved"] = ["invented replacement"]
        (self.root / path).write_bytes(closure.encoded(request))
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)

    def test_closure_requires_passing_full_verification_and_reviews(self):
        closure, request, path, raw = self._closure_fixture()
        self._closure_review(passing=False)
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)
        report = self._closure_review()
        p = self.root / report
        original = p.read_text()
        m = gate._extract_manifest(original)
        m["verification"] = [e for e in m["verification"] if e["command"] != "npm run verify"]
        p.write_text(original.replace(json.dumps(gate._extract_manifest(original)), json.dumps(m)))
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)

    def test_closure_interruption_replay_and_changed_replay_are_safe(self):
        closure, request, path, raw = self._closure_fixture()
        with mock.patch.object(closure.os, "link", side_effect=OSError("interrupted")):
            with self.assertRaises(gate.GateError):
                closure.close(self.root, path)
        self.assertFalse(closure.receipt_path(self.root, request["failed_state_sha256"]).exists())
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        closure.close(self.root, path)
        p = closure.receipt_path(self.root, request["failed_state_sha256"])
        receipt = p.read_bytes()
        closure.close(self.root, path)
        self.assertEqual(p.read_bytes(), receipt)
        request["disposition"]["retained"] = ["changed replay"]
        (self.root / path).write_bytes(closure.encoded(request))
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)

    def test_closure_history_integrity_is_separate_from_workspace_readiness(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        (self.root / "later.md").write_text("Later work needs admission.\n")
        status = gate.redacted_status(self.root)
        self.assertTrue(status["valid"])
        self.assertFalse(status["workspace_valid"])
        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, "next-task")
        p = closure.receipt_path(self.root, request["failed_state_sha256"])
        p.write_bytes(p.read_bytes().replace(b'"approved": true', b'"approved": false'))
        self.assertFalse(gate.redacted_status(self.root)["valid"])

    def test_closure_dependent_or_unapproved_successors_stay_blocked(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request, relationship="dependent")
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, "independent-task", admission=admission)
        body, admission = self._closure_admission(closure, request)
        body["owner_authorization"]["approved"] = False
        (self.root / admission).write_bytes(closure.encoded(body))
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, "independent-task", admission=admission)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_closure_independent_authorized_successor_uses_normal_completion(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request)
        gate.begin_gate(self.root, "independent-task", admission=admission)
        self.assertEqual(gate.read_state(self.root)["status"], "active")
        (self.root / "next.txt").write_text("bounded implementation\n")
        report = self._closure_review("independent-task")
        gate.finalize_gate(self.root, "independent-task", report)
        self.assertTrue(gate.redacted_status(self.root)["valid"])
        record = closure.historical(self.root, request["failed_state_sha256"])
        self.assertEqual(record["raw_state"].encode(), raw)
        self.assertEqual(json.loads(record["raw_state"])["quality_gate"], "FAIL")
        # Future legitimate code changes cannot erase immutable failure evidence.
        (self.root / "next.txt").write_text("changed after completion\n")
        self.assertEqual(closure.historical(self.root, request["failed_state_sha256"])["raw_state"].encode(), raw)
        self.assertFalse(gate.redacted_status(self.root)["valid"])

    def test_closure_admission_drift_and_scope_escape_fail_closed(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request)
        (self.root / "drift.md").write_text("drift\n")
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, "independent-task", admission=admission)
        (self.root / "drift.md").unlink()
        gate.begin_gate(self.root, "independent-task", admission=admission)
        (self.root / "README.md").write_text("outside scope\n")
        report = self._closure_review("independent-task")
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_closure_rejects_symlink_evidence_and_malformed_requests(self):
        closure, request, path, raw = self._closure_fixture()
        p = Path(request["backup_directory"]) / "before/README.md"
        p.unlink()
        p.symlink_to(self.root / "README.md")
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)
        (self.root / path).write_text("{not json")
        with self.assertRaises(gate.GateError):
            closure.close(self.root, path)

    def test_closure_successor_failure_retains_history_without_completion(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request)
        gate.begin_gate(self.root, "independent-task", admission=admission)
        report = self._closure_review("independent-task", passing=False)
        gate.close_failed_gate(self.root, "independent-task", report)
        self.assertTrue(gate.redacted_status(self.root)["valid"])
        self.assertNotIn("completion_marker", gate.read_state(self.root))
        with self.assertRaises(gate.GateError):
            gate.begin_gate(self.root, "another-task", admission=admission)
        self.assertEqual(closure.historical(self.root, request["failed_state_sha256"])["raw_state"].encode(), raw)

    def test_closure_actual_cli_and_stop_preserve_terminal_bytes(self):
        closure, request, path, raw = self._closure_fixture()
        result = subprocess.run([sys.executable, "-B", str(MODULE_PATH),
                                 "close-without-completion", "--request", path],
                                cwd=self.root, capture_output=True, text=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stderr)
        result = subprocess.run([sys.executable, "-B", str(MODULE_PATH), "stop"],
                                cwd=self.root, input=json.dumps(self._stop_payload()),
                                capture_output=True, text=True, timeout=30)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "")
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_closure_missing_history_and_wrong_state_fail_closed(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        wrong = json.loads(raw)
        wrong["report_sha256"] = "0" * 64
        self.assertFalse(gate.validate_failed_state(self.root, wrong))
        p = closure.receipt_path(self.root, request["failed_state_sha256"])
        record = p.read_bytes()
        p.unlink()
        self.assertFalse(gate.redacted_status(self.root)["valid"])
        p.write_bytes(record)
        original_report = self.root / json.loads(raw)["report_path"]
        original_report.write_bytes(original_report.read_bytes() + b"changed")
        self.assertFalse(gate.redacted_status(self.root)["valid"])

    def test_closure_admission_interruption_retries_without_lost_history(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        body, admission = self._closure_admission(closure, request)
        with mock.patch.object(gate, "write_state", side_effect=gate.GateError("interrupted")):
            with self.assertRaises(gate.GateError):
                gate.begin_gate(self.root, "independent-task", admission=admission)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        gate.begin_gate(self.root, "independent-task", admission=admission)
        self.assertEqual(gate.read_state(self.root)["status"], "active")
        p = self.root / body["readiness_report"]
        p.write_bytes(p.read_bytes() + b"tampered")
        report = self._closure_review("independent-task")
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "independent-task", report)

    def test_closure_cannot_reclose_or_finalize_original_failure(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        original_report = json.loads(raw)["report_path"]
        with self.assertRaisesRegex(gate.GateError, "immutable"):
            gate.close_failed_gate(self.root, "04g", original_report)
        with self.assertRaises(gate.GateError):
            gate.finalize_gate(self.root, "04g", original_report)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)

    def test_closure_malformed_nested_evidence_is_a_controlled_rejection(self):
        closure, request, path, raw = self._closure_fixture()
        closure.close(self.root, path)
        p = closure.receipt_path(self.root, request["failed_state_sha256"])
        original = p.read_bytes()
        for malformed in (None, [], {}):
            record = json.loads(original)
            record["payload"]["raw_state"] = malformed
            record["sha256"] = closure.digest(closure.encoded(record["payload"]))
            p.write_bytes(closure.encoded(record))
            self.assertFalse(gate.redacted_status(self.root)["valid"])
            self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)
        p.write_bytes(original)
        body, admission = self._closure_admission(closure, request)
        gate.begin_gate(self.root, "independent-task", admission=admission)
        state = gate.read_state(self.root)
        state["schema_version"] = 1
        with self.assertRaises(gate.GateError):
            gate.validate_state(state)

class GovernanceIntegrationTests(unittest.TestCase):
    setUp = PostIncrementGateTests.setUp
    tearDown = PostIncrementGateTests.tearDown
    _git = PostIncrementGateTests._git
    _stop_payload = PostIncrementGateTests._stop_payload
    _write_report = PostIncrementGateTests._write_report
    _closure_review = ClosureWithoutCompletionTests._closure_review
    _closure_fixture = ClosureWithoutCompletionTests._closure_fixture

    def _linked_root(self):
        other = tempfile.TemporaryDirectory()
        self.addCleanup(other.cleanup)
        path = Path(other.name).resolve() / "linked"
        self._git("worktree", "add", "--detach", str(path), "HEAD")
        original = self.root
        self.addCleanup(setattr, self, "root", original)
        self.root = path
        (path / "docs/reviews").mkdir(parents=True)
        gate.begin_gate(path, "04g")
        return original

    def test_linked_worktree_closure_uses_own_index_and_retains_raw_failure(self):
        original = self._linked_root()
        closure, request, path, raw = self._closure_fixture()
        index = closure.git_index_path(self.root)
        self.assertTrue((self.root / ".git").is_file())
        self.assertNotEqual(index, closure.git_index_path(original))
        before = index.read_bytes()
        closure.close(self.root, path)
        self.assertEqual(index.read_bytes(), before)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        self.assertTrue(gate.validate_failed_state(self.root, gate.read_state(self.root)))
        self.assertFalse(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_linked_worktree_index_drift_blocks_closure(self):
        self._linked_root()
        closure, request, path, raw = self._closure_fixture()
        self._git("add", "DECISIONS.md")
        with self.assertRaisesRegex(gate.GateError, "index drift"):
            closure.close(self.root, path)
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        self.assertIsNone(closure.current_receipt(self.root))

    def test_git_index_path_rejects_malformed_or_unavailable_output(self):
        closure = gate._closure()
        for value in (b"", b"one\ntwo", b"\xff", b"missing-index"):
            with self.subTest(value=value), mock.patch.object(gate, "_run_git", return_value=value):
                with self.assertRaises(gate.GateError):
                    closure.git_index_path(self.root)

    def test_git_index_path_rejects_symlink_without_resolving_it(self):
        closure = gate._closure()
        link = self.root / "index-link"
        link.symlink_to(closure.git_index_path(self.root))
        with mock.patch.object(gate, "_run_git", return_value=str(link).encode()):
            with self.assertRaises(gate.GateError):
                closure.git_index_path(self.root)

    def test_active_legacy_state_is_not_converted_or_rewritten(self):
        before = gate.state_path(self.root).read_bytes()
        gate.begin_gate(self.root, "04g")
        self.assertEqual(gate.state_path(self.root).read_bytes(), before)
        with self.assertRaisesRegex(gate.GateError, "cannot convert"):
            gate.begin_gate(self.root, "04g", admission="missing.json")
        self.assertEqual(gate.state_path(self.root).read_bytes(), before)
        self.assertTrue(gate.evaluate_stop_payload(self._stop_payload()).should_continue)

    def test_mixed_admission_cli_and_api_are_rejected(self):
        before = gate.state_path(self.root).read_bytes()
        with self.assertRaisesRegex(gate.GateError, "choose one"):
            gate.begin_gate(self.root, "04g", "one.json", admission="two.json")
        with mock.patch("sys.stderr", io.StringIO()), self.assertRaises(SystemExit):
            gate.build_parser().parse_args(["begin", "--increment", "04g", "--admission", "a", "--acceptance-request", "b"])
        self.assertEqual(gate.state_path(self.root).read_bytes(), before)

    def test_mixed_lineage_and_unbound_milestone_evidence_are_rejected(self):
        state = gate.read_state(self.root)
        state.update(schema_version=4, acceptance_lineage={"maintenance": "0" * 64, "admission": "1" * 64},
                     closure_lineage={"closures": [], "admission": "2" * 64})
        with self.assertRaisesRegex(gate.GateError, "cannot be combined"):
            gate.validate_state(state)
        with self.assertRaisesRegex(gate.GateError, "needs objective-based admission"):
            gate._closure().milestone_report(self.root, "04g", {"milestone": {}})

    def test_general_closure_cannot_adopt_d133_maintenance(self):
        import lifecycle_acceptance as acceptance
        closure, request, path, raw = self._closure_fixture()
        pointer = self.root / acceptance.POINTER
        pointer.parent.mkdir(parents=True, exist_ok=True)
        pointer.write_text("Retained D-133 fixture; never adopted by general closure.")
        with self.assertRaisesRegex(gate.GateError, "D-133"):
            closure.close(self.root, path)
        with self.assertRaisesRegex(gate.GateError, "D-133"):
            closure.begin(self.root, "other", "missing.json")
        self.assertEqual(gate.state_path(self.root).read_bytes(), raw)
        self.assertIsNone(closure.current_receipt(self.root))


if __name__ == "__main__":
    unittest.main()
