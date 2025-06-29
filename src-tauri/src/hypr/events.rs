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

            // Register all event handlers
            Self::register_workspace_handler(&mut event_listener, app_handle.clone());
            Self::register_active_window_handler(&mut event_listener, app_handle.clone());
            Self::register_fullscreen_handler(&mut event_listener, app_handle.clone());
            Self::register_active_monitor_handler(&mut event_listener, app_handle.clone());

            // Start the event listener
            if let Err(e) = event_listener.start_listener() {
                eprintln!("Failed to start Hyprland event listener: {}", e);
            }
        });
    }

    // Register handler for workspace changes
    fn register_workspace_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
        event_listener.add_workspace_changed_handler(move |workspace_event| {
            println!("Workspace changed: {workspace_event:?}");
            app_handle
                .emit("workspace-changed", workspace_event.id)
                .unwrap();
        });
    }

    // Register handler for active window changes
    fn register_active_window_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
        event_listener.add_active_window_changed_handler(move |data| {
            println!("{data:#?}");
            // if some window emit class and title in a json
            if let Some(window) = data {
                let json_data = serde_json::json!({
                    "class": window.class,
                    "title": window.title,
                });
                app_handle.emit("active-window-changed", json_data).unwrap();
            } else {
                app_handle
                    .emit("active-window-changed", serde_json::Value::Null)
                    .unwrap();
            }
        });
    }

    // Register handler for fullscreen state changes
    fn register_fullscreen_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
        event_listener.add_fullscreen_state_changed_handler(move |fstate| {
            println!("Window {} fullscreen", if fstate { "is" } else { "is not" });

            // Emit fullscreen state event
            app_handle.emit("fullscreen-changed", fstate).unwrap();
        });
    }

    // Register handler for active monitor changes
    fn register_active_monitor_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
        event_listener.add_active_monitor_changed_handler(move |state| {
            println!("Monitor state: {state:#?}");

            let json_data = serde_json::json!({
                "monitor_name": state.monitor_name,
                "workspace_name": state.workspace_name,
            });
            app_handle
                .emit("active-monitor-changed", json_data)
                .unwrap();
        });
    }
}
