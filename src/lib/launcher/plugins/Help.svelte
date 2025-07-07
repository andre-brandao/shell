<script lang="ts">
  import { appState } from "../launcher.svelte";
  import type { LauncherPluginComponentProps } from "../types";
  let { input }: LauncherPluginComponentProps = $props();
</script>

<main>
  <div class="list">
    {#each appState.plugins as p, i (p.prefix)}
      <button
        class="plugin-card"
        onclick={() => {
          appState.input = p.prefix;
        }}
      >
        <span class="plugin-prefix">
          {p.prefix}
        </span>

        <span class="plugin-description">
          {p.description}
        </span>
      </button>
    {/each}
  </div>
</main>

<style>
  .list {
    display: flex;
    flex-direction: column;
    gap: 10px;
    padding: 10px;
  }

  .plugin-card {
    border: 1px solid #ccc;
    padding: 10px;
    border-radius: 5px;
    display: flex;
    flex-direction: row;
    gap: 5px;
    /* justify between */
    justify-content: space-between;
    align-items: center;
  }

  .plugin-prefix {
    font-weight: bold;
    font-size: 1.1em;
    color: lightgray;
  }

  .plugin-description {
    font-style: italic;
    color: white;
  }
</style>
