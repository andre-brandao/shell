<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  let info = $state<
    {
      name: string;
      total_space: number;
      available_space: number;
      type: string;
      mount_pont: string;
    }[]
  >();

  function getBat() {
    invoke<any>("get_disk_info")
      .then((e) => {
        console.log(e);
        info = [e[0]];
      })
      .catch(console.error);
  }

  $effect(() => {
    let timeOutId: number | null = null;
    getBat();
    timeOutId = setTimeout(getBat, 10000);
    return () => clearTimeout(timeOutId);
  });
</script>

{#if info}
  {#each info as disk}
    <div class="disks-widget">
      <span class="disk-icon">💾</span>
      <span class="disk-usage">
        {(
          (disk.total_space - disk.available_space) /
          (1024 * 1024 * 1024)
        ).toFixed(2)} GB used</span
      >
    </div>

    <!-- <div>
        {(
          (disk.total_space - disk.available_space) /
          (1024 * 1024 * 1024)
        ).toFixed(2)} GB used
      </div> -->
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
