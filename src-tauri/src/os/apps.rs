use serde_json::{json, Value};

use applications::{common::SearchPath, AppInfo, AppInfoContext};

/**
/home/andre/.nix-profile/share
/nix/profile/share
/home/andre/.local/state/nix/profile/share
/etc/profiles/per-user/andre/share
/nix/var/nix/profiles/default/share
/run/current-system/sw/share
*/

#[tauri::command]
pub fn get_apps() -> Option<Value> {
    let mut ctx = AppInfoContext::new(vec![SearchPath::new(
        std::path::PathBuf::from("/home/user/andre"),
        1,
    )]);
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let apps = ctx.get_all_apps();
    println!("Apps: {:#?}", apps);

    let apps_json: Value = json!(apps
        .iter()
        .map(|app| {
            json!({
                "name": app.name,
                "icon_path": app.icon_path,
                "app_path_exe": app.app_path_exe,
                "app_desktop": app.app_desktop_path,

            })
        })
        .collect::<Vec<_>>());
    Some(apps_json)
}

#[tauri::command]
pub fn get_running_apps() -> Option<Value> {
    let mut ctx = AppInfoContext::new(vec![SearchPath::new(
        std::path::PathBuf::from("/home/user/andre"),
        1,
    )]);
    ctx.refresh_apps().unwrap(); // must refresh apps before getting them

    let running_apps = ctx.get_running_apps();
    println!("Running Apps: {:#?}", running_apps);

    // map to json
    let apps_json: Value = json!(running_apps
        .iter()
        .map(|app| {
            json!({
                "name": app.name,
                "icon_path": app.icon_path,
                "app_path_exe": app.app_path_exe,
                "app_desktop": app.app_desktop_path,

            })
        })
        .collect::<Vec<_>>());
    Some(apps_json)
}
