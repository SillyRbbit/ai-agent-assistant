interface PageStateProps {
  readonly description: string;
  readonly icon: string;
  readonly title: string;
  readonly tone?: "empty" | "error" | "loading";
}

export function PageState({ description, icon, title, tone = "empty" }: PageStateProps) {
  return (
    <section
      aria-live={tone === "loading" ? "polite" : undefined}
      className={`page-state page-state--${tone}`}
      role={tone === "error" ? "alert" : undefined}
    >
      <div className="page-state__icon" aria-hidden="true">
        {icon}
      </div>
      <h2>{title}</h2>
      <p>{description}</p>
    </section>
  );
}
