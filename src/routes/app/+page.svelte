<script lang="ts">
  import { appState } from "$lib/launcher/launcher.svelte";
  import Input from "$lib/launcher/Input.svelte";
  import type {
    LauncherComponent,
    LauncherPluginComponentProps,
    LauncherPluginExports,
  } from "$lib/launcher/types";
  import Help from "$lib/launcher/plugins/Help.svelte";
  import type { Component } from "svelte";

  // onMount(() => {
  //   // Initialize the app state when the component mounts
  //   appState.getApps();
  // });

  let commandRef: ReturnType<LauncherComponent> | undefined = $state();
  $inspect(appState.command);
</script>

<div class="app-launcher" role="application">
  <Input
    onChange={(e) => commandRef?.onInputChanged?.(e)}
    onEnterPresed={() => commandRef?.onEnterPressed?.()}
  />
  <div class="content-area">
    {#key appState.command}
      <appState.command bind:this={commandRef} input={appState.search} />
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
