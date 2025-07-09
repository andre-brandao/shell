use serde_json::{json, Value};
use tauri::Manager;

// STATS -------------------------------

use std::fs;
use sysinfo::{Disks, Networks, System};

#[tauri::command]
pub fn get_system_info() -> Option<Value> {
    let mut sys = System::new_all();
    sys.refresh_all();

    // CPU usage with better error handling
    let cpu_usage: f32 = if sys.cpus().is_empty() {
        0.0
    } else {
        let total_usage: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
        total_usage / sys.cpus().len() as f32
    };

    // Memory usage calculation
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let free_memory = sys.free_memory();
    let available_memory = sys.available_memory();

    let memory_usage_percent = if total_memory == 0 {
        0.0
    } else {
        (used_memory as f64 / total_memory as f64) * 100.0
    };

    // Swap information
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let swap_usage_percent = if total_swap == 0 {
        0.0
    } else {
        (used_swap as f64 / total_swap as f64) * 100.0
    };

    // System uptime
    let uptime = System::uptime();

    // Load average
    let load_avg = System::load_average();

    Some(json!({
        "cpu_usage": cpu_usage,
        "cpu_count": sys.cpus().len(),
        "ram_usage": memory_usage_percent,
        "ram_total": total_memory,
        "ram_used": used_memory,
        "ram_free": free_memory,
        "ram_available": available_memory,
        "swap_usage": swap_usage_percent,
        "swap_total": total_swap,
        "swap_used": used_swap,
        "uptime_seconds": uptime,
        "load_average": {
            "one": load_avg.one,
            "five": load_avg.five,
            "fifteen": load_avg.fifteen
        }
    }))
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
            "file_system": disk.file_system().to_string_lossy(),
            "is_removable": disk.is_removable(),
        }));
    }
    Some(json!(disk_info))
}

#[tauri::command]
pub fn get_battery_info() -> Option<Value> {
    // Try multiple battery paths (BAT0, BAT1, etc.)
    for i in 0..5 {
        let battery_path = format!("/sys/class/power_supply/BAT{}", i);

        if let (Ok(capacity), Ok(status)) = (
            fs::read_to_string(format!("{}/capacity", battery_path)),
            fs::read_to_string(format!("{}/status", battery_path)),
        ) {
            let current_capacity = capacity.trim().parse::<u8>().unwrap_or(0);
            let charging_status = status.trim().to_string();

            // Try to get additional battery info
            let technology = fs::read_to_string(format!("{}/technology", battery_path))
                .unwrap_or_else(|_| "Unknown".to_string())
                .trim()
                .to_string();

            let health = fs::read_to_string(format!("{}/health", battery_path))
                .unwrap_or_else(|_| "Unknown".to_string())
                .trim()
                .to_string();

            // Voltage and current (if available)
            let voltage_now = fs::read_to_string(format!("{}/voltage_now", battery_path))
                .ok()
                .and_then(|v| v.trim().parse::<u64>().ok());

            let current_now = fs::read_to_string(format!("{}/current_now", battery_path))
                .ok()
                .and_then(|c| c.trim().parse::<i64>().ok());

            let power_now = fs::read_to_string(format!("{}/power_now", battery_path))
                .ok()
                .and_then(|p| p.trim().parse::<u64>().ok());

            let model_name = fs::read_to_string(format!("{}/model_name", battery_path))
                .unwrap_or_else(|_| "Unknown".to_string())
                .trim()
                .to_string();

            return Some(json!({
                "current": current_capacity,
                "status": charging_status,
                "technology": technology,
                "health": health,
                "voltage_now": voltage_now,
                "current_now": current_now,
                "power_now": power_now,
                "model_name": model_name,
                "battery_index": i
            }));
        }
    }

    // If no battery found, return null or empty state
    None
}

#[tauri::command]
pub fn get_network_info() -> Option<Value> {
    let mut networks = Networks::new();
    networks.refresh(true);

    // let networks = sys.networks();
    let mut network_info = Vec::new();
    let mut total_received = 0;
    let mut total_transmitted = 0;

    for (interface_name, network) in &networks {
        let received = network.received();
        let transmitted = network.transmitted();
        total_received += received;
        total_transmitted += transmitted;

        network_info.push(json!({
            "name": interface_name,
            "received_bytes": received,
            "transmitted_bytes": transmitted,
            "packets_received": network.packets_received(),
            "packets_transmitted": network.packets_transmitted(),
            "errors_on_received": network.errors_on_received(),
            "errors_on_transmitted": network.errors_on_transmitted(),
        }));
    }

    Some(json!({
        "interfaces": network_info,
        "total_received": total_received,
        "total_transmitted": total_transmitted,
        "interface_count": network_info.len()
    }))
}
