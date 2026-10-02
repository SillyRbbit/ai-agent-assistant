import type { Edge } from "@xyflow/react";
import type { KnowledgeItem } from "../../infrastructure/tauri/knowledge-client";
export function neighborhood(items: readonly KnowledgeItem[], selected: number) {
  const ids = new Set([selected]);
  const edges: Edge[] = [];
  for (const i of items)
    for (const l of i.versions.at(-1)?.links ?? []) {
      if (
        l.status !== "resolved" ||
        l.targetId === null ||
        (i.id !== selected && l.targetId !== selected)
      )
        continue;
      if (ids.size >= 25 && (!ids.has(i.id) || !ids.has(l.targetId))) continue;
      ids.add(i.id);
      ids.add(l.targetId);
      const id = `${String(i.id)}-${String(l.targetId)}`;
      if (!edges.some((e) => e.id === id))
        edges.push({ id, source: String(i.id), target: String(l.targetId) });
    }
  return { ids, edges };
}
