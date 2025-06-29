use std::fs;

#[tauri::command]
pub fn get_battery_sys() -> Option<u8> {
    let capacity = fs::read_to_string("/sys/class/power_supply/BAT0/capacity").ok()?;

    // Some(capacity.trim().parse::<u8>().unwrap_or(0))
    match capacity.trim().parse::<u8>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}
