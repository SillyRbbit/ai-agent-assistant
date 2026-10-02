import { useEffect, useRef } from "react";
export function UnsavedKnowledgePrompt({
  onCancel,
  onDiscard,
}: {
  readonly onCancel: () => void;
  readonly onDiscard: () => void;
}) {
  const keep = useRef<HTMLButtonElement>(null);
  const discard = useRef<HTMLButtonElement>(null);
  useEffect(() => {
    const previous = document.activeElement;
    keep.current?.focus();
    return () => {
      if (previous instanceof HTMLElement) previous.focus();
    };
  }, []);
  return (
    <div className="knowledge-discard-backdrop">
      <section
        role="alertdialog"
        aria-modal="true"
        aria-label="Unsaved Knowledge edits"
        onKeyDown={(e) => {
          if (e.key === "Escape") {
            e.preventDefault();
            onCancel();
          }
          if (e.key === "Tab") {
            e.preventDefault();
            if (document.activeElement === keep.current) discard.current?.focus();
            else keep.current?.focus();
          }
        }}
      >
        <h2>Discard unsaved edits?</h2>
        <p>Your saved versions remain unchanged. Keep editing to save this draft first.</p>
        <button ref={keep} type="button" onClick={onCancel}>
          Keep editing
        </button>
        <button ref={discard} type="button" onClick={onDiscard}>
          Discard edits and continue
        </button>
      </section>
    </div>
  );
}
