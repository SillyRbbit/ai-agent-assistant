from __future__ import annotations

import importlib.util
import io
import subprocess
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path


def load_module():
    module_path = Path(__file__).resolve().parents[1] / "ci_change_scope.py"
    spec = importlib.util.spec_from_file_location("ci_change_scope", module_path)
    if spec is None or spec.loader is None:
        raise RuntimeError("ci_change_scope module could not be loaded")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


scope = load_module()


class ChangeScopeTests(unittest.TestCase):
    def test_documentation_only_skips_application_jobs(self) -> None:
        self.assertEqual(
            scope.classify(("README.md", "docs/guide.md", "prompts/README.md")),
            scope.Scope(frontend=False, rust=False, audit=False),
        )

    def test_frontend_only_runs_frontend(self) -> None:
        self.assertEqual(
            scope.classify(("src/components/PageState.tsx",)),
            scope.Scope(frontend=True, rust=False, audit=False),
        )

    def test_application_brand_asset_runs_frontend(self) -> None:
        self.assertEqual(
            scope.classify(("assets/branding/logo-dark.png",)),
            scope.Scope(frontend=True, rust=False, audit=False),
        )

    def test_rust_test_only_runs_rust(self) -> None:
        self.assertEqual(
            scope.classify(("src-tauri/tests/smoke.rs",)),
            scope.Scope(frontend=False, rust=True, audit=False),
        )

    def test_tauri_ipc_change_runs_both_application_jobs(self) -> None:
        self.assertEqual(
            scope.classify(("src/infrastructure/tauri/app-info-client.ts",)),
            scope.Scope(frontend=True, rust=True, audit=True),
        )

    def test_security_sensitive_rust_change_runs_both_jobs(self) -> None:
        self.assertEqual(
            scope.classify(("src-tauri/src/approvals/manager.rs",)),
            scope.Scope(frontend=True, rust=True, audit=True),
        )

    def test_repository_policy_change_runs_only_audit(self) -> None:
        self.assertEqual(
            scope.classify(("scripts/repository_health.py",)),
            scope.Scope(frontend=False, rust=False, audit=True),
        )

    def test_hook_change_runs_only_repository_audit(self) -> None:
        self.assertEqual(
            scope.classify((".codex/hooks/session_end_gate.py",)),
            scope.Scope(frontend=False, rust=False, audit=True),
        )

    def test_documentation_workflow_change_runs_only_repository_audit(self) -> None:
        self.assertEqual(
            scope.classify((".github/workflows/documentation.yml",)),
            scope.Scope(frontend=False, rust=False, audit=True),
        )

    def test_dependency_change_runs_both_jobs_and_audit(self) -> None:
        self.assertEqual(
            scope.classify(("src-tauri/Cargo.lock",)),
            scope.Scope(frontend=True, rust=True, audit=True),
        )

    def test_ci_workflow_change_runs_every_application_job(self) -> None:
        self.assertEqual(
            scope.classify((".github/workflows/ci.yml",)),
            scope.Scope(frontend=True, rust=True, audit=True),
        )

    def test_unknown_non_documentation_path_fails_closed(self) -> None:
        self.assertEqual(
            scope.classify(("new-runtime/config.custom",)),
            scope.Scope(frontend=True, rust=True, audit=False),
        )

    def test_manual_dispatch_runs_every_application_job(self) -> None:
        output = io.StringIO()
        with redirect_stdout(output):
            result = scope.main(
                (
                    "classify",
                    "--event",
                    "workflow_dispatch",
                    "--head",
                    "a" * 40,
                )
            )

        self.assertEqual(result, 0)
        self.assertEqual(
            output.getvalue(), "frontend=true\nrust=true\naudit=true\n"
        )

    def test_schedule_runs_only_dependency_audit(self) -> None:
        output = io.StringIO()
        with redirect_stdout(output):
            result = scope.main(("classify", "--event", "schedule"))

        self.assertEqual(result, 0)
        self.assertEqual(
            output.getvalue(), "frontend=false\nrust=false\naudit=true\n"
        )

    def test_unsafe_path_is_rejected(self) -> None:
        with self.assertRaisesRegex(RuntimeError, "unsafe changed path"):
            scope.classify(("../outside",))

    def test_pull_request_uses_merge_base_and_includes_deletion(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.run_git(root, "init", "-q")
            self.run_git(root, "config", "user.email", "ci@example.invalid")
            self.run_git(root, "config", "user.name", "CI fixture")
            (root / "README.md").write_text("base\n", encoding="utf-8")
            self.run_git(root, "add", "README.md")
            self.run_git(root, "commit", "-qm", "base")
            base = self.run_git(root, "rev-parse", "HEAD").stdout.strip()
            (root / "README.md").unlink()
            (root / "src").mkdir()
            (root / "src" / "main.ts").write_text("export {};\n", encoding="utf-8")
            self.run_git(root, "add", "-A")
            self.run_git(root, "commit", "-qm", "change")
            head = self.run_git(root, "rev-parse", "HEAD").stdout.strip()

            paths = scope.changed_paths(root, "pull_request", base, head)

            self.assertEqual(paths, ("README.md", "src/main.ts"))

    def test_push_uses_before_and_head_range(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.run_git(root, "init", "-q")
            self.run_git(root, "config", "user.email", "ci@example.invalid")
            self.run_git(root, "config", "user.name", "CI fixture")
            (root / "README.md").write_text("base\n", encoding="utf-8")
            self.run_git(root, "add", "README.md")
            self.run_git(root, "commit", "-qm", "base")
            before = self.run_git(root, "rev-parse", "HEAD").stdout.strip()
            (root / "src-tauri").mkdir()
            (root / "src-tauri" / "Cargo.toml").write_text(
                "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n",
                encoding="utf-8",
            )
            self.run_git(root, "add", "src-tauri/Cargo.toml")
            self.run_git(root, "commit", "-qm", "change")
            head = self.run_git(root, "rev-parse", "HEAD").stdout.strip()

            paths = scope.changed_paths(root, "push", before, head)

            self.assertEqual(paths, ("src-tauri/Cargo.toml",))

    @staticmethod
    def run_git(root: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
        return subprocess.run(
            ["git", *arguments],
            cwd=root,
            check=True,
            capture_output=True,
            text=True,
        )


if __name__ == "__main__":
    unittest.main()
