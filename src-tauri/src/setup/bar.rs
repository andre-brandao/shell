use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

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
    .decorations(false)
    .build()
    .unwrap();
    webview_window.hide().unwrap();
    let gtk_window =
        // gtk::ApplicationWindow::new(&webview_window.gtk_window().unwrap().application().unwrap());
    gtk::ApplicationWindow::builder()
        .application(&webview_window.gtk_window().unwrap().application().unwrap())
        .app_paintable(true)
        .margin(0)
        .height_request(30)
        .show_menubar(false)
        .build();
    let vbox = webview_window.default_vbox().unwrap();
    webview_window.gtk_window().unwrap().remove(&vbox);
    gtk_window.add(&vbox);
    gtk_window.init_layer_shell();
    gtk_window.set_monitor(&monitor);
    gtk_window.set_layer(Layer::Top);
    gtk_window.set_anchor(Edge::Top, true);
    gtk_window.set_anchor(Edge::Left, true);
    gtk_window.set_anchor(Edge::Right, true);
    gtk_window.set_keyboard_mode(KeyboardMode::None);
    // gtk_window.set_exclusive_zone(30);
    gtk_window.auto_exclusive_zone_enable();
    // gtk_window.set_keyboard_mode(gtk_layer_shell::KeyboardMode::Exclusive); // to allow keyboard input
    gtk_window.show_all();

    return (gtk_window, webview_window);
}
