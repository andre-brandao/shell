import type { Component } from 'svelte';
import Help from './Help.svelte';
import AppsList from './AppsList.svelte';
import DisksInfo from './DisksInfo.svelte';
import BatteryInfo from './BatteryInfo.svelte';
import type { LauncherPlugin, LauncherStateConfig, LauncherPluginComponentProps } from './types';





class LauncherState {
  input = $state("")
  inputRef = $state<HTMLInputElement | null>(null)

  command: Component<LauncherPluginComponentProps> = $derived(this.getCommand())
  hasPrefix = $state(false)

  plugins: LauncherPlugin[]
  constructor({ plugins }: LauncherStateConfig) {
    this.plugins = plugins
  }

  getCommand(): Component<LauncherPluginComponentProps> {
    if (!this.input.startsWith(":")) {
      return AppsList
    }
    for (const { prefix, component } of this.plugins) {
      if (this.input.startsWith(prefix)) {
        this.hasPrefix = true
        return component
      }
    }
    this.hasPrefix = false
    // return Help
    return AppsList
  }

  clearInput() {
    this.input = ""
  }
}

export const appState = new LauncherState({
  plugins: [{
    component: AppsList,
    description: "System Apps",
    prefix: ":app"
  },
    // {
    //   component: DisksInfo,
    //   description: "Disk Information",
    //   prefix: ":disk"
    // },
    // {
    //   component: BatteryInfo,
    //   description: "Battery Information",
    //   prefix: ":bat"
    // },
    // {
    //   component: Help,
    //   description: "Help",
    //   prefix: ":help"
    // }

  ]
});
