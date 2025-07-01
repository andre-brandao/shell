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
  <div class="sysinfo-widget">
    <span class="metric">CPU {info.cpu_usage}%</span>
    <span class="separator">•</span>
    <span class="metric">RAM {info.ram_usage.toFixed(0)}%</span>
  </div>
{/if}

<style>
  .sysinfo-widget {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .metric {
    font-size: 11px;
    font-weight: 500;
    color: #ffffff;
    font-variant-numeric: tabular-nums;
  }

  .separator {
    color: #8e8e93;
    font-size: 10px;
  }
</style>
