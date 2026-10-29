import { useId, useState } from "react";
import "./account-capacity.css";
import "./usage-disclosure.css";

type CapacityExample = "not-connected" | "unverified";

const CARDS = [
  {
    kind: "subscription",
    title: "Codex subscription allowance",
    description:
      "Account allowance, not a token balance. Headless is an execution mode, not a separate pool.",
    fields: ["Remaining allowance", "Reset window"],
  },
  {
    kind: "rate-limit",
    title: "API rate-limit capacity",
    description:
      "Short-window request and token limits. These are not credits or a spending budget.",
    fields: ["Requests remaining", "Tokens remaining", "Reset window"],
  },
  {
    kind: "spending",
    title: "API spending / balance",
    description:
      "Spending, configured budgets and credit balances are separate measures, not interchangeable limits.",
    fields: ["Reported spending", "Configured budget", "Credit balance", "Reporting period"],
  },
] as const;

export function AccountCapacityPanel() {
  const id = useId();
  const [example, setExample] = useState<CapacityExample>("not-connected");
  const status = example === "unverified" ? "Unverified" : "Not connected";

  return (
    <section className="account-capacity" aria-labelledby={`${id}-title`}>
      <header className="account-capacity__header">
        <div>
          <p className="account-capacity__eyebrow">Separate account scope</p>
          <h2 id={`${id}-title`}>Account Capacity preview</h2>
        </div>
        <span className="account-capacity__badge">Mock layout only</span>
      </header>
      <p className="account-capacity__disclosure">
        <strong>Mock account-capacity layout—not your balances.</strong> No account is connected or
        identified in this preview. This does not describe your other Cortexa connections. Future
        verified account limits may include activity outside Cortexa.
      </p>
      <div className="account-capacity__controls">
        <p role="status" aria-live="polite" aria-atomic="true">
          {example === "unverified"
            ? "Unverified illustration — no account check has been attempted; no values are shown."
            : "Not connected — no capacity source is connected to this preview."}
        </p>
        <details className="usage-disclosure">
          <summary>Capacity preview controls</summary>
          <label>
            Capacity illustration
            <select
              value={example}
              onChange={(event) => {
                const next = event.target.value;
                if (next === "not-connected" || next === "unverified") setExample(next);
              }}
            >
              <option value="not-connected">Not connected</option>
              <option value="unverified">Unverified illustration</option>
            </select>
          </label>
        </details>
      </div>
      <div className="account-capacity__cards">
        {CARDS.map((card) => (
          <article key={card.kind} aria-labelledby={`${id}-${card.kind}`}>
            <h3 id={`${id}-${card.kind}`}>{card.title}</h3>
            <p className="account-capacity__summary-status">{status}</p>
            <p>{card.description}</p>
            <details className="account-capacity__fields usage-disclosure">
              <summary aria-label={`View fields — ${card.title}`}>View fields</summary>
              <dl>
                {card.fields.map((field) => (
                  <div key={field}>
                    <dt>{field}</dt>
                    <dd>
                      <span className="account-capacity__status-badge">{status}</span>
                    </dd>
                  </div>
                ))}
              </dl>
            </details>
          </article>
        ))}
      </div>
      <p className="account-capacity__footnote">
        Activity source and date filters do not apply here. No remaining allowance or balance is
        calculated from token activity. Unknown is not zero. Provider and account scope remain
        unverified; these labels do not establish a supported connection.
      </p>
    </section>
  );
}
