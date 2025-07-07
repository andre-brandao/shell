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
      // if (nhResponse?.results.length > 0) {
      //   launchApp(nhResponse.results[0]);
      // }
    },
    onInputChanged(value) {
      debouncedSearchPkgs(value);
    },
  };

  let nhResponse: NhSearchResponse | null = $state(null);
  let isLoading: boolean = $state(false);

  async function launchApp(app: NhSearchResult) {
    let result = await Command.create("exec-sh", [
      "-c",
      `nix run nixpkgs#${app.package_attr_name}`,
    ]).execute();
    console.log(result);
  }

  async function searchNixpkgs(query: string = "") {
    if (!query.trim()) {
      nhResponse = null;
      return;
    }

    isLoading = true;

    try {
      let result = await Command.create("exec-sh", [
        "-c",
        `nh search --limit 25 --json "${query}"`,
      ]).execute();

      nhResponse = JSON.parse(result.stdout) as NhSearchResponse;
    } catch (error) {
      console.error("Error searching nixpkgs:", error);
      nhResponse = null;
    } finally {
      isLoading = false;
    }
  }

  const debouncedSearchPkgs = debounce(searchNixpkgs, 450);

  onMount(async () => {
    // Don't search on mount, wait for user input
  });
</script>

<div class="container">
  {#if nhResponse}
    <div class="search-meta">
      <span class="query-info">
        Found {nhResponse.results.length} packages for "{nhResponse.query}"
      </span>
      <span class="meta-info">
        {nhResponse.channel} • {nhResponse.elapsed_ms}ms
      </span>
    </div>
  {/if}

  <div class="app-list">
    {#each nhResponse?.results || [] as app, i (i)}
      <div class="app-item">
        <div class="app-content">
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
              <button
                class="app-homepage"
                onclick={() => open(app.package_homepage[0])}
              >
                🔗 Homepage
              </button>
            {/if}
            <button
              class="app-source"
              onclick={() => {
                let posInfo = app.package_position.split(":");
                let fileLoc = `${posInfo[0]}#L${posInfo[1]}`;
                open(
                  `https://github.com/NixOS/nixpkgs/blob/${nhResponse?.channel}/${fileLoc}`,
                );
              }}
            >
              📦 Source
            </button>
            {#if app.package_license_set?.length > 0}
              <span class="app-license">{app.package_license_set[0]}</span>
            {/if}
          </div>
        </div>
        <button class="run-button" onclick={() => launchApp(app)}>
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path d="M8 5V19L19 12L8 5Z" fill="currentColor" />
          </svg>
          Run
        </button>
      </div>
    {:else}
      <div class="no-apps">
        {#if isLoading}
          <div class="loading">
            <div class="spinner"></div>
            Searching packages...
          </div>
        {:else if input}
          No packages found for "{input}"
        {:else}
          Start typing to search for packages...
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .container {
    display: flex;
    flex-direction: column;
    gap: 8px;
    height: 100%;
    overflow: hidden;
  }

  .search-meta {
    position: sticky;
    top: 0;
    z-index: 10;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 8px 16px;
    background: #1c1c1e;
    border-radius: 8px;
    font-size: 12px;
    border: 1px solid #2c2c2e;
    flex-shrink: 0;
  }

  .query-info {
    color: #ffffff;
    font-weight: 500;
  }

  .meta-info {
    color: #8e8e93;
    font-weight: 400;
  }

  .app-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
    overflow-y: auto;
    flex: 1;
    min-height: 0;
  }

  .app-item {
    background: transparent;
    border: 1px solid #2c2c2e;
    color: #ffffff;
    padding: 16px;
    font-family: inherit;
    font-size: 14px;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    width: 100%;
    max-width: 100%;
    border-radius: 8px;
    transition: all 0.2s ease;
    box-sizing: border-box;
  }

  .app-item:hover {
    background-color: #1c1c1e;
    border-color: #3a3a3c;
  }

  .app-content {
    display: flex;
    flex-direction: column;
    gap: 6px;
    flex: 1;
    min-width: 0;
    overflow: hidden;
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }

  .app-name {
    font-weight: 600;
    color: #ffffff;
    font-size: 16px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    min-width: 0;
  }

  .app-version {
    font-size: 12px;
    color: #8e8e93;
    font-weight: 400;
    background: #2c2c2e;
    padding: 2px 6px;
    border-radius: 4px;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .app-attr-name {
    font-size: 12px;
    color: #64d2ff;
    font-weight: 500;
    font-family: "SF Mono", "Monaco", "Inconsolata", "Fira Code", monospace;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .app-description {
    font-size: 13px;
    color: #a1a1a6;
    font-weight: 400;
    line-height: 1.4;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .app-meta {
    display: flex;
    gap: 12px;
    align-items: center;
    font-size: 11px;
    color: #8e8e93;
    overflow: hidden;
  }

  .app-homepage {
    background: none;
    border: none;
    color: #64d2ff;
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    padding: 0;
    font-size: inherit;
    font-family: inherit;
    text-decoration: underline;
    transition: color 0.2s ease;
  }

  .app-homepage:hover {
    color: #5ac8fa;
  }

  .app-source {
    background: none;
    border: none;
    color: #64d2ff;
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    padding: 0;
    font-size: inherit;
    font-family: inherit;
    text-decoration: underline;
    transition: color 0.2s ease;
  }

  .app-source:hover {
    color: #5ac8fa;
  }

  .app-license {
    color: #8e8e93;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .run-button {
    background: #007aff;
    border: none;
    color: white;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
    white-space: nowrap;
  }

  .run-button:hover {
    background: #0056b3;
  }

  .run-button:active {
    background: #004494;
    transform: scale(0.98);
  }

  .no-apps {
    text-align: center;
    padding: 60px 20px;
    color: #8e8e93;
    font-size: 14px;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid #2c2c2e;
    border-top: 2px solid #8e8e93;
    border-radius: 50%;
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
