import type { AgentId } from "../../infrastructure/tauri/agent-chat-client";
export const BOT_COLOR_VALUES = {
  blue: "#67aaff",
  teal: "#40d8c2",
  green: "#60d68b",
  orange: "#ffae55",
  coral: "#ff9585",
  pink: "#ff9cd4",
  purple: "#c5a0ff",
  gold: "#f3d05b",
  cyan: "#65dfff",
  lime: "#b8de63",
  rose: "#ff9eb1",
  indigo: "#a5b4ff",
} as const;
export const ROLE_DESCRIPTIONS: Readonly<Record<AgentId, string>> = {
  "personal-assistant":
    "Clarifies the objective, coordinates the selected workflow, and synthesizes results.",
  research: "Analyzes supplied evidence, compares findings, and identifies uncertainties.",
  coding: "Reviews supplied code and proposes technical solutions or changes.",
  "cloud-infrastructure":
    "Assesses supplied infrastructure configurations and proposes improvements.",
  "systems-operations":
    "Analyzes supplied operational information and proposes diagnostic or recovery procedures.",
  "knowledge-document":
    "Organizes supplied information into summaries, structured documents, and reusable drafts.",
  "qa-validation":
    "Reviews outputs against requirements and evidence, identifying gaps and proposed tests.",
  "security-risk":
    "Assesses supplied material for security concerns, assumptions, and mitigations.",
  "workflow-automation":
    "Designs workflow proposals with steps, dependencies, approval points, and acceptance criteria.",
};
