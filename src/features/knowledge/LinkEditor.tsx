import { useState, useRef } from "react";
import type { KnowledgeItem } from "../../infrastructure/tauri/knowledge-client";
export function LinkEditor({
  content,
  onChange,
  items,
}: {
  readonly content: string;
  readonly onChange: (value: string) => void;
  readonly items: readonly KnowledgeItem[];
}) {
  const ref = useRef<HTMLTextAreaElement>(null);
  const [cursor, setCursor] = useState(0),
    [choice, setChoice] = useState(0),
    [dismissed, setDismissed] = useState(false);
  const match = /\[\[([^\]\n|]*)$/.exec(content.slice(0, cursor));
  const query = match?.[1];
  const choices =
    query === undefined || dismissed
      ? []
      : items
          .filter((i) => i.versions.at(-1)?.title.toLowerCase().includes(query.toLowerCase()))
          .slice(0, 8);
  function select(item: KnowledgeItem) {
    if (!match || query === undefined) return;
    const title = item.versions.at(-1)?.title ?? "";
    const suffix = content.slice(cursor).replace(/^\]\]/, "");
    const next = `${content.slice(0, cursor - query.length)}${title}]]${suffix}`;
    onChange(next);
    setDismissed(true);
    ref.current?.focus();
  }
  return (
    <>
      <label>
        Markdown content
        <textarea
          ref={ref}
          value={content}
          aria-autocomplete="list"
          aria-controls={choices.length ? "knowledge-link-options" : undefined}
          aria-activedescendant={
            choices.length ? `knowledge-link-option-${String(choice % choices.length)}` : undefined
          }
          onChange={(e) => {
            onChange(e.target.value);
            setCursor(e.target.selectionStart);
            setChoice(0);
            setDismissed(false);
          }}
          onSelect={(e) => {
            setCursor(e.currentTarget.selectionStart);
          }}
          onKeyDown={(e) => {
            if (!choices.length) return;
            if (e.key === "ArrowDown" || e.key === "ArrowUp") {
              e.preventDefault();
              setChoice(
                (n) => (n + (e.key === "ArrowDown" ? 1 : choices.length - 1)) % choices.length,
              );
            }
            if (e.key === "Escape") {
              e.preventDefault();
              setDismissed(true);
            }
            if (e.key === "Enter") {
              const item = choices[choice % choices.length];
              if (item) {
                e.preventDefault();
                select(item);
              }
            }
          }}
        />
      </label>
      {choices.length > 0 && (
        <ul id="knowledge-link-options" role="listbox" aria-label="Link suggestions">
          {choices.map((i, n) => (
            <li
              key={i.id}
              id={`knowledge-link-option-${String(n)}`}
              role="option"
              aria-selected={n === choice % choices.length}
            >
              <button
                type="button"
                onClick={() => {
                  select(i);
                }}
              >
                {i.versions.at(-1)?.title} · document {i.id}
              </button>
            </li>
          ))}
        </ul>
      )}
      <p>
        Type [[ to link an existing note. Duplicate titles remain ambiguous; use a unique title. No
        missing note is created automatically.
      </p>
    </>
  );
}
