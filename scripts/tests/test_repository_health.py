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


def write_valid_ui_native_boundary(root: Path) -> None:
    app_info = root / "src" / "infrastructure" / "tauri" / "app-info-client.ts"
    menu_route = root / "src" / "infrastructure" / "tauri" / "menu-route-client.ts"
    projection_client = (
        root
        / "src"
        / "infrastructure"
        / "tauri"
        / "research-knowledge-demo-projection-client.ts"
    )
    lifecycle_client = (
        root
        / "src"
        / "infrastructure"
        / "tauri"
        / "research-knowledge-demo-lifecycle-client.ts"
    )
    command_center = root / "src" / "features" / "command-center" / "fixture.ts"
    lifecycle_panel = (
        root
        / "src"
        / "features"
        / "command-center"
        / "ResearchKnowledgeLifecyclePanel.tsx"
    )
    command_center_page = (
        root / "src" / "features" / "command-center" / "CommandCenterPage.tsx"
    )
    lib = root / "src-tauri" / "src" / "lib.rs"
    projection_rust = (
        root / "src-tauri" / "src" / "research_knowledge_demo_projection.rs"
    )
    lifecycle_rust = (
        root / "src-tauri" / "src" / "research_knowledge_demo_lifecycle_tauri.rs"
    )
    menu_action = root / "src-tauri" / "src" / "menu_bar" / "action.rs"
    menu_adapter = root / "src-tauri" / "src" / "menu_bar" / "tauri_adapter.rs"
    capability = root / "src-tauri" / "capabilities" / "default.json"
    configuration = root / "src-tauri" / "tauri.conf.json"
    app_info.parent.mkdir(parents=True)
    command_center.parent.mkdir(parents=True)
    lib.parent.mkdir(parents=True)
    capability.parent.mkdir(parents=True)
    app_info.write_text(
        'import { invoke } from "@tauri-apps/api/core";\n'
        'invoke<unknown>("get_app_info");\n',
        encoding="utf-8",
    )
    menu_route.write_text(
        'import { listen, type UnlistenFn } from "@tauri-apps/api/event";\n'
        'export const ASSISTANT_MENU_ROUTE_EVENT = "assistant-menu-route";\n'
        "listen<unknown>(ASSISTANT_MENU_ROUTE_EVENT, () => undefined);\n",
        encoding="utf-8",
    )
    projection_client.write_text(
        'import { invoke } from "@tauri-apps/api/core";\n'
        'invoke<unknown>("get_research_knowledge_demo_projection");\n',
        encoding="utf-8",
    )
    lifecycle_client.write_text(
        (
            Path(__file__).resolve().parents[2]
            / "src"
            / "infrastructure"
            / "tauri"
            / "research-knowledge-demo-lifecycle-client.ts"
        ).read_text(encoding="utf-8"),
        encoding="utf-8",
    )
    command_center.write_text("export const fixture = true;\n", encoding="utf-8")
    direct_client = "src/infrastructure/tauri/personal-assistant-direct-client.ts"
    (root / direct_client).write_text(
        (Path(__file__).resolve().parents[2] / direct_client).read_text(encoding="utf-8"),
        encoding="utf-8",
    )
    agent_client = "src/infrastructure/tauri/agent-chat-client.ts"
    (root / agent_client).write_text(
        (Path(__file__).resolve().parents[2] / agent_client).read_text(encoding="utf-8"),
        encoding="utf-8",
    )
    lifecycle_panel.write_text(
        (
            Path(__file__).resolve().parents[2]
            / "src"
            / "features"
            / "command-center"
            / "ResearchKnowledgeLifecyclePanel.tsx"
        ).read_text(encoding="utf-8"),
        encoding="utf-8",
    )
    command_center_page.write_text(
        'import { ResearchKnowledgeLifecyclePanel } from "./ResearchKnowledgeLifecyclePanel";\n'
        "export function CommandCenterPage() {\n"
        '  const state = { scenarioId: "research-knowledge-active" };\n'
        "  return (\n"
        "    <>\n"
        '      {state.scenarioId === "research-knowledge-active" ? (\n'
        "        <ResearchKnowledgeLifecyclePanel />\n"
        "      ) : null}\n"
        "    </>\n"
        "  );\n"
        "}\n",
        encoding="utf-8",
    )
    lib.write_text(
        ".invoke_handler(tauri::generate_handler![\n"
        "app_info::get_app_info,\n"
        "research_knowledge_demo_projection::get_research_knowledge_demo_projection,\n"
        "research_knowledge_demo_lifecycle_tauri::get_research_knowledge_demo_lifecycle_snapshot,\n"
        "research_knowledge_demo_lifecycle_tauri::start_research_knowledge_demo_lifecycle,\n"
        "research_knowledge_demo_lifecycle_tauri::advance_research_knowledge_demo_lifecycle,\n"
        "research_knowledge_demo_lifecycle_tauri::cancel_research_knowledge_demo_lifecycle,\n"
        "personal_assistant_direct_tauri::start_personal_assistant_direct,\n"
        "personal_assistant_direct_tauri::poll_personal_assistant_direct,\n"
        "personal_assistant_direct_tauri::cancel_personal_assistant_direct,\n"
        "agent_chat_tauri::list_agent_preferences,\n"
        "agent_chat_tauri::save_agent_preferences,\n"
        "agent_chat_tauri::clear_agent_note,\n"
        "agent_chat_tauri::restore_agent_defaults,\n"
        "agent_chat_tauri::list_agent_connections,\n"
        "agent_chat_tauri::discover_agent_models,\n"
        "agent_chat_tauri::start_agent_conversation,\n"
        "agent_chat_tauri::send_agent_message,\n"
        "agent_chat_tauri::poll_agent_conversation,\n"
        "agent_chat_tauri::cancel_agent_conversation\n"
        "])\n",
        encoding="utf-8",
    )
    projection_rust.write_text(
        "#[tauri::command]\n"
        "pub(crate) fn get_research_knowledge_demo_projection() -> "
        "Result<ResearchKnowledgeDemoProjection, "
        "ResearchKnowledgeDemoProjectionError> { todo!() }\n",
        encoding="utf-8",
    )
    lifecycle_rust.write_text(
        'const EVENT: &str = "research-knowledge-demo-lifecycle-v1";\n'
        "#[tauri::command]\n"
        "pub(crate) fn get_research_knowledge_demo_lifecycle_snapshot(\n"
        "state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,\n"
        ") -> Result<Snapshot, Error> { todo!() }\n"
        "#[tauri::command]\n"
        "pub(crate) fn start_research_knowledge_demo_lifecycle(\n"
        "app: tauri::AppHandle,\n"
        "state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,\n"
        ") -> Result<Snapshot, Error> { app.emit(EVENT, value); todo!() }\n"
        "#[tauri::command]\n"
        "pub(crate) fn advance_research_knowledge_demo_lifecycle(\n"
        "app: tauri::AppHandle,\n"
        "state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,\n"
        ") -> Result<Snapshot, Error> { app.emit(EVENT, value); todo!() }\n"
        "#[tauri::command]\n"
        "pub(crate) fn cancel_research_knowledge_demo_lifecycle(\n"
        "app: tauri::AppHandle,\n"
        "state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,\n"
        ") -> Result<Snapshot, Error> { app.emit(EVENT, value); todo!() }\n",
        encoding="utf-8",
    )
    menu_adapter.parent.mkdir(parents=True)
    menu_action.write_text(
        'pub const MENU_ROUTE_EVENT: &str = "assistant-menu-route";\n', encoding="utf-8"
    )
    menu_adapter.write_text("app.emit(MENU_ROUTE_EVENT, value);\n", encoding="utf-8")
    capability.write_text(
        json.dumps(health.EXPECTED_CAPABILITY_CONFIGURATION), encoding="utf-8"
    )
    configuration.write_text(
        json.dumps(
            {
                "app": {
                    "security": {
                        "csp": health.EXPECTED_TAURI_CSP,
                        "devCsp": health.EXPECTED_TAURI_DEV_CSP,
                    }
                }
            }
        ),
        encoding="utf-8",
    )


