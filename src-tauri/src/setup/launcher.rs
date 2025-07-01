// use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
// use gtk_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

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

    // return (gtk_window, webview_window);
}
