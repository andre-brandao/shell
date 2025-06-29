use hyprland::data::{Monitor, Monitors, Workspace, Workspaces};
use hyprland::event_listener::EventListener;
use hyprland::shared::{HyprData, WorkspaceType};
use serde::{Deserialize, Serialize};
use std::thread;
use tauri::{AppHandle, Emitter};

// Struct to hold methods related to Hyprland events
pub struct HyprlandEvents;

impl HyprlandEvents {
    // Initialize and start the Hyprland event listener
    pub fn init_event_listener(app_handle: AppHandle) {
        // Start Hyprland event listener in a separate thread
        thread::spawn(move || {
            let mut event_listener = EventListener::new();

            // Listen for workspace changes
            let workspace_app = app_handle.clone();
            event_listener.add_workspace_changed_handler(move |workspace_event| {
                println!("Workspace changed: {workspace_event:?}");
                workspace_app
                    .emit("workspace-changed", workspace_event.id)
                    .unwrap();
            });

            let active_window_app = app_handle.clone();
            event_listener.add_active_window_changed_handler(move |data| {
                println!("{data:#?}");
                // if some window emit class and title in a json
                if let Some(window) = data {
                    let json_data = serde_json::json!({
                        "class": window.class,
                        "title": window.title,
                    });
                    active_window_app
                        .emit("active-window-changed", json_data)
                        .unwrap();
                } else {
                    active_window_app
                        .emit("active-window-changed", serde_json::Value::Null)
                        .unwrap();
                }
            });
            event_listener.add_fullscreen_state_changed_handler(|fstate| {
                println!("Window {} fullscreen", if fstate { "is" } else { "is not" })
            });
            event_listener
                .add_active_monitor_changed_handler(|state| println!("Monitor state: {state:#?}"));

            // Start the event listener
            if let Err(e) = event_listener.start_listener() {
                eprintln!("Failed to start Hyprland event listener: {}", e);
            }
        });
    }
}
