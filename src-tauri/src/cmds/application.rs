use gio::traits::IconExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::panic;
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

fn create_app_details(app: &gio::AppInfo) -> Result<AppDetails, String> {
    use gio::traits::AppInfoExt;

    let id = panic::catch_unwind(|| {
        app.id()
            .map_or_else(|| "Unknown ID".to_string(), |id| id.to_string())
    })
    .map_err(|_| "Failed to get app ID".to_string())?;

    let name = panic::catch_unwind(|| app.name().to_string())
        .map_err(|_| "Failed to get app name".to_string())?;

    let display_name = panic::catch_unwind(|| app.display_name().to_string())
        .map_err(|_| "Failed to get app display name".to_string())?;

    let description = panic::catch_unwind(|| {
        app.description()
            .map_or_else(|| "".to_string(), |d| d.to_string())
    })
    .map_err(|_| "Failed to get app description".to_string())?;

    let exec = panic::catch_unwind(|| app.executable())
        .map_err(|_| "Failed to get app executable".to_string())?;

    let commandline = panic::catch_unwind(|| app.commandline())
        .map_err(|_| "Failed to get app commandline".to_string())?;

    let icon = panic::catch_unwind(|| app.icon().and_then(|icon| get_icon_path(&icon)))
        .map_err(|_| "Failed to get app icon".to_string())?;

    Ok(AppDetails {
        id,
        name,
        display_name,
        description,
        exec,
        commandline,
        icon,
    })
}

#[tauri::command]
pub fn get_apps() -> Vec<AppDetails> {
    use gio::traits::AppInfoExt;
    // let info_monitor = gio::AppInfoMonitor::get();
    // let info = gio::DesktopAppInfo::
    let info = gio::AppInfo::all();
    let mut apps = Vec::new();

    for app in info {
        match create_app_details(&app) {
            Ok(details) => apps.push(details),
            Err(err) => {
                let app_name = panic::catch_unwind(|| app.name().to_string())
                    .unwrap_or_else(|_| "Unknown App".to_string());
                println!("Warning: Skipping problematic app: {}: {}", app_name, err);
            }
        }
    }
    println!("Found {} apps", apps.len());
    // app0
    return apps;
}
