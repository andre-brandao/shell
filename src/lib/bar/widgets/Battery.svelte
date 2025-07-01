<script lang="ts">
  import { batteryInfo } from "$lib/state/sysInfo.svelte";
</script>

<div class="battery-widget">
  <span class="battery-icon">🔋</span>
  <span class="battery-level">{batteryInfo.value}%</span>
</div>

<style>
  .battery-widget {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .battery-icon {
    font-size: 10px;
  }

  .battery-level {
    font-size: 11px;
    font-weight: 500;
    color: #ffffff;
    font-variant-numeric: tabular-nums;
  }
</style>
