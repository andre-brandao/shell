// use serde::{Deserialize, Serialize};
// use std::io::{prelude::*, BufReader};
// use std::{
//     path::{Path, PathBuf},
//     sync::{atomic::AtomicBool, Arc, Mutex},
// };

// use std::collections::{HashMap, HashSet};

// #[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize, Hash)]
// pub struct SearchPath {
//     pub path: PathBuf,
//     pub depth: u8,
// }
// impl SearchPath {
//     pub fn new(path: PathBuf, depth: u8) -> Self {
//         Self { path, depth }
//     }
// }
// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default, Eq, Hash)]
// pub struct App {
//     pub name: String,
//     pub icon_path: Option<PathBuf>,
//     pub app_path_exe: Option<PathBuf>, // Path to the .app file for mac, or Exec for Linux, or .exe for Windows
//     pub app_desktop_path: PathBuf,     // Path to the .desktop file for Linux, .app for Mac
// }

// pub trait AppInfo {
//     /// It could take a few seconds to retrieve all apps, so a cache needs to be maintained
//     /// This method is used to refresh the cache
//     fn refresh_apps(&mut self) -> Result<Vec<App>, String>;
//     fn get_all_apps(&self) -> Vec<App>;
//     fn open_file_with(&self, file_path: PathBuf, app: App);
//     fn get_running_apps(&self) -> Vec<App>;
//     // fn get_frontmost_application(&self) -> Result<App>;
//     fn is_refreshing(&self) -> bool;
//     fn empty_cache(&mut self);
// }
// #[derive(Debug, PartialEq, Clone, Default, Eq, Hash, Serialize, Deserialize)]
// pub struct AppIcon {
//     name: String,
//     path: PathBuf,
//     dimensions: Option<u16>,
// }

// pub fn get_default_search_paths() -> Vec<SearchPath> {
//     let mut search_paths = vec![];
//     // read XDG_DATA_DIRS env var
//     let xdg_data_dirs = std::env::var("XDG_DATA_DIRS").unwrap_or("/usr/share".to_string());
//     let xdg_data_dirs: Vec<&str> = xdg_data_dirs.split(':').collect();
//     // make a string sett from xdg_data_dirs
//     let home_dir = std::env::var("HOME").unwrap();
//     let home_path = PathBuf::from(home_dir);
//     let local_share_apps = home_path.join(".local/share/applications");
//     let mut default_search_paths = vec![
//         "/usr/share/applications",
//         "/usr/share/xsessions",
//         "/etc/xdg/autostart",
//         "/var/lib/snapd/desktop/applications",
//         local_share_apps.to_str().unwrap(),
//     ];
//     for path in xdg_data_dirs {
//         default_search_paths.push(path);
//     }

//     for path in default_search_paths {
//         search_paths.push(SearchPath::new(PathBuf::from(path), 1));
//     }
//     search_paths
// }

// pub fn brute_force_find_entry(
//     desktop_file_path: &Path,
//     entry_names: Vec<&str>,
// ) -> Result<Option<String>, std::io::Error> {
//     let file = std::fs::File::open(desktop_file_path)?;
//     let reader = BufReader::new(file);

//     for line in reader.lines() {
//         match line {
//             Ok(line) => {
//                 for entry_name in entry_names.iter() {
//                     if line.starts_with(entry_name) {
//                         // let entry = line.split("=").last().unwrap();
//                         let entry = line[entry_name.len() + 1..line.len()].trim();
//                         return Ok(Some(entry.to_string()));
//                     }
//                 }
//             }
//             Err(_e) => {}
//         }
//     }
//     Ok(None)
// }

