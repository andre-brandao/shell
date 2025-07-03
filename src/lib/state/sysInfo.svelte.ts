import { createKeyedWatcher } from "./whatcher.svelte";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
const INTERVAL = 10000; // 1 second

function mkSysInfo() {
  const sysInfo: SysInfo = $state({
    cpu_usage: 0,
    cpu_count: 0,
    ram_usage: 0,
    ram_total: 0,
    ram_available: 0,
    ram_free: 0,
    ram_used: 0,
    swap_total: 0,
    swap_usage: 0,
    swap_used: 0,
    uptime_seconds: 0,
    load_average: {
      one: 0,
      five: 0,
      fifteen: 0
    }

  });

  const getSystemInfo = () => {
    invoke<SysInfo>("get_system_info")
      .then((e) => {
        console.log(e);
        sysInfo.cpu_usage = e.cpu_usage ?? 0;
        sysInfo.ram_usage = e.ram_usage ?? 0;
      })
      .catch(console.error);
  }
  const setup = () => {
    getSystemInfo();
    const timeout = setInterval(getSystemInfo, INTERVAL + 1000);

    return () => clearInterval(timeout);
  }

  const watcher = createKeyedWatcher();
  return {
    get value() {
      watcher.watch(setup);
      return sysInfo;
    }
  };
}

export const sysInfo = mkSysInfo();


function mkDiskInfo() {
  let value: Disk[] = $state([]);


  const getDiskInfo = () => {
    invoke<any>("get_disk_info")
      .then((e) => {
        console.log(e);

        const disk = e[0];
        if (!disk) {
          console.warn("No disk info received");
          return;
        }
        value = [disk];
      })
      .catch(console.error);
  }
  const setup = () => {
    getDiskInfo();
    const timeout = setInterval(getDiskInfo, INTERVAL + 1500);
    return () => clearInterval(timeout);
  }

  const watcher = createKeyedWatcher();
  return {
    get value() {
      watcher.watch(setup);
      return value;
    }
  };
}

export const discInfo = mkDiskInfo();


function mkBatteryInfo() {
  let value: Battery = $state({
    current: 0,
    status: "Discharging",
    technology: "Unknown",
    health: "Unknown",
    model_name: "Unknown",
  });

  const getBatteryInfo = () => {

    invoke<Battery>("get_battery_info")
      .then((e) => {
        if (!e) return
        value = e
      })
      .catch(console.error);
  }
  const setup = () => {
    getBatteryInfo();
    const timeout = setInterval(getBatteryInfo, INTERVAL + 2000);

    return () => clearInterval(timeout);
  }

  const watcher = createKeyedWatcher();
  return {
    get value() {
      watcher.watch(setup);
      return value;
    }
  };
}

export const batteryInfo = mkBatteryInfo();
