import type { ReactNode } from "react";

import { ApplicationStateProvider } from "./application/ApplicationStateProvider";
import { NAVIGATION_ITEMS, type AppRoute } from "./application/navigation";
import { useMockAssistantRun } from "./application/useMockAssistantRun";
import { useCoreConnection, type AppInfoLoader } from "./application/useCoreConnection";
import {
  useMenuRouteSubscription,
  type MenuRouteConnectionStatus,
} from "./application/useMenuRouteSubscription";
import { useApplicationDispatch, useApplicationState } from "./application/useApplicationState";
import { ApplicationSidebar } from "./components/ApplicationSidebar";
import { ConversationWorkspace } from "./features/conversations/ConversationWorkspace";
import { PermissionCenter } from "./features/permissions/PermissionCenter";
import { SettingsPage } from "./features/settings/SettingsPage";
import { PlaceholderPage } from "./features/shared/PlaceholderPage";
import { TasksPage } from "./features/tasks/TasksPage";
import { fetchAppInfo } from "./infrastructure/tauri/app-info-client";
import {
  tauriMenuRouteSource,
  type MenuRouteSource,
} from "./infrastructure/tauri/menu-route-client";

export interface AppServices {
  readonly appInfoLoader: AppInfoLoader;
  readonly menuRouteSource: MenuRouteSource;
}

interface AppProps {
  readonly services?: AppServices;
}

const DEFAULT_APP_SERVICES: AppServices = {
  appInfoLoader: fetchAppInfo,
  menuRouteSource: tauriMenuRouteSource,
};

export function App({ services = DEFAULT_APP_SERVICES }: AppProps) {
  return (
    <ApplicationStateProvider>
      <ApplicationShell services={services} />
    </ApplicationStateProvider>
  );
}

interface ApplicationShellProps {
  readonly services: AppServices;
}

function ApplicationShell({ services }: ApplicationShellProps) {
  const state = useApplicationState();
  const dispatch = useApplicationDispatch();
  const coreConnection = useCoreConnection(services.appInfoLoader);
  const menuRouteStatus = useMenuRouteSubscription(services.menuRouteSource);
  const mockRunActions = useMockAssistantRun(state.activeRun, dispatch);
  const runStatus = state.activeRun?.status ?? "idle";

  const pages: Readonly<Record<AppRoute, ReactNode>> = {
    activity: (
      <PlaceholderPage
        description="A transparent local history of runs, context, tools, approvals, and results."
        emptyDescription="Mock activity stays with the current conversation until the reviewed audit model is introduced."
        emptyTitle="No persisted activity"
        eyebrow="Transparency"
        headingId="activity-page-title"
        icon="A"
        title="Activity"
      />
    ),
    conversations: (
      <ConversationWorkspace
        activeApproval={state.activeApproval}
        composerDraft={state.composerDraft}
        messages={state.messages}
        onApprovalDecision={mockRunActions.decideApproval}
        onComposerDraftChange={(value) => {
          dispatch({ type: "composer-draft-changed", value });
        }}
        onStop={mockRunActions.stop}
        onSubmit={mockRunActions.submit}
        runStatus={runStatus}
        toolActivities={state.toolActivities}
      />
    ),
    integrations: (
      <PlaceholderPage
        description="Explicitly connected services will be managed from this page."
        emptyDescription="No account, OAuth token, API key, or cloud service is configured."
        emptyTitle="No integrations connected"
        eyebrow="Connections"
        headingId="integrations-page-title"
        icon="I"
        title="Integrations"
      />
    ),
    memory: (
      <PlaceholderPage
        description="User-controlled session, working, and preference memory will appear here."
        emptyDescription="Nothing is stored until memory controls and persistence are reviewed."
        emptyTitle="No saved memory"
        eyebrow="Context"
        headingId="memory-page-title"
        icon="M"
        title="Memory"
      />
    ),
    permissions: <PermissionCenter />,
    settings: <SettingsPage coreConnection={coreConnection} menuRouteStatus={menuRouteStatus} />,
    tasks: <TasksPage />,
  };

  const activeLabel = NAVIGATION_ITEMS.find((item) => item.route === state.activeRoute)?.label;

  return (
    <div className="application-shell">
      <ApplicationSidebar
        activeRoute={state.activeRoute}
        onNavigate={(route) => {
          dispatch({ route, type: "navigate" });
        }}
      />

      <main className="application-main" id="main-content" tabIndex={-1}>
        <div className="application-toolbar">
          <p className="application-toolbar__location">{activeLabel ?? "Workspace"}</p>
          <CoreStatus connectionStatus={coreConnection.status} menuRouteStatus={menuRouteStatus} />
        </div>
        <div className="application-content">{pages[state.activeRoute]}</div>
      </main>
    </div>
  );
}

interface CoreStatusProps {
  readonly connectionStatus: "checking" | "error" | "ready";
  readonly menuRouteStatus: MenuRouteConnectionStatus;
}

function CoreStatus({ connectionStatus, menuRouteStatus }: CoreStatusProps) {
  const hasError = connectionStatus === "error" || menuRouteStatus === "error";
  const isReady = connectionStatus === "ready" && menuRouteStatus === "ready";
  const label = hasError
    ? "Diagnostics need attention"
    : isReady
      ? "Local core ready"
      : "Connecting";
  const state = hasError ? "error" : isReady ? "ready" : "checking";

  return (
    <p className={`application-toolbar__status application-toolbar__status--${state}`}>
      <span aria-hidden="true" />
      {label}
    </p>
  );
}
