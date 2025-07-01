<script lang="ts">
  import { discInfo } from "$lib/state/sysInfo.svelte";
</script>

{#if discInfo}
  {#each discInfo.value as disk}
    <div class="disks-widget">
      <span class="disk-icon">💾</span>
      <span class="disk-usage">
        {(
          (disk.total_space - disk.available_space) /
          (1024 * 1024 * 1024)
        ).toFixed(2)} GB used</span
      >
    </div>
  {/each}
{/if}

<style>
  .disks-widget {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .disk-icon {
    font-size: 10px;
  }

  .disk-usage {
    font-size: 11px;
    font-weight: 500;
    color: #ffffff;
    font-variant-numeric: tabular-nums;
  }
</style>
