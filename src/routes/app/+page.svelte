<script lang="ts">
  import { onMount } from "svelte";
  import { appState } from "$lib/launcher/launcher.svelte";
  import AppsList from "$lib/launcher/AppsList.svelte";
  import Help from "$lib/launcher/Help.svelte";
  import Input from "$lib/launcher/Input.svelte";
  import BatteryInfo from "$lib/launcher/BatteryInfo.svelte";
  import DisksInfo from "$lib/launcher/DisksInfo.svelte";

  // onMount(() => {
  //   // Initialize the app state when the component mounts
  //   appState.getApps();
  // });
  $inspect(appState.command);
</script>

<div class="app-launcher" role="application">
  <Input />
  <div class="content-area">
    {#key appState.command}
      <appState.command input={appState.input} />
    {/key}
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
