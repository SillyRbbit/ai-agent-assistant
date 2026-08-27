import { useEffect, useState } from "react";

import type { AppInfo } from "../infrastructure/tauri/app-info-client";

export type AppInfoLoader = () => Promise<AppInfo>;

export type CoreConnection =
  | { readonly status: "checking" }
  | { readonly info: AppInfo; readonly status: "ready" }
  | { readonly message: string; readonly status: "error" };

function getErrorMessage(): string {
  return "The Rust core did not return application information.";
}

export function useCoreConnection(loader: AppInfoLoader): CoreConnection {
  const [connection, setConnection] = useState<CoreConnection>({ status: "checking" });

  useEffect(() => {
    let isMounted = true;

    const connect = async () => {
      try {
        const info = await loader();

        if (isMounted) {
          setConnection({ info, status: "ready" });
        }
      } catch {
        if (isMounted) {
          setConnection({ message: getErrorMessage(), status: "error" });
        }
      }
    };

    void connect();

    return () => {
      isMounted = false;
    };
  }, [loader]);

  return connection;
}
