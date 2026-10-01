import { createRoot } from "react-dom/client";
import { mockIPC } from "@tauri-apps/api/mocks";
import { App } from "../../src/App";
import { AGENT_IDS, DEFAULT_BOT_IDENTITY } from "../../src/infrastructure/tauri/agent-chat-client";
import { WORKFLOWS, type Workflow } from "../../src/infrastructure/tauri/collaboration-client";
import "../../src/styles.css";
Object.defineProperty(window, "isTauri", { value: true });
const names = ["Nova", "Mira", "Ada", "Atlas", "Orion", "Clio", "Vera", "Sable", "Tempo"];
const query = new URLSearchParams(location.search);
const long = query.has("long");
const roles = [
  "Personal Assistant",
  "Research Agent",
  "Coding Agent",
  "Cloud Infrastructure Agent",
  "Systems Operations Agent",
  "Knowledge & Document Agent",
  "QA & Validation Agent",
  "Security & Risk Agent",
  "Workflow Automation Agent",
];
const profiles = AGENT_IDS.map((agentId, i) => ({
  agentId,
  displayName: roles[i],
  identity: {
    ...DEFAULT_BOT_IDENTITY,
    nickname: long ? "界".repeat(48) : names[i],
    description: long
      ? "Long synthetic profile description. ".repeat(7)
      : "Synthetic browser QA profile",
    avatar: "compass",
    color: "teal",
  },
  connection: "simulation",
  model: "simulation",
  effort: "default",
  endpoint: "",
  localAuth: false,
  allowUnknownLocalityNotes: false,
  ownerInstructions: "PRIVATE SENTINEL DO NOT PROJECT",
  memoryMode: "off",
  note: "PRIVATE NOTE DO NOT PROJECT",
  revision: 1,
}));
let rooms = (Object.keys(WORKFLOWS) as Workflow[]).map((workflow, n) => ({
  id: n + 1,
  title: `Synthetic ${workflow}`,
  runs: ["completed", "cancelled", "partial", "interrupted"].map((status, k) => ({
    id: `room-${String(n + 1)}/run-${String(k + 1)}`,
    input: {
      workflow,
      objective: "Review synthetic supplied material",
      sources: [{ label: "S1", text: "Synthetic source" }],
    },
    status,
    sequence: 20,
    error: null,
    stages: WORKFLOWS[workflow].map((id, i) => {
      const p = profiles.find((p) => p.agentId === id);
      if (!p?.displayName) throw Error("Missing synthetic participant");
      return {
        id: `room-${String(n + 1)}/run-${String(k + 1)}/stage/${String(i)}`,
        participant: { ...p, role: p.displayName },
        status,
        timestamp: 1000,
        provisional: status === "completed" ? "" : "Synthetic provisional fragment — not validated",
        input: i ? "synthetic transmitted input" : "",
        handoff:
          status === "completed"
            ? {
                version: 1,
                stage: i,
                agentId: id,
                status: "complete",
                summary: "Synthetic schema-validated result",
                findings: [],
                evidence: ["S1"],
                limitations: ["Not live-provider evidence"],
              }
            : null,
      };
    }),
  })),
}));
if (query.has("active") || query.has("failed")) {
  const status = query.has("active") ? "running" : "failed";
  rooms = rooms.map((r, ri) => ({
    ...r,
    runs: r.runs.map((run, i) =>
      ri === 0 && i === 0
        ? {
            ...run,
            status,
            stages: run.stages.map((s, si) => ({
              ...s,
              status: si === 0 ? status : "queued",
              handoff: null,
              provisional: si === 0 ? "Synthetic provisional fragment — not validated" : "",
            })),
          }
        : run,
    ),
  }));
}
let unavailable = false;
const controls = document.createElement("div");
controls.setAttribute("role", "group");
controls.setAttribute("aria-label", "Synthetic harness controls");
for (const label of [
  "Advance synthetic snapshot",
  "Toggle unavailable snapshots",
  "Remove synthetic rooms",
]) {
  const button = document.createElement("button");
  button.textContent = label;
  button.onclick = () => {
    if (label === "Toggle unavailable snapshots") unavailable = !unavailable;
    else if (label === "Remove synthetic rooms") rooms = [];
    else
      rooms = rooms.map((r) => ({
        ...r,
        runs: r.runs.map((run) => ({
          ...run,
          status: "cancelled",
          sequence: run.sequence + 1,
          stages: run.stages.map((s) => ({ ...s, status: "cancelled" })),
        })),
      }));
  };
  controls.append(button);
}
document.body.prepend(controls);
mockIPC((command) => {
  if (command === "list_collaboration_rooms") {
    if (unavailable) throw Error("Synthetic unavailable");
    return rooms;
  }
  if (command === "list_agent_preferences") return profiles;
  if (command === "plugin:event|listen") return 0;
  if (command === "plugin:event|unlisten") return null;
  throw Error("Browser fixture blocks execution and unsupported IPC");
});
const root = document.getElementById("root");
if (root) createRoot(root).render(<App />);
