import { useState } from "react";
import {
  knowledgeClient,
  knowledgeError,
  type KnowledgeClient,
} from "../../infrastructure/tauri/knowledge-client";
import type { Source } from "../../infrastructure/tauri/collaboration-client";
export function SourcePicker({
  value,
  onChange,
  client = knowledgeClient,
}: {
  readonly value: readonly Source[];
  readonly onChange: (s: readonly Source[]) => void;
  readonly client?: KnowledgeClient;
}) {
  const [query, setQuery] = useState(""),
    [matches, setMatches] = useState<readonly { readonly source: Source }[]>([]),
    [error, setError] = useState(""),
    [searched, setSearched] = useState(false),
    [busy, setBusy] = useState(false);
  function add(s: Source) {
    if (value.some((x) => x.label === s.label)) return;
    if (
      value.length >= 6 ||
      new TextEncoder().encode(value.map((x) => x.text).join("") + s.text).length > 16384
    ) {
      setError("Source limit reached. Narrow your selection.");
      return;
    }
    onChange([...value, s]);
  }
  return (
    <section className="knowledge" aria-label="Select library sources">
      <h3>Library sources</h3>
      <p>
        Select passages or a whole small document. Combined with pasted sources: at most six, 4096
        characters each, 16 KiB total. Exact content is reviewed before Start.
      </p>
      <label>
        Find library passages
        <input
          value={query}
          maxLength={200}
          onChange={(e) => {
            setQuery(e.currentTarget.value);
          }}
        />
      </label>
      <button
        type="button"
        disabled={busy || !query.trim()}
        onClick={() => {
          setBusy(true);
          setError("");
          void client
            .search(query)
            .then((x) => {
              setMatches(x);
              setSearched(true);
            })
            .catch((e: unknown) => {
              setError(knowledgeError(e));
            })
            .finally(() => {
              setBusy(false);
            });
        }}
      >
        Find passages
      </button>
      {error && <p role="alert">{error}</p>}
      {searched && matches.length === 0 && <p>No matching evidence. Nothing attached.</p>}
      {matches.map(({ source: s }) => (
        <article key={s.label}>
          <strong>
            {s.origin?.title} · v{s.origin?.version} · lines {s.origin?.startLine}–
            {s.origin?.endLine}
          </strong>
          <pre className="knowledge-content">{s.text}</pre>
          <button
            type="button"
            onClick={() => {
              add(s);
            }}
          >
            Select {s.label}
          </button>
          <button
            type="button"
            onClick={() => {
              const o = s.origin;
              if (o) {
                setBusy(true);
                void client
                  .select(o.documentId, o.version, 0)
                  .then(add)
                  .catch((e: unknown) => {
                    setError(knowledgeError(e));
                  })
                  .finally(() => {
                    setBusy(false);
                  });
              }
            }}
          >
            Select whole small document
          </button>
        </article>
      ))}
      {value.map((s) => (
        <details key={s.label} open>
          <summary>
            Selected: {s.origin?.title} · {s.label}
          </summary>
          <pre className="knowledge-content">{s.text}</pre>
          <button
            type="button"
            onClick={() => {
              onChange(value.filter((x) => x.label !== s.label));
            }}
          >
            Detach {s.label}
          </button>
        </details>
      ))}
    </section>
  );
}
