mod hyprland_commands;
mod hyprland_events;

use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::LayerShell;
use hyprland_commands::*;
use hyprland_events::HyprlandEvents;
use tauri::Manager;
// use tauri::{AppHandle, Emitter};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            // Hyprland data commands
            change_workspace,
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.hide().unwrap();

            let gtk_window = gtk::ApplicationWindow::new(
                &main_window.gtk_window().unwrap().application().unwrap(),
            );

            // To prevent the window from being black initially.
            gtk_window.set_app_paintable(true);

            let vbox = main_window.default_vbox().unwrap();
            main_window.gtk_window().unwrap().remove(&vbox);
            gtk_window.add(&vbox);
            // Doesn't throw errors.
            gtk_window.init_layer_shell();

            // Just works.
            gtk_window.set_layer(gtk_layer_shell::Layer::Top);
            // gtk_window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive); // to allow keyboard input
            gtk_window.set_height_request(30);
            gtk_window.set_exclusive_zone(30);
            gtk_window.set_margin(0);
            // gtk_window.set_position(gtk::WindowPosition::Mouse);

            // set anchor top left and right
            gtk_window.set_anchor(gtk_layer_shell::Edge::Top, true);
            gtk_window.set_anchor(gtk_layer_shell::Edge::Left, true);
            gtk_window.set_anchor(gtk_layer_shell::Edge::Right, true);
            gtk_window.show_all();

            HyprlandEvents::init_event_listener(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap()
}
