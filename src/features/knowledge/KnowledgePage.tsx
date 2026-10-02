import { UnsavedKnowledgePrompt } from "./UnsavedKnowledgePrompt";
import { KnowledgeMarkdown } from "./KnowledgeMarkdown";
import { KnowledgeProperties } from "./PropertiesEditor";
import { KnowledgeGraph } from "./KnowledgeGraph";
import { LinkEditor } from "./LinkEditor";
import { properties, TEMPLATES } from "./knowledgeProperties";
import { useUnsavedKnowledge } from "./useUnsavedKnowledge";
import { useEffect, useRef, useState } from "react";
import {
  knowledgeClient,
  knowledgeError,
  type KnowledgeClient,
  type KnowledgeItem,
  type Draft,
} from "../../infrastructure/tauri/knowledge-client";
import type { Source } from "../../infrastructure/tauri/collaboration-client";
import "./knowledge.css";
export function SourceEvidence({
  sources,
  labels,
}: {
  readonly sources: readonly Source[];
  readonly labels: readonly string[];
}) {
  return (
    <section aria-label="Historical source references">
      <p>Provenance identifies supplied material; it does not prove a claim follows from it.</p>
      {labels.length === 0 && <p>No source references supplied.</p>}
      {labels.map((label) => {
        const s = sources.find((s) => s.label === label);
        return (
          <details key={label}>
            <summary>
              {label}
              {s?.origin
                ? ` · ${s.origin.title} · v${String(s.origin.version)} · lines ${String(s.origin.startLine)}–${String(s.origin.endLine)}`
                : ""}
            </summary>
            {s ? (
              <>
                <p>
                  {s.origin
                    ? `Document ${String(s.origin.documentId)} / passage ${String(s.origin.passageId)} · SHA-256 ${s.origin.hash}`
                    : "Owner-pasted source snapshot"}
                </p>
                <pre className="knowledge-content">{s.text}</pre>
              </>
            ) : (
              <p>Unknown reference; no historical content is substituted.</p>
            )}
          </details>
        );
      })}
    </section>
  );
}
export function NoteEditor({
  initial,
  onSaved,
  onCancel,
  client = knowledgeClient,
}: {
  readonly initial: Draft;
  readonly onSaved: () => void;
  readonly onCancel: () => void;
  readonly client?: KnowledgeClient;
}) {
  const editor = useRef<HTMLElement>(null);
  useEffect(() => {
    editor.current?.focus();
  }, []);
  const [title, setTitle] = useState(initial.title),
    [content, setContent] = useState(initial.content),
    [busy, setBusy] = useState(false),
    [error, setError] = useState(""),
    [notice, setNotice] = useState("");
  const guard = useUnsavedKnowledge(title !== initial.title || content !== initial.content, busy);
  return (
    <section className="knowledge" aria-label="Review generated draft" ref={editor} tabIndex={-1}>
      {guard.pending && (
        <UnsavedKnowledgePrompt onCancel={guard.cancel} onDiscard={guard.discard} />
      )}
      <h3>Review generated draft</h3>
      <p>
        Generated material is unverified. Edit before saving; saving never adds it automatically to
        a bot’s context.
      </p>
      <label>
        Draft title
        <input
          maxLength={120}
          value={title}
          onChange={(e) => {
            setTitle(e.currentTarget.value);
          }}
        />
      </label>
      <label>
        Draft Markdown
        <textarea
          value={content}
          onChange={(e) => {
            setContent(e.currentTarget.value);
          }}
        />
      </label>
      {error && <p role="alert">{error}</p>}
      {notice && <p role="status">{notice}</p>}
      <button
        disabled={busy}
        onClick={() => {
          setBusy(true);
          void client
            .save({ id: null, expectedVersion: null, title, content, draft: true })
            .then(onSaved)
            .catch((e: unknown) => {
              setError(knowledgeError(e));
            })
            .finally(() => {
              setBusy(false);
            });
        }}
      >
        Save draft to Knowledge
      </button>
      <button
        disabled={busy}
        onClick={() => {
          setBusy(true);
          setError("");
          void client
            .exportDraft({ title, content })
            .then((saved) => {
              if (saved) setNotice("Reviewed draft exported; library unchanged.");
            })
            .catch((e: unknown) => {
              setError(knowledgeError(e));
            })
            .finally(() => {
              setBusy(false);
            });
        }}
      >
        Export reviewed draft
      </button>
      <button
        disabled={busy}
        onClick={() => {
          guard.attempt(onCancel);
        }}
      >
        Cancel draft
      </button>
    </section>
  );
}
export function KnowledgePage({ client = knowledgeClient }: { readonly client?: KnowledgeClient }) {
  const [items, setItems] = useState<readonly KnowledgeItem[]>([]),
    [selected, setSelected] = useState<number | null>(null),
    [version, setVersion] = useState<number | null>(null),
    [title, setTitle] = useState(""),
    [content, setContent] = useState(""),
    [editing, setEditing] = useState(false),
    [busy, setBusy] = useState(false),
    [error, setError] = useState(""),
    [notice, setNotice] = useState(""),
    [query, setQuery] = useState(""),
    [results, setResults] = useState<readonly { readonly source: Source }[] | null>(null),
    [confirm, setConfirm] = useState(false),
    [loading, setLoading] = useState(true),
    [original, setOriginal] = useState({ title: "", content: "", version: null as number | null }),
    [template, setTemplate] = useState<keyof typeof TEMPLATES>("Blank note"),
    [tagFilter, setTagFilter] = useState(""),
    [projectFilter, setProjectFilter] = useState(""),
    [typeFilter, setTypeFilter] = useState(""),
    [reviewFilter, setReviewFilter] = useState("");
  const dirty = editing && (title !== original.title || content !== original.content);
  const guard = useUnsavedKnowledge(dirty, busy);
  useEffect(() => {
    let alive = true;
    void client
      .list()
      .then((v) => {
        if (alive) setItems(v);
      })
      .catch((e: unknown) => {
        if (alive) setError(knowledgeError(e));
      })
      .finally(() => {
        if (alive) setLoading(false);
      });
    return () => {
      alive = false;
    };
  }, [client]);
  const item = items.find((i) => i.id === selected),
    current = item?.versions.at(-1),
    v = item?.versions.find((v) => v.id === version) ?? current;
  async function action(fn: () => Promise<void>) {
    setBusy(true);
    setError("");
    setNotice("");
    try {
      await fn();
      setItems(await client.list());
      setResults(null);
    } catch (e) {
      setError(knowledgeError(e));
    } finally {
      setBusy(false);
    }
  }
  function choose(i: KnowledgeItem) {
    setSelected(i.id);
    setVersion(null);
    setEditing(false);
    setConfirm(false);
  }
  function open(id: number) {
    const target = items.find((i) => i.id === id);
    if (target)
      guard.attempt(() => {
        choose(target);
      });
  }
  const visible = items.filter((i) => {
    const p = properties(i.versions.at(-1)?.content ?? "").values;
    return (
      (!tagFilter || p.tags.some((t) => t.toLowerCase().includes(tagFilter.toLowerCase()))) &&
      p.project.toLowerCase().includes(projectFilter.toLowerCase()) &&
      p.note_type.toLowerCase().includes(typeFilter.toLowerCase()) &&
      p.review_status.toLowerCase().includes(reviewFilter.toLowerCase())
    );
  });
  return (
    <section className="knowledge" aria-labelledby="knowledge-title">
      {guard.pending && (
        <UnsavedKnowledgePrompt onCancel={guard.cancel} onDiscard={guard.discard} />
      )}
      <h1 id="knowledge-title">Knowledge &amp; Documents</h1>
      <p>
        Local snapshots and reusable Markdown notes. Private bot notes stay in Bots and are never
        imported automatically.
      </p>
      <p>
        16 KiB per version · 200 items · 8 versions/item · 4 MiB total. No encryption-at-rest claim.
        No vault scanning, filesystem-link resolution or automatic sharing.
      </p>
      {error && <p role="alert">{error}</p>}
      {notice && <p role="status">{notice}</p>}
      <label>
        New note template
        <select
          value={template}
          onChange={(e) => {
            setTemplate(e.target.value as keyof typeof TEMPLATES);
          }}
        >
          {Object.keys(TEMPLATES).map((t) => (
            <option key={t}>{t}</option>
          ))}
        </select>
      </label>
      <div className="knowledge-actions">
        <button
          disabled={busy}
          onClick={() => {
            guard.attempt(() => {
              void action(async () => {
                const i = await client.import();
                if (i) {
                  choose(i);
                  setNotice("Imported local snapshot. Original file unchanged.");
                }
              });
            });
          }}
        >
          Import selected file
        </button>
        <button
          disabled={busy}
          onClick={() => {
            guard.attempt(() => {
              setSelected(null);
              setTitle("");
              setContent(TEMPLATES[template]);
              setOriginal({ title: "", content: "", version: null });
              setEditing(true);
              setConfirm(false);
            });
          }}
        >
          New Markdown note
        </button>
        <button
          disabled={busy}
          onClick={() => {
            guard.attempt(() => {
              void action(() => {
                setEditing(false);
                setNotice("Library reloaded.");
                return Promise.resolve();
              });
            });
          }}
        >
          Reload library
        </button>
      </div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          setError("");
          setBusy(true);
          void client
            .search(query)
            .then(setResults)
            .catch((e: unknown) => {
              setError(knowledgeError(e));
            })
            .finally(() => {
              setBusy(false);
            });
        }}
      >
        <label>
          Search local passages
          <input
            maxLength={200}
            value={query}
            onChange={(e) => {
              setQuery(e.currentTarget.value);
            }}
          />
        </label>
        <button disabled={busy || !query.trim()}>Search</button>
      </form>
      {results && (
        <section aria-label="Search results">
          <p>
            {results.length} matching passages (up to 50). Keyword matches are not proof of support.
          </p>
          {results.length === 0 && (
            <p>No matching evidence. Try other terms or import a relevant document.</p>
          )}
          {results.map(({ source: s }) => (
            <article key={s.label}>
              <strong>
                {s.origin?.title} · version {s.origin?.version} · lines {s.origin?.startLine}–
                {s.origin?.endLine}
              </strong>
              <pre className="knowledge-content">{s.text}</pre>
            </article>
          ))}
        </section>
      )}
      <details>
        <summary>Filter properties</summary>
        <label>
          Filter tags
          <input
            value={tagFilter}
            onChange={(e) => {
              setTagFilter(e.target.value);
            }}
          />
        </label>
        <label>
          Filter project
          <input
            value={projectFilter}
            onChange={(e) => {
              setProjectFilter(e.target.value);
            }}
          />
        </label>
        <label>
          Filter note type
          <input
            value={typeFilter}
            onChange={(e) => {
              setTypeFilter(e.target.value);
            }}
          />
        </label>
        <label>
          Filter review status
          <input
            value={reviewFilter}
            onChange={(e) => {
              setReviewFilter(e.target.value);
            }}
          />
        </label>
      </details>
      <div className="knowledge-layout">
        <aside aria-label="Knowledge items">
          {loading && <p role="status">Loading local library…</p>}
          {!loading && items.length === 0 && <p>No library items yet.</p>}
          {visible.map((i) => {
            const x = i.versions.at(-1);
            return (
              <button
                key={i.id}
                disabled={busy}
                aria-pressed={selected === i.id}
                onClick={() => {
                  guard.attempt(() => {
                    choose(i);
                  });
                }}
              >
                {x?.title}
                <small>
                  {i.kind.replaceAll("_", " ")} · {x?.format} ·{" "}
                  {new TextEncoder().encode(x?.content ?? "").length} bytes · v{x?.id}
                </small>
              </button>
            );
          })}
        </aside>
        <section aria-label="Knowledge detail">
          {!editing && v && item && (
            <>
              <h2>{v.title}</h2>
              <p>
                {item.kind === "generated_draft"
                  ? "Generated draft — claims unverified"
                  : item.kind === "imported"
                    ? "Imported snapshot — editing does not change the original file"
                    : "Owner-authored note"}
              </p>
              <label>
                Version
                <select
                  value={v.id}
                  onChange={(e) => {
                    setVersion(Number(e.currentTarget.value));
                  }}
                >
                  {item.versions.map((x) => (
                    <option key={x.id} value={x.id}>
                      Version {x.id} · {new Date(x.createdMs).toLocaleString()}
                    </option>
                  ))}
                </select>
              </label>
              <p className="knowledge-hash">SHA-256 {v.hash}</p>
              <p>
                Reading mode · HTML and remote resources are inert. Internal links navigate only
                within this library.
              </p>
              {properties(v.content).error && <p role="alert">{properties(v.content).error}</p>}
              {v.id !== current?.id && (
                <p>
                  Historical version. Link navigation is unavailable in historical versions. Link
                  graph and backlinks below refer to the current saved library only.
                </p>
              )}
              <KnowledgeMarkdown
                content={v.content}
                historical={v.id !== current?.id}
                links={v.id === current?.id ? (current.links ?? []) : []}
                onOpen={open}
              />
              <details>
                <summary>Original Markdown</summary>
                <pre className="knowledge-content">{v.content}</pre>
              </details>
              <dl>
                <dt>Tags</dt>
                <dd>{properties(v.content).values.tags.join(", ") || "None"}</dd>
                <dt>Project</dt>
                <dd>{properties(v.content).values.project || "None"}</dd>
                <dt>Note type</dt>
                <dd>{properties(v.content).values.note_type || "None"}</dd>
                <dt>Review status (owner label, not verification)</dt>
                <dd>{properties(v.content).values.review_status || "None"}</dd>
              </dl>
              <section aria-label="Backlinks">
                <h3>Backlinks · current saved library</h3>
                {items.flatMap((source) =>
                  (source.versions.at(-1)?.links ?? [])
                    .filter((l) => l.status === "resolved" && l.targetId === item.id)
                    .map((l, n) => (
                      <article key={`${String(source.id)}-${String(n)}`}>
                        <button
                          onClick={() => {
                            open(source.id);
                          }}
                        >
                          {source.versions.at(-1)?.title}
                        </button>
                        <p>{l.context}</p>
                      </article>
                    )),
                )}
              </section>
              <KnowledgeGraph items={items} selected={item.id} onOpen={open} />
              <div className="knowledge-actions">
                <button
                  disabled={busy || v.id !== current?.id}
                  onClick={() => {
                    setOriginal({ title: v.title, content: v.content, version: v.id });
                    setTitle(v.title);
                    setContent(v.content);
                    setEditing(true);
                  }}
                >
                  Edit local copy
                </button>
                {item.kind === "imported" && (
                  <button
                    disabled={busy}
                    onClick={() =>
                      void action(async () => {
                        const updated = await client.import(item.id, current?.id);
                        if (updated) {
                          setVersion(null);
                          setNotice("New snapshot version imported.");
                        }
                      })
                    }
                  >
                    Re-import selected file as new version
                  </button>
                )}
                <button
                  disabled={busy}
                  onClick={() =>
                    void action(async () => {
                      if (await client.export(item.id, v.id))
                        setNotice("Markdown exported without overwriting an existing file.");
                    })
                  }
                >
                  Export Markdown
                </button>
                <button
                  disabled={busy}
                  onClick={() => {
                    setConfirm(true);
                  }}
                >
                  Remove library item
                </button>
              </div>
              {confirm && (
                <section aria-label="Confirm library removal">
                  <p>
                    Remove all library versions from future selection? Historical room evidence
                    remains until you explicitly delete those rooms. Original and exported files
                    remain untouched.
                  </p>
                  <button
                    disabled={busy}
                    onClick={() =>
                      void action(async () => {
                        await client.remove(item.id, current?.id ?? 0);
                        setSelected(null);
                        setConfirm(false);
                        setNotice("Library item removed. Room evidence retained.");
                      })
                    }
                  >
                    Confirm remove library item
                  </button>
                  <button
                    onClick={() => {
                      setConfirm(false);
                    }}
                  >
                    Keep item
                  </button>
                </section>
              )}
            </>
          )}
          {editing && (
            <form
              onSubmit={(e) => {
                e.preventDefault();
                void action(async () => {
                  const saved = await client.save({
                    id: selected,
                    expectedVersion: original.version,
                    title,
                    content,
                    draft: item?.kind === "generated_draft",
                  });
                  choose(saved);
                  setNotice("Saved immutable version; earlier room evidence unchanged.");
                });
              }}
            >
              <h2>{selected ? "Edit local copy" : "New owner note"}</h2>
              <label>
                Note title
                <input
                  maxLength={120}
                  value={title}
                  onChange={(e) => {
                    setTitle(e.currentTarget.value);
                  }}
                />
              </label>
              <p role="status">
                {dirty ? "Unsaved changes — explicit Save required" : "No unsaved changes"}
              </p>
              <KnowledgeProperties content={content} onChange={setContent} />
              <LinkEditor content={content} onChange={setContent} items={items} />
              <details>
                <summary>Preview unsaved Markdown</summary>
                <KnowledgeMarkdown content={content} onOpen={open} />
              </details>
              <p>{new TextEncoder().encode(content).length} / 16384 bytes</p>
              <button disabled={busy}>Save note version</button>
              <button
                type="button"
                onClick={() => {
                  guard.attempt(() => {
                    setEditing(false);
                  });
                }}
              >
                Cancel edit
              </button>
            </form>
          )}
        </section>
      </div>
    </section>
  );
}
