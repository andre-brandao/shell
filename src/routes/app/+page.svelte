<script lang="ts">
  import { onMount } from "svelte";
  import { appState } from "$lib/launcher/apps.svelte";
  import AppsList from "$lib/launcher/AppsList.svelte";
  import Help from "$lib/launcher/Help.svelte";
  import Input from "$lib/launcher/Input.svelte";

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
    {:else}
      <Help />
    {/if}
  </div>
</div>

<style>
  .app-launcher {
    background-color: #1c1c1e;
    /* height: 100vh; */
  }

  .content-area {
    max-height: 400px;
    overflow-y: auto;
  }
</style>
