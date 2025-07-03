use serde_json::{json, Value};
use tauri::Manager;
use tauri_plugin_svelte::ManagerExt;

#[tauri::command]
fn get_counter(app: tauri::AppHandle) -> i32 {
    let value = app.svelte().get("store", "counter").unwrap();

    serde_json::from_value(value).unwrap()
}

#[tauri::command]
fn set_counter(app: tauri::AppHandle) {
    let _ = app.svelte().set("app_state", "counter", 42).unwrap();
}
#[tauri::command]
fn try_get_counter(app: tauri::AppHandle) -> i32 {
    app.svelte().try_get::<i32>("store", "counter").unwrap()
}

#[tauri::command]
fn watch_store(app: tauri::AppHandle) {
    let id = app.svelte().watch("store", |app| {
        let counter = app.svelte().try_get::<i32>("store", "counter")?;

        println!("counter: {counter}");

        Ok(())
    });

    // It returns an id that can be used to remove the watcher.
    if let Ok(id) = id {
        app.svelte().unwatch("store", id).unwrap();
    }
}
