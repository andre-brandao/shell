mod hypr;
mod os;

use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::LayerShell;

use hypr::commands::*;
use hypr::events::HyprlandEvents;
use os::stats::*;
// use std::collections::HashMap;
// use tauri::Manager;
// use tauri::{AppHandle, Emitter};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn create_window(app: tauri::AppHandle, label: &str) -> tauri::WebviewWindow {
    let webview_window =
        tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App("/".into()))
            .build()
            .unwrap();
    return webview_window;
}
fn create_bar(
    app: tauri::AppHandle,
    monitor: gdk::Monitor,
) -> (gtk::ApplicationWindow, tauri::WebviewWindow) {
    let webview_window = create_window(app, "main");
    webview_window.hide().unwrap();
    // Set the monitor for the webview window
    let gtk_window =
        gtk::ApplicationWindow::new(&webview_window.gtk_window().unwrap().application().unwrap());
    gtk_window.set_app_paintable(true);
    let vbox = webview_window.default_vbox().unwrap();
    webview_window.gtk_window().unwrap().remove(&vbox);
    gtk_window.add(&vbox);
    gtk_window.init_layer_shell();

    gtk_window.set_monitor(&monitor);
    // Just works.
    gtk_window.set_layer(gtk_layer_shell::Layer::Top);
    // gtk_window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive); // to allow keyboard input
    gtk_window.set_height_request(30);
    // gtk_window.set_exclusive_zone(30);
    gtk_window.auto_exclusive_zone_enable();
    gtk_window.set_margin(0);
    // gtk_window.set_position(gtk::WindowPosition::Mouse);

    // set anchor top left and right
    gtk_window.set_anchor(gtk_layer_shell::Edge::Top, true);
    gtk_window.set_anchor(gtk_layer_shell::Edge::Left, true);
    gtk_window.set_anchor(gtk_layer_shell::Edge::Right, true);

    gtk_window.show_all();

    return (gtk_window, webview_window);
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            // Hyprland data commands
            change_workspace,
            get_battery_sys,
            get_system_info,
            get_disk_info,
        ])
        .setup(|app| {
            // let monitors = app.available_monitors().unwrap();
            let display = gdk::Display::default().unwrap();
            for i in 0..display.n_monitors() {
                let monitor = display.monitor(i).unwrap();
                println!("Monitor {}: {:?}", i, monitor);
                create_bar(app.handle().clone(), monitor);
            }

            // display.connect_monitor_added(|_, monitor| {
            //     println!("Monitor added: {:?}", monitor);
            //     create_bar(app.handle().clone(), *monitor);
            // });
            // display.connect_monitor_removed(|_, monitor| {
            //     println!("Monitor removed: {:?}", *monitor);
            // });
            HyprlandEvents::init_event_listener(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap()
}
