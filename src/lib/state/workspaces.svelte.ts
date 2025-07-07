import { hyprland } from "$lib/cmds";
import type { Workspaces } from "$lib/types";
import { createKeyedWatcher } from "./whatcher.svelte";
// import { invoke } from "@tauri-apps/api/core";
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
    changeWorkpace: hyprland.workspace.changeTo,
  };
}

export const workspace = mkWorkspaces();
