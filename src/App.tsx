import {
  lazy,
  Suspense,
  useCallback,
  useEffect,
  useMemo,
  useReducer,
  useRef,
  useState,
  type CSSProperties,
  type ReactNode,
} from "react";
import { createPortal } from "react-dom";
import { PanelRightClose } from "lucide-react";

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
import { ApplicationActivityDock } from "./components/ApplicationActivityDock";
import { ApplicationHeader } from "./components/ApplicationHeader";
import { ApplicationInspector } from "./components/ApplicationInspector";
import { ApplicationSidebar } from "./components/ApplicationSidebar";
import { PageState } from "./components/PageState";
import {
  ACTIVITY_PANEL_MIN_HEIGHT,
  applicationShellReducer,
  createApplicationShellState,
} from "./components/applicationShellState";
import {
  ApplicationWorkspacePanelsContext,
  type ApplicationWorkspacePanels,
  useApplicationWorkspacePanels,
} from "./components/applicationWorkspacePanels";
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
import {
  fetchResearchKnowledgeDemoProjection,
  type ResearchKnowledgeDemoProjectionLoader,
} from "./infrastructure/tauri/research-knowledge-demo-projection-client";

const CommandCenterPage = lazy(() => import("./features/command-center/CommandCenterPage"));

export interface AppServices {
  readonly appInfoLoader: AppInfoLoader;
  readonly menuRouteSource: MenuRouteSource;
  readonly mockRunDriver: MockRunDriver;
  readonly researchKnowledgeDemoProjectionLoader: ResearchKnowledgeDemoProjectionLoader;
}

interface AppProps {
  readonly services?: AppServices;
}

const DEFAULT_APP_SERVICES: AppServices = {
  appInfoLoader: fetchAppInfo,
  menuRouteSource: tauriMenuRouteSource,
  mockRunDriver: browserMockRunDriver,
  researchKnowledgeDemoProjectionLoader: fetchResearchKnowledgeDemoProjection,
};

export function App({ services = DEFAULT_APP_SERVICES }: AppProps) {
  return (
    <ApplicationStateProvider>
      <ApplicationShell services={services} />
    </ApplicationStateProvider>
  );
}

export function CommandCenterLoadingPage() {
  const workspacePanels = useApplicationWorkspacePanels();
  const inspectorHeaderPortal =
    workspacePanels?.inspectorHeaderTarget === null ||
    workspacePanels?.inspectorHeaderTarget === undefined
      ? null
      : createPortal(
          <div className="application-panel-header">
            <div>
              <span className="application-panel-header__eyebrow">Command Center loading</span>
              <h2 id="application-inspector-title">Preparing Command Center</h2>
            </div>
            <button
              aria-label="Close workspace inspector"
              className="application-panel-header__button"
              data-application-inspector-close="true"
              onClick={workspacePanels.closeInspector}
              title="Close workspace inspector"
              type="button"
            >
              <PanelRightClose aria-hidden="true" />
            </button>
          </div>,
          workspacePanels.inspectorHeaderTarget,
        );
  const inspectorBodyPortal =
    workspacePanels?.inspectorBodyTarget === null ||
    workspacePanels?.inspectorBodyTarget === undefined
      ? null
      : createPortal(
          <div aria-busy="true" className="application-panel-empty">
            <strong>No Command Center selection available yet</strong>
            <p>
              The deterministic frontend fixture workspace is still loading. No live runtime data is
              available here.
            </p>
          </div>,
          workspacePanels.inspectorBodyTarget,
        );
  const activitySummaryPortal =
    workspacePanels?.activitySummaryTarget === null ||
    workspacePanels?.activitySummaryTarget === undefined
      ? null
      : createPortal(
          <>Deterministic fixture activity loading</>,
          workspacePanels.activitySummaryTarget,
        );
  const activityBodyPortal =
    workspacePanels?.activityBodyTarget === null ||
    workspacePanels?.activityBodyTarget === undefined
      ? null
      : createPortal(
          <div aria-busy="true" className="application-panel-empty application-panel-empty--inline">
            <strong>No Command Center fixture activity available yet</strong>
            <p>
              Bounded presentation events appear after the deterministic frontend fixture loads;
              this panel is not live telemetry.
            </p>
          </div>,
          workspacePanels.activityBodyTarget,
        );

  return (
    <>
      <section
        aria-busy="true"
        aria-labelledby="command-center-loading-title"
        className="page-stack"
      >
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
      {inspectorHeaderPortal}
      {inspectorBodyPortal}
      {activitySummaryPortal}
      {activityBodyPortal}
    </>
  );
}

interface ApplicationShellProps {
  readonly services: AppServices;
}

