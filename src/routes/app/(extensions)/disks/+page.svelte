<script lang="ts">
  import { discInfo } from "$lib/state/sysInfo.svelte";

  // You'll need to add disks data to your appState or create a separate disk state
  // For now, I'll assume you have a disks property in appState

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";

    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + " " + sizes[i];
  }

  function getUsagePercentage(total: number, available: number): number {
    if (total === 0) return 0;
    const used = total - available;
    return Math.round((used / total) * 100);
  }

  function getUsageColor(percentage: number): string {
    if (percentage > 90) return "#ff453a"; // Red
    if (percentage > 75) return "#ff9f0a"; // Orange
    if (percentage > 50) return "#ffcc02"; // Yellow
    return "#32d74b"; // Green
  }

  function getDiskIcon(type: string, isRemovable: boolean): string {
    if (isRemovable) return "💾";
    if (type.toLowerCase().includes("ssd")) return "⚡";
    if (type.toLowerCase().includes("hdd")) return "💽";
    return "💿";
  }

  function getDiskTypeDisplay(type: string): string {
    return type.toUpperCase();
  }
</script>

{#if discInfo.value.length > 0}
  <div class="disks-container">
    {#each discInfo.value as disk, i (i)}
      {@const usedSpace = disk.total_space - disk.available_space}
      {@const usagePercentage = getUsagePercentage(
        disk.total_space,
        disk.available_space,
      )}

      <div class="disk-card">
        <div class="disk-header">
          <div class="disk-icon-name">
            <span class="disk-icon"
              >{getDiskIcon(disk.type, disk.is_removable)}</span
            >
            <div class="disk-name-info">
              <span class="disk-name">{disk.name}</span>
              <span class="disk-type">{getDiskTypeDisplay(disk.type)}</span>
            </div>
          </div>
          <div class="disk-usage">
            <span
              class="usage-percentage"
              style="color: {getUsageColor(usagePercentage)}"
              >{usagePercentage}%</span
            >
            <span class="usage-text">used</span>
          </div>
        </div>

        <div class="disk-progress">
          <div class="progress-bar">
            <div
              class="progress-fill"
              style="width: {usagePercentage}%; background-color: {getUsageColor(
                usagePercentage,
              )}"
            ></div>
          </div>
        </div>

        <div class="disk-details">
          <div class="detail-row">
            <span class="detail-label">Total Space</span>
            <span class="detail-value">{formatBytes(disk.total_space)}</span>
          </div>

          <div class="detail-row">
            <span class="detail-label">Used Space</span>
            <span class="detail-value">{formatBytes(usedSpace)}</span>
          </div>

          <div class="detail-row">
            <span class="detail-label">Available</span>
            <span class="detail-value">{formatBytes(disk.available_space)}</span
            >
          </div>

          <div class="detail-row">
            <span class="detail-label">Mount Point</span>
            <span class="detail-value">{disk.mount_pont}</span>
          </div>

          <div class="detail-row">
            <span class="detail-label">File System</span>
            <span class="detail-value">{disk.file_system}</span>
          </div>

          <div class="detail-row">
            <span class="detail-label">Type</span>
            <span class="detail-value">
              {disk.type}
              {#if disk.is_removable}
                <span class="removable-badge">Removable</span>
              {/if}
            </span>
          </div>
        </div>
      </div>
    {/each}
  </div>
{:else}
  <div class="disk-card">
    <div class="no-disks">
      <span>No disk information available</span>
    </div>
  </div>
{/if}

<style>
  .disks-container {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .disk-card {
    background-color: #2c2c2e;
    border: 1px solid #3a3a3c;
    border-radius: 8px;
    padding: 16px;
    color: #ffffff;
  }

  .disk-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .disk-icon-name {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .disk-icon {
    font-size: 20px;
  }

  .disk-name-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .disk-name {
    font-size: 14px;
    font-weight: 600;
    color: #ffffff;
  }

  .disk-type {
    font-size: 11px;
    color: #8e8e93;
    font-weight: 500;
    letter-spacing: 0.5px;
  }

  .disk-usage {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 2px;
  }

  .usage-percentage {
    font-size: 16px;
    font-weight: 600;
  }

  .usage-text {
    font-size: 10px;
    color: #8e8e93;
    font-weight: 400;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .disk-progress {
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

  .disk-details {
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
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .removable-badge {
    background-color: #48484a;
    color: #8e8e93;
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 10px;
    font-weight: 500;
    text-transform: uppercase;
    letter-spacing: 0.3px;
  }

  .no-disks {
    text-align: center;
    padding: 20px;
    color: #8e8e93;
    font-size: 14px;
  }
</style>
