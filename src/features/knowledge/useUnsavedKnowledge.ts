import { useCallback, useEffect, useState } from "react";
export function allowKnowledgeNavigation(proceed: () => void = () => undefined) {
  return window.dispatchEvent(
    new CustomEvent("cortexa-knowledge-leave", { cancelable: true, detail: proceed }),
  );
}
export function useUnsavedKnowledge(dirty: boolean, busy = false) {
  const [pending, setPending] = useState<(() => void) | null>(null);
  const attempt = useCallback(
    (action: () => void) => {
      if (busy) return;
      if (dirty) setPending(() => action);
      else action();
    },
    [dirty, busy],
  );
  useEffect(() => {
    if (!dirty && !busy) return;
    const leave = (event: Event) => {
      event.preventDefault();
      const action: unknown = (event as CustomEvent<unknown>).detail;
      if (!busy && typeof action === "function") setPending(() => action as () => void);
    };
    const unload = (event: BeforeUnloadEvent) => {
      event.preventDefault();
    };
    window.addEventListener("cortexa-knowledge-leave", leave);
    window.addEventListener("beforeunload", unload);
    return () => {
      window.removeEventListener("cortexa-knowledge-leave", leave);
      window.removeEventListener("beforeunload", unload);
    };
  }, [dirty, busy]);
  return {
    pending: pending !== null,
    attempt,
    cancel: () => {
      setPending(null);
    },
    discard: () => {
      if (!busy) {
        setPending(null);
        pending?.();
      }
    },
  };
}
