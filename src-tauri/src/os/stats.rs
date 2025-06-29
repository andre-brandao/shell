use serde_json::{json, Value};
use std::fs;

use sysinfo::System;

#[tauri::command]
pub fn get_system_info() -> Option<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // Return None if we can't get basic system info
    if sys.cpus().is_empty() {
        return None;
    }

    // CPU usage
    let cpu_usage: f32 =
        sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum::<f32>() / sys.cpus().len() as f32;

    // Memory usage
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();

    // Return None if memory info is invalid
    if total_memory == 0 {
        return None;
    }

    let memory_usage = (used_memory as f64 / total_memory as f64) * 100.0;

    Some(json!({
          "cpu_usage": cpu_usage,
          "ram_usage": memory_usage,
          // "total_memory":total_memory,
          // "used_memory":used_memory,
      }
    ))
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
