import { lazy, Suspense, useEffect, useRef, type ReactNode } from "react";

import { ApplicationStateProvider } from "./application/ApplicationStateProvider";
import { NAVIGATION_ITEMS, type AppRoute } from "./application/navigation";
import { browserMockRunDriver, type MockRunDriver } from "./application/mockRunDriver";
import { useMockAssistantRun } from "./application/useMockAssistantRun";
import { useCoreConnection, type AppInfoLoader } from "./application/useCoreConnection";
import {
  useMenuRouteSubscription,
  type MenuRouteConnectionStatus,
} from "./application/useMenuRouteSubscription";
import { useApplicationDispatch, useApplicationState } from "./application/useApplicationState";
import { ApplicationSidebar } from "./components/ApplicationSidebar";
import { PageState } from "./components/PageState";
import { ActivityPage } from "./features/activity/ActivityPage";
import { ConversationWorkspace } from "./features/conversations/ConversationWorkspace";
import { PermissionCenter } from "./features/permissions/PermissionCenter";
import { SettingsPage } from "./features/settings/SettingsPage";
import { PlaceholderPage } from "./features/shared/PlaceholderPage";
import { PageHeader } from "./features/shared/PageHeader";
import { TasksPage } from "./features/tasks/TasksPage";
import { fetchAppInfo } from "./infrastructure/tauri/app-info-client";
import {
  tauriMenuRouteSource,
  type MenuRouteSource,
} from "./infrastructure/tauri/menu-route-client";

const CommandCenterPage = lazy(() => import("./features/command-center/CommandCenterPage"));

export interface AppServices {
  readonly appInfoLoader: AppInfoLoader;
  readonly menuRouteSource: MenuRouteSource;
  readonly mockRunDriver: MockRunDriver;
}

interface AppProps {
  readonly services?: AppServices;
}

const DEFAULT_APP_SERVICES: AppServices = {
  appInfoLoader: fetchAppInfo,
  menuRouteSource: tauriMenuRouteSource,
  mockRunDriver: browserMockRunDriver,
};

export function App({ services = DEFAULT_APP_SERVICES }: AppProps) {
  return (
    <ApplicationStateProvider>
      <ApplicationShell services={services} />
    </ApplicationStateProvider>
  );
}

function CommandCenterLoadingPage() {
  return (
    <section aria-busy="true" aria-labelledby="command-center-loading-title" className="page-stack">
      <PageHeader
        description="Preparing the deterministic multi-agent operations workspace."
        eyebrow="Operations"
        headingId="command-center-loading-title"
        title="Command Center"
      />
      <PageState
        description="Loading the local visual prototype."
        icon="O"
        title="Preparing Command Center"
        tone="loading"
      />
    </section>
  );
}

interface ApplicationShellProps {
  readonly services: AppServices;
}

function ApplicationShell({ services }: ApplicationShellProps) {
  const state = useApplicationState();
  const mainContentRef = useRef<HTMLElement>(null);
  const previousRouteRef = useRef(state.activeRoute);
  const dispatch = useApplicationDispatch();
  const coreConnection = useCoreConnection(services.appInfoLoader);
  const menuRouteStatus = useMenuRouteSubscription(services.menuRouteSource);
  const mockRunActions = useMockAssistantRun(state.activeRun, dispatch, services.mockRunDriver);
  const runStatus = state.activeRun?.status ?? "idle";
  const activeConversation = state.conversations.find(
    (conversation) => conversation.id === state.activeConversationId,
  );

  useEffect(() => {
    if (previousRouteRef.current !== state.activeRoute) {
      previousRouteRef.current = state.activeRoute;
      mainContentRef.current?.focus();
    }
  }, [state.activeRoute]);

  const pages: Readonly<Record<AppRoute, ReactNode>> = {
    "command-center": (
      <Suspense fallback={<CommandCenterLoadingPage />}>
        <CommandCenterPage />
      </Suspense>
    ),
    activity: <ActivityPage events={state.activityEvents} />,
    conversations: (
      <ConversationWorkspace
        activeApproval={state.activeApproval}
        composerDraft={state.composerDraft}
        contextProvenance={activeConversation?.contextProvenance ?? []}
        conversationTitle={activeConversation?.title ?? "Conversation unavailable"}
        finalAnswers={activeConversation?.finalAnswers ?? []}
        messages={activeConversation?.messages ?? []}
        onApprovalDecision={mockRunActions.decideApproval}
        onComposerDraftChange={(value) => {
          dispatch({ type: "composer-draft-changed", value });
        }}
        onRetry={mockRunActions.retry}
        onStop={mockRunActions.stop}
        onSubmit={mockRunActions.submit}
        runStatus={runStatus}
        retryableMessageId={
          state.retryableRun?.conversationId === state.activeConversationId
            ? state.retryableRun.assistantMessageId
            : null
        }
        toolActivities={activeConversation?.toolActivities ?? []}
        toolResults={activeConversation?.toolResults ?? []}
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
        activeConversationId={state.activeConversationId}
        activeRoute={state.activeRoute}
        conversationNavigationDisabled={state.activeRun !== null || state.activeApproval !== null}
        conversations={state.conversations}
        onNavigate={(route) => {
          dispatch({ route, type: "navigate" });
        }}
        onNewConversation={() => {
          dispatch({ type: "new-conversation-requested" });
        }}
        onSelectConversation={(conversationId) => {
          dispatch({ conversationId, type: "conversation-selected" });
        }}
      />

      <main
        className="application-main"
        data-scroll-region="application-main"
        id="main-content"
        ref={mainContentRef}
        role="main"
        tabIndex={-1}
      >
        <div className="application-toolbar">
          <p aria-atomic="true" aria-live="polite" className="application-toolbar__location">
            {activeLabel ?? "Workspace"}
          </p>
          <CoreStatus connectionStatus={coreConnection.status} menuRouteStatus={menuRouteStatus} />
        </div>
        <div className="application-content" data-scroll-region="application-content">
          {pages[state.activeRoute]}
        </div>
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
