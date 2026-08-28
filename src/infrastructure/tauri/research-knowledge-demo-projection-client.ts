import { invoke } from "@tauri-apps/api/core";

export const RESEARCH_KNOWLEDGE_DEMO_DISCLOSURE = "DEMO MODE · SIMULATED AGENT DATA" as const;
export const RESEARCH_KNOWLEDGE_DEMO_PROOF_BOUNDARY =
  "Command Center, Conversations mock, and Rust acceptance workflows are separate deterministic proofs." as const;

export interface ResearchKnowledgeDemoRole {
  readonly id: "personal-assistant" | "research" | "knowledge-document";
  readonly label: "Personal Assistant" | "Research Agent" | "Knowledge & Document Agent";
  readonly state: "ready";
}

export interface ResearchKnowledgeDemoProjection {
  readonly schemaVersion: "research-knowledge-demo-projection-v1";
  readonly scenarioId: "research-knowledge-demo-v1";
  readonly disclosure: typeof RESEARCH_KNOWLEDGE_DEMO_DISCLOSURE;
  readonly proofBoundary: typeof RESEARCH_KNOWLEDGE_DEMO_PROOF_BOUNDARY;
  readonly fixtureProvenance: "application-owned-synthetic-fixture";
  readonly roles: readonly [
    ResearchKnowledgeDemoRole,
    ResearchKnowledgeDemoRole,
    ResearchKnowledgeDemoRole,
  ];
  readonly simulatedOutcomes: readonly ["succeeded", "failed", "cancelled"];
}

export type ResearchKnowledgeDemoProjectionLoader = () => Promise<ResearchKnowledgeDemoProjection>;

const PROJECTION_FIELDS = [
  "schemaVersion",
  "scenarioId",
  "disclosure",
  "proofBoundary",
  "fixtureProvenance",
  "roles",
  "simulatedOutcomes",
] as const;
const ROLE_FIELDS = ["id", "label", "state"] as const;
const EXPECTED_ROLES = [
  { id: "personal-assistant", label: "Personal Assistant", state: "ready" },
  { id: "research", label: "Research Agent", state: "ready" },
  { id: "knowledge-document", label: "Knowledge & Document Agent", state: "ready" },
] as const;
const EXPECTED_OUTCOMES = ["succeeded", "failed", "cancelled"] as const;

function isRecord(value: unknown): value is Record<string, unknown> {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function isUnknownArray(value: unknown): value is readonly unknown[] {
  return Array.isArray(value);
}

function hasExactKeys(record: Record<string, unknown>, fields: readonly string[]): boolean {
  const keys = Object.keys(record);
  return keys.length === fields.length && keys.every((key) => fields.includes(key));
}

function isBoundedString(value: unknown, maximum: number): value is string {
  return typeof value === "string" && value.length > 0 && Array.from(value).length <= maximum;
}

export function parseResearchKnowledgeDemoProjection(
  value: unknown,
): ResearchKnowledgeDemoProjection | undefined {
  if (!isRecord(value) || !hasExactKeys(value, PROJECTION_FIELDS)) return undefined;

  const roles = value["roles"];
  const simulatedOutcomes = value["simulatedOutcomes"];

  if (
    !isBoundedString(value["schemaVersion"], 128) ||
    value["schemaVersion"] !== "research-knowledge-demo-projection-v1" ||
    !isBoundedString(value["scenarioId"], 128) ||
    value["scenarioId"] !== "research-knowledge-demo-v1" ||
    !isBoundedString(value["disclosure"], 128) ||
    value["disclosure"] !== RESEARCH_KNOWLEDGE_DEMO_DISCLOSURE ||
    !isBoundedString(value["proofBoundary"], 160) ||
    value["proofBoundary"] !== RESEARCH_KNOWLEDGE_DEMO_PROOF_BOUNDARY ||
    !isBoundedString(value["fixtureProvenance"], 128) ||
    value["fixtureProvenance"] !== "application-owned-synthetic-fixture" ||
    !isUnknownArray(roles) ||
    roles.length !== EXPECTED_ROLES.length ||
    !isUnknownArray(simulatedOutcomes) ||
    simulatedOutcomes.length !== EXPECTED_OUTCOMES.length
  ) {
    return undefined;
  }

  for (const [index, expected] of EXPECTED_ROLES.entries()) {
    const role = roles[index];
    if (
      !isRecord(role) ||
      !hasExactKeys(role, ROLE_FIELDS) ||
      !isBoundedString(role["id"], 128) ||
      !isBoundedString(role["label"], 128) ||
      !isBoundedString(role["state"], 128) ||
      role["id"] !== expected.id ||
      role["label"] !== expected.label ||
      role["state"] !== expected.state
    ) {
      return undefined;
    }
  }

  if (!EXPECTED_OUTCOMES.every((expected, index) => simulatedOutcomes[index] === expected)) {
    return undefined;
  }

  return Object.freeze({
    schemaVersion: "research-knowledge-demo-projection-v1",
    scenarioId: "research-knowledge-demo-v1",
    disclosure: RESEARCH_KNOWLEDGE_DEMO_DISCLOSURE,
    proofBoundary: RESEARCH_KNOWLEDGE_DEMO_PROOF_BOUNDARY,
    fixtureProvenance: "application-owned-synthetic-fixture",
    roles: Object.freeze([
      Object.freeze({ id: "personal-assistant", label: "Personal Assistant", state: "ready" }),
      Object.freeze({ id: "research", label: "Research Agent", state: "ready" }),
      Object.freeze({
        id: "knowledge-document",
        label: "Knowledge & Document Agent",
        state: "ready",
      }),
    ] as const),
    simulatedOutcomes: Object.freeze(["succeeded", "failed", "cancelled"] as const),
  });
}

export async function fetchResearchKnowledgeDemoProjection(): Promise<ResearchKnowledgeDemoProjection> {
  try {
    const result = await invoke<unknown>("get_research_knowledge_demo_projection");
    const projection = parseResearchKnowledgeDemoProjection(result);

    if (projection !== undefined) {
      return projection;
    }
  } catch {
    // Collapse native and transport details into one application-owned error.
  }

  throw new Error("Research/Knowledge demo projection unavailable.");
}
