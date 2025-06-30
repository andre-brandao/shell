mod hypr;
mod os;

use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

use hypr::commands::*;
use hypr::events::HyprlandEvents;
use os::apps::*;
use os::stats::*;

// use std::collections::HashMap;
// use tauri::Manager;
// use tauri::{AppHandle, Emitter};
// use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use tauri::{menu::MenuBuilder, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn toggle_launcher(app_handle: tauri::AppHandle) {
    let window = get_launcher(&app_handle);

    match window.is_visible() {
        Ok(true) => window.hide().unwrap(),
        Ok(false) => window.show().unwrap(),
        Err(e) => println!("Error checking visibility: {}", e),
    }
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
            get_apps,
            get_running_apps,
            toggle_launcher,
        ])
        .setup(|app| {
            let display = gdk::Display::default().unwrap();
            let monitor = display.monitor(1).unwrap();
            println!("Monitor {}: {:?}", 1, monitor);
            create_bar(app.handle().clone(), monitor);
            // for i in 0..display.n_monitors() {
            //     let monitor = display.monitor(i).unwrap();
            //     println!("Monitor {}: {:?}", i, monitor);
            //     create_bar(app.handle().clone(), monitor);
            // }
            // display.connect_monitor_added(|_, monitor| {
            //     println!("Monitor added: {:?}", monitor);
            //     create_bar(app.handle().clone(), *monitor);
            // });
            // display.connect_monitor_removed(|_, monitor| {
            //     println!("Monitor removed: {:?}", *monitor);
            // });

            create_launcher(app.handle().clone());

            HyprlandEvents::init_event_listener(app.handle().clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap()
}

fn create_window(app: tauri::AppHandle, label: &str, path: &str) -> tauri::WebviewWindow {
    let webview_window =
        tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App(path.into()))
            .build()
            .unwrap();
    return webview_window;
}
fn create_bar(
    app: tauri::AppHandle,
    monitor: gdk::Monitor,
) -> (gtk::ApplicationWindow, tauri::WebviewWindow) {
    let webview_window = create_window(app, "main", "/");
    webview_window.set_decorations(false);
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

fn create_launcher(app: tauri::AppHandle) {
    // let webview_window = create_window(app, "launcher", "/launcher");
    let webview_window = tauri::WebviewWindowBuilder::new(
        &app,
        "launcher",
        tauri::WebviewUrl::App("/launcher".into()),
    )
    .inner_size(800.0, 600.0)
    .always_on_top(true)
    .decorations(false)
    .center()
    .transparent(true)
    .maximized(false)
    .build()
    .unwrap();
    // webview_window.set_decorations(false).unwrap();
    // webview_window.set_always_on_top(true).unwrap();
    // webview_window.set_min_size(size)

    // // webview_window.gt
    // let gtk_win = webview_window
    //     .gtk_window()
    //     .expect("Failed to get GTK window for launcher");
    // gtk_win.set_keep_above(true);
    // gtk_win.init_layer_shell();
    // gtk_win.set_layer(gtk_layer_shell::Layer::Overlay);

    // -----------------GTK WINDOW------------------------
    // webview_window.hide().unwrap();

    // let gtk_window =
    //     gtk::ApplicationWindow::new(&webview_window.gtk_window().unwrap().application().unwrap());
    // gtk_window.set_app_paintable(true);

    // let vbox = webview_window.default_vbox().unwrap();
    // webview_window.gtk_window().unwrap().remove(&vbox);
    // gtk_window.add(&vbox);

    // gtk_window.init_layer_shell();
    // gtk_window.set_layer(gtk_layer_shell::Layer::Overlay);

    // // gtk_window.set_monitor(&monitor);
    // gtk_window.set_layer_shell_margin(Edge::Left, 40);
    // gtk_window.set_layer_shell_margin(Edge::Right, 40);
    // gtk_window.set_layer_shell_margin(Edge::Top, 20);

    // // Center the window (this works with layer shell)
    // gtk_window.set_anchor(Edge::Top, false);
    // gtk_window.set_anchor(Edge::Bottom, false);
    // gtk_window.set_anchor(Edge::Left, false);
    // gtk_window.set_anchor(Edge::Right, false);

    // // gtk_window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive); // to allow keyboard input
    // gtk_window.set_keyboard_interactivity(true);
    // gtk_window.set_height_request(640);
    // gtk_window.set_width_request(640);
    // gtk_window.set_position(gtk::WindowPosition::Center);
    // gtk_window.show

    // // set anchor top left and right
    // // gtk_window.set_anchor(gtk_layer_shell::IsA::);
    // // gtk_window.set_anchor(gtk_layer_shell::Edge::Left, true);
    // // gtk_window.set_anchor(gtk_layer_shell::Edge::Right, true);

    // gtk_window.show_all();
    // gtk_window.hide();

    // return (gtk_window, webview_window);
}

fn get_launcher(app_handle: &tauri::AppHandle) -> tauri::WebviewWindow {
    let launcher = app_handle
        .get_webview_window("launcher")
        .expect("launcher window not found");
    return launcher;
}
