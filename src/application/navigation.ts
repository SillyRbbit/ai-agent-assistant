export const APP_ROUTES = [
  "command-center",
  "conversations",
  "tasks",
  "memory",
  "activity",
  "integrations",
  "permissions",
  "settings",
] as const;

export type AppRoute = (typeof APP_ROUTES)[number];

export interface NavigationItem {
  readonly description: string;
  readonly label: string;
  readonly route: AppRoute;
}

export const NAVIGATION_ITEMS: readonly NavigationItem[] = [
  {
    description: "Inspect the deterministic multi-agent operations prototype.",
    label: "Command Center",
    route: "command-center",
  },
  {
    description: "Plan requests and work with your local assistant.",
    label: "Conversations",
    route: "conversations",
  },
  {
    description: "Review local tasks and future reminders.",
    label: "Tasks",
    route: "tasks",
  },
  {
    description: "Review working and approved preference memory.",
    label: "Memory",
    route: "memory",
  },
  {
    description: "Inspect local run and tool activity.",
    label: "Activity",
    route: "activity",
  },
  {
    description: "Manage explicitly connected services.",
    label: "Integrations",
    route: "integrations",
  },
  {
    description: "Review permission status without requesting access.",
    label: "Permissions",
    route: "permissions",
  },
  {
    description: "Review local settings and diagnostics.",
    label: "Settings",
    route: "settings",
  },
];

export const ASSISTANT_MENU_ROUTES = ["new_request", "tasks_placeholder"] as const;

export type AssistantMenuRoute = (typeof ASSISTANT_MENU_ROUTES)[number];

export function isAssistantMenuRoute(value: unknown): value is AssistantMenuRoute {
  return ASSISTANT_MENU_ROUTES.some((route) => route === value);
}

export function destinationForMenuRoute(route: AssistantMenuRoute): AppRoute {
  return route === "new_request" ? "conversations" : "tasks";
}
