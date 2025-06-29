use serde_json::{json, Value};
use std::fs;

use sysinfo::{Disks, System};

#[tauri::command]
pub fn get_system_info() -> Option<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();
    // CPU usage
    let cpu_usage: f32 = {
        let mut total_usage: f32 = 0.0;
        if sys.cpus().is_empty() {
            total_usage;
        }
        for cpu in sys.cpus() {
            total_usage += cpu.cpu_usage();
        }
        if sys.cpus().len() > 0 {
            total_usage / sys.cpus().len() as f32
        } else {
            0.0 // Handle the case where there are no CPUs
        }
    };
    let memory_usage: f64 = {
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        if total_memory == 0 {
            return None;
        } else {
            (used_memory as f64 / total_memory as f64) * 100.0
        }
    };

    Some(json!({
          "cpu_usage": cpu_usage,
          "ram_usage": memory_usage,
          // "total_memory":total_memory,
          // "used_memory":used_memory,
      }
    ))
}

#[tauri::command]
pub fn get_disk_info() -> Option<Value> {
    let disks = Disks::new_with_refreshed_list();
    let mut disk_info = Vec::new();
    for disk in disks.list() {
        disk_info.push(json!({
            "name": format!("{:?}", disk.name()),
            "total_space": disk.total_space(),
            "available_space": disk.available_space(),
            "type": format!("{:?}", disk.kind()),
            "mount_point": format!("{:?}", disk.mount_point()),
        }));
    }
    Some(json!(disk_info))
}

#[tauri::command]
pub fn get_battery_sys() -> Option<u8> {
    let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok()?;

    // Some(capacity.trim().parse::<u8>().unwrap_or(0))
    match capacity.trim().parse::<u8>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
