import { invoke, isTauri } from "@tauri-apps/api/core";
import type { Source } from "./collaboration-client";
export interface KnowledgeLink {
  readonly title: string;
  readonly label: string;
  readonly context: string;
  readonly targetId: number | null;
  readonly status: "resolved" | "missing" | "ambiguous" | "removed";
}
export interface KnowledgeVersion {
  readonly links?: readonly KnowledgeLink[];
  readonly id: number;
  readonly title: string;
  readonly content: string;
  readonly hash: string;
  readonly format: string;
  readonly createdMs: number;
}
export interface KnowledgeItem {
  readonly id: number;
  readonly kind: "imported" | "owner_note" | "generated_draft";
  readonly versions: readonly KnowledgeVersion[];
}
export interface Origin {
  readonly documentId: number;
  readonly version: number;
  readonly passageId: number;
  readonly title: string;
  readonly hash: string;
  readonly startLine: number;
  readonly endLine: number;
}
export interface NoteInput {
  readonly id: number | null;
  readonly expectedVersion: number | null;
  readonly title: string;
  readonly content: string;
  readonly draft: boolean;
}
export interface Draft {
  readonly title: string;
  readonly content: string;
}
export interface KnowledgeClient {
  readonly list: () => Promise<readonly KnowledgeItem[]>;
  readonly import: (id?: number, expectedVersion?: number) => Promise<KnowledgeItem | null>;
  readonly save: (input: NoteInput) => Promise<KnowledgeItem>;
  readonly remove: (id: number, version: number) => Promise<void>;
  readonly search: (text: string) => Promise<readonly { readonly source: Source }[]>;
  readonly select: (id: number, version: number, passage: number) => Promise<Source>;
  readonly export: (id: number, version: number) => Promise<boolean>;
  readonly exportDraft: (draft: Draft) => Promise<boolean>;
  readonly draft: (roomId: number, runId: string, stageId: string) => Promise<Draft>;
}
function obj(v: unknown): Record<string, unknown> {
  if (!v || typeof v !== "object" || Array.isArray(v)) throw Error("protocol");
  return v as Record<string, unknown>;
}
function num(v: unknown): v is number {
  return typeof v === "number" && Number.isSafeInteger(v) && v >= 0;
}
function str(v: unknown, n: number): v is string {
  return typeof v === "string" && v.length <= n;
}
export function parseOrigin(v: unknown): Origin {
  const o = obj(v);
  if (
    !num(o["documentId"]) ||
    !num(o["version"]) ||
    !num(o["passageId"]) ||
    !str(o["title"], 240) ||
    !str(o["hash"], 64) ||
    !num(o["startLine"]) ||
    !num(o["endLine"])
  )
    throw Error("protocol");
  return v as Origin;
}
export function parseItem(v: unknown): KnowledgeItem {
  const o = obj(v);
  if (
    !num(o["id"]) ||
    !["imported", "owner_note", "generated_draft"].includes(String(o["kind"])) ||
    !Array.isArray(o["versions"]) ||
    o["versions"].length < 1 ||
    o["versions"].length > 8
  )
    throw Error("protocol");
  for (const raw of o["versions"]) {
    const x = obj(raw);
    if (x["links"] !== undefined) {
      if (!Array.isArray(x["links"]) || x["links"].length > 4096) throw Error("protocol");
      for (const rawLink of x["links"]) {
        const l = obj(rawLink);
        if (
          !str(l["title"], 240) ||
          !str(l["label"], 16384) ||
          !str(l["context"], 440) ||
          (l["targetId"] !== null && !num(l["targetId"])) ||
          !["resolved", "missing", "ambiguous", "removed"].includes(String(l["status"]))
        )
          throw Error("protocol");
      }
    }
    if (
      !num(x["id"]) ||
      !str(x["title"], 240) ||
      !str(x["content"], 16384) ||
      !str(x["hash"], 64) ||
      !["md", "txt"].includes(String(x["format"])) ||
      !num(x["createdMs"])
    )
      throw Error("protocol");
  }
  return v as KnowledgeItem;
}
function requireNative(): void {
  if (!isTauri()) throw new Error("native_required");
}
function source(v: unknown): Source {
  const o = obj(v);
  if (!str(o["label"], 32) || !str(o["text"], 16384)) throw Error("protocol");
  parseOrigin(o["origin"]);
  return v as Source;
}
export const knowledgeClient: KnowledgeClient = {
  async list() {
    requireNative();
    const v = await invoke<unknown>("list_knowledge");
    if (!Array.isArray(v) || v.length > 200) throw Error("protocol");
    return v.map(parseItem);
  },
  async import(id, expectedVersion) {
    requireNative();
    const request = { id: id ?? null, expectedVersion: expectedVersion ?? null };
    const v = await invoke<unknown>("import_knowledge", { request });
    return v === null ? null : parseItem(v);
  },
  async save(input) {
    requireNative();
    return parseItem(await invoke<unknown>("save_knowledge", { request: input }));
  },
  async remove(id, version) {
    requireNative();
    await invoke<unknown>("remove_knowledge", { request: { id, version } });
  },
  async search(text) {
    requireNative();
    const v = await invoke<unknown>("search_knowledge", { request: { text } });
    if (!Array.isArray(v) || v.length > 50) throw Error("protocol");
    return v.map((x) => ({ source: source(obj(x)["source"]) }));
  },
  async select(id, version, passage) {
    requireNative();
    const request = { id, version, passage };
    return source(await invoke<unknown>("select_knowledge_source", { request }));
  },
  async export(id, version) {
    requireNative();
    const v = await invoke<unknown>("export_knowledge", { request: { id, version } });
    if (typeof v !== "boolean") throw Error("protocol");
    return v;
  },
  async exportDraft(draft) {
    requireNative();
    const v = await invoke<unknown>("export_knowledge_draft", { request: draft });
    if (typeof v !== "boolean") throw Error("protocol");
    return v;
  },
  async draft(roomId, runId, stageId) {
    requireNative();
    const request = { roomId, runId, stageId };
    const v = await invoke<unknown>("knowledge_draft", { request });
    const d = obj(v);
    if (!str(d["title"], 240) || !str(d["content"], 16384)) throw Error("protocol");
    return v as Draft;
  },
};
export function knowledgeError(e: unknown): string {
  const code = typeof e === "string" ? e : e instanceof Error ? e.message : "";
  return (
    (
      {
        native_required: "Open the native app for the local library and file dialogs.",
        limit:
          "Storage limit reached: 16 KiB/version, 200 items, 8 versions/item or 4 MiB total. Nothing was saved or pruned. Export your work before deciding what to remove.",
        duplicate: "This content is already in the library. No new version was saved.",
        stale_selection:
          "The item changed or was removed. Reload, select its current version and review again.",
        invalid_content:
          "Use a nonempty title (120 characters) and valid text without binary/control characters.",
        invalid_utf8: "The selected file is not valid UTF-8.",
        unsupported_file: "Select a .md or .txt file; exports require .md.",
        unsafe_path: "Linked or unsafe files are not supported. Select an ordinary file.",
        changed_file: "The selected file changed while being read. Select it again explicitly.",
        destination_exists:
          "The destination already exists. Nothing was overwritten; choose a new filename.",
        unavailable: "The selected item or file is unavailable. No automatic retry was made.",
      } as Record<string, string>
    )[code] ?? "The local operation could not complete. No automatic retry was made."
  );
}
