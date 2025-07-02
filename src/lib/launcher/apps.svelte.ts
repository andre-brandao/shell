import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";

/**
:h -> help
:sh -> shell
:rdp -> rdp
:ssh -> ssh

*/

// const commandPrefix: Record<keyof CommandState, ""> = {
//   "": ""
// } as const

class AppState {
  search = $state("")
  searchInput = $state<HTMLInputElement | null>(null)
  apps = $state<LinuxApp[]>([])
  filteredApps = $derived(this.filterApps())
  command = $derived(this.getCommand())
  filterApps() {
    const query = this.search.toLowerCase().trim();
    if (!query) {
      return this.apps;
    }
    return this.apps
      .filter(app =>
        app.name
          .toLowerCase()
          .includes(query)
      );
  }
  getCommand(): CommandState {
    if (!this.search.startsWith(":")) {
      return "search"
    }
    if (this.search.startsWith(":h")) {
      return "help";
    }
    if (this.search.startsWith(":sh")) {
      return "shell";
    }
    if (this.search.startsWith(":rdp")) {
      return "rdp";
    }
    if (this.search.startsWith(":ssh")) {
      return "ssh";
    }

    return "unknown"
  }
  chApp(app: LinuxApp) {
    console.log(`Launching ${app.app_path_exe}`);
    openPath(app.app_path_exe);
  }
  launchApp(app: LinuxApp) {
    console.log(`Launching ${app.app_path_exe}`);
    openPath(app.app_path_exe);
  }
  getApps() {
    invoke<LinuxApp[]>("get_apps")
      .then((e) => {
        console.log(e);
        this.apps = e;
      })
      .catch(console.error);
  }

  clearSearch() {
    this.search = "";
    // searchInput?.focus();
  }
}

export const appState = new AppState();
