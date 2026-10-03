import conductor from "../../../assets/mascots/conductor/atlas.webp";
import type { AgentId } from "../../infrastructure/tauri/agent-chat-client";
import sprite0 from "../../../assets/mascots/personal-assistant.webp";
import sprite1 from "../../../assets/mascots/research.webp";
import sprite2 from "../../../assets/mascots/coding.webp";
import sprite3 from "../../../assets/mascots/cloud-infrastructure.webp";
import sprite4 from "../../../assets/mascots/systems-operations.webp";
import sprite5 from "../../../assets/mascots/knowledge-document.webp";
import sprite6 from "../../../assets/mascots/qa-validation.webp";
import sprite7 from "../../../assets/mascots/security-risk.webp";
import sprite8 from "../../../assets/mascots/workflow-automation.webp";

export const BOT_MASCOTS = {
  "personal-assistant": { src: sprite0, blinkSeconds: 4.7 },
  research: { src: sprite1, blinkSeconds: 5.3 },
  coding: { src: sprite2, blinkSeconds: 6.1 },
  "cloud-infrastructure": { src: sprite3, blinkSeconds: 5.7 },
  "systems-operations": { src: sprite4, blinkSeconds: 6.7 },
  "knowledge-document": { src: sprite5, blinkSeconds: 4.9 },
  "qa-validation": { src: sprite6, blinkSeconds: 7.1 },
  "security-risk": { src: sprite7, blinkSeconds: 5.9 },
  "workflow-automation": { src: sprite8, blinkSeconds: 6.3 },
} as const satisfies Record<AgentId, { readonly src: string; readonly blinkSeconds: number }>;

// Presentation-only coordinator: deliberately outside the canonical nine-AgentId map.
export const CONDUCTOR_MASCOT = { src: conductor, blinkSeconds: 6.5 };
