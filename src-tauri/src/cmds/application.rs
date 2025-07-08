use gio::traits::IconExt;
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
    icon: Option<String>,
}

fn get_icon_path(icon: &gio::Icon) -> Option<String> {
    use gio::traits::IconExt;
    use glib::Cast;
    // Try to downcast to FileIcon first
    if let Some(file_icon) = icon.downcast_ref::<gio::FileIcon>() {
        use gio::prelude::FileExt;
        let file = file_icon.file();

        return file.path().and_then(|f| f.to_str().map(|s| s.to_string()));
    }

    if let Some(themed_icon) = icon.downcast_ref::<gio::ThemedIcon>() {
        let names = themed_icon.names();
        if let Some(name) = names.first() {
            // Try to resolve the themed icon to an actual path
            if let Some(icon_theme) = gtk::IconTheme::default() {
                use gtk::prelude::IconThemeExt;
                if let Some(icon_info) =
                    icon_theme.lookup_icon(name, 48, gtk::IconLookupFlags::empty())
                {
                    return icon_info
                        .filename()
                        .and_then(|f| f.to_str().map(|s| s.to_string()));
                }
            }
        }
    }

    None
}

#[tauri::command]
pub fn get_apps() -> Vec<AppDetails> {
    use gio::traits::AppInfoExt;
    // let info_monitor = gio::AppInfoMonitor::get();
    // let info = gio::DesktopAppInfo::
    let info = gio::AppInfo::all();
    let mut apps = Vec::new();

    for app in info {
        let icon = app.icon().and_then(|icon| get_icon_path(&icon));

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
            icon,
        };

        apps.push(details);
    }
    println!("Found {} apps", apps.len());
    // app0
    return apps;
}
