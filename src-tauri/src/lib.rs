use tauri::{AppHandle, Manager};

mod cmds;
mod setup;

pub fn run() {
    let mut builder = tauri::Builder::default();
    let context = tauri::generate_context!();
    // PLUGINS
    builder = builder
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            let launcher_window = app
                .get_webview_window("launcher")
                .expect("Launcher window not found");

            if args.get(1).map_or(false, |arg| arg == "toggle") {
                if launcher_window.is_visible().unwrap_or(false) {
                    println!("Hiding launcher window");
                    launcher_window.hide().unwrap();
                } else {
                    print!("Showing laucher window");
                    launcher_window.show().unwrap();
                }
            } else {
                let _ = launcher_window.set_focus();
            }
        }))
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init());

    // COMMANDS
    builder = builder.invoke_handler(tauri::generate_handler![
        cmds::hyprland::change_workspace,
        cmds::windows::toggle_launcher,
        cmds::sysinfo::get_battery_info,
        cmds::sysinfo::get_system_info,
        cmds::sysinfo::get_disk_info,
        // commands::get_apps,
        cmds::menu::show_context_menu,
        cmds::application::get_apps // toggle_launcher,
    ]);

    // SETUP
    builder = builder.setup(|app| {
        setup::bar::setup_bar(app.handle().clone());
        setup::launcher::setup_launcher(app.handle().clone());
        setup::hyprland::listen_events(app.handle().clone());
        Ok(())
    });

    let app = builder
        // .register_uri_scheme_protocol("appicon", |_app, request| {})
        .build(context)
        .expect("error while running tauri application");

    // RUN
    app.run(|_app_handle, event| match event {
        // tauri::RunEvent::W
        tauri::RunEvent::WindowEvent { label, event, .. } => {
            if label == "launcher" {
                match event {
                    tauri::WindowEvent::CloseRequested { api, .. } => {
                        api.prevent_close();
                        println!("launcher window close requested, hiding");
                        let window = _app_handle.get_webview_window("launcher").unwrap();
                        window.hide().unwrap();
                    }
                    // tauri::WindowEvent::Resized(physical_size) => todo!(),
                    // tauri::WindowEvent::Moved(physical_position) => todo!(),
                    // tauri::WindowEvent::Destroyed => todo!(),
                    // tauri::WindowEvent::Focused(_) => todo!(),
                    // tauri::WindowEvent::ThemeChanged(theme) => todo!(),
                    _ => {}
                }
            }
        }
        _ => {}
    });
    // builder.run(tauri::generate_context!()).unwrap();
}
