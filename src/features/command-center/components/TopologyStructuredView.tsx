import { ListTree } from "lucide-react";

export interface StructuredNodeView {
  readonly groupId: string | null;
  readonly id: string;
  readonly kind: string;
  readonly label: string;
  readonly status: string;
}

export interface StructuredGroupView {
  readonly id: string;
  readonly description: string;
  readonly label: string;
}

export interface StructuredEdgeView {
  readonly id: string;
  readonly kind: string;
  readonly label: string;
  readonly sourceId: string;
  readonly sourceLabel: string;
  readonly status: string;
  readonly targetId: string;
  readonly targetLabel: string;
}

interface TopologyStructuredViewProps {
  readonly edges: readonly StructuredEdgeView[];
  readonly groups: readonly StructuredGroupView[];
  readonly nodes: readonly StructuredNodeView[];
  readonly onSelect: (id: string) => void;
  readonly selectedId: string | null;
}

function readable(value: string): string {
  return value.replaceAll("-", " ");
}

export function TopologyStructuredView({
  edges,
  groups,
  nodes,
  onSelect,
  selectedId,
}: TopologyStructuredViewProps) {
  const ungrouped = nodes.filter((node) => node.groupId === null);

  return (
    <div className="command-center-structured" data-scroll-region="command-center-structured">
      <p className="command-center-toolbar__help">
        <ListTree aria-hidden="true" size={14} /> Every visible graph entity and relationship is
        available here without canvas interaction.
      </p>

      {ungrouped.length > 0 ? (
        <section
          aria-labelledby="command-center-authority-group"
          className="command-center-structured__group"
        >
          <h3 id="command-center-authority-group">Application authority and work</h3>
          <ul className="command-center-structured__list">
            {ungrouped.map((node) => (
              <li key={node.id}>
                <button
                  aria-pressed={selectedId === node.id}
                  className="command-center-entity-button"
                  onClick={() => {
                    onSelect(node.id);
                  }}
                  type="button"
                >
                  <strong>{node.label}</strong>
                  <small>
                    {readable(node.kind)} · {readable(node.status)}
                  </small>
                </button>
              </li>
            ))}
          </ul>
        </section>
      ) : null}

      {groups.map((group) => {
        const groupNodes = nodes.filter((node) => node.groupId === group.id);
        if (groupNodes.length === 0) {
          return null;
        }

        return (
          <section
            aria-labelledby={`command-center-group-${group.id}`}
            className="command-center-structured__group"
            key={group.id}
          >
            <h3 id={`command-center-group-${group.id}`}>
              <button
                aria-pressed={selectedId === group.id}
                className="command-center-entity-button"
                onClick={() => {
                  onSelect(group.id);
                }}
                type="button"
              >
                <strong>{group.label}</strong>
                <small>{group.description}</small>
              </button>
            </h3>
            <ul className="command-center-structured__list">
              {groupNodes.map((node) => (
                <li key={node.id}>
                  <button
                    aria-pressed={selectedId === node.id}
                    className="command-center-entity-button"
                    onClick={() => {
                      onSelect(node.id);
                    }}
                    type="button"
                  >
                    <strong>{node.label}</strong>
                    <small>
                      {readable(node.kind)} · {readable(node.status)}
                    </small>
                  </button>
                </li>
              ))}
            </ul>
          </section>
        );
      })}

      <section
        aria-labelledby="command-center-relationships"
        className="command-center-relationship"
      >
        <h3 id="command-center-relationships">Relationship table</h3>
        <div className="command-center-relationship__table-wrap">
          <table>
            <thead>
              <tr>
                <th scope="col">Source</th>
                <th scope="col">Relationship</th>
                <th scope="col">Target</th>
                <th scope="col">State</th>
              </tr>
            </thead>
            <tbody>
              {edges.map((edge) => (
                <tr key={edge.id}>
                  <td>
                    <button
                      onClick={() => {
                        onSelect(edge.sourceId);
                      }}
                      type="button"
                    >
                      {edge.sourceLabel}
                    </button>
                  </td>
                  <td>
                    <button
                      aria-pressed={selectedId === edge.id}
                      onClick={() => {
                        onSelect(edge.id);
                      }}
                      type="button"
                    >
                      {edge.label} ({readable(edge.kind)})
                    </button>
                  </td>
                  <td>
                    <button
                      onClick={() => {
                        onSelect(edge.targetId);
                      }}
                      type="button"
                    >
                      {edge.targetLabel}
                    </button>
                  </td>
                  <td>{readable(edge.status)}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </section>
    </div>
  );
}
