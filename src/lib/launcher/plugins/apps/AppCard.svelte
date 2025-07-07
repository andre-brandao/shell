<script lang="ts">
  import type { AppDetails } from "$lib/types";
  import { launchApp } from "./helpers";

  type Props = {
    app: AppDetails;
  };

  let { app }: Props = $props();
</script>

<button class="app-item" onclick={() => launchApp(app)}>
  <span class="app-name">{app.name}</span>
  {#if app.description}
    <span class="app-desktop">{app.description}</span>
  {/if}
</button>

<style>
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
</style>
