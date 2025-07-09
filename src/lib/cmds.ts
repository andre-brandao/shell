
import type { AppDetails, Battery, Disk, SysInfo, Workspaces, HyprBinds, HyprClient, HyprAnimations, HyprWorkspace, NetworkInfo } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";

export const sysinfo = {
  usage: {
    get: () => invoke<SysInfo>("get_system_info")
  },
  disks: {
    get: () => invoke<Disk[]>("get_disk_info")
  },
  battery: {
    get: () => invoke<Battery>("get_battery_info")
  },
  apps: {
    get: () => invoke<AppDetails[]>("get_apps")
  },
  network: {
    get: () => invoke<NetworkInfo>("get_network_info")
  }
}
export const hyprland = {
  binds: {
    get: () => invoke<HyprBinds>("hypr_binds")
  },
  workspace: {
    changeTo: (id: Workspaces["id"]) => invoke("change_workspace", { workspace: id + 1 }),
    get: () => invoke<HyprWorkspace>("hypr_workspaces")
  },
  monitors: {
    get: invoke("hypr_monitors")
  },
  animations: {
    get: invoke<HyprAnimations>("hypr_animations")
  },
  devices: {
    keyboards: {
      get: () => invoke("hypr_keyboards")
    },
    mice: {
      get: () => invoke("hypr_mice")
    },
    tablets: {
      get: () => invoke("hypr_tablets")
    }
  }
}
