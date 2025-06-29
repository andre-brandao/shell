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
  <div class="info">
    {#each info as disk}
      <div>
        {(
          (disk.total_space - disk.available_space) /
          (1024 * 1024 * 1024)
        ).toFixed(2)} GB used
      </div>
    {/each}
  </div>
{/if}

<style>
  .info {
    display: flex;
    gap: 10px;
    font-size: 14px;
    color: white;
  }

  .info div {
    background-color: #444;
    padding: 5px 10px;
    border-radius: 4px;
  }
  .info div:hover {
    background-color: #555;
  }
</style>
