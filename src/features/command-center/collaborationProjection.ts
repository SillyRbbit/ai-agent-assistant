import { EXPECTED_AGENT_GROUPS } from "./commandCenterProjection";
import {
  AGENT_IDS,
  type AgentId,
  type AgentProfile,
  type BotIdentity,
} from "../../infrastructure/tauri/agent-chat-client";
import type { Run, Status } from "../../infrastructure/tauri/collaboration-client";
import { ROLE_DESCRIPTIONS } from "../agents/botAppearanceValues";
export interface OperationalCard {
  readonly id: string;
  readonly kind: "coordinator" | "bot" | "stage";
  readonly label: string;
  readonly role: string;
  readonly agentId: AgentId | null;
  readonly identity: BotIdentity | null;
  readonly description: string;
  readonly status: Status | "inactive" | "participant" | "roster";
  readonly settings: string;
  readonly attribution: string;
  readonly timestamp: number | null;
  readonly summary: string;
  readonly provisional: string;
  readonly sources: readonly string[];
  readonly limitations: readonly string[];
}
export interface OperationalLink {
  readonly id: string;
  readonly source: string;
  readonly target: string;
  readonly kind: "coordination" | "dependency" | "handoff";
}
export interface OperationalProjection {
  readonly cards: readonly OperationalCard[];
  readonly links: readonly OperationalLink[];
  readonly provenance: string;
  readonly error: string | null;
}
export function projectCollaboration(
  profiles: readonly AgentProfile[],
  run: Run | null,
): OperationalProjection {
  const cards: OperationalCard[] = [
    {
      id: "AgentOrchestrator",
      kind: "coordinator",
      label: "Conductor",
      role: "Application coordinator",
      agentId: null,
      identity: null,
      description:
        "Application-owned coordination. Not a bot. Graph inspection grants no execution authority.",
      status: run?.status ?? "roster",
      settings: "No model or provider",
      attribution: "AgentOrchestrator",
      timestamp: null,
      summary: "",
      provisional: "",
      sources: [],
      limitations: [],
    },
  ];
  for (const id of AGENT_IDS) {
    const profile = profiles.find((p) => p.agentId === id);
    const participant = run?.stages.find((s) => s.participant.agentId === id)?.participant;
    const identity = run ? participant?.identity : profile?.identity;
    const role = participant?.role ?? profile?.displayName ?? id;
    cards.push({
      id: `bot:${id}`,
      kind: "bot",
      agentId: id,
      identity: identity ?? null,
      label: identity && identity.nickname.length > 0 ? identity.nickname : role,
      role,
      description: identity
        ? identity.description || ROLE_DESCRIPTIONS[id]
        : run
          ? "Not a participant in this run; no historical identity recorded."
          : "Current profile unavailable.",
      status: run ? (participant ? "participant" : "inactive") : "roster",
      settings: participant
        ? `${participant.connection} · ${participant.model} · ${participant.effort}`
        : !run && profile
          ? `${profile.connection} · ${profile.model} · ${profile.effort}`
          : "Not recorded for this run",
      attribution: participant
        ? "Saved participant identity"
        : run
          ? "Inactive canonical roster member"
          : "Current saved profile",
      timestamp: null,
      summary: "",
      provisional: "",
      sources: [],
      limitations: [],
    });
  }
  const links: OperationalLink[] = [];
  for (const [i, s] of (run?.stages ?? []).entries()) {
    cards.push({
      id: s.id,
      kind: "stage",
      agentId: s.participant.agentId,
      label: `Stage ${String(i + 1)} · ${s.participant.identity.nickname || s.participant.role}`,
      role: s.participant.role,
      identity: s.participant.identity,
      description: s.participant.identity.description || ROLE_DESCRIPTIONS[s.participant.agentId],
      status: s.status,
      settings: `${s.participant.connection} · ${s.participant.model} · ${s.participant.effort}`,
      attribution: "Saved stage participant; separate execution identity",
      timestamp: s.timestamp,
      summary: s.handoff?.summary ?? "",
      provisional: s.provisional,
      sources: s.handoff?.evidence ?? [],
      limitations: s.handoff?.limitations ?? [],
    });
    links.push({
      id: `coord:${s.id}`,
      source: "AgentOrchestrator",
      target: s.id,
      kind: "coordination",
    });
    const prior = run?.stages[i - 1];
    if (prior) {
      links.push({ id: `dependency:${s.id}`, source: prior.id, target: s.id, kind: "dependency" });
      // A validated partial result stops the route; it is not a consumed handoff.
      if (prior.handoff?.status === "complete" && s.input.length > 0)
        links.push({ id: `handoff:${s.id}`, source: prior.id, target: s.id, kind: "handoff" });
    }
  }
  return {
    cards,
    links,
    error: run?.error ?? null,
    provenance: !run
      ? "Current bot roster · no execution"
      : run.stages.every((s) => s.participant.connection === "simulation")
        ? "Simulation configuration · no provider calls"
        : "Live-provider configuration · configuration alone does not establish provider success",
  };
}

export interface OperationalFilters {
  search: string;
  status: string;
  kind: string;
  group: string;
  participant: string;
}
export function matchesCard(c: OperationalCard, f: OperationalFilters): boolean {
  return (
    (f.kind === "all" || c.kind === f.kind) &&
    (f.status === "all" || c.status === f.status) &&
    (f.participant === "all" || c.agentId === f.participant) &&
    (f.group === "all" || (c.agentId !== null && EXPECTED_AGENT_GROUPS[c.agentId] === f.group)) &&
    `${c.label} ${c.role} ${c.agentId ?? ""}`
      .toLocaleLowerCase()
      .includes(f.search.toLocaleLowerCase())
  );
}
