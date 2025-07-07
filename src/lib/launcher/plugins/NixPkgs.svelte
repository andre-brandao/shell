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

  type NhSearchResult = {
    package_attr_name: string;
    package_attr_set: string;
    package_pname: string;
    package_pversion: string;
    package_platforms: string[];
    package_outputs: string[];
    package_default_output: string;
    package_programs: string[];
    package_license_set: string[];
    package_description: string;
    package_longDescription: string | null;
    package_hydra: string | null;
    package_system: string;
    package_homepage: string[];
    package_position: string;
  };

  type NhSearchResponse = {
    query: string;
    channel: string;
    elapsed_ms: number;
    results: NhSearchResult[];
  };

  //
  let { input }: LauncherPluginComponentProps = $props();
  export const { onEnterPressed, onInputChanged }: LauncherPluginExports = {
    onEnterPressed() {
      console.log("Enter Pressed");
      // launchApp(filteredApps[0]);
    },
    onInputChanged(value) {
      debouncedSearchPkgs(value);
    },
  };

  let apps: NhSearchResult[] = $state([]);
  let isLoading: boolean = $state(false);

  let filteredApps: NhSearchResult[] = $derived(filterApps(input));

  function filterApps(search: string) {
    if (!search.trim()) return apps;

    return apps.filter(
      (app) =>
        app.package_pname.toLowerCase().includes(search.toLowerCase()) ||
        app.package_description.toLowerCase().includes(search.toLowerCase()) ||
        app.package_attr_name.toLowerCase().includes(search.toLowerCase()),
    );
  }

  async function launchApp(app: NhSearchResult) {
    let result = await Command.create("exec-sh", [
      "-c",
      `nix run nixpkgs#${app.package_attr_name}`,
    ]).execute();
    console.log(result);
  }

  async function searchNixpkgs(query: string = "") {
    if (!query.trim()) {
      apps = [];
      return;
    }

    isLoading = true;

    try {
      let result = await Command.create("exec-sh", [
        "-c",
        `nh search --limit 10 --json "${query}"`,
      ]).execute();
      console.log(result);
      const searchResponse = JSON.parse(result.stdout) as NhSearchResponse;
      console.log(searchResponse.results);
      apps = searchResponse.results;
    } catch (error) {
      console.error("Error searching nixpkgs:", error);
      apps = [];
    } finally {
      isLoading = false;
    }
  }

  const debouncedSearchPkgs = debounce(searchNixpkgs, 450);

  onMount(async () => {
    // Don't search on mount, wait for user input
  });
</script>

<div class="app-list">
  {#each filteredApps || [] as app, i (i)}
    <button class="app-item" onclick={() => launchApp(app)}>
      <div class="app-header">
        <span class="app-name">{app.package_pname}</span>
        <span class="app-version">{app.package_pversion}</span>
      </div>
      <div class="app-attr-name">{app.package_attr_name}</div>
      {#if app.package_description}
        <div class="app-description">{app.package_description}</div>
      {/if}
      <div class="app-meta">
        {#if app.package_homepage?.length > 0}
          <span class="app-homepage">🔗 Homepage</span>
        {/if}
        {#if app.package_license_set?.length > 0}
          <span class="app-license">{app.package_license_set[0]}</span>
        {/if}
      </div>
    </button>
  {:else}
    <div class="no-apps">
      {#if isLoading}
        Loading packages...
      {:else if input}
        No packages found for "{input}"
      {:else}
        Start typing to search for packages...
      {/if}
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
    padding: 16px;
    cursor: pointer;
    font-family: inherit;
    font-size: 14px;
    text-align: left;
    transition: all 0.2s ease;
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
    border-bottom: 1px solid #2c2c2e;
    border-radius: 8px;
    margin-bottom: 4px;
  }

  .app-item:hover {
    background-color: #2c2c2e;
  }

  .app-item:active {
    background-color: #3a3a3c;
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
  }

  .app-name {
    font-weight: 600;
    color: #ffffff;
    font-size: 16px;
  }

  .app-version {
    font-size: 12px;
    color: #8e8e93;
    font-weight: 400;
    background: #2c2c2e;
    padding: 2px 6px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .app-attr-name {
    font-size: 12px;
    color: #64d2ff;
    font-weight: 500;
    font-family: "SF Mono", "Monaco", "Inconsolata", "Fira Code", monospace;
  }

  .app-description {
    font-size: 13px;
    color: #a1a1a6;
    font-weight: 400;
    line-height: 1.4;
  }

  .app-meta {
    display: flex;
    gap: 12px;
    align-items: center;
    font-size: 11px;
    color: #8e8e93;
  }

  .app-homepage {
    color: #64d2ff;
    font-weight: 500;
  }

  .app-license {
    color: #8e8e93;
  }

  .no-apps {
    text-align: center;
    padding: 60px 20px;
    color: #8e8e93;
    font-size: 14px;
  }
</style>
