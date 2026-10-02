import { useState } from "react";
import { KnowledgeProperties } from "./PropertiesEditor";
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
import { render, screen, fireEvent, waitFor, within } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { KnowledgeMarkdown } from "./KnowledgeMarkdown";
import { properties, updateProperties, TEMPLATES } from "./knowledgeProperties";
import { neighborhood } from "./knowledgeNeighborhood";
import { KnowledgePage } from "./KnowledgePage";
import { LinkEditor } from "./LinkEditor";
import { allowKnowledgeNavigation } from "./useUnsavedKnowledge";
import type { KnowledgeItem, KnowledgeClient } from "../../infrastructure/tauri/knowledge-client";
function note(id: number, title: string, content = "# Content"): KnowledgeItem {
  return {
    id,
    kind: "owner_note",
    versions: [{ id: 1, title, content, hash: "a".repeat(64), format: "md", createdMs: 1 }],
  };
}
function client(items = [note(1, "Incident")]): {
  -readonly [K in keyof KnowledgeClient]: KnowledgeClient[K];
} {
  return {
    list: vi.fn(() => Promise.resolve(items)),
    save: vi.fn(() => Promise.resolve(items[0] ?? note(1, "Fallback"))),
    remove: vi.fn(() => Promise.resolve(undefined)),
    import: vi.fn(() => Promise.resolve(null)),
    export: vi.fn(() => Promise.resolve(false)),
    exportDraft: vi.fn(() => Promise.resolve(false)),
    search: vi.fn(() => Promise.resolve([])),
    select: vi.fn(),
    draft: vi.fn(),
  };
}
describe("Connected Knowledge", () => {
  it("renders GFM as React elements with inert HTML, unsafe URLs and remote images", () => {
    const { container } = render(
      <KnowledgeMarkdown
        content={
          "# Title\n\n**Bold** *Em*\n\n- [x] Done\n\n> Quote\n\n| A | B |\n|---|---|\n| 1 | 2 |\n\n```js\n[[code]]\n```\n\n<img src=x onerror=alert(1)>\n\n![image](https://example.test/a) [unsafe](javascript:alert(1)) <script>alert(1)</script>"
        }
        onOpen={vi.fn()}
      />,
    );
    expect(screen.getByRole("heading", { name: "Title" })).toBeInTheDocument();
    expect(container.querySelector("strong")?.textContent).toBe("Bold");
    expect(container.querySelector("table")).not.toBeNull();
    expect(container.querySelector("blockquote")).not.toBeNull();
    expect(container.querySelector("script,img,iframe,a,style")).toBeNull();
    expect(container.querySelector("code")?.textContent).toBe("[[code]]");
  });
  it("navigates only bound library identities and shows ambiguous/missing/removed links", () => {
    const open = vi.fn();
    render(
      <KnowledgeMarkdown
        content="[[Old title|Read]] [[Duplicate]] [[Gone]] [[Missing]] `[[Old title]]`"
        links={[
          { title: "Old title", label: "Read", context: "", targetId: 42, status: "resolved" },
          { title: "Duplicate", label: "", context: "", targetId: null, status: "ambiguous" },
          { title: "Gone", label: "", context: "", targetId: 7, status: "removed" },
        ]}
        onOpen={open}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "Read" }));
    expect(open).toHaveBeenCalledWith(42);
    expect(screen.getByText(/ambiguous/)).toBeInTheDocument();
    expect(screen.getByText(/removed/)).toBeInTheDocument();
    expect(screen.getByText(/missing/)).toBeInTheDocument();
    expect(screen.getAllByRole("button")).toHaveLength(1);
  });
  it("keeps Markdown punctuation inside wikilink titles and aliases literal", () => {
    const open = vi.fn();
    render(
      <KnowledgeMarkdown
        content="[[A *special* title|**Read**]]"
        links={[
          {
            title: "A *special* title",
            label: "**Read**",
            context: "",
            targetId: 2,
            status: "resolved",
          },
        ]}
        onOpen={open}
      />,
    );
    fireEvent.click(screen.getByRole("button", { name: "**Read**" }));
    expect(open).toHaveBeenCalledWith(2);
  });
  it("keeps historical wikilinks literal and inert even when bindings are supplied", () => {
    const open = vi.fn();
    const { container } = render(
      <KnowledgeMarkdown
        historical
        content="[[Incident|Read]] [[Missing]] [[Duplicate]] [[Gone]] `[[Code]]`"
        links={[
          { title: "Incident", label: "Read", context: "", targetId: 42, status: "resolved" },
          { title: "Missing", label: "", context: "", targetId: null, status: "missing" },
          { title: "Duplicate", label: "", context: "", targetId: null, status: "ambiguous" },
          { title: "Gone", label: "", context: "", targetId: 7, status: "removed" },
        ]}
        onOpen={open}
      />,
    );
    expect(screen.getByText("[[Incident|Read]]")).toBeInTheDocument();
    expect(screen.getByText("[[Missing]]")).toBeInTheDocument();
    expect(screen.getByText("[[Duplicate]]")).toBeInTheDocument();
    expect(screen.getByText("[[Gone]]")).toBeInTheDocument();
    expect(container.querySelector("code")).toHaveTextContent("[[Code]]");
    expect(container.querySelector("button,a,.knowledge-unresolved")).toBeNull();
    fireEvent.click(screen.getByText("[[Incident|Read]]"));
    expect(open).not.toHaveBeenCalled();
  });
  it("switches v2/v1/v2 without substituting current links or changing immutable versions", async () => {
    const link = {
      title: "Incident",
      label: "Read",
      context: "",
      targetId: 2,
      status: "resolved" as const,
    };
    const first = {
      id: 1,
      title: "Change request",
      content: "[[Incident|Read]]",
      hash: "a".repeat(64),
      format: "md",
      createdMs: 1,
    };
    const second = {
      ...first,
      id: 2,
      content: first.content + "\n\nQA version 2",
      hash: "b".repeat(64),
      links: [link],
    };
    const items = [
      { ...note(1, "Change request"), versions: [first, second] },
      note(2, "Incident"),
    ];
    const frozen = JSON.stringify(items);
    const c = client(items);
    render(<KnowledgePage client={c} />);
    fireEvent.click(
      await within(screen.getByRole("complementary", { name: "Knowledge items" })).findByRole(
        "button",
        { name: /Change request/ },
      ),
    );
    const detail = screen.getByRole("region", { name: "Knowledge detail" });
    function markdownView() {
      const view = detail.querySelector(".knowledge-markdown");
      if (!(view instanceof HTMLElement)) throw Error("missing Markdown view");
      return within(view);
    }
    expect(markdownView().getByRole("button", { name: "Read" })).toBeInTheDocument();
    expect(within(detail).getByText("SHA-256 " + second.hash)).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Version"), { target: { value: "1" } });
    expect(
      within(detail).getByText(/Link navigation is unavailable in historical versions/),
    ).toBeInTheDocument();
    expect(markdownView().getByText("[[Incident|Read]]")).toBeInTheDocument();
    expect(markdownView().queryAllByRole("button")).toHaveLength(0);
    expect(markdownView().queryByText(/missing/)).not.toBeInTheDocument();
    expect(markdownView().queryByText("QA version 2")).not.toBeInTheDocument();
    expect(within(detail).getByText("SHA-256 " + first.hash)).toBeInTheDocument();
    expect(within(detail).getByRole("button", { name: "Edit local copy" })).toBeDisabled();
    expect(
      within(detail).getByRole("region", { name: "Local knowledge graph" }),
    ).toBeInTheDocument();
    fireEvent.change(screen.getByLabelText("Version"), { target: { value: "2" } });
    expect(markdownView().getByRole("button", { name: "Read" })).toBeInTheDocument();
    expect(markdownView().getByText("QA version 2")).toBeInTheDocument();
    expect(within(detail).getByText("SHA-256 " + second.hash)).toBeInTheDocument();
    expect(c.save).not.toHaveBeenCalled();
    expect(JSON.stringify(items)).toBe(frozen);
    fireEvent.click(markdownView().getByRole("button", { name: "Read" }));
    expect(await within(detail).findByRole("heading", { name: "Incident" })).toBeInTheDocument();
  });
  it("preserves unknown frontmatter and CRLF while round-tripping canonical fields", () => {
    const input =
      "---\r\ncustom: [keep, exactly]\r\n# keep comment\r\nproject: before\r\n---\r\n# Body\r\n";
    const updated = updateProperties(input, {
      tags: ["incident", "ops"],
      project: "Orion",
      note_type: "runbook",
      review_status: "owner reviewed",
    });
    expect(updated).toContain("custom: [keep, exactly]\r\n# keep comment");
    expect(updated.endsWith("# Body\r\n")).toBe(true);
    expect(properties(updated).values).toEqual({
      tags: ["incident", "ops"],
      project: "Orion",
      note_type: "runbook",
      review_status: "owner reviewed",
    });
    expect(updateProperties(updated, properties(updated).values)).toBe(updated);
  });
  it.each([
    "---\ntags: &unsafe value\n---\nbody",
    "---\nproject: |\n  multiline\n---\nbody",
    "---\nproject: a\nproject: b\n---\nbody",
    "---\nproject: unfinished",
  ])("retains unsupported frontmatter unchanged: %s", (content) => {
    const p = properties(content);
    expect(p.error).not.toBe("");
    expect(updateProperties(content, p.values)).toBe(content);
  });
  it("has all five inert templates and the exact change section order", () => {
    expect(Object.keys(TEMPLATES)).toHaveLength(5);
    expect(TEMPLATES["Change request"].split("\n").filter((l) => l.startsWith("## "))).toEqual(
      [
        "Change title",
        "Risk level",
        "Risk scope",
        "Description",
        "Project objective",
        "Roll-out plan/steps",
        "Test/validation plan",
        "Communication plan",
        "Roll-back plan",
      ].map((s) => `## ${s}`),
    );
  });
  it("deduplicates graph edges, handles cycles and caps the one-hop neighborhood", () => {
    const all = Array.from({ length: 40 }, (_, i) => note(i + 1, `Note ${String(i + 1)}`));
    const first = all[0];
    if (!first?.versions[0]) throw Error("test fixture");
    all[0] = {
      ...first,
      versions: [
        {
          ...first.versions[0],
          links: all.flatMap((i) => [
            { title: "label", label: "", context: "", targetId: i.id, status: "resolved" as const },
            { title: "label", label: "", context: "", targetId: i.id, status: "resolved" as const },
          ]),
        },
      ],
    };
    const result = neighborhood(all, 1);
    expect(result.ids.size).toBe(25);
    expect(new Set(result.edges.map((e) => e.id)).size).toBe(result.edges.length);
    expect(neighborhood([note(1, "Isolated")], 1).edges).toEqual([]);
  });
  it("autocomplete supports keyboard selection without saving or creating notes", () => {
    function Editor() {
      const [text, setText] = useState("");
      return (
        <LinkEditor
          content={text}
          items={[note(2, "Runbook"), note(3, "Research")]}
          onChange={setText}
        />
      );
    }
    render(<Editor />);
    const area = screen.getByLabelText("Markdown content");
    fireEvent.change(area, { target: { value: "[[R", selectionStart: 3 } });
    expect(screen.getByRole("listbox")).toBeInTheDocument();
    fireEvent.keyDown(area, { key: "ArrowDown" });
    fireEvent.keyDown(area, { key: "Enter" });
    expect(area).toHaveValue("[[Research]]");
    fireEvent.change(area, { target: { value: "[[R", selectionStart: 3 } });
    fireEvent.keyDown(area, { key: "Escape" });
    expect(screen.queryByRole("listbox")).not.toBeInTheDocument();
  });
  it("adds tags explicitly without truncation and preserves commas in a tag", () => {
    function Editor() {
      const [text, setText] = useState("# Draft");
      return (
        <>
          <KnowledgeProperties content={text} onChange={setText} />
          <output>{text}</output>
        </>
      );
    }
    render(<Editor />);
    fireEvent.change(screen.getByLabelText("New tag"), { target: { value: "ops, incidents" } });
    fireEvent.click(screen.getByRole("button", { name: "Add tag" }));
    expect(screen.getByRole("status")).toHaveTextContent('tags: ["ops, incidents"]');
    expect(properties(screen.getByRole("status").textContent).values.tags).toEqual([
      "ops, incidents",
    ]);
    fireEvent.change(screen.getByLabelText("New tag"), { target: { value: "x".repeat(65) } });
    fireEvent.click(screen.getByRole("button", { name: "Add tag" }));
    expect(screen.getByRole("alert")).toHaveTextContent("1–64");
    expect(screen.getByLabelText("New tag")).toHaveValue("x".repeat(65));
  });
  it("protects unsaved edits across note, cancel and application-route navigation", async () => {
    const c = client([note(1, "Incident"), note(2, "Runbook")]);

    render(<KnowledgePage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: /Incident/ }));
    fireEvent.click(screen.getByRole("button", { name: "Edit local copy" }));
    fireEvent.change(screen.getByLabelText("Markdown content"), { target: { value: "Unsaved" } });
    fireEvent.click(screen.getByRole("button", { name: /Runbook/ }));
    expect(screen.getByLabelText("Markdown content")).toHaveValue("Unsaved");
    expect(screen.getByRole("alertdialog")).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }));
    fireEvent.click(screen.getByRole("button", { name: "Cancel edit" }));
    expect(screen.getByLabelText("Markdown content")).toHaveValue("Unsaved");
    fireEvent.click(screen.getByRole("button", { name: "Keep editing" }));
    expect(allowKnowledgeNavigation()).toBe(false);
    expect(c.save).not.toHaveBeenCalled();
  });
  it("retains the draft after stale save failure and sends its original expected revision", async () => {
    const c = client();
    c.save = vi.fn(() => Promise.reject(Error("stale_selection")));
    render(<KnowledgePage client={c} />);
    fireEvent.click(await screen.findByRole("button", { name: /Incident/ }));
    fireEvent.click(screen.getByRole("button", { name: "Edit local copy" }));
    fireEvent.change(screen.getByLabelText("Markdown content"), { target: { value: "Keep me" } });
    fireEvent.click(screen.getByRole("button", { name: "Save note version" }));
    await waitFor(() => {
      expect(screen.getByRole("alert")).toHaveTextContent("changed or was removed");
    });
    expect(screen.getByLabelText("Markdown content")).toHaveValue("Keep me");
    expect(c.save).toHaveBeenCalledWith(
      expect.objectContaining({ expectedVersion: 1, content: "Keep me" }),
    );
  });
});
