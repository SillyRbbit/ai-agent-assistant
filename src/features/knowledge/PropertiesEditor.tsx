import { useState } from "react";
import { properties, updateProperties, type Properties } from "./knowledgeProperties";
export function KnowledgeProperties({
  content,
  onChange,
}: {
  readonly content: string;
  readonly onChange: (content: string) => void;
}) {
  const p = properties(content);
  const [tag, setTag] = useState("");
  const [error, setError] = useState("");
  function change(value: Properties) {
    onChange(updateProperties(content, value));
  }
  return (
    <fieldset disabled={!!p.error}>
      <legend>Properties</legend>
      {p.error && <p role="alert">{p.error}</p>}
      <label>
        New tag
        <input
          value={tag}
          maxLength={65}
          onChange={(e) => {
            setTag(e.target.value);
            setError("");
          }}
        />
      </label>
      <button
        type="button"
        onClick={() => {
          const next = tag.trim();
          if (
            !next ||
            next.length > 64 ||
            p.values.tags.length >= 16 ||
            p.values.tags.includes(next)
          ) {
            setError("Use one unique tag of 1–64 characters; at most 16 tags per note.");
            return;
          }
          change({ ...p.values, tags: [...p.values.tags, next] });
          setTag("");
          setError("");
        }}
      >
        Add tag
      </button>
      {error && <p role="alert">{error}</p>}
      <ul aria-label="Draft tags">
        {p.values.tags.map((value) => (
          <li key={value}>
            {value}{" "}
            <button
              type="button"
              aria-label={`Remove tag ${value}`}
              onClick={() => {
                change({ ...p.values, tags: p.values.tags.filter((v) => v !== value) });
              }}
            >
              Remove
            </button>
          </li>
        ))}
      </ul>
      <p>Add the tag before saving. Tag-entry text is not part of the note until added.</p>
      {(["project", "note_type", "review_status"] as const).map((key) => (
        <label key={key}>
          {key.replaceAll("_", " ")}
          <input
            maxLength={120}
            value={p.values[key]}
            onChange={(e) => {
              change({ ...p.values, [key]: e.target.value });
            }}
          />
        </label>
      ))}
      <p>
        Organizational labels only. “Reviewed” does not verify generated claims. Unsupported
        metadata remains in source.
      </p>
    </fieldset>
  );
}
