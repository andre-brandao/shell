// use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
// use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};
use crate::cmds::windows::hide_launcher;
use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt, WidgetExtManual};

pub fn setup_launcher(app: tauri::AppHandle) {
    // let webview_window = create_window(app, "launcher", "/launcher");
    let _webview_window = tauri::WebviewWindowBuilder::new(
        &app,
        "launcher",
        tauri::WebviewUrl::App("/splashscreen".into()),
    )
    .title("DedsLauncher")
    .window_classname("deds_launcher")
    .inner_size(800.0, 600.0)
    .always_on_top(true)
    .decorations(false)
    .center()
    .transparent(true)
    // .maximized(false)
    .build()
    .unwrap();
    let gtk_window = _webview_window
        .gtk_window()
        .expect("Failed to get GTK window from WebviewWindow");

    let app_pe = app.clone();
    let _ = gtk_window.connect_key_press_event(move |_win, event| {
        let keyval = event.keyval();
        // Key(65307)
        if keyval.to_unicode() == /*ESC*/ Some('\u{1b}') {
            // Hide the launcher window when Escape is pressed
            // _webview_window.hide().unwrap();
            hide_launcher(app_pe.clone());
        }

        return glib::Propagation::Proceed;
    });

    // _webview_window.hide().unwrap();

    // return (gtk_window, webview_window);
}

// let app_clone = app.clone();
// let _ = gtk_window.connect_focus_out_event(move |_win, _event| {
//     println!("Window lost focus, hiding launcher");
//     hide_launcher(app_clone.clone());
//     glib::Propagation::Proceed
// });

// -----------------GTK WINDOW------------------------
// webview_window.hide().unwrap();

// let gtk_window =
//     // gtk::ApplicationWindow::new(&webview_window.gtk_window().unwrap().application().unwrap());
//     gtk::ApplicationWindow::builder()
//         .application(&webview_window.gtk_window().unwrap().application().unwrap())
//         .app_paintable(true)
//         .width_request(800)
//         .height_request(600)
//         .window_position(gtk::WindowPosition::Center)
//         .modal(true)
//         // .show_menubar(false)
//         .build();

// let vbox = webview_window.default_vbox().unwrap();
// webview_window.gtk_window().unwrap().remove(&vbox);
// gtk_window.add(&vbox);
// gtk_window.init_layer_shell();
// gtk_window.set_layer(Layer::Overlay);
// gtk_window.set_keyboard_mode(KeyboardMode::Exclusive); // to allow keyboard input
// gtk_window.set_exclusive_zone(0);
// gtk_window.show_all();
// /-------------------
// gtk_window.set_layer_shell_margin(Edge::Left, 40);
// gtk_window.set_layer_shell_margin(Edge::Right, 40);
// gtk_window.set_layer_shell_margin(Edge::Top, 20);

// gtk_window.set_anchor(Edge::Top, false);
// gtk_window.set_anchor(Edge::Bottom, false);
// gtk_window.set_anchor(Edge::Left, false);
// gtk_window.set_anchor(Edge::Right, false);
