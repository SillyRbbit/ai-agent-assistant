import type {
  CommandCenterEventId,
  CommandCenterEventKind,
  CommandCenterEventSeverity,
  CommandCenterProjection,
  DemoOrigin,
} from "./commandCenterProjection";

export const COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE = "Unavailable in fixture data" as const;

export function commandCenterEventSeverityLabel(value: string): string {
  if (value === "danger") return "Error";
  return value
    .split("-")
    .map((part) => `${part.slice(0, 1).toUpperCase()}${part.slice(1)}`)
    .join(" ");
}

export interface CommandCenterEventPresentation {
  readonly associatedAgentLabel: string | null;
  readonly demoOrigin: DemoOrigin;
  readonly description: string;
  readonly eventId: CommandCenterEventId;
  readonly eventKind: CommandCenterEventKind;
  readonly ordinal: number;
  readonly relatedEntityLabels: readonly string[];
  readonly redaction: string;
  readonly severity: CommandCenterEventSeverity;
  readonly simulatedAt: string;
  readonly sourceLabel: string;
  readonly statusLabel: typeof COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
  readonly summary: string;
  readonly targetLabel: typeof COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE;
  readonly taskLabel: string | null;
  readonly timeLabel: string;
  readonly workflowLabel: string | null;
}

export function buildCommandCenterEventPresentations(
  projection: CommandCenterProjection,
): readonly CommandCenterEventPresentation[] {
  const agentLabelById = new Map(
    projection.nodes
      .filter((node) => node.kind === "agent")
      .map((node) => [node.agentId, node.label] as const),
  );
  const nodeLabelById = new Map(projection.nodes.map((node) => [node.id, node.label] as const));

  return projection.events.map((event) => {
    const associatedAgentLabel =
      event.agentId === null ? null : (agentLabelById.get(event.agentId) ?? event.agentId);
    return {
      associatedAgentLabel,
      demoOrigin: event.demoOrigin,
      description: event.detail,
      eventId: event.id,
      eventKind: event.kind,
      ordinal: event.ordinal,
      relatedEntityLabels: event.relatedNodeIds.map((id) => nodeLabelById.get(id) ?? id),
      redaction: event.redaction,
      severity: event.severity,
      simulatedAt: event.simulatedAt,
      sourceLabel: COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
      statusLabel: COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
      summary: event.summary,
      targetLabel: COMMAND_CENTER_EVENT_FIELD_UNAVAILABLE,
      taskLabel:
        event.taskNodeId === null
          ? null
          : (nodeLabelById.get(event.taskNodeId) ?? event.taskNodeId),
      timeLabel: event.timeLabel,
      workflowLabel:
        event.workflowNodeId === null
          ? null
          : (nodeLabelById.get(event.workflowNodeId) ?? event.workflowNodeId),
    };
  });
}
