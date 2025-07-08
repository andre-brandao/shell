<script lang="ts">
  import type { AppDetails } from "$lib/types";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import { launchApp } from "./helpers";

  type Props = {
    app: AppDetails;
  };

  let { app }: Props = $props();
  console.log(app.icon);
</script>

<button class="app-item" onclick={() => launchApp(app)}>
  <div class="app-content">
    <div class="app-info">
      <span class="app-name">{app.name}</span>
      {#if app.description}
        <span class="app-desktop">{app.description}</span>
      {/if}
    </div>
    {#if app.icon}
      <div class="app-icon">
        <!-- <img src="file://{app.icon}" alt={app.name} /> -->
        <img src={convertFileSrc(app.icon)} alt="" />
      </div>
    {/if}
  </div>
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
    width: 100%;
    border-bottom: 1px solid #2c2c2e;
  }

  .app-item:hover {
    background-color: #2c2c2e;
  }

  .app-item:active {
    background-color: #3a3a3c;
  }

  .app-content {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
  }

  .app-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
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

  .app-icon {
    flex-shrink: 0;
    margin-left: 12px;
  }

  .app-icon img {
    width: 32px;
    height: 32px;
    border-radius: 6px;
    object-fit: cover;
  }
</style>
