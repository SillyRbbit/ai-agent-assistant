from __future__ import annotations

import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

HOOKS_DIRECTORY = Path(__file__).resolve().parents[1]
if str(HOOKS_DIRECTORY) not in sys.path:
    sys.path.insert(0, str(HOOKS_DIRECTORY))

import common


class CommonHookTests(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary_directory = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary_directory.name).resolve()
        self._git("init", "--quiet")
        self._git("config", "user.email", "tests@example.invalid")
        self._git("config", "user.name", "Common Hook Tests")
        (self.root / "src").mkdir()
        (self.root / "src/app.ts").write_text(
            "export const value = 1;\n", encoding="utf-8"
        )
        self._git("add", "src/app.ts")
        self._git("commit", "--quiet", "-m", "Create fixture")
        self.default_branch = self._git_output("branch", "--show-current")

    def tearDown(self) -> None:
        self.temporary_directory.cleanup()

    def _run_git(self, *arguments: str) -> subprocess.CompletedProcess[bytes]:
        return subprocess.run(
            ["git", *arguments],
            cwd=self.root,
            check=False,
            capture_output=True,
        )

    def _git(self, *arguments: str) -> None:
        result = self._run_git(*arguments)
        if result.returncode != 0:
            self.fail("fixture Git command failed")

    def _git_output(self, *arguments: str) -> str:
        result = self._run_git(*arguments)
        if result.returncode != 0:
            self.fail("fixture Git command failed")
        return result.stdout.decode("utf-8").strip()

    def test_changed_paths_are_classified_without_overlap(self) -> None:
        (self.root / "src/app.ts").write_text(
            "export const value = 2;\n", encoding="utf-8"
        )
        (self.root / "staged.md").write_text("staged\n", encoding="utf-8")
        self._git("add", "staged.md")
        (self.root / "untracked.md").write_text("untracked\n", encoding="utf-8")

        self.assertEqual(common.unstaged_paths(self.root), ("src/app.ts",))
        self.assertEqual(common.staged_paths(self.root), ("staged.md",))
        self.assertEqual(common.untracked_paths(self.root), ("untracked.md",))
        self.assertEqual(
            common.changed_paths(self.root),
            ("src/app.ts", "staged.md", "untracked.md"),
        )

    def test_repository_root_rejects_directory_outside_git(self) -> None:
        with tempfile.TemporaryDirectory() as outside_directory:
            with self.assertRaises(common.GateError):
                common.repository_root(Path(outside_directory))

    def test_unsafe_relative_paths_are_rejected(self) -> None:
        for path in ("../outside", "/absolute", ".git/config"):
            with self.subTest(path=path):
                with self.assertRaises(common.GateError):
                    common.validate_relative_path(path)

    def test_real_merge_conflict_is_detected(self) -> None:
        (self.root / "conflict.txt").write_text("base\n", encoding="utf-8")
        self._git("add", "conflict.txt")
        self._git("commit", "--quiet", "-m", "Add conflict fixture")
        self._git("switch", "-c", "topic")
        (self.root / "conflict.txt").write_text("topic\n", encoding="utf-8")
        self._git("commit", "--quiet", "-am", "Change on topic")
        self._git("switch", self.default_branch)
        (self.root / "conflict.txt").write_text("main\n", encoding="utf-8")
        self._git("commit", "--quiet", "-am", "Change on default branch")

        merge = self._run_git("merge", "topic")

        self.assertNotEqual(merge.returncode, 0)
        self.assertEqual(common.conflicted_paths(self.root), ("conflict.txt",))
        self.assertTrue(common.has_merge_conflicts(self.root))

    def test_suspicious_paths_use_closed_patterns(self) -> None:
        paths = (".env", "docs/review.md", "src-tauri/target/output", "token.pem")

        self.assertEqual(
            common.suspicious_changed_paths(paths),
            (".env", "src-tauri/target/output", "token.pem"),
        )


if __name__ == "__main__":
    unittest.main()
