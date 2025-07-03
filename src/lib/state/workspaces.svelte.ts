import { createKeyedWatcher } from "./whatcher.svelte";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";


function mkWorkspaces() {
  const workspace: Workspaces = $state({
    id: -1,
    name: ""
  });

  const setupActiveClient = () => {
    const unlisten = listen<Workspaces>("workspace-changed", (event) => {
      console.log(event)
      workspace.id = event.payload?.id ?? -1;
      workspace.name = event.payload?.name ?? "";
    })
    return () => unlisten.then(f => f())
  }

  const watcher = createKeyedWatcher();
  return {
    get activeWorkpace() {
      watcher.watch(setupActiveClient);
      return workspace;
    },

    async changeWorkspace(id: Workspaces["id"]) {
      invoke("change_workspace", { workspace: id + 1 })
        .then(console.log)
        .catch(console.error)
    }
  };
}

export const workspace = mkWorkspaces();
