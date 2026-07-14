interface PageHeaderProps {
  readonly description: string;
  readonly eyebrow?: string | undefined;
  readonly headingId: string;
  readonly title: string;
}

export function PageHeader({
  description,
  eyebrow = "Workspace",
  headingId,
  title,
}: PageHeaderProps) {
  return (
    <header className="page-header">
      <div>
        <p className="section-kicker">{eyebrow}</p>
        <h1 id={headingId}>{title}</h1>
        <p>{description}</p>
      </div>
      <span className="local-only-badge">Local only</span>
    </header>
  );
}
