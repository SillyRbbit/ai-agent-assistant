import { Fragment, type ReactNode } from "react";
import { Marked, type Token, type Tokens } from "marked";
import type { KnowledgeLink } from "../../infrastructure/tauri/knowledge-client";
import { properties } from "./knowledgeProperties";
const markdown = new Marked({ gfm: true });
markdown.use({
  extensions: [
    {
      name: "knowledge_link",
      level: "inline",
      start: (text: string) => text.indexOf("[["),
      tokenizer(text: string) {
        const match = /^\[\[[^\]\n]+\]\]/.exec(text);
        return match ? { type: "knowledge_link", raw: match[0] } : undefined;
      },
    },
  ],
});
export function KnowledgeMarkdown({
  content,
  links = [],
  historical = false,
  onOpen,
}: {
  readonly content: string;
  readonly links?: readonly KnowledgeLink[];
  readonly historical?: boolean;
  readonly onOpen: (id: number) => void;
}) {
  function text(value: string): ReactNode {
    const parts = value.split(/(\[\[[^\]\n]+\]\])/g);
    return parts.map((p, n) => {
      if (!p.startsWith("[[") || !p.endsWith("]]")) return p;
      if (historical) return <span key={n}>{p}</span>;
      const [name, alias] = p.slice(2, -2).split("|");
      const link = links.find((l) => l.title === name?.trim());
      return link?.status === "resolved" && link.targetId !== null ? (
        <button
          className="knowledge-wikilink"
          key={n}
          onClick={() => {
            if (link.targetId !== null) onOpen(link.targetId);
          }}
        >
          {alias ?? name}
        </button>
      ) : (
        <span key={n} className="knowledge-unresolved" title={link?.status ?? "missing"}>
          {p} ({link?.status ?? "missing"})
        </span>
      );
    });
  }
  function nodes(tokens: readonly Token[]): ReactNode {
    return tokens.map((t, n) => <Fragment key={n}>{node(t)}</Fragment>);
  }
  function node(t: Token): ReactNode {
    switch (t.type) {
      case "checkbox": // Marked 18 emits this inside task items; the parent renders it.
      case "space":
        return null;
      case "heading": {
        const x = t as Tokens.Heading;
        const H = `h${String(x.depth)}` as "h1" | "h2" | "h3" | "h4" | "h5" | "h6";
        return <H>{nodes(x.tokens)}</H>;
      }
      case "paragraph":
        return <p>{nodes((t as Tokens.Paragraph).tokens)}</p>;
      case "knowledge_link":
        return text(t.raw);
      case "text": {
        const x = t as Tokens.Text;
        return x.tokens ? nodes(x.tokens) : text(x.text);
      }
      case "escape":
        return (t as Tokens.Escape).text;
      case "strong":
        return <strong>{nodes((t as Tokens.Strong).tokens)}</strong>;
      case "em":
        return <em>{nodes((t as Tokens.Em).tokens)}</em>;
      case "del":
        return <del>{nodes((t as Tokens.Del).tokens)}</del>;
      case "codespan":
        return <code>{(t as Tokens.Codespan).text}</code>;
      case "code":
        return (
          <pre>
            <code>{(t as Tokens.Code).text}</code>
          </pre>
        );
      case "blockquote":
        return <blockquote>{nodes((t as Tokens.Blockquote).tokens)}</blockquote>;
      case "br":
        return <br />;
      case "hr":
        return <hr />;
      case "list": {
        const x = t as Tokens.List;
        const items = x.items.map((i, n) => (
          <li key={n}>
            {i.task && (
              <input
                type="checkbox"
                checked={i.checked ?? false}
                disabled
                aria-label="Task status"
              />
            )}
            {nodes(i.tokens)}
          </li>
        ));
        return x.ordered ? (
          <ol start={typeof x.start === "number" ? x.start : 1}>{items}</ol>
        ) : (
          <ul>{items}</ul>
        );
      }
      case "table": {
        const x = t as Tokens.Table;
        return (
          <div className="knowledge-table">
            <table>
              <thead>
                <tr>
                  {x.header.map((c, n) => (
                    <th key={n}>{nodes(c.tokens)}</th>
                  ))}
                </tr>
              </thead>
              <tbody>
                {x.rows.map((row, n) => (
                  <tr key={n}>
                    {row.map((c, k) => (
                      <td key={k}>{nodes(c.tokens)}</td>
                    ))}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        );
      }
      // Never render raw HTML, URL anchors, images, styles or executable embeds.
      case "html":
      case "image":
      case "link":
        return <span>{t.raw}</span>;
      default:
        return t.raw;
    }
  }
  return (
    <div className="knowledge-markdown">{nodes(markdown.lexer(properties(content).body))}</div>
  );
}
