import { App } from "../../src/App";
import { mockIPC } from "@tauri-apps/api/mocks";
import { AGENT_IDS, DEFAULT_BOT_IDENTITY } from "../../src/infrastructure/tauri/agent-chat-client";
import { WORKFLOWS } from "../../src/infrastructure/tauri/collaboration-client";
import { createRoot } from "react-dom/client";
import { useState } from "react";
import {
  KnowledgePage,
  NoteEditor,
  SourceEvidence,
} from "../../src/features/knowledge/KnowledgePage";
import { SourcePicker } from "../../src/features/knowledge/SourcePicker";
import type {
  KnowledgeClient,
  KnowledgeItem,
} from "../../src/infrastructure/tauri/knowledge-client";
import type { Source } from "../../src/infrastructure/tauri/collaboration-client";
import "../../src/styles.css";
const params = new URLSearchParams(location.search);
const sample =
  "---\ntags: [operations]\n---\n# Incident runbook 界\nInvestigate synthetic service availability.\n[[Related note]] <script>never execute</script> ![remote](https://invalid.test/no)\n";
let items: KnowledgeItem[] = params.has("long")
  ? [
      {
        id: 1,
        kind: "imported",
        versions: [
          {
            id: 1,
            title: "界".repeat(120),
            content: sample + "Long Unicode paragraph 界. ".repeat(300),
            hash: "a".repeat(64),
            format: "md",
            createdMs: 1790800000000,
          },
        ],
      },
    ]
  : [];
