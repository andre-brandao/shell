import { createKeyedWatcher } from "./whatcher.svelte";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
const INTERVAL = 3000; // 1 second

function mkSysInfo() {
  const sysInfo: SysInfo = $state({
    cpu_usage: 0,
    ram_usage: 0
  });

  const setupInfo = () => {
    const timeout = setInterval(() => {
      invoke<SysInfo>("get_system_info")
        .then((e) => {
          console.log(e);
          sysInfo.cpu_usage = e.cpu_usage ?? 0;
          sysInfo.ram_usage = e.ram_usage ?? 0;
        })
        .catch(console.error);
    }, INTERVAL);

    return () => clearInterval(timeout);
  }

  const watcher = createKeyedWatcher();
  return {
    get value() {
      watcher.watch(setupInfo);
      return sysInfo;
    }
  };
}

export const sysInfo = mkSysInfo();


function mkDiskInfo() {
  let value: Disk[] = $state([]);

  const setup = () => {
    const timeout = setInterval(() => {
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
    }, INTERVAL);

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
  let value = $state(0);

  const setup = () => {
    const timeout = setInterval(() => {
      invoke<number>("get_battery_info")
        .then((e) => {
          if (!e) return
          value = e
        })
        .catch(console.error);
    }, INTERVAL);

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
