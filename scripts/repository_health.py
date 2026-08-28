#!/usr/bin/env python3
"""Read-only repository health checks used locally and in GitHub Actions."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Sequence
from urllib.parse import unquote, urlsplit

MAX_TEXT_BYTES = 2 * 1024 * 1024

MARKDOWN_LINK = re.compile(r"!?\[[^\]]*\]\((?P<target>[^)]+)\)")
MARKDOWN_REFERENCE = re.compile(r"^\s*\[[^\]]+\]:\s*(?P<target>\S+)", re.MULTILINE)
HTML_REFERENCE = re.compile(r"\b(?:href|src)=[\"'](?P<target>[^\"']+)[\"']", re.IGNORECASE)
NPM_RUN_COMMAND = re.compile(r"\bnpm\s+run\s+(?P<script>[A-Za-z0-9:_-]+)")
ACTION_USE = re.compile(
    r"^\s*(?:-\s*)?uses:\s*(?P<reference>\S+)(?:\s+#.*)?$", re.MULTILINE
)
IMMUTABLE_ACTION = re.compile(r"^[^@\s]+@[0-9a-fA-F]{40}$")
IMMUTABLE_CONTAINER = re.compile(r"^docker://[^@\s]+@sha256:[0-9a-fA-F]{64}$")
WRITE_PERMISSION = re.compile(r"^\s*[a-z][a-z-]*:\s*write\s*$", re.MULTILINE)
RUNNER_SELECTOR = re.compile(r"^\s*runs-on:\s*(?P<selector>.+?)\s*$", re.MULTILINE)
PROMPT_PLACEHOLDER = re.compile(r"\{\{(?P<name>[A-Z][A-Z0-9_]*)\}\}")
PROMPT_PATH_REFERENCE = re.compile(r"\bprompts/[A-Za-z0-9_./-]+\.md\b")
REQUIREMENT_IDENTIFIER = re.compile(r"^\s*- \*\*(?P<identifier>FR-[0-9]+[A-Z]?)\*\*:", re.MULTILINE)
EXPECTED_WORKFLOWS = frozenset({"ci.yml", "documentation.yml"})
LINUX_RUNNER_SELECTOR = "[self-hosted, Linux, X64, cortexa-ci]"
MACOS_RUNNER_SELECTOR = "[self-hosted, macOS, X64, cortexa-ci]"
TRUSTED_WORKFLOW_BRANCHES = (
    '- "codex/**"',
    '- "feature/**"',
    '- "fix/**"',
    '- "refactor/**"',
    '- "meta/**"',
    '- "phase*/**"',
)
REQUIRED_PROMPT_METADATA = (
    "Category",
    "Purpose",
    "Use when",
    "Do not use when",
    "Required inputs",
    "Expected outputs",
    "Related skills",
    "Related prompts",
    "Last reviewed",
)
ACTIVE_PROMPT_REFERENCE_PATHS = (
    "AGENTS.md",
    "ASSISTANT_USAGE.md",
    "CODE_REVIEW.md",
    "CONTRIBUTING.md",
    "ENGINEERING_GUIDE.md",
    "README.md",
    "SECURITY.md",
    "TESTING_GUIDE.md",
)
DOCUMENTATION_TRUTH_MARKERS = (
    (
        "PRODUCT_REQUIREMENTS.md",
        "matrix is complete and verified",
        "matrix remains pending",
    ),
    (
        "ARCHITECTURE.md",
        "runtime-narrows the app-info response",
        "The app-info response is not yet runtime narrowed",
    ),
    (
        "ARCHITECTURE.md",
        "production CSP excludes development\n  WebSocket sources and inline-script execution",
        "production CSP retains the\n  development `ws://localhost:1420` allowance",
    ),
)
EXPECTED_TAURI_IMPORTS = {
    "src/infrastructure/tauri/app-info-client.ts": (
        'import { invoke } from "@tauri-apps/api/core";'
    ),
    "src/infrastructure/tauri/menu-route-client.ts": (
        'import { listen, type UnlistenFn } from "@tauri-apps/api/event";'
    ),
    "src/infrastructure/tauri/research-knowledge-demo-projection-client.ts": (
        'import { invoke } from "@tauri-apps/api/core";'
    ),
}
EXPECTED_TAURI_INVOKES = {
    "src/infrastructure/tauri/app-info-client.ts": 'invoke<unknown>("get_app_info")',
    "src/infrastructure/tauri/research-knowledge-demo-projection-client.ts": (
        'invoke<unknown>("get_research_knowledge_demo_projection")'
    ),
}
COMMAND_CENTER_PROHIBITED_TOKENS = (
    "@tauri-apps/api",
    "fetch(",
    "WebSocket",
    "EventSource",
    "localStorage",
    "sessionStorage",
    "indexedDB",
)
EXPECTED_CAPABILITY_FILES = frozenset({"default.json"})
EXPECTED_CAPABILITY_CONFIGURATION = {
    "$schema": "../gen/schemas/desktop-schema.json",
    "identifier": "main-window",
    "description": "Minimum Tauri core capability for the main application window.",
    "windows": ["main"],
    "permissions": ["core:default"],
}
EXPECTED_INVOKE_HANDLER = (
    ".invoke_handler(tauri::generate_handler!["
    "app_info::get_app_info,"
    "research_knowledge_demo_projection::get_research_knowledge_demo_projection"
    "])"
)
PROJECTION_RUST_PROHIBITED_TOKENS = (
    "AgentOrchestrator",
    "AgentRuntime",
    "crate::agent",
    "crate::approvals",
    "crate::audit",
    "crate::credentials",
    "crate::documents",
    "crate::memory",
    "crate::policy",
    "crate::storage",
    "crate::tools",
    "reqwest",
    "rusqlite",
    "serde::Deserialize",
    "std::fs",
    "std::net",
    "std::process",
    "tauri::State",
    "tokio",
)
PROJECTION_CLIENT_PROHIBITED_TOKENS = (
    "EventSource",
    "WebSocket",
    "emit(",
    "fetch(",
    "indexedDB",
    "listen(",
    "localStorage",
    "sessionStorage",
)
EXPECTED_TAURI_CSP = {
    "default-src": "'self'",
    "connect-src": "'self' ipc: http://ipc.localhost",
    "font-src": "'self' data:",
    "img-src": "'self' asset: http://asset.localhost data:",
    "script-src": "'self'",
    "style-src": "'self' 'unsafe-inline'",
}
EXPECTED_TAURI_DEV_CSP = {
    "default-src": "'self'",
    "connect-src": "'self' ipc: http://ipc.localhost ws://localhost:1420",
    "font-src": "'self' data:",
    "img-src": "'self' asset: http://asset.localhost data:",
    "script-src": "'self'",
    "style-src": "'self' 'unsafe-inline'",
}

SECRET_PATTERNS = (
    ("private-key", re.compile(r"-{5}BEGIN (?:EC |OPENSSH |RSA )?PRIVATE KEY-{5}")),
    ("github-token", re.compile(r"\bgh[pousr]_[A-Za-z0-9]{36,}\b")),
    ("openai-key", re.compile(r"\bsk-(?:proj-)?[A-Za-z0-9_-]{32,}\b")),
    ("aws-access-key", re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b")),
    ("slack-token", re.compile(r"\bxox[baprs]-[A-Za-z0-9-]{20,}\b")),
)

GENERATED_TOP_LEVEL_DIRECTORIES = frozenset(
    {".next", "build", "coverage", "dist", "node_modules", "out"}
)
GENERATED_SUFFIXES = (
    ".bak",
    ".backup",
    ".db",
    ".db-shm",
    ".db-wal",
    ".log",
    ".sqlite",
    ".sqlite-shm",
    ".sqlite-wal",
    ".sqlite3",
    ".sqlite3-shm",
    ".sqlite3-wal",
)
LICENSE_EVIDENCE = (
    "LICENSE",
    "LICENSE.md",
    "COPYING",
    "docs/github/LICENSING.md",
)
AUTHORITATIVE_COMMAND_DOCUMENTS = (
    "AGENTS.md",
    "CODE_REVIEW.md",
    "CONTRIBUTING.md",
    "ENGINEERING_GUIDE.md",
    "README.md",
    "RELEASE_CHECKLIST.md",
    "TESTING_GUIDE.md",
)
REQUIRED_SCRIPTS = frozenset(
    {
        "build",
        "build:frontend",
        "docs:check",
        "format:check",
        "format:frontend",
        "lint",
        "repository:check",
        "security:scan",
        "tauri",
        "test",
        "test:frontend",
        "test:hooks",
        "test:integration",
        "test:repository",
        "test:unit",
        "typecheck",
        "verify",
    }
)


@dataclass(frozen=True)
class Finding:
    check: str
    path: str
    detail: str
    line: int | None = None

    def render(self) -> str:
        location = self.path if self.line is None else f"{self.path}:{self.line}"
        return f"{self.check}: {location}: {self.detail}"


def run_git(root: Path, *arguments: str) -> bytes:
    result = subprocess.run(
        ["git", *arguments],
        cwd=root,
        check=False,
        capture_output=True,
    )
    if result.returncode != 0:
        raise RuntimeError("Git repository inspection failed")
    return result.stdout


def repository_root(cwd: Path) -> Path:
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        cwd=cwd,
        check=False,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError("Working directory is not inside a Git repository")
    return Path(result.stdout.strip()).resolve(strict=True)


def repository_paths(root: Path) -> tuple[str, ...]:
    payload = run_git(
        root,
        "ls-files",
        "--cached",
        "--others",
        "--exclude-standard",
        "-z",
    )
    paths: list[str] = []
    for raw_path in payload.split(b"\0"):
        if not raw_path:
            continue
        relative_path = raw_path.decode("utf-8")
        path = Path(relative_path)
        if path.is_absolute() or ".." in path.parts or ".git" in path.parts:
            raise RuntimeError("Git returned an unsafe repository path")
        if (root / path).is_file():
            paths.append(relative_path)
    return tuple(sorted(set(paths)))


def read_text(path: Path) -> str | None:
    try:
        payload = path.read_bytes()
    except OSError as error:
        raise RuntimeError(f"Repository file could not be read: {path}") from error
    if len(payload) > MAX_TEXT_BYTES or b"\0" in payload:
        return None
    try:
        return payload.decode("utf-8")
    except UnicodeDecodeError:
        return None


def without_fenced_code(text: str) -> str:
    output: list[str] = []
    fence: str | None = None
    for line in text.splitlines(keepends=True):
        stripped = line.lstrip()
        marker = "```" if stripped.startswith("```") else "~~~" if stripped.startswith("~~~") else None
        if marker is not None:
            fence = None if fence == marker else marker if fence is None else fence
            output.append("\n")
        elif fence is None:
            output.append(line)
        else:
            output.append("\n")
    return "".join(output)


def normalize_link_target(raw_target: str) -> str:
    target = raw_target.strip()
    if target.startswith("<") and ">" in target:
        return target[1 : target.index(">")]
    if " " in target or "\t" in target:
        return target.split(maxsplit=1)[0]
    return target


def local_link_target(source: Path, root: Path, raw_target: str) -> Path | None:
    root = root.resolve(strict=True)
    source = source.resolve(strict=True)
    target = normalize_link_target(raw_target)
    if not target or target.startswith("#"):
        return None
    parsed = urlsplit(target)
    if parsed.scheme or parsed.netloc:
        return None
    decoded_path = unquote(parsed.path)
    if not decoded_path:
        return None
    candidate = root / decoded_path.lstrip("/") if decoded_path.startswith("/") else source.parent / decoded_path
    try:
        resolved = candidate.resolve(strict=False)
        resolved.relative_to(root)
    except (OSError, ValueError):
        return Path("/__outside_repository__")
    return resolved


def link_findings(root: Path, paths: Sequence[str]) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    for relative_path in paths:
        if not relative_path.lower().endswith(".md"):
            continue
        source = root / relative_path
        text = read_text(source)
        if text is None:
            continue
        inspected = without_fenced_code(text)
        matches = [
            *MARKDOWN_LINK.finditer(inspected),
            *MARKDOWN_REFERENCE.finditer(inspected),
            *HTML_REFERENCE.finditer(inspected),
        ]
        for match in matches:
            raw_target = match.group("target")
            target = local_link_target(source, root, raw_target)
            if target is None or target.exists():
                continue
            line = inspected.count("\n", 0, match.start()) + 1
            findings.append(
                Finding("links", relative_path, f"missing local target {normalize_link_target(raw_target)!r}", line)
            )
    return tuple(findings)


def secret_findings(root: Path, paths: Sequence[str]) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    for relative_path in paths:
        text = read_text(root / relative_path)
        if text is None:
            continue
        for name, pattern in SECRET_PATTERNS:
            for match in pattern.finditer(text):
                line = text.count("\n", 0, match.start()) + 1
                findings.append(Finding("secrets", relative_path, f"matched {name} pattern", line))
    return tuple(findings)


def generated_path_findings(paths: Sequence[str]) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    for relative_path in paths:
        path = Path(relative_path)
        lower_parts = tuple(part.lower() for part in path.parts)
        lower_name = path.name.lower()
        is_environment = lower_name == ".env" or (
            lower_name.startswith(".env.") and lower_name != ".env.example"
        )
        is_generated_directory = bool(lower_parts) and lower_parts[0] in GENERATED_TOP_LEVEL_DIRECTORIES
        is_tauri_target = len(lower_parts) >= 2 and lower_parts[:2] == ("src-tauri", "target")
        is_root_backup = bool(lower_parts) and lower_parts[0] == "backups"
        if (
            lower_name == ".ds_store"
            or is_environment
            or is_generated_directory
            or is_tauri_target
            or is_root_backup
            or lower_name.endswith(GENERATED_SUFFIXES)
        ):
            findings.append(Finding("generated", relative_path, "generated or local-only path is tracked"))
    return tuple(findings)


def license_findings(root: Path) -> tuple[Finding, ...]:
    for relative_path in LICENSE_EVIDENCE:
        path = root / relative_path
        if path.is_file() and path.stat().st_size > 0:
            return ()
    return (Finding("license", ".", "no license file or licensing decision record exists"),)


def command_findings(root: Path) -> tuple[Finding, ...]:
    package_path = root / "package.json"
    try:
        package = json.loads(package_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise RuntimeError("package.json could not be parsed") from error
    scripts = package.get("scripts")
    if not isinstance(scripts, dict) or not all(isinstance(name, str) for name in scripts):
        raise RuntimeError("package.json scripts are invalid")

    findings = [
        Finding("commands", "package.json", f"required npm script {name!r} is missing")
        for name in sorted(REQUIRED_SCRIPTS - scripts.keys())
    ]
    for relative_path in AUTHORITATIVE_COMMAND_DOCUMENTS:
        path = root / relative_path
        if not path.is_file():
            findings.append(Finding("commands", relative_path, "authoritative command document is missing"))
            continue
        text = read_text(path)
        if text is None:
            findings.append(Finding("commands", relative_path, "authoritative command document is not UTF-8 text"))
            continue
        for match in NPM_RUN_COMMAND.finditer(text):
            script = match.group("script")
            if script not in scripts:
                line = text.count("\n", 0, match.start()) + 1
                findings.append(Finding("commands", relative_path, f"documents missing npm script {script!r}", line))
    return tuple(findings)


def prompt_findings(root: Path) -> tuple[Finding, ...]:
    prompt_root = root / "prompts"
    if not prompt_root.is_dir():
        return (Finding("prompts", "prompts", "prompt library is missing"),)

    findings: list[Finding] = []
    prompt_paths = tuple(
        path
        for path in sorted(prompt_root.rglob("*.md"))
        if path != prompt_root / "README.md"
    )
    if not prompt_paths:
        findings.append(Finding("prompts", "prompts", "no reusable prompts exist"))

    for path in prompt_paths:
        relative_path = path.relative_to(root).as_posix()
        text = read_text(path)
        if text is None:
            findings.append(Finding("prompts", relative_path, "prompt is not UTF-8 text"))
            continue
        prompt_offset = text.find("\n## Prompt")
        template_offset = text.find("\n## Template")
        body_offsets = tuple(offset for offset in (prompt_offset, template_offset) if offset >= 0)
        if not body_offsets:
            findings.append(Finding("prompts", relative_path, "prompt body heading is missing"))
            metadata_text = text
        else:
            metadata_text = text[: min(body_offsets)]

        for name in REQUIRED_PROMPT_METADATA:
            marker = f"- **{name}:**"
            if marker not in metadata_text:
                findings.append(
                    Finding("prompts", relative_path, f"required metadata {name!r} is missing")
                )

        required_inputs = next(
            (
                line
                for line in metadata_text.splitlines()
                if line.startswith("- **Required inputs:**")
            ),
            "",
        )
        placeholders = {match.group("name") for match in PROMPT_PLACEHOLDER.finditer(text)}
        undeclared = tuple(
            name for name in sorted(placeholders) if f"{{{{{name}}}}}" not in required_inputs
        )
        if undeclared:
            findings.append(
                Finding(
                    "prompts",
                    relative_path,
                    f"placeholders are not declared as required inputs: {', '.join(undeclared)}",
                )
            )

    active_paths = (*ACTIVE_PROMPT_REFERENCE_PATHS, "prompts/README.md")
    for relative_path in active_paths:
        path = root / relative_path
        if not path.is_file():
            continue
        text = read_text(path)
        if text is None:
            continue
        for match in PROMPT_PATH_REFERENCE.finditer(without_fenced_code(text)):
            target = root / match.group(0)
            if target.is_file():
                continue
            line = text.count("\n", 0, match.start()) + 1
            findings.append(
                Finding("prompts", relative_path, f"stale prompt path {match.group(0)!r}", line)
            )
    return tuple(findings)


def documentation_truth_findings(root: Path) -> tuple[Finding, ...]:
    product_requirements = root / "PRODUCT_REQUIREMENTS.md"
    text = read_text(product_requirements)
    if text is None:
        return (
            Finding("documentation", "PRODUCT_REQUIREMENTS.md", "requirements document is not UTF-8 text"),
        )

    findings: list[Finding] = []
    seen_identifiers: set[str] = set()
    for match in REQUIREMENT_IDENTIFIER.finditer(text):
        identifier = match.group("identifier")
        if identifier in seen_identifiers:
            line = text.count("\n", 0, match.start()) + 1
            findings.append(
                Finding("documentation", "PRODUCT_REQUIREMENTS.md", f"duplicate requirement identifier {identifier!r}", line)
            )
        seen_identifiers.add(identifier)

    for relative_path, required, prohibited in DOCUMENTATION_TRUTH_MARKERS:
        path = root / relative_path
        document_text = read_text(path)
        if document_text is None:
            findings.append(Finding("documentation", relative_path, "required current-state document is not UTF-8 text"))
            continue
        if required not in document_text:
            findings.append(Finding("documentation", relative_path, f"required current-state marker is missing: {required!r}"))
        if prohibited and prohibited in document_text:
            findings.append(Finding("documentation", relative_path, f"stale current-state marker is present: {prohibited!r}"))
    return tuple(findings)


def ui_native_boundary_findings(root: Path) -> tuple[Finding, ...]:
    findings: list[Finding] = []
    source_root = root / "src"
    for path in sorted((*source_root.rglob("*.ts"), *source_root.rglob("*.tsx"))):
        if path.name.endswith((".test.ts", ".test.tsx")):
            continue
        relative_path = path.relative_to(root).as_posix()
        text = read_text(path)
        if text is None:
            findings.append(Finding("ui-native-boundary", relative_path, "source file is not UTF-8 text"))
            continue
        if "@tauri-apps/api" in text:
            expected_import = EXPECTED_TAURI_IMPORTS.get(relative_path)
            if (
                expected_import is None
                or text.count("@tauri-apps/api") != 1
                or text.count(expected_import) != 1
            ):
                findings.append(Finding("ui-native-boundary", relative_path, "Tauri API import is outside the exact allowlist"))
        if relative_path.startswith("src/features/command-center/"):
            for token in COMMAND_CENTER_PROHIBITED_TOKENS:
                if token in text:
                    findings.append(Finding("ui-native-boundary", relative_path, f"Command Center contains prohibited boundary token {token!r}"))

    for relative_path, expected_call in EXPECTED_TAURI_INVOKES.items():
        client_text = read_text(root / relative_path)
        if client_text is None:
            findings.append(
                Finding(
                    "ui-native-boundary",
                    relative_path,
                    "required Tauri invoke client is not UTF-8 text",
                )
            )
            continue
        invoke_calls = re.findall(r"\binvoke\s*(?:<[^>]+>)?\s*\(", client_text)
        if len(invoke_calls) != 1 or client_text.count(expected_call) != 1:
            findings.append(
                Finding(
                    "ui-native-boundary",
                    relative_path,
                    "Tauri invoke is not the exact no-argument command allowlist",
                )
            )

    projection_client_path = "src/infrastructure/tauri/research-knowledge-demo-projection-client.ts"
    projection_client_text = read_text(root / projection_client_path)
    if projection_client_text is not None:
        for token in PROJECTION_CLIENT_PROHIBITED_TOKENS:
            if token in projection_client_text:
                findings.append(
                    Finding(
                        "ui-native-boundary",
                        projection_client_path,
                        f"projection client contains prohibited boundary token {token!r}",
                    )
                )

    menu_route_path = "src/infrastructure/tauri/menu-route-client.ts"
    menu_route_text = read_text(root / menu_route_path)
    if menu_route_text is None:
        findings.append(
            Finding(
                "ui-native-boundary",
                menu_route_path,
                "menu event client is not UTF-8 text",
            )
        )
    else:
        listener_calls = re.findall(r"\blisten\s*(?:<[^>]+>)?\s*\(", menu_route_text)
        if (
            len(listener_calls) != 1
            or 'export const ASSISTANT_MENU_ROUTE_EVENT = "assistant-menu-route";' not in menu_route_text
            or "listen<unknown>(ASSISTANT_MENU_ROUTE_EVENT," not in menu_route_text
        ):
            findings.append(
                Finding(
                    "ui-native-boundary",
                    menu_route_path,
                    "menu listener differs from the sole exact event allowlist",
                )
            )

    lib_path = root / "src-tauri" / "src" / "lib.rs"
    lib_text = read_text(lib_path)
    if lib_text is None:
        findings.append(Finding("ui-native-boundary", "src-tauri/src/lib.rs", "Tauri entrypoint is not UTF-8 text"))
    else:
        normalized_lib = re.sub(r"\s+", "", lib_text)
        if (
            normalized_lib.count(".invoke_handler(") != 1
            or normalized_lib.count("generate_handler![") != 1
            or EXPECTED_INVOKE_HANDLER not in normalized_lib
        ):
            findings.append(Finding("ui-native-boundary", "src-tauri/src/lib.rs", "invoke handler is not the exact two-command allowlist"))

    projection_rust_path = "src-tauri/src/research_knowledge_demo_projection.rs"
    projection_rust_text = read_text(root / projection_rust_path)
    if projection_rust_text is None:
        findings.append(
            Finding(
                "ui-native-boundary",
                projection_rust_path,
                "projection command source is not UTF-8 text",
            )
        )
    else:
        command_signature = re.compile(
            r"#\[tauri::command\]\s*"
            r"pub\(crate\)\s+fn\s+get_research_knowledge_demo_projection\s*"
            r"\(\s*\)\s*->\s*Result\s*<\s*ResearchKnowledgeDemoProjection\s*,\s*"
            r"ResearchKnowledgeDemoProjectionError\s*>",
            re.MULTILINE,
        )
        if command_signature.search(projection_rust_text) is None:
            findings.append(
                Finding(
                    "ui-native-boundary",
                    projection_rust_path,
                    "projection command is not the exact zero-argument closed-result signature",
                )
            )
        for token in PROJECTION_RUST_PROHIBITED_TOKENS:
            if token in projection_rust_text:
                findings.append(
                    Finding(
                        "ui-native-boundary",
                        projection_rust_path,
                        f"projection command contains prohibited boundary token {token!r}",
                    )
                )

    rust_emitters: list[str] = []
    rust_source_root = root / "src-tauri" / "src"
    for rust_path in sorted(rust_source_root.rglob("*.rs")):
        rust_text = read_text(rust_path)
        if rust_text is None:
            continue
        rust_emitters.extend(
            rust_path.relative_to(root).as_posix()
            for _ in re.finditer(
                r"\.\s*(?:emit|emit_to|emit_filter|emit_str|emit_str_to|emit_str_filter)\s*\(",
                rust_text,
            )
        )
    if rust_emitters != ["src-tauri/src/menu_bar/tauri_adapter.rs"]:
        findings.append(
            Finding(
                "ui-native-boundary",
                "src-tauri/src",
                "Rust event emission differs from the sole menu-route allowlist",
            )
        )
    menu_action_path = "src-tauri/src/menu_bar/action.rs"
    menu_action_text = read_text(root / menu_action_path)
    menu_adapter_path = "src-tauri/src/menu_bar/tauri_adapter.rs"
    menu_adapter_text = read_text(root / menu_adapter_path)
    if (
        menu_action_text is None
        or 'pub const MENU_ROUTE_EVENT: &str = "assistant-menu-route";' not in menu_action_text
        or menu_adapter_text is None
        or ".emit(MENU_ROUTE_EVENT," not in menu_adapter_text
    ):
        findings.append(
            Finding(
                "ui-native-boundary",
                "src-tauri/src/menu_bar",
                "Rust menu-route event name or emitter differs from the exact baseline",
            )
        )

    capability_directory = root / "src-tauri" / "capabilities"
    capability_files = {
        path.relative_to(capability_directory).as_posix()
        for path in capability_directory.rglob("*")
        if path.is_file()
    }
    if capability_files != EXPECTED_CAPABILITY_FILES:
        findings.append(Finding("ui-native-boundary", "src-tauri/capabilities", "capability file allowlist changed"))
    capability_path = capability_directory / "default.json"
    try:
        capability = json.loads(capability_path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError):
        findings.append(Finding("ui-native-boundary", "src-tauri/capabilities/default.json", "capability configuration is invalid"))
    else:
        if capability != EXPECTED_CAPABILITY_CONFIGURATION:
            findings.append(Finding("ui-native-boundary", "src-tauri/capabilities/default.json", "capability configuration differs from the complete exact baseline"))

    configuration_path = root / "src-tauri" / "tauri.conf.json"
    try:
        configuration = json.loads(configuration_path.read_text(encoding="utf-8"))
        security = configuration["app"]["security"]
        csp = security["csp"]
        dev_csp = security["devCsp"]
    except (OSError, UnicodeError, json.JSONDecodeError, KeyError, TypeError):
        findings.append(Finding("ui-native-boundary", "src-tauri/tauri.conf.json", "Tauri configuration or CSP is invalid"))
    else:
        if "plugins" in configuration:
            findings.append(Finding("ui-native-boundary", "src-tauri/tauri.conf.json", "Tauri plugin configuration is outside the reviewed baseline"))
        if set(security) != {"csp", "devCsp"}:
            findings.append(Finding("ui-native-boundary", "src-tauri/tauri.conf.json", "Tauri security configuration keys differ from the complete exact baseline"))
        if csp != EXPECTED_TAURI_CSP:
            findings.append(Finding("ui-native-boundary", "src-tauri/tauri.conf.json", "production CSP differs from the reviewed F-07 baseline"))
        if dev_csp != EXPECTED_TAURI_DEV_CSP:
            findings.append(Finding("ui-native-boundary", "src-tauri/tauri.conf.json", "development CSP differs from the reviewed F-07 baseline"))
        if security.get("dangerousDisableAssetCspModification", False) is not False:
            findings.append(Finding("ui-native-boundary", "src-tauri/tauri.conf.json", "Tauri asset CSP modification is disabled"))
    return tuple(findings)


def workflow_findings(root: Path) -> tuple[Finding, ...]:
    workflow_directory = root / ".github" / "workflows"
    if not workflow_directory.is_dir():
        return (Finding("workflows", ".github/workflows", "workflow directory is missing"),)

    findings: list[Finding] = []
    workflows = sorted((*workflow_directory.glob("*.yml"), *workflow_directory.glob("*.yaml")))
    if not workflows:
        return (Finding("workflows", ".github/workflows", "no workflow files exist"),)
    workflow_names = {path.name for path in workflows}
    for name in sorted(EXPECTED_WORKFLOWS - workflow_names):
        findings.append(Finding("workflows", f".github/workflows/{name}", "required workflow is missing"))
    for name in sorted(workflow_names - EXPECTED_WORKFLOWS):
        findings.append(Finding("workflows", f".github/workflows/{name}", "unexpected workflow is present"))
    for path in workflows:
        relative_path = path.relative_to(root).as_posix()
        text = read_text(path)
        if text is None:
            findings.append(Finding("workflows", relative_path, "workflow is not UTF-8 text"))
            continue
        for match in ACTION_USE.finditer(text):
            reference = match.group("reference")
            if reference.startswith("./"):
                continue
            if not IMMUTABLE_ACTION.fullmatch(reference) and not IMMUTABLE_CONTAINER.fullmatch(reference):
                line = text.count("\n", 0, match.start()) + 1
                findings.append(Finding("workflows", relative_path, "action reference is not pinned to an immutable digest", line))
        prohibited = (
            ("pull_request_target:", "pull_request_target is prohibited"),
            (
                "pull_request:",
                "persistent self-hosted workflows must not subscribe to pull_request",
            ),
            ("persist-credentials: true", "checkout credentials must not persist"),
            ("${{ secrets.", "workflow must not require repository secrets"),
            ("git commit ", "workflow must not create commits"),
            ("git push", "workflow must not push"),
            ("sudo ", "self-hosted workflows must not provision the runner with sudo"),
            ("npm publish", "workflow must not publish packages"),
            ("cargo publish", "workflow must not publish crates"),
            ("continue-on-error:", "mandatory workflow checks must not continue on error"),
        )
        for needle, detail in prohibited:
            offset = text.find(needle)
            if offset >= 0:
                findings.append(Finding("workflows", relative_path, detail, text.count("\n", 0, offset) + 1))
        for match in WRITE_PERMISSION.finditer(text):
            findings.append(Finding("workflows", relative_path, "write workflow permission is prohibited", text.count("\n", 0, match.start()) + 1))
        if "permissions:\n  contents: read" not in text:
            findings.append(Finding("workflows", relative_path, "top-level contents: read permission is required"))
        selectors = tuple(match.group("selector") for match in RUNNER_SELECTOR.finditer(text))
        for match in RUNNER_SELECTOR.finditer(text):
            if match.group("selector") not in {
                LINUX_RUNNER_SELECTOR,
                MACOS_RUNNER_SELECTOR,
            }:
                findings.append(
                    Finding(
                        "workflows",
                        relative_path,
                        "runner selector is not an approved dedicated Cortexa runner",
                        text.count("\n", 0, match.start()) + 1,
                    )
                )

        required_common = (
            "push:",
            "workflow_dispatch:",
            "paths:",
            "concurrency:",
            "cancel-in-progress: true",
        )
        missing_common = tuple(value for value in required_common if value not in text)
        if missing_common:
            findings.append(
                Finding(
                    "workflows",
                    relative_path,
                    f"required trigger or concurrency policy is missing: {', '.join(missing_common)}",
                )
            )
        required_branches = ("branches:\n      - main", *TRUSTED_WORKFLOW_BRANCHES)
        missing_branches = tuple(value for value in required_branches if value not in text)
        if missing_branches:
            findings.append(
                Finding(
                    "workflows",
                    relative_path,
                    f"trusted push branch allowlist is incomplete: {', '.join(missing_branches)}",
                )
            )

        if path.name == "ci.yml":
            required_ci = (
                "schedule:",
                "scripts/ci_change_scope.py",
                "Frontend validation",
                "Linux Rust validation",
                "Target-Mac Rust validation",
                "Dependency and secret audit",
                LINUX_RUNNER_SELECTOR,
                MACOS_RUNNER_SELECTOR,
                ".codex/hooks/tests",
                "scripts/tests",
                '"src/**"',
                '"src-tauri/**"',
                '"assets/branding/**"',
                '"package-lock.json"',
                '".prettierrc*"',
                '".codex/**"',
                '".github/workflows/ci.yml"',
                '".github/workflows/documentation.yml"',
            )
            missing_ci = tuple(value for value in required_ci if value not in text)
            if missing_ci:
                findings.append(
                    Finding(
                        "workflows",
                        relative_path,
                        f"application CI policy is incomplete: {', '.join(missing_ci)}",
                    )
                )
        elif path.name == "documentation.yml":
            required_documentation = (
                '"**/*.md"',
                '"LICENSE*"',
                '"prompts/**"',
                '".agents/**"',
                "npm run docs:check",
                "npm run repository:check",
                LINUX_RUNNER_SELECTOR,
            )
            missing_documentation = tuple(
                value for value in required_documentation if value not in text
            )
            if missing_documentation:
                findings.append(
                    Finding(
                        "workflows",
                        relative_path,
                        f"documentation CI policy is incomplete: {', '.join(missing_documentation)}",
                    )
                )
            if selectors and any(
                selector != LINUX_RUNNER_SELECTOR for selector in selectors
            ):
                findings.append(
                    Finding(
                        "workflows",
                        relative_path,
                        "documentation jobs must use the dedicated Linux runner",
                    )
                )
    return tuple(findings)


def checks_for(command: str) -> tuple[str, ...]:
    if command == "all":
        return ("links", "secrets", "generated", "license", "commands", "prompts", "documentation", "ui-native-boundary", "workflows")
    return (command,)


def run_checks(root: Path, command: str) -> tuple[Finding, ...]:
    paths = repository_paths(root)
    findings: list[Finding] = []
    for check in checks_for(command):
        if check == "links":
            findings.extend(link_findings(root, paths))
        elif check == "secrets":
            findings.extend(secret_findings(root, paths))
        elif check == "generated":
            findings.extend(generated_path_findings(paths))
        elif check == "license":
            findings.extend(license_findings(root))
        elif check == "commands":
            findings.extend(command_findings(root))
        elif check == "prompts":
            findings.extend(prompt_findings(root))
        elif check == "documentation":
            findings.extend(documentation_truth_findings(root))
        elif check == "ui-native-boundary":
            findings.extend(ui_native_boundary_findings(root))
        elif check == "workflows":
            findings.extend(workflow_findings(root))
        else:
            raise RuntimeError(f"Unknown check: {check}")
    return tuple(findings)


def parse_arguments(arguments: Sequence[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "check",
        choices=("all", "links", "secrets", "generated", "license", "commands", "prompts", "documentation", "ui-native-boundary", "workflows"),
    )
    return parser.parse_args(arguments)


def main(arguments: Sequence[str] | None = None) -> int:
    parsed = parse_arguments(sys.argv[1:] if arguments is None else arguments)
    try:
        root = repository_root(Path.cwd())
        findings = run_checks(root, parsed.check)
    except (OSError, RuntimeError, UnicodeError) as error:
        print(f"repository-health: ERROR: {error}", file=sys.stderr)
        return 2
    if findings:
        for finding in findings:
            print(finding.render(), file=sys.stderr)
        print(f"repository-health: FAIL ({len(findings)} finding(s))", file=sys.stderr)
        return 1
    print(f"repository-health: PASS ({parsed.check})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
