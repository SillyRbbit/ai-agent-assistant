import { useState } from "react";
import type { Run } from "../../infrastructure/tauri/collaboration-client";
import { CoordinatorAvatar } from "./BotAppearance";

function conductorWorking(run: Run | null, current: boolean): boolean {
  return (
    current &&
    run?.status === "running" &&
    run.error === null &&
    run.stages.some((stage) => stage.status === "running") &&
    run.stages.every((stage) => ["queued", "running", "completed"].includes(stage.status))
  );
}
function conductorSucceeded(previous: Run | null, next: Run | null): boolean {
  return (
    previous !== null &&
    next !== null &&
    previous.id === next.id &&
    next.sequence > previous.sequence &&
    ["running", "queued"].includes(previous.status) &&
    next.status === "completed" &&
    next.error === null &&
    next.stages.length > 0 &&
    next.stages.every(
      (stage) => stage.status === "completed" && stage.handoff?.status === "complete",
    )
  );
}
// Observe accepted application snapshots only. Mounting/revisiting history establishes
// a baseline, never a completion event. No provider action or polling is introduced.
export function ConductorIdentity({
  run = null,
  current = false,
  expressive = false,
}: {
  readonly run?: Run | null;
  readonly current?: boolean;
  readonly expressive?: boolean;
}) {
  const [observed, setObserved] = useState({
    run,
    current,
    successes: 0,
    settled: run?.status === "completed",
  });
  if (observed.run !== run || observed.current !== current) {
    setObserved({
      run,
      current,
      settled: (observed.run?.id === run?.id && observed.settled) || run?.status === "completed",
      successes:
        observed.successes +
        (current && observed.current && !observed.settled && conductorSucceeded(observed.run, run)
          ? 1
          : 0),
    });
  }
  return (
    <div className="conductor-summary">
      <CoordinatorAvatar
        key={run?.id ?? "roster"}
        expressive={expressive}
        greeting={expressive}
        success={observed.successes}
        working={conductorWorking(run, current)}
      />
      <p>Conductor · Application coordinator</p>
      <p>
        {run ? `Overall workflow: ${run.status}` : "No coordinated workflow selected"}
        {!current && run
          ? " · historical or unavailable snapshot; no current activity claimed"
          : ""}
      </p>
    </div>
  );
}
