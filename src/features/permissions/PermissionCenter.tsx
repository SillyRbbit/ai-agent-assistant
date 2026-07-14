import { PageHeader } from "../shared/PageHeader";

interface PermissionRow {
  readonly description: string;
  readonly name: string;
  readonly status: "Disabled in Phase 2" | "Not requested";
}

const PERMISSIONS: readonly PermissionRow[] = [
  {
    description: "Future calendar event listing and personal event changes.",
    name: "Calendar",
    status: "Not requested",
  },
  {
    description: "Future personal reminder creation and updates.",
    name: "Reminders",
    status: "Not requested",
  },
  {
    description: "Future user-directed contact search.",
    name: "Contacts",
    status: "Not requested",
  },
  {
    description: "Future completion alerts requested by the user.",
    name: "Notifications",
    status: "Not requested",
  },
  {
    description: "Future access to files and folders explicitly selected by the user.",
    name: "Selected files",
    status: "Not requested",
  },
  {
    description: "Privileged interface automation is outside the current phase.",
    name: "Accessibility",
    status: "Disabled in Phase 2",
  },
  {
    description: "Screen context is outside the current phase.",
    name: "Screen Recording",
    status: "Disabled in Phase 2",
  },
  {
    description: "Apple Events automation is outside the current phase.",
    name: "Automation",
    status: "Disabled in Phase 2",
  },
  {
    description: "Voice input is outside the current phase.",
    name: "Microphone",
    status: "Disabled in Phase 2",
  },
];

export function PermissionCenter() {
  return (
    <section aria-labelledby="permissions-page-title" className="page-stack">
      <PageHeader
        description="Review why a capability may be needed before the application ever requests it."
        eyebrow="Privacy and control"
        headingId="permissions-page-title"
        title="Permissions"
      />

      <section aria-label="Permission status list" className="permission-card page-panel">
        <div className="permission-card__header">
          <div>
            <h2>No device permission has been requested</h2>
            <p>This shell never triggers an operating-system permission prompt.</p>
          </div>
          <span>0 granted</span>
        </div>

        <ul className="permission-list">
          {PERMISSIONS.map((permission) => (
            <li key={permission.name}>
              <div className="permission-list__icon" aria-hidden="true">
                {permission.name.slice(0, 1)}
              </div>
              <div className="permission-list__copy">
                <strong>{permission.name}</strong>
                <span>{permission.description}</span>
              </div>
              <span
                className={
                  permission.status === "Disabled in Phase 2"
                    ? "permission-status permission-status--disabled"
                    : "permission-status"
                }
              >
                {permission.status}
              </span>
            </li>
          ))}
        </ul>
      </section>
    </section>
  );
}
