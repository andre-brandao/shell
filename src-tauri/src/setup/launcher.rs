use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

pub fn setup_launcher(app: tauri::AppHandle) {
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