class RepositoryHealthTests(unittest.TestCase):
    def test_agent_chat_boundary_rejects_command_payload_and_import_broadening(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = root / "src/infrastructure/tauri/agent-chat-client.ts"
            baseline = client.read_text(encoding="utf-8")
            self.assertEqual(health.ui_native_boundary_findings(root), ())
            replacements = (
                ('"list_agent_preferences"', '"execute_arbitrary"'),
                ('"list_agent_connections"', '"get_credentials"'),
                ('"discover_agent_models"', '"discover_arbitrary_url"'),
                ('"save_agent_preferences"', '"write_arbitrary_file"'),
                ('"clear_agent_note"', '"clear_all_notes"'),
                ('"restore_agent_defaults"', '"modify_authority"'),
                ('"start_agent_conversation"', '"start_runtime"'),
                ('"send_agent_message"', '"execute_action"'),
                ('"poll_agent_conversation"', '"get_provider_body"'),
                ('"cancel_agent_conversation"', '"kill_arbitrary_process"'),
                ('{ request }', '{ request, key: "fixture" }'),
                ('{ agentId, revision }', '{ agentId, revision, allAgents: true }'),
                ('{ agentId }', '{ agentId, note: "caller context" }'),
                ('{ conversationId, message, acknowledgment }', '{ conversationId, message, acknowledgment, providerId: "caller" }'),
                ('{ conversationId }', '{ conversationId, url: "caller" }'),
                ('{ invoke, isTauri }', '{ invoke, isTauri, transformCallback }'),
                ('{ invoke, isTauri }', '{ invoke as alternateInvoke, isTauri }'),
            )
            for old, new in replacements:
                with self.subTest(new=new):
                    self.assertIn(old, baseline)
                    client.write_text(baseline.replace(old, new), encoding="utf-8")
                    findings = health.ui_native_boundary_findings(root)
                    self.assertTrue(any(finding.path == client.relative_to(root).as_posix() for finding in findings))
            client.write_text(baseline + '\ninvoke<unknown>("extra_command");\n', encoding="utf-8")
            self.assertTrue(health.ui_native_boundary_findings(root))
            client.write_text(baseline, encoding="utf-8")
            self.assertEqual(health.ui_native_boundary_findings(root), ())

    def test_agent_chat_boundary_rejects_removed_calls_and_handler_changes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = root / "src/infrastructure/tauri/agent-chat-client.ts"
            baseline = client.read_text(encoding="utf-8")
            for command in (
                "list_agent_preferences", "list_agent_connections", "discover_agent_models", "save_agent_preferences",
                "clear_agent_note", "restore_agent_defaults", "start_agent_conversation",
                "send_agent_message", "poll_agent_conversation", "cancel_agent_conversation",
            ):
                with self.subTest(removed=command):
                    client.write_text(baseline.replace(f'invoke<unknown>("{command}"', f'undefined /* "{command}" */ ('), encoding="utf-8")
                    self.assertTrue(health.ui_native_boundary_findings(root))
            client.write_text(baseline, encoding="utf-8")
            lib = root / "src-tauri/src/lib.rs"
            registered = lib.read_text(encoding="utf-8")
            for replacement in (
                registered.replace("agent_chat_tauri::send_agent_message,\n", ""),
                registered.replace("agent_chat_tauri::send_agent_message", "agent_chat_tauri::execute_action"),
                registered.replace("agent_chat_tauri::cancel_agent_conversation\n", "agent_chat_tauri::cancel_agent_conversation,\narbitrary::execute\n"),
            ):
                with self.subTest(handler=replacement):
                    lib.write_text(replacement, encoding="utf-8")
                    self.assertTrue(any("invoke handler" in finding.detail for finding in health.ui_native_boundary_findings(root)))
            lib.write_text(registered, encoding="utf-8")
            self.assertEqual(health.ui_native_boundary_findings(root), ())

    def test_direct_client_rejects_command_and_payload_broadening(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            write_valid_ui_native_boundary(root)
            client = root / "src/infrastructure/tauri/personal-assistant-direct-client.ts"
            baseline = client.read_text(encoding="utf-8")
            for old, new in [
                ('"start_personal_assistant_direct"', '"execute_arbitrary"'),
                ('request: { version: 1,', 'request: { prompt: "arbitrary", version: 1,'),
                ('request: { handle, cursor }', 'request: { handle, cursor, url: "arbitrary" }'),
                ('request: { handle }', 'request: { handle, key: "arbitrary" }'),
            ]:
                with self.subTest(new=new):
                    client.write_text(baseline.replace(old, new), encoding="utf-8")
                    self.assertTrue(health.ui_native_boundary_findings(root))

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

    def test_documentation_truth_rejects_duplicate_requirement_and_stale_markers(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "PRODUCT_REQUIREMENTS.md").write_text(
                "- **FR-020**: First requirement.\n"
                "- **FR-020**: Duplicate requirement.\n"
                "matrix remains pending\n",
                encoding="utf-8",
            )
            (root / "ARCHITECTURE.md").write_text(
                "The app-info response is not yet runtime narrowed.\n",
                encoding="utf-8",
            )

            findings = health.documentation_truth_findings(root)

            self.assertTrue(any("duplicate requirement identifier" in finding.detail for finding in findings))
            self.assertTrue(any("stale current-state marker" in finding.detail for finding in findings))
            self.assertTrue(any("required current-state marker is missing" in finding.detail for finding in findings))

    def test_documentation_truth_accepts_current_markers_and_unique_requirements(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "PRODUCT_REQUIREMENTS.md").write_text(
                "- **FR-019A**: Synthetic demo.\n"
                "- **FR-020**: Tool identity.\n"
                "matrix is complete and verified\n",
                encoding="utf-8",
            )
            (root / "ARCHITECTURE.md").write_text(
                "src/infrastructure/tauri/ runtime-narrows the app-info response.\n"
                "The production CSP excludes development\n"
                "  WebSocket sources and inline-script execution.\n",
                encoding="utf-8",
            )

            self.assertEqual(health.documentation_truth_findings(root), ())

    def test_ui_native_boundary_accepts_exact_current_baseline(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)

            self.assertEqual(health.ui_native_boundary_findings(root), ())

    def test_ui_native_boundary_rejects_alternate_lifecycle_client_consumers(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            alternate = (
                root / "src" / "features" / "command-center" / "AlternateConsumer.tsx"
            )
            consumers = (
                'import "../../infrastructure/tauri/research-knowledge-demo-lifecycle-client";\n',
                'void import("../../infrastructure/tauri/research-knowledge-demo-lifecycle-client");\n',
                'export * from "../../infrastructure/tauri/research-knowledge-demo-lifecycle-client";\n',
            )
            for consumer in consumers:
                with self.subTest(consumer=consumer):
                    alternate.write_text(consumer, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any("lifecycle client consumer" in finding.detail for finding in findings)
                    )

            javascript_consumer = (
                root / "src" / "features" / "command-center" / "AlternateConsumer.jsx"
            )
            javascript_consumer.write_text(
                'export * from "../../infrastructure/tauri/research-knowledge-demo-lifecycle-client";\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(
                any("lifecycle client consumer" in finding.detail for finding in findings)
            )

    def test_ui_native_boundary_rejects_raw_tauri_lifecycle_transport(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            alternate = (
                root / "src" / "features" / "command-center" / "AlternateConsumer.tsx"
            )
            alternate.write_text(
                "type RawTauri = Window & { readonly __TAURI_INTERNALS__: { "
                "invoke(command: string): Promise<unknown> } };\n"
                "void (window as RawTauri).__TAURI_INTERNALS__.invoke("
                '"start_research_knowledge_demo_lifecycle");\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("raw Tauri global" in finding.detail for finding in findings))
            self.assertTrue(
                any("sole exact client allowlist" in finding.detail for finding in findings)
            )

    def test_ui_native_boundary_rejects_lifecycle_panel_import_symbol_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            mutations = (
                baseline.replace(
                    "  RESEARCH_KNOWLEDGE_DEMO_LIFECYCLE_UNAVAILABLE,\n", "", 1
                ),
                baseline.replace(
                    "  type ResearchKnowledgeDemoLifecycleSnapshot,\n",
                    "  type ResearchKnowledgeDemoLifecycleSnapshot,\n"
                    "  futureLifecycleSelector,\n",
                    1,
                ),
            )
            for mutation in mutations:
                with self.subTest(mutation=mutation):
                    panel.write_text(mutation, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any("exact client-symbol allowlist" in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_rejects_alternate_lifecycle_panel_references(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            alternate = root / "src" / "features" / "command-center" / "index.ts"
            references = (
                'export { ResearchKnowledgeLifecyclePanel } from "./ResearchKnowledgeLifecyclePanel";\n',
                'void import("./ResearchKnowledgeLifecyclePanel");\n',
                "const Alternate = ResearchKnowledgeLifecyclePanel;\n",
            )
            for reference in references:
                with self.subTest(reference=reference):
                    alternate.write_text(reference, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any("lifecycle panel reference" in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_rejects_panel_mount_or_scenario_broadening(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            page = (
                root / "src" / "features" / "command-center" / "CommandCenterPage.tsx"
            )
            baseline = page.read_text(encoding="utf-8")
            mutations = (
                baseline.replace(
                    'state.scenarioId === "research-knowledge-active"',
                    'state.scenarioId !== "infrastructure-blocked"',
                    1,
                ),
                baseline.replace(
                    "<ResearchKnowledgeLifecyclePanel />",
                    '<ResearchKnowledgeLifecyclePanel outcome="success" />',
                    1,
                ),
                baseline.replace(
                    'import { ResearchKnowledgeLifecyclePanel } from "./ResearchKnowledgeLifecyclePanel";',
                    'import { ResearchKnowledgeLifecyclePanel as LifecyclePanel } from "./ResearchKnowledgeLifecyclePanel";',
                    1,
                ),
            )
            for mutation in mutations:
                with self.subTest(mutation=mutation):
                    page.write_text(mutation, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any("selected-scenario zero-prop" in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_rejects_lifecycle_panel_props_and_selectors(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            panel.write_text(
                baseline.replace(
                    "export function ResearchKnowledgeLifecyclePanel()",
                    "export function ResearchKnowledgeLifecyclePanel(props: { outcome: string })",
                    1,
                ),
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("zero-prop component" in finding.detail for finding in findings))

            panel.write_text(baseline + '\nconst selectedOutcome = "success";\n', encoding="utf-8")
            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(
                any("caller-selectable lifecycle fixture" in finding.detail for finding in findings)
            )

    def test_ui_native_boundary_rejects_factory_disposal_and_operation_broadening(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            mutations = (
                (
                    baseline
                    + "\ncreateResearchKnowledgeDemoLifecycleClient(() => undefined);\n",
                    "factory or disposal",
                ),
                (
                    baseline
                    + "\nconst factory = createResearchKnowledgeDemoLifecycleClient;\n",
                    "factory or disposal",
                ),
                (baseline + "\nclient.dispose();\n", "factory or disposal"),
                (baseline + "\nconst dispose = client.dispose;\n", "factory or disposal"),
                (
                    baseline.replace(
                        "await client.start();",
                        'await client.start({ outcome: "caller" });',
                        1,
                    ),
                    "explicit switch dispatcher",
                ),
                (
                    baseline + "\nconst startOperation = client.start;\n",
                    "explicit switch dispatcher",
                ),
                (baseline + '\nclient["start"]();\n', "computed lifecycle operation"),
            )
            for mutation, expected_detail in mutations:
                with self.subTest(expected_detail=expected_detail):
                    panel.write_text(mutation, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(expected_detail in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_rejects_automatic_or_aliased_lifecycle_dispatch(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            relocated_start = baseline.replace(
                "        const current: ResearchKnowledgeDemoLifecycleSnapshot = await client.snapshot();",
                "        const current: ResearchKnowledgeDemoLifecycleSnapshot = await client.snapshot();\n"
                "        await client.start();",
                1,
            ).replace(
                "        nextSnapshot = await client.start();",
                "        nextSnapshot = {} as ResearchKnowledgeDemoLifecycleSnapshot;",
                1,
            )
            mutations = (
                (relocated_start, "explicit switch dispatcher"),
                (
                    baseline + '\nvoid invokeOperation("start");\n',
                    "three-button explicit dispatcher allowlist",
                ),
                (
                    baseline
                    + '\nconst automaticDispatch = invokeOperation;\nautomaticDispatch("start");\n',
                    "three-button explicit dispatcher allowlist",
                ),
                (
                    baseline.replace(
                        'invokeOperation("start")',
                        'invokeOperation("cancel")',
                        1,
                    ),
                    "three-button explicit dispatcher allowlist",
                ),
                (
                    baseline
                    + "\nconst { start: automaticStart } = client;\n"
                    + "void automaticStart();\n",
                    "aliased lifecycle operation",
                ),
                (
                    baseline
                    + '\nconst automaticStart = Reflect.get(client, "start");\n'
                    + "void automaticStart();\n",
                    "aliased lifecycle operation",
                ),
            )
            for mutation, expected_detail in mutations:
                with self.subTest(expected_detail=expected_detail):
                    panel.write_text(mutation, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(expected_detail in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_digest_rejects_equivalent_operation_aliases(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            mutations = (
                "const transport = client; void transport[\"start\"]();",
                "const launch = client?.start; void launch?.();",
                "const launch = Object.values(client)[2] as () => Promise<unknown>; "
                "void launch();",
                'void Object.getOwnPropertyDescriptor(client, "start")?.value();',
            )
            for mutation in mutations:
                with self.subTest(mutation=mutation):
                    panel.write_text(
                        baseline.replace(
                            "    return () => {",
                            f"    {mutation}\n    return () => {{",
                            1,
                        ),
                        encoding="utf-8",
                    )

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any("exact reviewed F-12 digest" in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_rejects_snapshot_outside_mount_hydration(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            panel.write_text(
                baseline.replace(
                    "        const current: ResearchKnowledgeDemoLifecycleSnapshot = await client.snapshot();",
                    "        const current = client as unknown as ResearchKnowledgeDemoLifecycleSnapshot;",
                    1,
                ).replace(
                    "  const invokeOperation = async",
                    "  void lifecycleClient?.snapshot();\n"
                    "  const invokeOperation = async",
                    1,
                ),
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(
                any("sole mount-time hydration" in finding.detail for finding in findings)
            )

    def test_ui_native_boundary_rejects_lifecycle_panel_button_drift(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            mutations = (
                baseline.replace(
                    "</section>", '<button type="button">Retry</button></section>', 1
                ),
                baseline.replace(
                    "Advance simulated lifecycle", "Choose failed outcome", 1
                ),
            )
            for mutation in mutations:
                with self.subTest(mutation=mutation):
                    panel.write_text(mutation, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(
                            "three-button explicit dispatcher allowlist" in finding.detail
                            for finding in findings
                        )
                    )

    def test_ui_native_boundary_rejects_every_lifecycle_panel_prohibited_surface(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            baseline = panel.read_text(encoding="utf-8")
            prohibited = (
                ("<form>", "form control"),
                ("<input />", "form control"),
                ("<select>", "form control"),
                ("<textarea>", "form control"),
                ("contentEditable={true}", "editable content"),
                ("new URLSearchParams()", "route or query selector"),
                ("useSearchParams()", "route or query selector"),
                ("useParams()", "route or query selector"),
                ("location.search", "route or query selector"),
                ("setTimeout(() => undefined, 1)", "timer or background worker"),
                ("setInterval(() => undefined, 1)", "timer or background worker"),
                ("requestAnimationFrame(() => undefined)", "timer or background worker"),
                ("requestIdleCallback(() => undefined)", "timer or background worker"),
                ("queueMicrotask(() => undefined)", "timer or background worker"),
                ('new Worker("fixture")', "timer or background worker"),
                ('new SharedWorker("fixture")', "timer or background worker"),
                ("new XMLHttpRequest()", "network, messaging, or storage API"),
                ('globalThis["fetch"]("/future")', "network, messaging, or storage API"),
                ('new BroadcastChannel("fixture")', "network, messaging, or storage API"),
                ("window.postMessage({})", "network, messaging, or storage API"),
                ("postMessage({})", "network, messaging, or storage API"),
                ("navigator.sendBeacon()", "network, messaging, or storage API"),
                ("sendBeacon()", "network, messaging, or storage API"),
                ("navigator.storage", "network, messaging, or storage API"),
                ('navigator["storage"]', "network, messaging, or storage API"),
                ("caches.open()", "network, messaging, or storage API"),
                ("document.cookie", "network, messaging, or storage API"),
                ('document["cookie"]', "network, messaging, or storage API"),
                ('const outcome = "success"', "caller-selectable lifecycle fixture"),
                ('let script = "failure"', "caller-selectable lifecycle fixture"),
                ('var stage = "research"', "caller-selectable lifecycle fixture"),
                ('const selectedFixture = "caller"', "caller-selectable lifecycle fixture"),
                ('const runId = "caller"', "caller-selectable lifecycle fixture"),
                ('const selectedAgent = "caller"', "caller-selectable lifecycle fixture"),
                ('const workflowName = "caller"', "caller-selectable lifecycle fixture"),
                ('client["start"]()', "computed lifecycle operation"),
            )
            for source, expected_detail in prohibited:
                with self.subTest(source=source):
                    panel.write_text(f"{baseline}\n{source};\n", encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(expected_detail in finding.detail for finding in findings),
                        msg=source,
                    )

    def test_ui_native_boundary_rejects_prohibited_ui_and_native_broadening(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            (root / "src" / "features" / "command-center" / "fixture.ts").write_text(
                'import { invoke } from "@tauri-apps/api/core";\nfetch("/api");\n',
                encoding="utf-8",
            )
            (root / "src-tauri" / "src" / "lib.rs").write_text(
                "tauri::generate_handler![app_info::get_app_info, future::command]\n",
                encoding="utf-8",
            )
            (root / "src-tauri" / "capabilities" / "default.json").write_text(
                json.dumps({"windows": ["main", "other"], "permissions": ["core:default", "shell:allow-open"]}),
                encoding="utf-8",
            )
            (root / "src-tauri" / "tauri.conf.json").write_text(
                json.dumps({"app": {"security": {"csp": {"default-src": "*"}}}}),
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertGreaterEqual(len(findings), 5)
            self.assertTrue(any("Command Center" in finding.detail for finding in findings))
            self.assertTrue(any("invoke handler" in finding.detail for finding in findings))
            self.assertTrue(any("capability" in finding.detail for finding in findings))
            self.assertTrue(any("CSP" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_every_command_center_boundary_token(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            command_center = root / "src" / "features" / "command-center" / "fixture.ts"
            for token in health.COMMAND_CENTER_PROHIBITED_TOKENS:
                command_center.write_text(f"// {token}\n", encoding="utf-8")

                findings = health.ui_native_boundary_findings(root)

                self.assertTrue(
                    any(token in finding.detail for finding in findings),
                    msg=token,
                )

    def test_ui_native_boundary_rejects_tauri_import_suffix(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            (root / "src" / "infrastructure" / "tauri" / "app-info-client.ts").write_text(
                'import { invoke } from "@tauri-apps/api/core/extended";\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("Tauri API import" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_tauri_plugin_import(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            panel = (
                root
                / "src"
                / "features"
                / "command-center"
                / "ResearchKnowledgeLifecyclePanel.tsx"
            )
            panel.write_text(
                panel.read_text(encoding="utf-8")
                + '\nimport { open } from "@tauri-apps/plugin-shell";\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("Tauri API import" in finding.detail for finding in findings))
            self.assertTrue(any("Command Center" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_extra_tauri_import_symbols(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = root / "src" / "infrastructure" / "tauri" / "app-info-client.ts"
            client.write_text(
                'import { invoke, transformCallback } from "@tauri-apps/api/core";\n'
                'invoke<unknown>("get_app_info");\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("Tauri API import" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_invoke_arguments_or_an_extra_command(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = (
                root
                / "src"
                / "infrastructure"
                / "tauri"
                / "research-knowledge-demo-projection-client.ts"
            )
            client.write_text(
                'import { invoke } from "@tauri-apps/api/core";\n'
                'invoke<unknown>("get_research_knowledge_demo_projection", { runId: "caller" });\n'
                'invoke("future_command");\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("exact no-argument" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_projection_command_arguments(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            projection = (
                root / "src-tauri" / "src" / "research_knowledge_demo_projection.rs"
            )
            projection.write_text(
                "#[tauri::command]\n"
                "pub(crate) fn get_research_knowledge_demo_projection(run_id: String) -> "
                "Result<ResearchKnowledgeDemoProjection, "
                "ResearchKnowledgeDemoProjectionError> { todo!() }\n",
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("zero-argument" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_every_projection_rust_boundary_token(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            projection = (
                root / "src-tauri" / "src" / "research_knowledge_demo_projection.rs"
            )
            baseline = projection.read_text(encoding="utf-8")
            for token in health.PROJECTION_RUST_PROHIBITED_TOKENS:
                with self.subTest(token=token):
                    projection.write_text(
                        f"{baseline}\n// {token}\n", encoding="utf-8"
                    )

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(token in finding.detail for finding in findings),
                        msg=token,
                    )

    def test_ui_native_boundary_rejects_every_projection_client_boundary_token(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = (
                root
                / "src"
                / "infrastructure"
                / "tauri"
                / "research-knowledge-demo-projection-client.ts"
            )
            baseline = client.read_text(encoding="utf-8")
            for token in health.PROJECTION_CLIENT_PROHIBITED_TOKENS:
                with self.subTest(token=token):
                    client.write_text(f"{baseline}\n// {token}\n", encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(token in finding.detail for finding in findings),
                        msg=token,
                    )

    def test_ui_native_boundary_rejects_extra_capability_keys(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            capability = root / "src-tauri" / "capabilities" / "default.json"
            parsed = json.loads(capability.read_text(encoding="utf-8"))
            parsed["remote"] = {"urls": ["https://example.invalid"]}
            capability.write_text(json.dumps(parsed), encoding="utf-8")

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("complete exact baseline" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_nested_or_non_json_capability_files(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            nested = root / "src-tauri" / "capabilities" / "nested" / "future.toml"
            nested.parent.mkdir()
            nested.write_text('identifier = "future"\n', encoding="utf-8")

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("capability file allowlist" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_a_second_listener_or_emitter(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            menu_route = root / "src" / "infrastructure" / "tauri" / "menu-route-client.ts"
            menu_route.write_text(
                menu_route.read_text(encoding="utf-8")
                + 'listen<unknown>("future-event", () => undefined);\n',
                encoding="utf-8",
            )
            (root / "src-tauri" / "src" / "future.rs").write_text(
                'app.emit("future-event", value);\n', encoding="utf-8"
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("sole exact event" in finding.detail for finding in findings))
            self.assertTrue(
                any("exact menu and lifecycle allowlist" in finding.detail for finding in findings)
            )

    def test_ui_native_boundary_rejects_tauri_emitter_variants(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            future = root / "src-tauri" / "src" / "future.rs"
            for emitter in (
                "emit_to",
                "emit_filter",
                "emit_str",
                "emit_str_to",
                "emit_str_filter",
            ):
                with self.subTest(emitter=emitter):
                    future.write_text(
                        f'app.{emitter}("future-event", value);\n', encoding="utf-8"
                    )

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(
                            "exact menu and lifecycle allowlist" in finding.detail
                            for finding in findings
                        )
                    )

    def test_ui_native_boundary_rejects_lifecycle_command_arguments(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            adapter = (
                root
                / "src-tauri"
                / "src"
                / "research_knowledge_demo_lifecycle_tauri.rs"
            )
            adapter.write_text(
                adapter.read_text(encoding="utf-8").replace(
                    "state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,\n)",
                    "state: tauri::State<'_, ResearchKnowledgeDemoLifecycleTauriState>,\nrun_id: String,\n)",
                    1,
                ),
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("no-caller-input" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_lifecycle_invoke_arguments_or_extra_listener(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = (
                root
                / "src"
                / "infrastructure"
                / "tauri"
                / "research-knowledge-demo-lifecycle-client.ts"
            )
            client.write_text(
                client.read_text(encoding="utf-8").replace(
                    'invoke<unknown>("start_research_knowledge_demo_lifecycle")',
                    'invoke<unknown>("start_research_knowledge_demo_lifecycle", '
                    '{ runId: "caller" })',
                    1,
                )
                + '\nvoid listen<unknown>("future-event", () => undefined);\n',
                encoding="utf-8",
            )

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("no-argument command allowlist" in finding.detail for finding in findings))
            self.assertTrue(any("notification allowlist" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_lifecycle_invoke_or_listener_aliases(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = (
                root
                / "src"
                / "infrastructure"
                / "tauri"
                / "research-knowledge-demo-lifecycle-client.ts"
            )
            baseline = client.read_text(encoding="utf-8")
            mutations = (
                (
                    baseline
                    + "\nconst alternateInvoke = invoke;\n"
                    + 'alternateInvoke("future_command", { outcome: "failed" });\n',
                    "no-argument command allowlist",
                ),
                (
                    baseline
                    + "\nconst alternateListen = listen;\n"
                    + 'void alternateListen("future-event", () => undefined);\n',
                    "notification allowlist",
                ),
            )
            for mutation, expected_detail in mutations:
                with self.subTest(expected_detail=expected_detail):
                    client.write_text(mutation, encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(expected_detail in finding.detail for finding in findings)
                    )

    def test_ui_native_boundary_rejects_every_lifecycle_client_boundary_token(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = (
                root
                / "src"
                / "infrastructure"
                / "tauri"
                / "research-knowledge-demo-lifecycle-client.ts"
            )
            baseline = client.read_text(encoding="utf-8")
            for token in health.LIFECYCLE_CLIENT_PROHIBITED_TOKENS:
                with self.subTest(token=token):
                    client.write_text(f"{baseline}\n// {token}\n", encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(token in finding.detail for finding in findings),
                        msg=token,
                    )

    def test_ui_native_boundary_rejects_lifecycle_client_browser_surfaces(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            client = (
                root
                / "src"
                / "infrastructure"
                / "tauri"
                / "research-knowledge-demo-lifecycle-client.ts"
            )
            baseline = client.read_text(encoding="utf-8")
            prohibited = (
                'globalThis["fetch"]("/future")',
                "new XMLHttpRequest()",
                'new BroadcastChannel("future")',
                "window.postMessage({})",
                "navigator.storage",
                "caches.open()",
                "document.cookie",
                "requestAnimationFrame(() => undefined)",
                'new Worker("future")',
            )
            for source in prohibited:
                with self.subTest(source=source):
                    client.write_text(f"{baseline}\n{source};\n", encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(
                            "prohibited" in finding.detail
                            and (
                                "network, messaging, or storage API" in finding.detail
                                or "timer or background worker" in finding.detail
                            )
                            for finding in findings
                        ),
                        msg=source,
                    )

    def test_ui_native_boundary_rejects_every_lifecycle_rust_boundary_token(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            adapter = (
                root
                / "src-tauri"
                / "src"
                / "research_knowledge_demo_lifecycle_tauri.rs"
            )
            baseline = adapter.read_text(encoding="utf-8")
            for token in health.LIFECYCLE_RUST_PROHIBITED_TOKENS:
                with self.subTest(token=token):
                    adapter.write_text(f"{baseline}\n// {token}\n", encoding="utf-8")

                    findings = health.ui_native_boundary_findings(root)

                    self.assertTrue(
                        any(token in finding.detail for finding in findings),
                        msg=token,
                    )

    def test_ui_native_boundary_rejects_production_development_csp_confusion(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            configuration = root / "src-tauri" / "tauri.conf.json"
            parsed = json.loads(configuration.read_text(encoding="utf-8"))
            parsed["app"]["security"]["csp"]["connect-src"] += " ws://localhost:1420"
            parsed["app"]["security"]["devCsp"]["script-src"] += " 'unsafe-inline'"
            configuration.write_text(json.dumps(parsed), encoding="utf-8")

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("production CSP" in finding.detail for finding in findings))
            self.assertTrue(any("development CSP" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_missing_development_csp(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            configuration = root / "src-tauri" / "tauri.conf.json"
            parsed = json.loads(configuration.read_text(encoding="utf-8"))
            del parsed["app"]["security"]["devCsp"]
            configuration.write_text(json.dumps(parsed), encoding="utf-8")

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("invalid" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_disabled_asset_csp_modification(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            configuration = root / "src-tauri" / "tauri.conf.json"
            parsed = json.loads(configuration.read_text(encoding="utf-8"))
            parsed["app"]["security"]["dangerousDisableAssetCspModification"] = True
            configuration.write_text(json.dumps(parsed), encoding="utf-8")

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("asset CSP modification" in finding.detail for finding in findings))

    def test_ui_native_boundary_rejects_plugins_and_extra_security_keys(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            write_valid_ui_native_boundary(root)
            configuration = root / "src-tauri" / "tauri.conf.json"
            parsed = json.loads(configuration.read_text(encoding="utf-8"))
            parsed["plugins"] = {"shell": {"open": True}}
            parsed["app"]["security"]["headers"] = {"X-Future": "enabled"}
            configuration.write_text(json.dumps(parsed), encoding="utf-8")

            findings = health.ui_native_boundary_findings(root)

            self.assertTrue(any("plugin configuration" in finding.detail for finding in findings))
            self.assertTrue(any("security configuration keys" in finding.detail for finding in findings))

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
