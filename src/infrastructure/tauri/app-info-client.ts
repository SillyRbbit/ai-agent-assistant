import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  readonly architecture: string;
  readonly environment: "development" | "production";
  readonly name: string;
  readonly secureCore: boolean;
  readonly target: string;
  readonly version: string;
}

export async function fetchAppInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("get_app_info");
}