let serial = items.length;
// Browser-only projection mirrors stable-title binding for presentation fixtures.
// Actual persistence, parsing and removal semantics are tested at the Rust boundary.
const bindings = new Map<string, number>();
function projected(): KnowledgeItem[] {
  return items.map((item) => ({
    ...item,
    versions: item.versions.map((v, n) =>
      n !== item.versions.length - 1
        ? v
        : {
            ...v,
            links: [...v.content.matchAll(/\[\[([^\]\n|]+)(?:\|([^\]\n]+))?\]\]/g)].map((m) => {
              const title = m[1]?.trim() ?? "";
              const key = `${String(item.id)}:${title}`;
              const matches = items.filter((i) => i.versions.at(-1)?.title === title);
              const targetId =
                bindings.get(key) ?? (matches.length === 1 ? matches[0]?.id : undefined) ?? null;
              if (targetId !== null) bindings.set(key, targetId);
              return {
                title,
                label: m[2] ?? title,
                context: m[0],
                targetId,
                status:
                  targetId !== null
                    ? items.some((i) => i.id === targetId)
                      ? "resolved"
                      : "removed"
                    : matches.length > 1
                      ? "ambiguous"
                      : "missing",
              };
            }),
          },
    ),
  }));
}
function source(item: KnowledgeItem): Source {
  const v = item.versions.at(-1);
  if (!v) throw Error("fixture");
  return {
    label: `K${String(item.id)}_V${String(v.id)}_P1`,
    text: v.content.slice(0, 900),
    origin: {
      documentId: item.id,
      version: v.id,
      passageId: 1,
      title: v.title,
      hash: v.hash,
      startLine: 1,
      endLine: 6,
    },
  };
}
const client: KnowledgeClient = {
  list() {
    if (params.has("loading")) return new Promise(() => undefined);
    if (params.has("error")) return Promise.reject(new Error("unavailable"));
    return Promise.resolve(projected());
  },
  import() {
    const id = ++serial;
    const item: KnowledgeItem = {
      id,
      kind: "imported",
      versions: [
        {
          id: 1,
          title: `Synthetic runbook ${String(id)}`,
          content: sample,
          hash: "a".repeat(64),
          format: "md",
          createdMs: 1790800000000,
        },
      ],
    };
    items = [item, ...items];
    return Promise.resolve(item);
  },
  save(input) {
    projected(); // Freeze old titles before a synthetic rename.
    const old = items.find((i) => i.id === input.id);
    const item: KnowledgeItem = {
      id: old?.id ?? ++serial,
      kind: old?.kind ?? (input.draft ? "generated_draft" : "owner_note"),
      versions: [
        ...(old?.versions ?? []),
        {
          id: (old?.versions.length ?? 0) + 1,
          title: input.title,
          content: input.content,
          hash: "b".repeat(64),
          format: "md",
          createdMs: 1790800000000,
        },
      ],
    };
    items = [item, ...items.filter((i) => i.id !== item.id)];
    return Promise.resolve(item);
  },
  remove(id) {
    items = items.filter((i) => i.id !== id);
    return Promise.resolve();
  },
  search(text) {
    return Promise.resolve(
      items
        .filter((i) => i.versions.at(-1)?.content.toLowerCase().includes(text.toLowerCase()))
        .map((i) => ({ source: source(i) })),
    );
  },
  select(id) {
    const item = items.find((i) => i.id === id);
    if (!item) return Promise.reject(new Error("stale_selection"));
    return Promise.resolve(source(item));
  },
  export() {
    return Promise.reject(new Error("destination_exists"));
  },
  exportDraft() {
    return Promise.resolve(true);
  },
  draft() {
    return Promise.resolve({
      title: "Synthetic result",
      content: "# Draft\nUnverified synthetic result\nSource attribution: Runbook v1",
    });
  },
};
export function Harness() {
  const [tab, setTab] = useState("library"),
    [selected, setSelected] = useState<readonly Source[]>([]);
  return (
    <main style={{ maxWidth: 1200, margin: "auto" }}>
      <nav aria-label="QA surfaces">
        <button
          onClick={() => {
            setTab("library");
          }}
        >
          Library surface
        </button>
        <button
          onClick={() => {
            setTab("sources");
          }}
        >
          Workflow source surface
        </button>
        <button
          onClick={() => {
            setTab("draft");
          }}
        >
          Draft surface
        </button>
      </nav>
      {tab === "library" ? (
        <KnowledgePage client={client} />
      ) : tab === "sources" ? (
        <>
          <SourcePicker client={client} value={selected} onChange={setSelected} />
          <SourceEvidence sources={selected} labels={selected.map((s) => s.label)} />
        </>
      ) : (
        <section className="knowledge">
          <NoteEditor
            client={client}
            initial={{
              title: "Operations draft",
              content:
                "# Draft\nUnverified synthesis.\n\n## Source attribution\nRunbook v1: synthetic source.",
            }}
            onSaved={() => {
              setTab("library");
            }}
            onCancel={() => {
              setTab("library");
            }}
          />
        </section>
      )}
    </main>
  );
}
// Actual-shell mode adds no execution route: unsupported IPC, including Start,
// always rejects. The original standalone component scenarios remain unchanged.
if (params.has("shell")) {
  Object.defineProperty(window, "isTauri", { value: true });
  const notice = document.querySelector<HTMLElement>("body > [role=note]");
  if (notice)
    notice.style.cssText = "height:24px;margin:0;font-size:12px;line-height:24px;overflow:hidden";
  document
    .getElementById("root")
    ?.style.setProperty("--app-viewport-height", "calc(100dvh - 24px)");
  const profiles = AGENT_IDS.map((agentId) => ({
    agentId,
    displayName: agentId,
    identity: DEFAULT_BOT_IDENTITY,
    connection: "simulation",
    model: "simulation",
    effort: "default",
    endpoint: "",
    localAuth: false,
    allowUnknownLocalityNotes: false,
    ownerInstructions: "",
    memoryMode: "off",
    note: "",
    revision: 1,
  }));
  mockIPC((command) => {
    if (command === "list_agent_preferences") return profiles;
    if (command === "list_agent_connections")
      return ["simulation", "openai_api", "codex", "anthropic_api", "lm_studio", "ollama"].map(
        (connection) => ({
          connection,
          status: connection === "simulation" ? "ready" : "blocked",
          message: "Synthetic layout fixture only; execution is unavailable.",
        }),
      );
    if (command === "list_knowledge") return items;
    if (command === "list_collaboration_rooms")
      return [{ id: 1, title: "Synthetic layout room", runs: [] }];
    if (command === "prepare_collaboration")
      return {
        previewSerial: 1,
        roomId: 1,
        maximumCalls: 4,
        simulation: true,
        run: {
          id: "room-1/run-1",
          status: "queued",
          sequence: 0,
          error: null,
          input: { workflow: "research", objective: "Synthetic layout only", sources: [] },
          stages: WORKFLOWS.research.map((id, i) => ({
            id: `room-1/run-1/stage/${String(i)}`,
            status: "queued",
            timestamp: 0,
            provisional: "",
            handoff: null,
            input: "",
            participant: { ...profiles.find((p) => p.agentId === id), role: id },
          })),
        },
      };
    if (command === "plugin:event|listen") return 0;
    if (command === "plugin:event|unlisten") return null;
    throw Error("Shell layout fixture blocks execution and unsupported IPC");
  });
}
const root = document.getElementById("root");
if (root) createRoot(root).render(params.has("shell") ? <App /> : <Harness />);
