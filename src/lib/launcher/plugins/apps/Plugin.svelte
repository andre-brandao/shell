<script lang="ts">
  import type { AppDetails } from "$lib/types";
  import type { LauncherPluginProps, LauncherPluginExports } from "../types";
  import { onMount } from "svelte";
  import * as cmds from "$lib/cmds";
  import { launchApp } from "./helpers";
  import AppCard from "./AppCard.svelte";
  let { input }: LauncherPluginProps = $props();
  export const { onEnterPressed, onInputChanged }: LauncherPluginExports = {
    onEnterPressed() {
      console.log("Enter Pressed");
      launchApp(filteredApps[0]);
    },
    onInputChanged(value) {
      console.log("input changed", value);
    },
  };

  let apps: AppDetails[] = $state([]);
  let filteredApps: AppDetails[] = $derived(filterApps(input));

  function filterApps(search: string) {
    return apps.filter(
      (app) =>
        app.name.toLowerCase().includes(search) ||
        app.display_name.toLowerCase().includes(search),
    );
  }

  onMount(() => {
    cmds
      .getApps()
      .then((e) => {
        console.log(e);
        apps = e;
      })
      .catch(console.error);
  });
</script>

<div class="app-list">
  {#each filteredApps || [] as app, i (i)}
    <AppCard {app} />
    <!-- <button class="app-item" onclick={() => launchApp(app)}>
      <span class="app-name">{app.name}</span>
      {#if app.description}
        <span class="app-desktop">{app.description}</span>
      {/if}
    </button> -->
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

  .no-apps {
    text-align: center;
    padding: 60px 20px;
    color: #8e8e93;
    font-size: 14px;
  }
</style>
