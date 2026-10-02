export interface Properties {
  tags: string[];
  project: string;
  note_type: string;
  review_status: string;
}
export const EMPTY_PROPERTIES: Properties = {
  tags: [],
  project: "",
  note_type: "",
  review_status: "",
};
const keys = ["tags", "project", "note_type", "review_status"] as const;
function scalar(s: string): string {
  const v = s.trim();
  if (!v) return "";
  if (v.startsWith('"')) {
    const x: unknown = JSON.parse(v);
    if (typeof x !== "string") throw Error();
    return x;
  }
  if (v.startsWith("'")) {
    if (!/^'(?:[^']|'')*'$/.test(v)) throw Error();
    return v.slice(1, -1).replaceAll("''", "'");
  }
  if (/[[\]{}#&*!|>:,\n\r]/.test(v) || /^(null|true|false|[-+]?\d)/i.test(v)) throw Error();
  return v;
}
export function properties(content: string) {
  const values: Properties = { ...EMPTY_PROPERTIES, tags: [] };
  const eol = content.includes("\r\n") ? "\r\n" : "\n";
  const lines = content.split(eol);
  if (lines[0] !== "---")
    return { values, error: "", body: content, lines: [] as string[], end: -1, eol };
  const end = lines.indexOf("---", 1);
  const fail =
    "Unsupported or malformed frontmatter. Original Markdown is preserved; edit it in source before using property controls.";
  if (end < 0) return { values, error: fail, body: content, lines, end, eol };
  try {
    const seen = new Set<string>();
    for (const line of lines.slice(1, end)) {
      if (!line.trim() || line.trimStart().startsWith("#")) continue;
      const m = /^([A-Za-z_][\w-]*):\s*(.*)$/.exec(line);
      if (!m?.[1] || m[2] === undefined || seen.has(m[1])) throw Error();
      seen.add(m[1]);
      const key = keys.find((k) => k === m[1]);
      if (!key) continue; // Unknown fields remain byte-for-byte in the original lines.
      if (key === "tags") {
        const text = m[2].trim();
        if (!text.startsWith("[") || !text.endsWith("]")) throw Error();
        const inner = text.slice(1, -1).trim();
        if (inner.startsWith('"')) {
          const decoded: unknown = JSON.parse(text);
          if (!Array.isArray(decoded) || !decoded.every((v): v is string => typeof v === "string"))
            throw Error();
          values.tags = decoded;
        } else values.tags = inner ? inner.split(",").map(scalar) : [];
        if (values.tags.length > 16 || values.tags.some((x) => !x || x.length > 64)) throw Error();
      } else {
        values[key] = scalar(m[2]);
        if (values[key].length > 120) throw Error();
      }
    }
    return { values, error: "", body: lines.slice(end + 1).join(eol), lines, end, eol };
  } catch {
    return {
      values: { ...EMPTY_PROPERTIES, tags: [] },
      error: fail,
      body: content,
      lines,
      end,
      eol,
    };
  }
}
export function updateProperties(content: string, values: Properties): string {
  const parsed = properties(content);
  if (parsed.error) return content;
  const encode = (k: (typeof keys)[number]) => `${k}: ${JSON.stringify(values[k])}`;
  if (parsed.end < 0) return ["---", ...keys.map(encode), "---", content].join(parsed.eol);
  const seen = new Set<string>();
  const front = parsed.lines.slice(1, parsed.end).map((line) => {
    const key = keys.find((k) => line.startsWith(`${k}:`));
    if (!key) return line;
    seen.add(key);
    return encode(key);
  });
  return [
    "---",
    ...front,
    ...keys.filter((k) => !seen.has(k)).map(encode),
    "---",
    ...parsed.lines.slice(parsed.end + 1),
  ].join(parsed.eol);
}
export const TEMPLATES = {
  "Blank note": "# New note\n\n",
  "Incident investigation":
    "# Incident investigation\n\n## Summary\n\n## Timeline\n\n## Evidence\n\n## Impact\n\n## Follow-up\n",
  "Technical runbook":
    "# Technical runbook\n\n## Purpose\n\n## Prerequisites\n\n## Steps\n\n## Validation\n\n## Recovery\n",
  "Research summary":
    "# Research summary\n\n## Question\n\n## Findings\n\n## Sources\n\n## Limitations\n",
  "Change request": [
    "Change title",
    "Risk level",
    "Risk scope",
    "Description",
    "Project objective",
    "Roll-out plan/steps",
    "Test/validation plan",
    "Communication plan",
    "Roll-back plan",
  ]
    .map((x) => `## ${x}\n\n`)
    .join(""),
} as const;
