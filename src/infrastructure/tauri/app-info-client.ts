import { invoke } from "@tauri-apps/api/core";

export interface AppInfo {
  readonly architecture: string;
  readonly environment: "development" | "production";
  readonly name: string;
  readonly secureCore: boolean;
  readonly target: string;
  readonly version: string;
}

const APP_INFO_FIELDS = [
  "architecture",
  "environment",
  "name",
  "secureCore",
  "target",
  "version",
] as const;

const MAX_APP_INFO_STRING_LENGTH = 128;

function isBoundedString(value: unknown): value is string {
  if (typeof value !== "string") {
    return false;
  }

  const length = Array.from(value).length;
  return length > 0 && length <= MAX_APP_INFO_STRING_LENGTH;
}

export function parseAppInfo(value: unknown): AppInfo | undefined {
  if (value === null || typeof value !== "object" || Array.isArray(value)) {
    return undefined;
  }

  const record = value as Record<string, unknown>;
  const keys = Object.keys(record);
  if (
    keys.length !== APP_INFO_FIELDS.length ||
    keys.some((key) => !APP_INFO_FIELDS.includes(key as never))
  ) {
    return undefined;
  }

  if (
    !isBoundedString(record["architecture"]) ||
    !isBoundedString(record["name"]) ||
    !isBoundedString(record["target"]) ||
    !isBoundedString(record["version"]) ||
    (record["environment"] !== "development" && record["environment"] !== "production") ||
    record["secureCore"] !== true
  ) {
    return undefined;
  }

  return {
    architecture: record["architecture"],
    environment: record["environment"],
    name: record["name"],
    secureCore: true,
    target: record["target"],
    version: record["version"],
  };
}

export async function fetchAppInfo(): Promise<AppInfo> {
  const result = await invoke<unknown>("get_app_info");
  const appInfo = parseAppInfo(result);

  if (appInfo === undefined) {
    throw new Error("Invalid application information.");
  }

  return appInfo;
}
