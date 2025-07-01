<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let workspaces = $state<number[]>(new Array(10).fill(0).map((_, i) => i + 1));
  let activeWorkspace = $state<number | null>(null);

  let unlisten: UnlistenFn;

  function handler(e: Event<number>) {
    console.log("workspace-changed", e);
    const workspaceId = e.payload;
    console.log("workspaceId", workspaceId);
    activeWorkspace = workspaceId;
  }

  onMount(async () => {
    unlisten = await listen<number>("workspace-changed", handler);
  });
  onDestroy(() => {
    unlisten();
  });
</script>

<div class="workspaces-container">
  {#each workspaces as workspace, i (workspace)}
    <button
      class="workspace-item"
      class:active={workspace === activeWorkspace}
      onclick={() =>
        invoke("change_workspace", { workspace })
          .then(console.log)
          .catch(console.error)}
    >
      {workspace}
    </button>
  {/each}
</div>

<style>
  button {
    /* remove default styles */
    background: none;
    border: none;
    padding: 0;
    cursor: pointer;
  }
  .workspaces-container {
    display: flex;
    align-items: center;
    height: 100%;
    gap: 2px;
  }

  .workspace-item {
    min-width: 24px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0 6px;
    border-radius: 3px;
    cursor: pointer;
    font-size: 12px;
    /* background-color: #444; */
    transition: all 0.2s ease;
  }

  .workspace-item:hover {
    background-color: #555;
  }

  .workspace-item.active {
    background-color: #666;
    font-weight: bold;
    color: #fff;
    width: 32px;
    box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.2);
  }
</style>
