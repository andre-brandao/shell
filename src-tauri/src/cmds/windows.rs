// use serde_json::{json, Value};
use tauri::Manager;

#[tauri::command]
pub fn toggle_launcher(app_handle: tauri::AppHandle) {
    let window = app_handle
        .get_webview_window("launcher")
        .expect("launcher window not found");

    match window.is_visible() {
        Ok(true) => window.hide().unwrap(),
        Ok(false) => {
            window.show().unwrap();
        }
        Err(e) => println!("Error checking visibility: {}", e),
    }
}

#[tauri::command]
pub fn hide_launcher(app_handle: tauri::AppHandle) {
    let window = app_handle
        .get_webview_window("launcher")
        .expect("launcher window not found");

    if window.is_visible().unwrap_or(false) {
        window.hide().unwrap();
    }
}
