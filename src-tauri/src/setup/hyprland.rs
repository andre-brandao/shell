use futures_lite::StreamExt;
use hyprland::event_listener::{Event, EventStream};
use hyprland::prelude::*;
use hyprland::shared::{HyprData, HyprDataActive, HyprDataActiveOptional};
use serde_json::json;
use tauri::{AppHandle, Emitter};
// use tokio;

pub fn listen_events(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut event_stream = EventStream::new();

        while let Some(event_result) = event_stream.next().await {
            match event_result {
                Ok(event) => {
                    handle_event(event, app.clone()).await;
                }
                Err(e) => {
                    eprintln!("Error receiving Hyprland event: {}", e);
                }
            }
        }
    });
}

async fn handle_event(event: hyprland::event_listener::Event, app_handle: AppHandle) {
    use hyprland::event_listener::Event;

    match event {
        Event::WorkspaceChanged(workspace_data) => {
            handle_workspace_changed(workspace_data, app_handle).await;
        }
        Event::ActiveWindowChanged(window_data) => {
            handle_active_window_changed(window_data, app_handle).await;
        }
        Event::FullscreenStateChanged(fullscreen_state) => {
            handle_fullscreen_changed(fullscreen_state, app_handle).await;
        }
        Event::ActiveMonitorChanged(monitor_data) => {
            handle_active_monitor_changed(monitor_data, app_handle).await;
        }
        Event::Screencast(screencast_data) => {
            handle_screencast(screencast_data, app_handle).await;
        }
        _ => {
            // Handle other events or ignore them
            // println!("Unhandled event: {:?}", event);
        }
    }
}

async fn handle_screencast(
    screencast_data: hyprland::event_listener::ScreencastEventData,
    app_handle: AppHandle,
) {
    let json_data = json!({
      "turning_on": screencast_data.turning_on,
      "monitor": screencast_data.monitor,
    });

    if let Err(e) = app_handle.emit("screencast-changed", json_data) {
        eprintln!("Failed to emit screencast-changed: {}", e);
    }
}

async fn handle_workspace_changed(
    workspace_data: hyprland::event_listener::WorkspaceEventData,
    app_handle: AppHandle,
) {
    let json_data = json!({
        "name": workspace_data.name,
        "id": workspace_data.id,
    });

    if let Err(e) = app_handle.emit("workspace-changed", json_data) {
        eprintln!("Failed to emit workspace-changed event: {}", e);
    }
}

async fn handle_active_window_changed(
    window_data: Option<hyprland::event_listener::WindowEventData>,
    app_handle: AppHandle,
) {
    let json_data = if let Some(window) = window_data {
        json!({
            "class": window.class,
            "title": window.title,
        })
    } else {
        serde_json::Value::Null
    };

    if let Err(e) = app_handle.emit("active-window-changed", json_data) {
        eprintln!("Failed to emit active-window-changed event: {}", e);
    }
}

async fn handle_fullscreen_changed(fullscreen_state: bool, app_handle: AppHandle) {
    if let Err(e) = app_handle.emit("fullscreen-changed", fullscreen_state) {
        eprintln!("Failed to emit fullscreen-changed event: {}", e);
    }
}

async fn handle_active_monitor_changed(
    monitor_data: hyprland::event_listener::MonitorEventData,
    app_handle: AppHandle,
) {
    let json_data = json!({
        "monitor_name": monitor_data.monitor_name,
        "workspace_name": monitor_data.workspace_name,
    });

    if let Err(e) = app_handle.emit("active-monitor-changed", json_data) {
        eprintln!("Failed to emit active-monitor-changed event: {}", e);
    }
}
