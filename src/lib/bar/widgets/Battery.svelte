<script lang="ts">
  import { batteryInfo } from "$lib/state/sysInfo.svelte";

  // Calculate fill percentage (0-100)
  const fillPercentage = $derived(
    Math.max(0, Math.min(100, batteryInfo.value.current)),
  );

  // Calculate fill width based on percentage (instead of height)
  const fillWidth = $derived((fillPercentage / 100) * 15);

  // Determine battery color based on level and status
  const batteryColor = $derived.by(() => {
    if (batteryInfo.value.status === "Charging") return "#00ff00"; // Green when charging
    if (fillPercentage <= 20) return "#ff4444"; // Red when low
    if (fillPercentage <= 50) return "#ffaa00"; // Orange when medium
    return "#ffffff"; // White when good
  });

  // Determine if we should show charging animation
  const isCharging = $derived(batteryInfo.value.status === "Charging");
</script>

<div class="battery-widget">
  <svg
    width="24"
    height="16"
    viewBox="0 0 24 16"
    fill="none"
    class="battery-icon"
    class:charging={isCharging}
  >
    <!-- Battery body outline -->
    <rect
      x="1"
      y="2"
      width="18"
      height="12"
      rx="2"
      ry="2"
      stroke="currentColor"
      stroke-width="1.5"
      fill="none"
    />

    <!-- Battery terminal (positive end) -->
    <rect x="19" y="6" width="2" height="4" rx="1" ry="1" fill="currentColor" />

    <!-- Battery fill -->
    <rect
      x="2.5"
      y="3.5"
      width={fillWidth}
      height="9"
      rx="1"
      ry="1"
      fill={batteryColor}
      class="battery-fill"
    />

    <!-- Charging bolt icon (only shown when charging) -->
    {#if isCharging}
      <path
        d="M12 4L8 9h3v3l4-5h-3V4z"
        fill="#ffff00"
        stroke="#ffaa00"
        stroke-width="0.5"
        class="charging-bolt"
      />
    {/if}
  </svg>

  <span class="battery-level">{batteryInfo.value.current}%</span>
</div>

<style>
  .battery-widget {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .battery-icon {
    color: #8e8e93;
    transition: all 0.3s ease;
  }

  .battery-level {
    color: #ffffff;
    font-size: 14px;
    font-weight: 500;
  }

  .battery-fill {
    transition: all 0.5s ease;
  }

  .charging {
    filter: drop-shadow(0 0 4px rgba(0, 255, 0, 0.3));
  }

  .charging-bolt {
    animation: pulse 1.5s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.6;
    }
  }
</style>