// pub fn get_all_apps(extra_search_paths: &Vec<SearchPath>) -> Result<Vec<App>, String> {
//     let default_search_paths = get_default_search_paths();
//     let mut search_dirs: HashSet<SearchPath> = default_search_paths
//         .into_iter()
//         .filter(|dir| dir.path.exists())
//         .map(|dir| SearchPath::new(dir.path, dir.depth))
//         .collect();
//     // Add extra search paths
//     for path in extra_search_paths {
//         search_dirs.insert(path.clone());
//     }
//     // let icons_db = find_all_app_icons()?;
//     // for each dir, search for .desktop files
//     let mut apps: HashSet<App> = HashSet::new();
//     for dir in search_dirs {
//         if !dir.path.exists() {
//             continue;
//         }
//         for entry in WalkDir::new(dir.path.clone()).max_depth(dir.depth as usize) {
//             if entry.is_err() {
//                 continue;
//             }
//             let entry = entry.unwrap();
//             let path = entry.path();
//             if path.extension().is_none() {
//                 continue;
//             }

//             if path.extension().unwrap() == "desktop" && path.is_file() {
//                 let (mut app, has_display) = parse_desktop_file(&path);
//                 // fill icon path if .desktop file contains only icon name
//                 if !has_display {
//                     continue;
//                 }
//                 // if app.icon_path.is_some() {
//                 //     let icon_path = app.icon_path.clone().unwrap();
//                 //     if !icon_path.exists() {
//                 //         // let icon_name = icon_path.file_name().unwrap().to_str().unwrap();
//                 //         if let Some(icons) = icons_db.get(icon_path.to_str().unwrap()) {
//                 //             if let Some(icon) = icons.first() {
//                 //                 app.icon_path = Some(icon.path.clone());
//                 //             }
//                 //         } else {
//                 //             app.icon_path = None;
//                 //         }
//                 //     }
//                 // }
//                 apps.insert(app);
//             }
//         }
//     }
//     Ok(apps.iter().cloned().collect())
// }

// pub fn open_file_with(file_path: PathBuf, app: App) {
//     let exe_path = app.app_path_exe.unwrap();
//     let exec_path_str = exe_path.to_str().unwrap();
//     let file_path_str = file_path.to_str().unwrap();
//     let output = std::process::Command::new(exec_path_str)
//         .arg(file_path_str)
//         .output()
//         .expect("failed to execute process");
// }

// /// return a tuple, first element is the app, second element is a boolean indicating if the desktop file has display
// /// Some apps like url handlers don't have display
// /// The display indicator is not reliable, default to true. It's false iff the desktop file contains `nodisplay=true`
// pub fn parse_desktop_file(desktop_file_path: &Path) -> (App, bool) {
//     let mut app = App::default();
//     app.app_desktop_path = desktop_file_path.to_path_buf();
//     let desktop_file_path_str = desktop_file_path.to_str().unwrap();
//     let map = ini!(desktop_file_path_str);
//     let desktop_entry_exists = map.contains_key("desktop entry");
//     let mut display = true;
//     if desktop_entry_exists {
//         let desktop_entry = map["desktop entry"].clone();
//         if desktop_entry.contains_key("nodisplay") {
//             // I don't want apps like a url handler that doesn't have GUI
//             let nodisplay = desktop_entry["nodisplay"].clone();
//             match nodisplay {
//                 Some(nodisplay) => {
//                     if nodisplay == "true" {
//                         display = false;
//                     }
//                 }
//                 None => {}
//             }
//         }

//         let raw_exec = desktop_entry
//             .get("exec")
//             .cloned()
//             // try to find it by brute if not found
//             .or_else(|| brute_force_find_exec(&desktop_file_path).ok())
//             .flatten();

//         if let Some(exec) = raw_exec {
//             app.app_path_exe = Some(PathBuf::from(clean_exec_path(&exec)));
//         }

//         if desktop_entry.contains_key("icon") {
//             let icon = desktop_entry["icon"].clone();
//             app.icon_path = Some(PathBuf::from(icon.unwrap()));
//         } else {
//             match brute_force_find_icon(&desktop_file_path) {
//                 Ok(icon) => {
//                     app.icon_path = icon.map(|icon| PathBuf::from(icon));
//                 }
//                 Err(_) => {}
//             };
//         }
//         if desktop_entry.contains_key("name") {
//             let name = desktop_entry["name"].clone();
//             app.name = name.unwrap();
//         }
//     }
//     return (app, display);
// }
