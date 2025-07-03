<script lang="ts">
  import { onMount } from "svelte";
  import { appState } from "$lib/launcher/apps.svelte";
  import AppsList from "$lib/launcher/AppsList.svelte";
  import Help from "$lib/launcher/Help.svelte";
  import Input from "$lib/launcher/Input.svelte";
  import BatteryInfo from "$lib/launcher/BatteryInfo.svelte";
  import DisksInfo from "$lib/launcher/DisksInfo.svelte";

  onMount(() => {
    // Initialize the app state when the component mounts
    appState.getApps();
  });
</script>

<div class="app-launcher" role="application">
  <Input />
  <div class="content-area">
    {#if appState.command === "search"}
      <AppsList />
    {:else if appState.command === "help"}
      <Help />
    {:else if appState.command === "battery"}
      <BatteryInfo />
    {:else if appState.command === "disk"}
      <DisksInfo />
    {:else}
      <Help />
    {/if}
  </div>
</div>

<style>
  .app-launcher {
    background-color: #1c1c1e;
    width: 100%;
    min-height: 500px; /* Add minimum height */
    display: flex;
    flex-direction: column;
  }

  .content-area {
    max-height: 420px;
    /* height: 100%; */
    width: 100%;
    overflow-y: auto;
    flex: 1;
  }
</style>
