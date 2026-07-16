from __future__ import annotations

import contextlib
import importlib.util
import io
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

HOOKS_DIRECTORY = Path(__file__).resolve().parents[1]
if str(HOOKS_DIRECTORY) not in sys.path:
    sys.path.insert(0, str(HOOKS_DIRECTORY))

MODULE_PATH = HOOKS_DIRECTORY / "session_end_gate.py"
SPEC = importlib.util.spec_from_file_location("session_end_gate", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("session-end gate module could not be loaded")
session_end_gate = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = session_end_gate
SPEC.loader.exec_module(session_end_gate)


class SessionEndGateTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary_directory.name).resolve()
        self._git("init", "--quiet")
        self._git("config", "user.email", "tests@example.invalid")
        self._git("config", "user.name", "Session End Gate Tests")
        (self.root / "src").mkdir()
        (self.root / "src/app.ts").write_text(
            "export const value = 1;\n", encoding="utf-8"
        )
        self._git("add", "src/app.ts")
        self._git("commit", "--quiet", "-m", "Create fixture")

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

    def test_inspection_reports_staged_unstaged_and_untracked_paths(self) -> None:
        (self.root / "src/app.ts").write_text(
            "export const value = 2;\n", encoding="utf-8"
        )
        (self.root / "staged.md").write_text("staged\n", encoding="utf-8")
        self._git("add", "staged.md")
        (self.root / "untracked.md").write_text("untracked\n", encoding="utf-8")

        snapshot = session_end_gate.inspect_repository(self.root)

        self.assertEqual(snapshot["staged_paths"], ["staged.md"])
        self.assertEqual(snapshot["unstaged_paths"], ["src/app.ts"])
        self.assertEqual(snapshot["untracked_paths"], ["untracked.md"])
        self.assertFalse(snapshot["has_conflicts"])

    def test_inspection_does_not_modify_application_source(self) -> None:
        source_path = self.root / "src/app.ts"
        before = source_path.read_bytes()

        session_end_gate.inspect_repository(self.root)

        self.assertEqual(source_path.read_bytes(), before)
        self.assertEqual(
            session_end_gate.inspect_repository(self.root)["staged_paths"], []
        )
        self.assertEqual(
            session_end_gate.inspect_repository(self.root)["unstaged_paths"], []
        )

    def test_main_rejects_directory_outside_repository(self) -> None:
        with tempfile.TemporaryDirectory() as outside_directory:
            stdout = io.StringIO()
            stderr = io.StringIO()
            with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
                result = session_end_gate.main(["--cwd", outside_directory])

        self.assertEqual(result, int(session_end_gate.ExitCode.VALIDATION_FAILED))
        self.assertEqual(stdout.getvalue(), "")
        self.assertIn("not inside a Git repository", stderr.getvalue())

    def test_main_returns_validation_failure_for_conflict(self) -> None:
        snapshot = {
            "conflicted_paths": ["conflict.txt"],
            "has_conflicts": True,
            "staged_paths": [],
            "unstaged_paths": ["conflict.txt"],
            "untracked_paths": [],
        }
        stdout = io.StringIO()
        with mock.patch.object(
            session_end_gate, "repository_root", return_value=self.root
        ), mock.patch.object(
            session_end_gate, "inspect_repository", return_value=snapshot
        ), contextlib.redirect_stdout(stdout):
            result = session_end_gate.main([])

        self.assertEqual(result, int(session_end_gate.ExitCode.VALIDATION_FAILED))
        self.assertIn('"has_conflicts": true', stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
