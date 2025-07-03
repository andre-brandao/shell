<script lang="ts">
  import { appState } from "$lib/launcher/apps.svelte";
</script>

<div class="app-list">
  {#each appState.filteredApps || [] as app, i (i)}
    <button class="app-item" onclick={() => appState.launchApp(app)}>
      <span class="app-name">{app.name}</span>
      {#if app.description}
        <span class="app-desktop">{app.description}</span>
      {/if}
    </button>
  {:else}
    <div class="no-apps">
      {appState.search ? "No apps found" : "Loading apps..."}
    </div>
  {/each}
</div>

<style>
  .app-list {
    padding: 8px 0;
    overflow-y: scroll;
    height: calc(100vh - 90px); /* Adjust height to fit the viewport */
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
