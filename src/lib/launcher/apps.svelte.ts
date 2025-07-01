import { invoke } from "@tauri-apps/api/core";
import { openPath } from "@tauri-apps/plugin-opener";

class AppState {
  search = $state("")
  searchInput = $state<HTMLInputElement | null>(null)
  apps = $state<LinuxApp[]>([])

  filteredApps = $derived(this.filterApps())

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
