
import type { Battery, Disk, SysInfo, Workspaces } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";

export const getSystemInfo = () => invoke<SysInfo>("get_system_info")

export const getDisksInfo = () => invoke<Disk[]>("get_disk_info")

export const getBatteryInfo = () => invoke<Battery>("get_battery_info")

export const changeWorkpace = (id: Workspaces["id"]) => invoke("change_workspace", { workspace: id + 1 })
