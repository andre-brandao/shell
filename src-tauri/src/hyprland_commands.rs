use hyprland::dispatch;
use hyprland::dispatch::{DispatchType, WindowIdentifier, WorkspaceIdentifierWithSpecial};
use serde::Serialize;

#[tauri::command]
pub async fn hypr_toggle_split() -> Option<String> {
    match dispatch::Dispatch::call(DispatchType::ToggleSplit) {
        Ok(_) => Some("Toggled split layout".to_string()),
        Err(_) => None,
    }
}

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

// Workspace management
#[tauri::command]
pub async fn change_workspace_relative(workspace: i32) -> Option<String> {
    match dispatch::Dispatch::call(DispatchType::Workspace(
        WorkspaceIdentifierWithSpecial::Relative(workspace),
    )) {
        Ok(_) => Some(format!("Switched to workspace {}", workspace)),
        Err(_) => None,
    }
}
