use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::Manager;

#[derive(Debug, Serialize)]
pub struct AppDetails {
    id: String,
    name: String,
    display_name: String,
    description: String,
    exec: std::path::PathBuf,
    commandline: Option<std::path::PathBuf>,
}

#[tauri::command]
pub fn get_apps() -> Vec<AppDetails> {
    use gio::traits::AppInfoExt;
    // let info_monitor = gio::AppInfoMonitor::get();
    // let info = gio::DesktopAppInfo::
    let info = gio::AppInfo::all();
    let mut apps = Vec::new();

    for app in info {
        // let icon = match app.icon() {
        //     Some(icon) => icon,
        //     None => {
        //         println!("App {} has no icon", app.name());
        //         continue;
        //     }
        // };
        let details = AppDetails {
            id: app
                .id()
                .map_or_else(|| "Unknown ID".to_string(), |id| id.to_string()),
            name: app.name().to_string(),
            display_name: app.display_name().to_string(),
            description: app
                .description()
                .map_or_else(|| "".to_string(), |d| d.to_string()),
            exec: app.executable(),
            commandline: app.commandline(),
        };

        apps.push(details);
    }
    println!("Found {} apps", apps.len());
    // app0
    return apps;
}
