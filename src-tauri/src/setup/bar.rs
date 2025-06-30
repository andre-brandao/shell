use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

// use hypr::commands::*;
// use hypr::events::HyprlandEvents;
// use os::apps::*;
// use os::stats::*;

// use std::collections::HashMap;
// use tauri::Manager;
// use tauri::{AppHandle, Emitter};
// use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

// use tauri::{menu::MenuBuilder, Manager};

pub fn setup_bar(app: tauri::AppHandle) {
    let display = gdk::Display::default().unwrap();
    for i in 0..display.n_monitors() {
        let monitor = display.monitor(i).unwrap();
        println!("Monitor {}: {:?}", i, monitor);
        create_bar(app.clone(), monitor, i);
    }
}
fn create_bar(
    app: tauri::AppHandle,
    monitor: gdk::Monitor,
    index: i32,
) -> (gtk::ApplicationWindow, tauri::WebviewWindow) {
    // bar + index
    // let webview_window = create_window(app, format!("bar_{}", index).as_str(), "/");
    let webview_window = tauri::WebviewWindowBuilder::new(
        &app,
        format!("bar_{}", index).as_str(),
        tauri::WebviewUrl::App("/".into()),
    )
    .build()
    .unwrap();
    // webview_window.set_decorations(false);
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
