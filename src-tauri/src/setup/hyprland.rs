use hyprland::event_listener::EventListener;
use serde_json::json;
use std::thread;
use tauri::{AppHandle, Emitter};

pub fn listen_events(app: AppHandle) {
    thread::spawn(move || {
        let mut event_listener = EventListener::new();

        // Register all event handlers
        register_workspace_handler(&mut event_listener, app.clone());
        register_active_window_handler(&mut event_listener, app.clone());
        register_fullscreen_handler(&mut event_listener, app.clone());
        register_active_monitor_handler(&mut event_listener, app.clone());

        // Start the event listener
        if let Err(e) = event_listener.start_listener() {
            eprintln!("Failed to start Hyprland event listener: {}", e);
        }
    });
}

// Register handler for workspace changes
fn register_workspace_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
    event_listener.add_workspace_changed_handler(move |workspace_event| {
        // println!("{workspace_event:#?}");
        app_handle
            .emit(
                "workspace-changed",
                json!({
                    "name": workspace_event.name,
                    "id": workspace_event.id,
                }),
            )
            .unwrap();
    });
}

// Register handler for active window changes
fn register_active_window_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
    event_listener.add_active_window_changed_handler(move |data| {
        // println!("{data:#?}");
        // if some window emit class and title in a json
        if let Some(window) = data {
            let json_data = json!({
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
        // println!("Window {} fullscreen", if fstate { "is" } else { "is not" });

        // Emit fullscreen state event
        app_handle.emit("fullscreen-changed", fstate).unwrap();
    });
}

// Register handler for active monitor changes
fn register_active_monitor_handler(event_listener: &mut EventListener, app_handle: AppHandle) {
    event_listener.add_active_monitor_changed_handler(move |state| {
        // println!("Monitor state: {state:#?}");

        let json_data = json!({
            "monitor_name": state.monitor_name,
            "workspace_name": state.workspace_name,
        });
        app_handle
            .emit("active-monitor-changed", json_data)
            .unwrap();
    });
}