function ApplicationShell({ services }: ApplicationShellProps) {
  const state = useApplicationState();
  const applicationContentRef = useRef<HTMLDivElement>(null);
  const mainContentRef = useRef<HTMLElement>(null);
  const inspectorToggleRef = useRef<HTMLButtonElement>(null);
  const inspectorReturnFocusRef = useRef<HTMLElement | null>(null);
  const previousRouteRef = useRef(state.activeRoute);
  const [activityBodyTarget, setActivityBodyTarget] = useState<HTMLDivElement | null>(null);
  const [activitySummaryTarget, setActivitySummaryTarget] = useState<HTMLSpanElement | null>(null);
  const [inspectorBodyTarget, setInspectorBodyTarget] = useState<HTMLDivElement | null>(null);
  const [inspectorHeaderTarget, setInspectorHeaderTarget] = useState<HTMLDivElement | null>(null);
  const [shellState, shellDispatch] = useReducer(
    applicationShellReducer,
    window.innerHeight,
    createApplicationShellState,
  );
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
      inspectorReturnFocusRef.current = inspectorToggleRef.current;
      if (applicationContentRef.current !== null) {
        applicationContentRef.current.scrollTop = 0;
      }
      mainContentRef.current?.focus();
    }
  }, [state.activeRoute]);

  useEffect(() => {
    const handleViewportResize = () => {
      shellDispatch({ height: window.innerHeight, type: "viewport-resized" });
    };

    window.addEventListener("resize", handleViewportResize);
    return () => {
      window.removeEventListener("resize", handleViewportResize);
    };
  }, []);

  const openInspector = useCallback((returnFocus?: HTMLElement | null) => {
    if (returnFocus !== undefined && returnFocus !== null) {
      inspectorReturnFocusRef.current = returnFocus;
    }
    shellDispatch({ type: "inspector-opened" });
  }, []);

  const closeInspector = useCallback(() => {
    shellDispatch({ type: "inspector-closed" });
    const returnFocus = inspectorReturnFocusRef.current ?? inspectorToggleRef.current;
    window.requestAnimationFrame(() => {
      returnFocus?.focus();
      if (returnFocus !== null && document.activeElement !== returnFocus) {
        inspectorToggleRef.current?.focus();
      }
    });
  }, []);

  useEffect(() => {
    if (!shellState.inspectorExpanded) return;
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key !== "Escape") return;
      if (document.querySelector('[role="dialog"][aria-modal="true"]') !== null) return;
      event.preventDefault();
      event.stopPropagation();
      closeInspector();
    };
    window.addEventListener("keydown", handleEscape, true);
    return () => {
      window.removeEventListener("keydown", handleEscape, true);
    };
  }, [closeInspector, shellState.inspectorExpanded]);

  const workspacePanels = useMemo<ApplicationWorkspacePanels>(
    () => ({
      activityBodyTarget,
      activitySummaryTarget,
      closeInspector,
      inspectorBodyTarget,
      inspectorHeaderTarget,
      openInspector,
    }),
    [
      activityBodyTarget,
      activitySummaryTarget,
      closeInspector,
      inspectorBodyTarget,
      inspectorHeaderTarget,
      openInspector,
    ],
  );

  const pages: Readonly<Record<AppRoute, ReactNode>> = {
    "command-center": (
      <Suspense fallback={<CommandCenterLoadingPage />}>
        <CommandCenterPage projectionLoader={services.researchKnowledgeDemoProjectionLoader} />
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
  const shellClassName = [
    "application-shell",
    shellState.navigationExpanded ? "" : "application-shell--navigation-collapsed",
    shellState.inspectorExpanded ? "application-shell--inspector-expanded" : "",
    shellState.activityExpanded ? "application-shell--activity-expanded" : "",
  ]
    .filter(Boolean)
    .join(" ");
  const shellStyle = {
    "--application-activity-height": `${String(shellState.activityHeight)}px`,
  } as CSSProperties;

  return (
    <ApplicationWorkspacePanelsContext.Provider value={workspacePanels}>
      <div className={shellClassName} style={shellStyle}>
        <ApplicationHeader
          activeLabel={activeLabel ?? "Workspace"}
          activityExpanded={shellState.activityExpanded}
          coreStatus={
            <CoreStatus
              connectionStatus={coreConnection.status}
              menuRouteStatus={menuRouteStatus}
            />
          }
          inspectorExpanded={shellState.inspectorExpanded}
          inspectorToggleRef={inspectorToggleRef}
          navigationExpanded={shellState.navigationExpanded}
          onToggleActivity={() => {
            shellDispatch({ type: "toggle-activity" });
          }}
          onToggleInspector={() => {
            if (shellState.inspectorExpanded) {
              closeInspector();
            } else {
              openInspector(inspectorToggleRef.current);
            }
          }}
          onToggleNavigation={() => {
            shellDispatch({ type: "toggle-navigation" });
          }}
        />

        <ApplicationSidebar
          activeConversationId={state.activeConversationId}
          activeRoute={state.activeRoute}
          conversationNavigationDisabled={state.activeRun !== null || state.activeApproval !== null}
          conversations={state.conversations}
          expanded={shellState.navigationExpanded}
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
          <div
            className="application-content"
            data-scroll-owner="route-content"
            data-scroll-region="application-content"
            ref={applicationContentRef}
          >
            {pages[state.activeRoute]}
          </div>
        </main>

        <ApplicationInspector
          bodyOutletRef={setInspectorBodyTarget}
          customContent={state.activeRoute === "command-center"}
          expanded={shellState.inspectorExpanded}
          headerOutletRef={setInspectorHeaderTarget}
          onClose={closeInspector}
        />

        <ApplicationActivityDock
          bodyOutletRef={setActivityBodyTarget}
          customContent={state.activeRoute === "command-center"}
          events={state.activityEvents}
          expanded={shellState.activityExpanded}
          height={shellState.activityHeight}
          maximumHeight={shellState.activityMaxHeight}
          minimumHeight={ACTIVITY_PANEL_MIN_HEIGHT}
          onResize={(height) => {
            shellDispatch({ height, type: "activity-resized" });
          }}
          onToggle={() => {
            shellDispatch({ type: "toggle-activity" });
          }}
          summaryOutletRef={setActivitySummaryTarget}
        />
      </div>
    </ApplicationWorkspacePanelsContext.Provider>
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
      <span className="application-toolbar__status-label visually-hidden-at-compact">{label}</span>
    </p>
  );
}
