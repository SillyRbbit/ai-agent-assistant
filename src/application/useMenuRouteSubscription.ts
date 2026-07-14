import { useEffect, useState } from "react";

import type { MenuRouteSource } from "../infrastructure/tauri/menu-route-client";
import { useApplicationDispatch } from "./useApplicationState";

export type MenuRouteConnectionStatus = "checking" | "error" | "ready";

export function useMenuRouteSubscription(source: MenuRouteSource): MenuRouteConnectionStatus {
  const dispatch = useApplicationDispatch();
  const [status, setStatus] = useState<MenuRouteConnectionStatus>("checking");

  useEffect(() => {
    let isDisposed = false;
    let stopListening: (() => void) | undefined;

    void source
      .subscribe((route) => {
        dispatch({ route, type: "menu-route-received" });
      })
      .then(
        (unlisten) => {
          if (isDisposed) {
            unlisten();
            return;
          }

          stopListening = unlisten;
          setStatus("ready");
        },
        () => {
          if (!isDisposed) {
            setStatus("error");
          }
        },
      );

    return () => {
      isDisposed = true;
      stopListening?.();
    };
  }, [dispatch, source]);

  return status;
}
