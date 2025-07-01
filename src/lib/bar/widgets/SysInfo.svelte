<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  let info = $state<{
    cpu_usage: number;
    ram_usage: number;
    // total_memory: number;
    // used_memory: number;
  }>();

  function getBat() {
    invoke<any>("get_system_info")
      .then((e) => {
        console.log(e);
        info = e || {};
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
    <div>
      CPU: {info.cpu_usage}
    </div>
    <div>
      RAM: {info.ram_usage.toFixed(0)}%
    </div>
    <!-- <div>
      Used Storage: {info.used_memory} GB
    </div> -->
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
