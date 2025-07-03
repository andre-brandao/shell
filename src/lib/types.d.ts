// type LinuxApp = {
//   name: string;
//   icon_path?: string;
//   app_path_exe: string;
//   app_desktop: string;
// };

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
  ram_usage: number;
}

type Disk = {
  name: string;
  total_space: number;
  available_space: number;
  type: string;
  mount_pont: string;
}
type CommandState = "search" | "help" | "shell" | "rdp" | "ssh" | "unknown";
