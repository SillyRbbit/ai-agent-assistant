import { PageState } from "../../components/PageState";
import { PageHeader } from "./PageHeader";

interface PlaceholderPageProps {
  readonly description: string;
  readonly emptyDescription: string;
  readonly emptyTitle: string;
  readonly eyebrow?: string;
  readonly headingId: string;
  readonly icon: string;
  readonly title: string;
}

export function PlaceholderPage({
  description,
  emptyDescription,
  emptyTitle,
  eyebrow,
  headingId,
  icon,
  title,
}: PlaceholderPageProps) {
  return (
    <section aria-labelledby={headingId} className="page-stack">
      <PageHeader description={description} eyebrow={eyebrow} headingId={headingId} title={title} />
      <div className="page-panel page-panel--centered">
        <PageState description={emptyDescription} icon={icon} title={emptyTitle} />
      </div>
    </section>
  );
}
