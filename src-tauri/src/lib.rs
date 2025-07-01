mod commands;
mod setup;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .invoke_handler(tauri::generate_handler![
            commands::change_workspace,
            commands::toggle_launcher,
            commands::get_battery_sys,
            commands::get_system_info,
            commands::get_disk_info,
            commands::get_apps,
            commands::get_running_apps,
            // toggle_launcher,
        ])
        .setup(|app| {
            setup::bar::setup_bar(app.handle().clone());
            setup::launcher::setup_launcher(app.handle().clone());
            setup::hyprland::listen_events(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .unwrap()
}
