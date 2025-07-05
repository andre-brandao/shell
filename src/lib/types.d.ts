type AppDetails = {
  id: string;
  name: string;
  display_name: string;
  description: string;
  exec: string;
  commandline: string;
}
type ActiveClient = {
  class: string;
  title: string;
}

type Workspaces = {
  id: number;
  name: string;
}

type SysInfo = {
  cpu_usage: number;
  cpu_count: number;
  ram_usage: number;
  ram_total: number;
  ram_used: number;
  /* “free” memory refers to unallocated memory whereas*/
  ram_free: number;
  /*  “available” memory refers to memory that is available for (re)use. */
  ram_available: number;
  swap_usage: number;
  swap_total: number;
  swap_used: number;
  uptime_seconds: number;
  load_average: {
    one: number;
    five: number;
    fifteen: number;
  }

}

type Disk = {
  name: string;
  total_space: number;
  available_space: number;
  type: string;
  mount_pont: string;
  file_system: string;
  is_removable: boolean;
}

type Battery = {
  current: number;
  status: "Charging" | "Discharging" | "Full";
  technology: string;
  health: string;
  voltage_now?: number;
  current_now?: number;
  power_now?: number;
  model_name: string;
  battery_index?: number;
}

type CommandState = "search" | "help" | "shell" | "rdp" | "ssh" | "battery" | "disk" | "htop" | "unknown";


type HyprBinds = {
  arg: string
  dispatcher: string
  key: string
  keycode: number
  locked: boolean
  modmask: number
  mouse: boolean
  release: boolean
  repeat: boolean
  submap: string
}

type HyprAnimations = {
  bezier: string | { name: string }
  enabled: true
  name: string
  overridden: boolean
  speed: number;
  style: string | Record<string, unknown>
}

type HyprClient = {
  at: [number, number];
  size: [number, number];
  class: string;
  floating: boolean;
  focus_history_id: number;
  fullscreen: number;
  fullscreen_client: number;
  grouped: string[];
  initial_class: string;
  initial_title: string;
  mapped: boolean;
  monitor: number;
  pid: number;
  pinned: boolean;
  swallowing: string;
  title: string;
  workspace: {
    id: number;
    name: string;
  };
  xwayland: boolean
}
