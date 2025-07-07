<script lang="ts">
  // <!--  -->
  import type {
    LauncherPluginComponentProps,
    LauncherPluginExports,
  } from "../types";
  import { onMount } from "svelte";
  import * as cmds from "$lib/cmds";
  import { Command, open } from "@tauri-apps/plugin-shell";
  import { debounce } from "$lib/utils";
  const PREFIX = "legacyPackages.x86_64-linux.";
  type Nixpkg = {
    description: string;
    pname: string;
    version: string;
  };
  //
  let { input }: LauncherPluginComponentProps = $props();
  export const { onEnterPressed, onInputChanged }: LauncherPluginExports = {
    onEnterPressed() {
      console.log("Enter Pressed");
      launchApp(filteredApps[0]);
    },
    onInputChanged(value) {
      debouncedSearchPkgs(value);
    },
  };
  let apps: Nixpkg[] = $state([]);
  let nixpkgs: Record<string, Nixpkg> = $state({});

  let filteredApps: Nixpkg[] = $derived(filterApps(input));

  function filterApps(search: string) {
    return apps.filter(
      (app) =>
        app.pname.toLowerCase().includes(search) ||
        app.description.toLowerCase().includes(search),
    );
  }

  async function launchApp(app: Nixpkg) {
    let result = await Command.create("exec-sh", [
      "-c",
      `nix run nixpkgs#${app}`,
    ]).execute();
    console.log(result);
  }

  async function searchNixpkgs(query: string = "") {
    let json = await Command.create("exec-sh", [
      "-c",
      "nix search nixpkgs ^ --json",
    ]).execute();

    const pkgs = JSON.parse(json.stdout) as Record<string, Nixpkg>;

    nixpkgs = pkgs;
    apps = Object.values(pkgs);
  }
  const debouncedSearchPkgs = debounce(searchNixpkgs, 450);
  onMount(async () => {
    searchNixpkgs();
  });
</script>

<div class="app-list">
  {#each filteredApps || [] as app, i (i)}
    <button class="app-item" onclick={() => launchApp(app)}>
      <span class="app-name">{app.pname}</span>
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
