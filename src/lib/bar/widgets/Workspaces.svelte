<script lang="ts">
  import { workspace } from "$lib/state/workspaces.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let workspaces = $state<number[]>(new Array(10).fill(0).map((_, i) => i + 1));
</script>

<div class="workspaces-widget">
  {#each workspaces as w, i (w)}
    {@const isActive = workspace.activeWorkpace.id === w}
    <button
      class="workspace"
      class:active={isActive}
      onclick={() => workspace.changeWorkpace(i)}
    >
      {isActive ? "_" : w}
    </button>
  {/each}
</div>

<style>
  .workspaces-widget {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .workspace {
    width: 16px;
    height: 16px;
    border-radius: 3px;
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 10px;
    font-weight: 600;
    background-color: #48484a;
    color: #8e8e93;
    transition: all 0.2s ease;
  }

  .workspace.active {
    background-color: #007aff;
    color: #ffffff;
  }

  .workspace:hover {
    background-color: #5a5a5c;
  }

  .workspace.active:hover {
    background-color: #0056cc;
  }
</style>
