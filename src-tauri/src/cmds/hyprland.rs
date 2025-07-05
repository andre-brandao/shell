use serde::Serialize;
use serde_json::{json, to_value, Value};
use tauri::Manager;

use hyprland::dispatch;
use hyprland::dispatch::{DispatchType, WorkspaceIdentifierWithSpecial};
// use hyprland::data::{Monitor, Monitors, Workspace, Workspaces};

// Workspace management
#[tauri::command]
pub async fn change_workspace(workspace: i32) -> Option<String> {
    match dispatch::Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
        workspace,
    ))) {
        Ok(_) => Some(format!("Switched to workspace {}", workspace)),
        Err(_) => None,
    }
}
// #[tauri::command]
// pub async fn hypr_toggle_split() -> Option<String> {
//     match dispatch::Dispatch::call(DispatchType::ToggleSplit) {
//         Ok(_) => Some("Toggled split layout".to_string()),
//         Err(_) => None,
//     }
// }
// #[tauri::command]
// pub async fn change_workspace_relative(workspace: i32) -> Option<String> {
//     match dispatch::Dispatch::call(DispatchType::Workspace(
//         WorkspaceIdentifierWithSpecial::Relative(workspace),
//     )) {
//         Ok(_) => Some(format!("Switched to workspace {}", workspace)),
//         Err(_) => None,
//     }
// }

use hyprland::data::{
    Animations, Binds, Client, Clients, Devices, Monitor, Monitors, Workspace, Workspaces,
};
use hyprland::shared::{HyprData, HyprDataActive, HyprDataActiveOptional};

#[tauri::command]
pub fn hypr_clients() -> Option<Value> {
    let clients = Clients::get()
        .ok()?
        .iter()
        .map(|c| {
            json!({
              "at": c.at,
              "size": c.size,
              "workspace": c.workspace,
              "floating": c.floating,
              "fullscreen": c.fullscreen,
              "fullscreen_client": c.fullscreen_client,
              "monitor": c.monitor,
              "initial_class": c.initial_class,
              "class": c.class,
              "initial_title": c.initial_title,
              "title": c.title,
              "pid": c.pid,
              "xwayland": c.xwayland,
              "pinned": c.pinned,
              "grouped": c.grouped,
              "mapped": c.mapped,
              "swallowing": c.swallowing,
              "focus_history_id": c.focus_history_id,
            })
        })
        .collect::<Vec<Value>>();

    to_value(clients).ok()
}

#[tauri::command]
pub fn hypr_animations() -> Option<Value> {
    let animations = Animations::get().unwrap();
    serde_json::to_value(animations.0).ok()
}

#[tauri::command]
pub fn hypr_binds() -> Option<Value> {
    let binds = Binds::get()
        .ok()?
        .iter()
        .map(|bind| {
            json!({
              "locked": bind.locked,
              "mouse": bind.mouse,
              "release": bind.release,
              "repeat": bind.repeat,
              "modmask": bind.modmask,
              "submap": bind.submap,
              "key": bind.key,
              "keycode": bind.keycode,
              "dispatcher": bind.dispatcher,
              "arg": bind.arg
            })
        })
        .collect::<Vec<_>>();
    to_value(binds).ok()
}

#[tauri::command]
pub fn hypr_keyboards() -> Option<Value> {
    let devices = Devices::get().ok()?;
    to_value(devices.keyboards).ok()
}

#[tauri::command]
pub fn hypr_mice() -> Option<Value> {
    let devices = Devices::get().ok()?;
    to_value(devices.mice).ok()
}

#[tauri::command]
pub fn hypr_tablets() -> Option<Value> {
    let devices = Devices::get().ok()?;
    to_value(devices.tablets).ok()
}

#[tauri::command]
pub fn hypr_monitors() -> Option<Value> {
    let monitors = Monitors::get()
        .ok()?
        .iter()
        .map(|m| {
            json!({
              "id": m.id,
              "name": m.name,
              "description": m.description,
              "width": m.width,
              "height": m.height,
              "refresh_rate": m.refresh_rate,
              "x": m.x,
              "y": m.y,
              "active_workspace": m.active_workspace,
              "special_workspace": m.special_workspace,
              "reserved": m.reserved,
              "scale": m.scale,
              "transform": m.transform,
              "focused": m.focused,
              "dpms_status": m.dpms_status,
              "vrr": m.vrr,
              "disabled": m.disabled
            })
        })
        .collect::<Vec<Value>>();

    to_value(monitors).ok()
}

#[tauri::command]
pub fn hypr_workspaces() -> Option<Value> {
    let workspaces = Workspaces::get()
        .ok()?
        .iter()
        .map(|w| {
            json!({
            "id": w.id,
            "name": w.name,
            "monitor": w.monitor,
            "monitor_id": w.monitor_id,
            "windows": w.windows,
            "fullscreen": w.fullscreen,
            "last_window": w.last_window,
            "last_window_title": w.last_window_title,
            })
        })
        .collect::<Vec<Value>>();
    to_value(workspaces).ok()
}
