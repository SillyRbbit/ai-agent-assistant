export const APP_ROUTES = [
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
  readonly glyph: string;
  readonly label: string;
  readonly route: AppRoute;
}

export const NAVIGATION_ITEMS: readonly NavigationItem[] = [
  {
    description: "Plan requests and work with your local assistant.",
    glyph: "C",
    label: "Conversations",
    route: "conversations",
  },
  {
    description: "Review local tasks and future reminders.",
    glyph: "T",
    label: "Tasks",
    route: "tasks",
  },
  {
    description: "Review working and approved preference memory.",
    glyph: "M",
    label: "Memory",
    route: "memory",
  },
  {
    description: "Inspect local run and tool activity.",
    glyph: "A",
    label: "Activity",
    route: "activity",
  },
  {
    description: "Manage explicitly connected services.",
    glyph: "I",
    label: "Integrations",
    route: "integrations",
  },
  {
    description: "Review permission status without requesting access.",
    glyph: "P",
    label: "Permissions",
    route: "permissions",
  },
  {
    description: "Review local settings and diagnostics.",
    glyph: "S",
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
