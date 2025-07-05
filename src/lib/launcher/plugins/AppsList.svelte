<script lang="ts">
  import type { AppDetails } from "$lib/types";
  import { invoke } from "@tauri-apps/api/core";
  import type { LauncherPluginComponentProps } from "../types";
  import { onMount } from "svelte";
  import * as cmds from "$lib/cmds";
  import { Command, open } from "@tauri-apps/plugin-shell";
  let { input }: LauncherPluginComponentProps = $props();

  let apps: AppDetails[] = $state([]);
  let filteredApps: AppDetails[] = $derived(filterApps(input));

  function filterApps(search: string) {
    return apps.filter(
      (app) =>
        app.name.toLowerCase().includes(search) ||
        app.display_name.toLowerCase().includes(search),
    );
  }

  // onEnterPressed: () => void }
  export function onEnterPressed() {
    launchApp(filteredApps[0]);
  }

  async function launchApp(app: AppDetails) {
    // open(app.commandline, "xdg-open,");
    console.log(app);
    let result = await Command.create("exec-sh", [
      "-c",
      app.commandline,
    ]).execute();
    console.log(result);
  }
  onMount(() => {
    // invoke<AppDetails[]>("get_apps")
    cmds
      .getApps()
      .then((e) => {
        console.log(e);
        apps = e;
      })
      .catch(console.error);
  });
  // import { appState } from "$lib/launcher/apps.svelte";
</script>

<div class="app-list">
  {#each filteredApps || [] as app, i (i)}
    <button class="app-item" onclick={() => launchApp(app)}>
      <span class="app-name">{app.name}</span>
      {#if app.description}
        <span class="app-desktop">{app.description}</span>
      {/if}
    </button>
  {:else}
    <div class="no-apps">
      {input ? "No apps found" : "Loading apps..."}
    </div>
  {/each}
</div>

<style>
  .app-list {
    padding: 8px 0;
  }
  .app-item {
    background: transparent;
    border: none;
    color: #ffffff;
    padding: 12px 16px;
    cursor: pointer;
    font-family: inherit;
    font-size: 14px;
    text-align: left;
    transition: all 0.2s ease;
    display: flex;
    flex-direction: column;
    gap: 2px;
    width: 100%;
    border-bottom: 1px solid #2c2c2e;
  }

  .app-item:hover {
    background-color: #2c2c2e;
  }

  .app-item:active {
    background-color: #3a3a3c;
  }

  .app-name {
    font-weight: 500;
    color: #ffffff;
  }

  .app-desktop {
    font-size: 12px;
    color: #8e8e93;
    font-weight: 400;
  }

  .no-apps {
    text-align: center;
    padding: 60px 20px;
    color: #8e8e93;
    font-size: 14px;
  }
</style>
