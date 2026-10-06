interface PageHeaderProps {
  readonly description: string;
  readonly badge?: string;
  readonly eyebrow?: string | undefined;
  readonly headingId: string;
  readonly title: string;
}

export function PageHeader({
  description,
  badge,
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
      {badge && <span className="page-header__badge">{badge}</span>}
    </header>
  );
}
