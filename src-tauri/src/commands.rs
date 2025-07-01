use serde_json::{json, Value};
use tauri::Manager;

#[tauri::command]
pub fn toggle_launcher(app_handle: tauri::AppHandle) {
    let window = app_handle
        .get_webview_window("launcher")
        .expect("launcher window not found");

    match window.is_visible() {
        Ok(true) => window.hide().unwrap(),
        Ok(false) => window.show().unwrap(),
        Err(e) => println!("Error checking visibility: {}", e),
    }
}

// HYPRLAND -------------------------------
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

// APPS -------------------------------

use applications::{common::SearchPath, AppInfo, AppInfoContext};

/**
/home/andre/.nix-profile/share
/nix/profile/share
/home/andre/.local/state/nix/profile/share
/etc/profiles/per-user/andre/share
/nix/var/nix/profiles/default/share
/run/current-system/sw/share
*/

#[tauri::command]
pub fn get_apps() -> Option<Value> {
    let mut ctx = AppInfoContext::new(vec![SearchPath::new(
        std::path::PathBuf::from("/home/user/andre"),
        1,
    )]);
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    // println!("Apps: {:#?}", apps);

    let apps_json: Value = json!(apps
        .iter()
        .map(|app| {
            json!({
                "name": app.name,
                "icon_path": app.icon_path,
                "app_path_exe": app.app_path_exe,
                "app_desktop": app.app_desktop_path,

            })
        })
        .collect::<Vec<_>>());
    Some(apps_json)
}

#[tauri::command]
pub fn get_running_apps() -> Option<Value> {
    let mut ctx = AppInfoContext::new(vec![SearchPath::new(
        std::path::PathBuf::from("/home/user/andre"),
        1,
    )]);
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let running_apps = ctx.get_running_apps();
    // println!("Running Apps: {:#?}", running_apps);

    // map to json
    let apps_json: Value = json!(running_apps
        .iter()
        .map(|app| {
            json!({
                "name": app.name,
                "icon_path": app.icon_path,
                "app_path_exe": app.app_path_exe,
                "app_desktop": app.app_desktop_path,

            })
        })
        .collect::<Vec<_>>());
    Some(apps_json)
}
// STATS -------------------------------

use std::fs;
use sysinfo::{Disks, System};

#[tauri::command]
pub fn get_system_info() -> Option<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();
    // CPU usage
    let cpu_usage: f32 = {
        let mut total_usage: f32 = 0.0;
        // if sys.cpus().is_empty() {
        //     total_usage;
        // }
        for cpu in sys.cpus() {
            total_usage += cpu.cpu_usage();
        }
        if sys.cpus().len() > 0 {
            total_usage / sys.cpus().len() as f32
        } else {
            0.0 // Handle the case where there are no CPUs
        }
    };
    let memory_usage: f64 = {
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        if total_memory == 0 {
            return None;
        } else {
            (used_memory as f64 / total_memory as f64) * 100.0
        }
    };

    Some(json!({
          "cpu_usage": cpu_usage,
          "ram_usage": memory_usage,
          // "total_memory":total_memory,
          // "used_memory":used_memory,
      }
    ))
}

#[tauri::command]
pub fn get_disk_info() -> Option<Value> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();
    for disk in disks.list() {
        disk_info.push(json!({
            "name": format!("{:?}", disk.name()),
            "total_space": disk.total_space(),
            "available_space": disk.available_space(),
            "type": format!("{:?}", disk.kind()),
            "mount_point": format!("{:?}", disk.mount_point()),
        }));
    }
    Some(json!(disk_info))
}

#[tauri::command]
pub fn get_battery_sys() -> Option<u8> {
    let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok()?;

    // Some(capacity.trim().parse::<u8>().unwrap_or(0))
    match capacity.trim().parse::<u8>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
