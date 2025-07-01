<script lang="ts">
  import { sysInfo } from "$lib/state/sysInfo.svelte";
</script>

{#if sysInfo}
  <div class="sysinfo-widget">
    <span class="metric">CPU {sysInfo.value.cpu_usage}%</span>
    <span class="separator">•</span>
    <span class="metric">RAM {sysInfo.value.ram_usage.toFixed(0)}%</span>
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
