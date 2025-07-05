<script lang="ts">
  import { batteryInfo } from "$lib/state/sysInfo.svelte";
  // You'll need to add battery data to your appState or create a separate battery state
  // For now, I'll assume you have a battery property in appState

  function formatUptime(seconds: number): string {
    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }

  function getBatteryIcon(status: string, level: number): string {
    if (status === "Charging") return "⚡";
    if (level > 75) return "🔋";
    if (level > 50) return "🔋";
    if (level > 25) return "🪫";
    return "🪫";
  }

  function getBatteryColor(level: number): string {
    if (level > 50) return "#32d74b"; // Green
    if (level > 25) return "#ff9f0a"; // Orange
    return "#ff453a"; // Red
  }
</script>

{#if batteryInfo.value}
  <div class="battery-card">
    <div class="battery-header">
      <div class="battery-icon-level">
        <span class="battery-icon"
          >{getBatteryIcon(
            batteryInfo.value.status,
            batteryInfo.value.current,
          )}</span
        >
        <span
          class="battery-level"
          style="color: {getBatteryColor(batteryInfo.value.current)}"
          >{batteryInfo.value.current}%</span
        >
      </div>
      <div class="battery-status">
        <span
          class="status-text"
          class:charging={batteryInfo.value.status === "Charging"}
          >{batteryInfo.value.status}</span
        >
      </div>
    </div>

    <div class="battery-progress">
      <div class="progress-bar">
        <div
          class="progress-fill"
          style="width: {batteryInfo.value
            .current}%; background-color: {getBatteryColor(
            batteryInfo.value.current,
          )}"
        ></div>
      </div>
    </div>

    <div class="battery-details">
      <div class="detail-row">
        <span class="detail-label">Technology</span>
        <span class="detail-value">{batteryInfo.value.technology}</span>
      </div>

      <div class="detail-row">
        <span class="detail-label">Health</span>
        <span class="detail-value">{batteryInfo.value.health}</span>
      </div>

      <div class="detail-row">
        <span class="detail-label">Model</span>
        <span class="detail-value">{batteryInfo.value.model_name}</span>
      </div>

      {#if batteryInfo.value.voltage_now}
        <div class="detail-row">
          <span class="detail-label">Voltage</span>
          <span class="detail-value"
            >{(batteryInfo.value.voltage_now / 1000000).toFixed(2)}V</span
          >
        </div>
      {/if}

      {#if batteryInfo.value.current_now}
        <div class="detail-row">
          <span class="detail-label">Current</span>
          <span class="detail-value"
            >{(batteryInfo.value.current_now / 1000).toFixed(0)}mA</span
          >
        </div>
      {/if}

      {#if batteryInfo.value.power_now}
        <div class="detail-row">
          <span class="detail-label">Power</span>
          <span class="detail-value"
            >{(batteryInfo.value.power_now / 1000000).toFixed(1)}W</span
          >
        </div>
      {/if}

      {#if batteryInfo.value.battery_index !== undefined}
        <div class="detail-row">
          <span class="detail-label">Battery Index</span>
          <span class="detail-value">{batteryInfo.value.battery_index}</span>
        </div>
      {/if}
    </div>
  </div>
{:else}
  <div class="battery-card">
    <div class="no-battery">
      <span>No battery information available</span>
    </div>
  </div>
{/if}

<style>
  .battery-card {
    background-color: #2c2c2e;
    border: 1px solid #3a3a3c;
    border-radius: 8px;
    padding: 16px;
    margin: 8px 0;
    color: #ffffff;
  }

  .battery-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .battery-icon-level {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .battery-icon {
    font-size: 20px;
  }

  .battery-level {
    font-size: 18px;
    font-weight: 600;
  }

  .battery-status {
    display: flex;
    align-items: center;
  }

  .status-text {
    font-size: 12px;
    font-weight: 500;
    color: #8e8e93;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .status-text.charging {
    color: #32d74b;
  }

  .battery-progress {
    margin-bottom: 16px;
  }

  .progress-bar {
    width: 100%;
    height: 6px;
    background-color: #48484a;
    border-radius: 3px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    transition: width 0.3s ease;
    border-radius: 3px;
  }

  .battery-details {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .detail-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 4px 0;
  }

  .detail-label {
    font-size: 12px;
    color: #8e8e93;
    font-weight: 400;
  }

  .detail-value {
    font-size: 12px;
    color: #ffffff;
    font-weight: 500;
    text-align: right;
  }

  .no-battery {
    text-align: center;
    padding: 20px;
    color: #8e8e93;
    font-size: 14px;
  }
</style>
