use serde_json::{json, Value};
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
