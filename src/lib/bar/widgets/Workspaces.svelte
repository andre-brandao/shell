<script lang="ts">
  import { workspace } from "$lib/state/workspaces.svelte";

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
    background-color: var(--secondary-bg);
    color: var(--disabled-fg);
    transition: all 0.2s ease;
  }

  .workspace.active {
    background-color: var(--primary-bg);
    color: var(--primary-fg);
  }

  .workspace:hover {
    background-color: var(--hover-bg);
  }

  .workspace.active:hover {
    background-color: var(--primary-bg);
    filter: brightness(0.8);
  }
</style>
