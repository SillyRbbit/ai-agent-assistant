import type { MockFinalAnswer } from "../../application/mockLoop";

interface FinalAnswerMessageProps {
  readonly answer: MockFinalAnswer;
}

export function FinalAnswerMessage({ answer }: FinalAnswerMessageProps) {
  return (
    <article
      aria-label="Mock final answer"
      className="conversation-message conversation-message--assistant conversation-final-answer"
    >
      <span>Final answer</span>
      <p>{answer.content}</p>
      <small>
        Deterministic frontend mock, turn {answer.modelTurn}, {answer.runId}
      </small>
    </article>
  );
}
