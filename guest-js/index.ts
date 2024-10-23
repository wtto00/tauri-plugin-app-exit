import { invoke } from "@tauri-apps/api/core";

export async function exitApp(code?: number): Promise<void> {
  return await invoke("plugin:app-exit|exit_app", { code });
}
