<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";

  let apps = $state<LinuxApp[]>();

  let searchQuery = $state("");
  let searchInput: HTMLInputElement | undefined;
  let filteredApps = $derived.by(() => {
    if (!searchQuery) return apps;
    const query = searchQuery.toLowerCase();
    return (
      apps?.filter(
        (app) =>
          app.name.toLowerCase().includes(query) ||
          (app.app_desktop && app.app_desktop.toLowerCase().includes(query)),
      ) ?? []
    );
  });

  function getApps() {
    setTimeout(() => {
      invoke<LinuxApp[]>("get_apps")
        .then((e) => {
          console.log(e);
          apps = e;
        })
        .catch(console.error);
    }, 1000);
  }
  function launchApp(app: LinuxApp) {
    console.log(`Launching ${app.app_path_exe}`);
    // In a real app, this would launch the application
    // You can add actual app launching logic here
  }

  function clearSearch() {
    searchQuery = "";
    // searchInput?.focus();
  }
  $effect(() => {
    getApps();
  });
</script>

<div class="app-launcher">
  <div class="search-container">
    <input
      bind:this={searchInput}
      bind:value={searchQuery}
      type="text"
      placeholder="Search apps..."
      class="search-input"
    />
    {#if searchQuery}
      <button class="clear-btn" onclick={clearSearch}>×</button>
    {/if}
  </div>

  <div class="app-list">
    {#each filteredApps || [] as app (app.app_path_exe)}
      <button class="app-item" onclick={() => launchApp(app)}>
        <span class="app-name">{app.name}</span>
        {#if app.app_desktop}
          <span class="app-desktop">{app.app_desktop}</span>
        {/if}
      </button>
    {:else}
      <div class="no-apps">
        {searchQuery ? "No apps found" : "Loading apps..."}
      </div>
    {/each}
  </div>
</div>

<style>
  .app-launcher {
    background-color: #1c1c1e;
    min-height: calc(100vh - 60px);
  }

  .search-container {
    position: relative;
    padding: 16px;
    background-color: #2c2c2e;
    border-bottom: 1px solid #3a3a3c;
  }

  .search-input {
    width: 100%;
    background-color: #3a3a3c;
    border: 1px solid #48484a;
    color: #ffffff;
    padding: 10px 16px;
    border-radius: 8px;
    font-family: inherit;
    font-size: 14px;
    box-sizing: border-box;
    transition: all 0.2s ease;
  }

  .search-input:focus {
    outline: none;
    border-color: #007aff;
    background-color: #48484a;
  }

  .search-input::placeholder {
    color: #8e8e93;
  }

  .clear-btn {
    position: absolute;
    right: 24px;
    top: 50%;
    transform: translateY(-50%);
    background: transparent;
    border: none;
    color: #8e8e93;
    font-size: 18px;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .clear-btn:hover {
    background-color: #48484a;
    color: #ffffff;
  }

  .app-list {
    padding: 8px 0;
    /* overflow-y: scroll; */
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
