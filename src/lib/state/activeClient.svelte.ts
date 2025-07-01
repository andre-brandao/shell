import { createKeyedWatcher } from "./whatcher.svelte";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";

function mkActiveClient() {
  const activeClient: ActiveClient = $state({
    class: "",
    title: ""
  });

  const setupActiveClient = () => {
    const unlisten = listen<{ class: string; title: string } | null>("active-client", (event) => {
      activeClient.class = event.payload?.class ?? "";
      activeClient.title = event.payload?.title ?? "";
    })
    return () => unlisten.then(f => f())
  }

  const watcher = createKeyedWatcher();
  return {
    get activeClient() {
      watcher.watch(setupActiveClient);
      return activeClient;
    }
  };
}

export const activeClient = mkActiveClient();
