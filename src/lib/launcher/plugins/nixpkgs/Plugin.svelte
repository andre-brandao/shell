<script module lang="ts">
</script>

<script lang="ts">
  // <!--  -->
  import type { LauncherPluginProps, LauncherPluginExports } from "../types";
  import { onMount } from "svelte";
  import { Command, open } from "@tauri-apps/plugin-shell";
  import { debounce } from "$lib/utils";
  import {
    launchApp,
    searchNixpkgs,
    type NhSearchResponse,
    type NhSearchResult,
  } from "./helpers";
  import NixPkgCard from "./NixPkgCard.svelte";

  let { input }: LauncherPluginProps = $props();
  export const { onEnterPressed, onInputChanged }: LauncherPluginExports = {
    onEnterPressed() {
      console.log("Enter Pressed");
    },
    onInputChanged(value) {
      debouncedSearchPkgs(value);
    },
  };

  let nhResponse: NhSearchResponse | null = $state(null);
  let isLoading: boolean = $state(false);

  const debouncedSearchPkgs = debounce(async (query: string = "") => {
    if (!query.trim()) {
      nhResponse = null;
      return;
    }
    isLoading = true;
    try {
      nhResponse = await searchNixpkgs(query);
    } catch (error) {
      console.error("Error searching nixpkgs:", error);
      nhResponse = null;
    } finally {
      isLoading = false;
    }
  }, 450);
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
      <NixPkgCard {app} channel={nhResponse?.channel ?? "nixpkgs-unstable"} />
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
