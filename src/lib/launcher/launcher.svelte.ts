import type { LauncherPlugin, LauncherStateConfig, LauncherPluginProps, LauncherComponent, LauncherPluginExports } from './plugins/types';
import type { Component } from 'svelte';
import Help from './plugins/Help.svelte';
import Apps from './plugins/apps/Plugin.svelte';
import NixPkgs from './plugins/nixpkgs/Plugin.svelte';





class LauncherState {
  input = $state("")
  search = $derived(this.getSearch())
  inputRef = $state<HTMLInputElement | null>(null)

  command: LauncherComponent = $derived(this.getCommand())
  hasPrefix = $state(false)

  plugins: LauncherPlugin[]
  constructor({ plugins }: LauncherStateConfig) {
    this.plugins = plugins
  }

  getCommand(): LauncherComponent {
    if (!this.input.startsWith(":")) {
      return Apps
    }
    for (const { prefix, component } of this.plugins) {
      if (this.input.startsWith(prefix)) {
        this.hasPrefix = true
        return component
      }
    }
    this.hasPrefix = false
    // return Help
    return Help
  }
  getSearch() {
    if (!this.input.startsWith(":")) {
      return this.input
    }
    for (const { prefix } of this.plugins) {
      if (this.input.startsWith(prefix)) {
        this.hasPrefix = true
        return this.input.replace(prefix, "")
      }
    }

    return this.input
  }

  clearInput() {
    this.input = ""
  }
}

export const appState = new LauncherState({
  plugins: [{
    component: Apps,
    description: "System Apps",
    prefix: ":app"
  },
  {
    component: NixPkgs,
    description: "Nix Pkgs Search",
    prefix: ":nx"
  },
  ]
});
