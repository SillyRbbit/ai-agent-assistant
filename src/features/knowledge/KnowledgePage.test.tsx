import { beforeEach } from "vitest";
beforeEach(() => {
  vi.stubGlobal(
    "ResizeObserver",
    class {
      observe() {
        /* Geometry is covered by the real browser harness. */
      }
      unobserve() {
        /* Geometry is covered by the real browser harness. */
      }
      disconnect() {
        /* Test observer. */
      }
    },
  );
});
import { fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { KnowledgePage, SourceEvidence, NoteEditor } from "./KnowledgePage";
import { SourcePicker } from "./SourcePicker";
import {
  parseItem,
  parseOrigin,
  knowledgeError,
  type KnowledgeClient,
  type KnowledgeItem,
} from "../../infrastructure/tauri/knowledge-client";
import type { Source } from "../../infrastructure/tauri/collaboration-client";
const content =
  "---\ntags: [incident]\n---\n# Runbook 界\n[[Linked note]] <script>unsafe()</script> ![image](https://invalid.test/x)";
const item: KnowledgeItem = {
  id: 1,
  kind: "imported",
  versions: [
    { id: 1, title: "Runbook 界", content, hash: "a".repeat(64), format: "md", createdMs: 1 },
  ],
};
const source: Source = {
  label: "K1_V1_P1",
  text: content,
  origin: {
    documentId: 1,
    version: 1,
    passageId: 1,
    title: "Runbook 界",
    hash: "a".repeat(64),
    startLine: 1,
    endLine: 6,
  },
};
function client(): { -readonly [K in keyof KnowledgeClient]: KnowledgeClient[K] } {
  return {
    list: vi.fn(() => Promise.resolve([item])),
    import: vi.fn(() => Promise.resolve(item)),
    save: vi.fn(() => Promise.resolve(item)),
    remove: vi.fn(() => Promise.resolve(undefined)),
    search: vi.fn(() => Promise.resolve([{ source }])),
    select: vi.fn(() => Promise.resolve(source)),
    export: vi.fn(() => Promise.resolve(true)),
    exportDraft: vi.fn(() => Promise.resolve(true)),
    draft: vi.fn(() => Promise.resolve({ title: "Draft", content })),
  };
}
describe("Knowledge & Documents", () => {
  it("presents asynchronous search results beside the search without replacing the reader", async () => {
    const c = client();
    let finish: (value: readonly { readonly source: Source }[]) => void = () => undefined;
    c.search = vi.fn(
      () =>
        new Promise<readonly { readonly source: Source }[]>((resolve) => {
          finish = resolve;
        }),
    );
    render(<KnowledgePage client={c} />);
    const library = screen.getByRole("complementary", { name: "Knowledge items" });
    fireEvent.click(await within(library).findByRole("button", { name: /Runbook 界/ }));
    const query = within(library).getByLabelText("Search local passages");
    query.focus();
    fireEvent.change(query, { target: { value: "incident" } });
    fireEvent.click(within(library).getByRole("button", { name: "Search" }));
    expect(c.search).toHaveBeenCalledWith("incident");
    finish([{ source }]);
    const results = await within(library).findByRole("region", { name: "Search results" });
    expect(within(results).getByRole("status")).toHaveTextContent("1 matching passages");
    expect(within(results).getByText(source.text, { normalizer: (value) => value })).toBeVisible();
    expect(query.compareDocumentPosition(results) & Node.DOCUMENT_POSITION_FOLLOWING).not.toBe(0);
    expect(query).toHaveFocus();
    expect(
      within(screen.getByRole("region", { name: "Knowledge detail" })).getByRole("heading", {
        name: "Runbook 界",
        level: 2,
      }),
    ).toBeVisible();
    expect(c.save).not.toHaveBeenCalled();
    expect(c.select).not.toHaveBeenCalled();
  });
  it("keeps the selected document readable when library filters have no matches", async () => {
    const c = client();
    render(<KnowledgePage client={c} />);
    const library = screen.getByRole("complementary", { name: "Knowledge items" });
    expect(screen.getByText("Your next idea starts here")).toBeInTheDocument();
    fireEvent.click(await within(library).findByRole("button", { name: /Runbook 界/ }));
    fireEvent.click(within(library).getByText("Filter properties"));
    fireEvent.change(within(library).getByLabelText("Filter tags"), {
      target: { value: "no-such-tag" },
    });
    expect(within(library).getByText("No items match these property filters.")).toBeVisible();
    expect(within(library).queryByRole("button", { name: /Runbook 界/ })).not.toBeInTheDocument();
    expect(
      within(screen.getByRole("region", { name: "Knowledge detail" })).getByRole("heading", {
        name: "Runbook 界",
        level: 2,
      }),
    ).toBeVisible();
    fireEvent.change(within(library).getByLabelText("Filter tags"), { target: { value: "" } });
    expect(within(library).getByRole("button", { name: /Runbook 界/ })).toHaveAttribute(
      "aria-pressed",
      "true",
    );
    expect(c.save).not.toHaveBeenCalled();
    expect(c.remove).not.toHaveBeenCalled();
    expect(c.select).not.toHaveBeenCalled();
  });
  it("shows loading then empty and sanitized errors", async () => {
    const c = client();
    let resolve: (value: readonly KnowledgeItem[]) => void = () => undefined;
    c.list = vi.fn(
      () =>
        new Promise<readonly KnowledgeItem[]>((r) => {
          resolve = r;
        }),
    );
    render(<KnowledgePage client={c} />);
    expect(screen.getByRole("status")).toHaveTextContent("Loading");
    resolve([]);
    expect(await screen.findByText("No library items yet.")).toBeInTheDocument();
    c.import = vi.fn(() => Promise.reject(new Error("invalid_utf8")));
    fireEvent.click(screen.getByRole("button", { name: "Import selected file" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("not valid UTF-8");
  });
  it("renders safe Markdown and preserves original source; edits create explicit versions", async () => {
    const c = client();
    const { container } = render(<KnowledgePage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: /Runbook 界/ }));
    expect(container.querySelector("pre.knowledge-content")?.textContent).toBe(content);
    expect(
      container.querySelector(
        ".knowledge-markdown script,.knowledge-markdown img,.knowledge-markdown a",
      ),
    ).toBeNull();
    fireEvent.click(screen.getByRole("button", { name: "Edit local copy" }));
    fireEvent.change(screen.getByLabelText("Markdown content"), {
      target: { value: "# Edited copy" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Save note version" }));
    await waitFor(() => {
      expect(c.save).toHaveBeenCalledWith({
        id: 1,
        expectedVersion: 1,
        title: "Runbook 界",
        content: "# Edited copy",
        draft: false,
      });
    });
  });
  it("requires explicit removal and export; reports overwrite errors", async () => {
    const c = client();
    c.export = vi.fn(() => Promise.reject(new Error("destination_exists")));
    render(<KnowledgePage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: /Runbook 界/ }));
    fireEvent.click(screen.getByRole("button", { name: "Export Markdown" }));
    expect(await screen.findByRole("alert")).toHaveTextContent("Nothing was overwritten");
    fireEvent.click(screen.getByRole("button", { name: "Remove library item" }));
    expect(c.remove).not.toHaveBeenCalled();
    expect(screen.getByText(/Historical room evidence remains/)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Confirm remove library item" }));
    await waitFor(() => {
      expect(c.remove).toHaveBeenCalledWith(1, 1);
    });
  });
  it("retains frozen evidence and honestly exposes unknown references", () => {
    const { container } = render(
      <SourceEvidence sources={[source]} labels={[source.label, "unknown"]} />,
    );
    expect(screen.getByText(/Unknown reference/)).toBeInTheDocument();
    expect(container.querySelector("pre.knowledge-content")?.textContent).toBe(content);
    expect(screen.getByText(/Provenance identifies/)).toBeInTheDocument();
  });
  it("requires review and explicit action for generated notes and exports", async () => {
    const c = client();
    const saved = vi.fn();
    render(
      <NoteEditor
        initial={{ title: "Draft", content }}
        client={c}
        onSaved={saved}
        onCancel={() => undefined}
      />,
    );
    expect(c.save).not.toHaveBeenCalled();
    fireEvent.change(screen.getByLabelText("Draft title"), { target: { value: "Reviewed" } });
    fireEvent.click(screen.getByRole("button", { name: "Export reviewed draft" }));
    await waitFor(() => {
      expect(c.exportDraft).toHaveBeenCalledWith({ title: "Reviewed", content });
    });
    expect(c.save).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Save draft to Knowledge" }));
    await waitFor(() => {
      expect(c.save).toHaveBeenCalledWith({
        id: null,
        expectedVersion: null,
        title: "Reviewed",
        content,
        draft: true,
      });
    });
    expect(saved).toHaveBeenCalled();
  });
  it("searches without attaching automatically and only selects explicit passages", async () => {
    const c = client(),
      change = vi.fn();
    render(<SourcePicker client={c} value={[]} onChange={change} />);
    fireEvent.change(screen.getByLabelText("Find library passages"), {
      target: { value: "incident" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Find passages" }));
    fireEvent.click(await screen.findByRole("button", { name: "Select K1_V1_P1" }));
    expect(change).toHaveBeenCalledWith([source]);
    expect(c.select).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Select whole small document" }));
    await waitFor(() => {
      expect(c.select).toHaveBeenCalledWith(1, 1, 0);
    });
  });
  it("enforces source count and displays stale selection errors", async () => {
    const c = client(),
      change = vi.fn();
    c.select = vi.fn(() => Promise.reject(new Error("stale_selection")));
    render(
      <SourcePicker
        client={c}
        value={Array.from({ length: 6 }, (_, n) => ({ ...source, label: `S${String(n)}` }))}
        onChange={change}
      />,
    );
    fireEvent.change(screen.getByLabelText("Find library passages"), {
      target: { value: "incident" },
    });
    fireEvent.click(screen.getByRole("button", { name: "Find passages" }));
    fireEvent.click(await screen.findByRole("button", { name: "Select K1_V1_P1" }));
    expect(screen.getByRole("alert")).toHaveTextContent("Source limit");
    expect(change).not.toHaveBeenCalled();
    fireEvent.click(screen.getByRole("button", { name: "Select whole small document" }));
    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent("changed or was removed");
    });
  });
  it("rejects malformed native payloads and never reflects arbitrary errors", () => {
    expect(parseItem(item)).toEqual(item);
    expect(() => parseItem({ ...item, versions: [] })).toThrow();
    expect(() => parseOrigin({ documentId: -1 })).toThrow();
    expect(knowledgeError("private payload")).not.toContain("private payload");
  });
});
