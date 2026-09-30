import {
  finalizeCommandCenterProjection,
  type CommandCenterAgentId,
  type CommandCenterProjection,
} from "./commandCenterProjection";

export type BotNameLoader = () => Promise<ReadonlyMap<CommandCenterAgentId, string>>;

// Names are presentation only: routing, ownership and simulated evidence stay canonical.
export function withBotNames(
  projection: CommandCenterProjection,
  names: ReadonlyMap<CommandCenterAgentId, string>,
): CommandCenterProjection {
  return finalizeCommandCenterProjection({
    ...projection,
    nodes: projection.nodes.map((node) => {
      if (node.kind !== "agent") return node;
      const nickname = names.get(node.agentId)?.trim();
      if (
        !nickname ||
        Array.from(nickname).length > 48 ||
        Array.from(nickname).some((c) => {
          const code = c.charCodeAt(0);
          return code < 32 || (code >= 127 && code <= 159);
        })
      )
        return node;
      return {
        ...node,
        label: nickname,
        inspector: {
          ...node.inspector,
          title: nickname,
          facts: [{ label: "Canonical role", value: node.label }, ...node.inspector.facts],
        },
      };
    }),
  });
}
