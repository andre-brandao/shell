<script lang="ts">
  import { sysinfo } from "$lib/cmds";
  import type { NetworkInfo } from "$lib/types";
  import { onDestroy, onMount } from "svelte";
  let info: NetworkInfo | undefined = $state();

  let timeoutId: number | undefined = $state();
  onMount(async () => {
    info = await sysinfo.network.get();
    timeoutId = setInterval(async () => {
      info = await sysinfo.network.get();
      console.log(info);
    }, 3000);
  });

  onDestroy(() => {
    clearInterval(timeoutId);
  });
  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  }
</script>

<div class="network-container">
  {#if info}
    <!-- Header Section -->
    <!-- <div class="network-header">
      <h1 class="network-title">Network Information</h1>
      <div class="network-summary">
        <div class="summary-card bg-primary text-primary-fg">
          <div class="summary-value">{info.interface_count}</div>
          <div class="summary-label">Interfaces</div>
        </div>
        <div class="summary-card bg-accent text-accent-fg">
          <div class="summary-value">{formatBytes(info.total_received)}</div>
          <div class="summary-label">Total Received</div>
        </div>
        <div class="summary-card bg-info text-info-fg">
          <div class="summary-value">{formatBytes(info.total_transmitted)}</div>
          <div class="summary-label">Total Transmitted</div>
        </div>
      </div>
    </div> -->

    <!-- Interface Cards -->
    <div class="interfaces-grid">
      {#each info.interfaces as nt, i (nt)}
        <div class="interface-card">
          <div class="interface-header">
            <h3 class="interface-name">{nt.name}</h3>
            <div class="interface-status">
              <span class="status-dot"></span>
              Active
            </div>
          </div>

          <div class="interface-stats">
            <div class="stat-group">
              <h4 class="stat-category">Data Transfer</h4>
              <div class="stat-row">
                <div class="stat-item">
                  <span class="stat-label">Received:</span>
                  <span class="stat-value received"
                    >{formatBytes(nt.received_bytes)}</span
                  >
                </div>
                <div class="stat-item">
                  <span class="stat-label">Transmitted:</span>
                  <span class="stat-value transmitted"
                    >{formatBytes(nt.transmitted_bytes)}</span
                  >
                </div>
              </div>
            </div>

            <div class="stat-group">
              <h4 class="stat-category">Packets</h4>
              <div class="stat-row">
                <div class="stat-item">
                  <span class="stat-label">Received:</span>
                  <span class="stat-value"
                    >{nt.packets_received.toLocaleString()}</span
                  >
                </div>
                <div class="stat-item">
                  <span class="stat-label">Transmitted:</span>
                  <span class="stat-value"
                    >{nt.packets_transmitted.toLocaleString()}</span
                  >
                </div>
              </div>
            </div>

            <div class="stat-group">
              <h4 class="stat-category">Errors</h4>
              <div class="stat-row">
                <div class="stat-item">
                  <span class="stat-label">RX Errors:</span>
                  <span class="stat-value error">{nt.errors_on_received}</span>
                </div>
                <div class="stat-item">
                  <span class="stat-label">TX Errors:</span>
                  <span class="stat-value error"
                    >{nt.errors_on_transmitted}</span
                  >
                </div>
              </div>
            </div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="loading-container">
      <div class="loading-spinner"></div>
      <p class="loading-text">Loading network information...</p>
    </div>
  {/if}
</div>

<style>
  .network-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: var(--spacing-lg);
    font-family: var(--font-family-sans);
    max-height: 100vh;
  }

  .network-header {
    margin-bottom: var(--spacing-xl);
  }

  .network-title {
    font-size: var(--font-size-3xl);
    font-weight: 600;
    color: var(--base-fg);
    margin-bottom: var(--spacing-lg);
    text-align: center;
  }

  .network-summary {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: var(--spacing-md);
    margin-bottom: var(--spacing-xl);
  }

  .summary-card {
    padding: var(--spacing-lg);
    border-radius: var(--border-radius);
    text-align: center;
    box-shadow: var(--shadow-md);
  }

  .summary-value {
    font-size: var(--font-size-2xl);
    font-weight: 700;
    margin-bottom: var(--spacing-xs);
  }

  .summary-label {
    font-size: var(--font-size-sm);
    opacity: 0.9;
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .interfaces-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(400px, 1fr));
    gap: var(--spacing-lg);
  }

  .interface-card {
    background: var(--base-100);
    border: 1px solid var(--border-color);
    border-radius: var(--border-radius);
    padding: var(--spacing-lg);
    box-shadow: var(--shadow-sm);
    transition: box-shadow 0.2s ease;
  }

  .interface-card:hover {
    box-shadow: var(--shadow-md);
  }

  .interface-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: var(--spacing-lg);
    padding-bottom: var(--spacing-md);
    border-bottom: 1px solid var(--border-color);
  }

  .interface-name {
    font-size: var(--font-size-xl);
    font-weight: 600;
    color: var(--base-fg);
    margin: 0;
    font-family: var(--font-family-mono);
  }

  .interface-status {
    display: flex;
    align-items: center;
    gap: var(--spacing-xs);
    font-size: var(--font-size-sm);
    color: var(--accent-bg);
    font-weight: 500;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent-bg);
    animation: pulse 2s infinite;
  }

  @keyframes pulse {
    0% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
    100% {
      opacity: 1;
    }
  }

  .interface-stats {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-lg);
  }

  .stat-group {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .stat-category {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--secondary-bg);
    text-transform: uppercase;
    letter-spacing: 0.05em;
    margin: 0;
  }

  .stat-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--spacing-md);
  }

  .stat-item {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: var(--spacing-sm);
    background: var(--base-200);
    border-radius: var(--border-radius);
  }

  .stat-label {
    font-size: var(--font-size-sm);
    color: var(--base-fg);
    font-weight: 500;
  }

  .stat-value {
    font-size: var(--font-size-sm);
    font-weight: 600;
    font-family: var(--font-family-mono);
  }

  .stat-value.received {
    color: var(--accent-bg);
  }

  .stat-value.transmitted {
    color: var(--info-bg);
  }

  .stat-value.error {
    color: var(--error-bg);
  }

  .loading-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--spacing-xl);
    min-height: 300px;
  }

  .loading-spinner {
    width: 40px;
    height: 40px;
    border: 4px solid var(--border-color);
    border-top: 4px solid var(--primary-bg);
    border-radius: 50%;
    animation: spin 1s linear infinite;
    margin-bottom: var(--spacing-md);
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }

  .loading-text {
    font-size: var(--font-size-lg);
    color: var(--secondary-bg);
    margin: 0;
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .network-container {
      padding: var(--spacing-md);
    }

    .network-summary {
      grid-template-columns: 1fr;
    }

    .interfaces-grid {
      grid-template-columns: 1fr;
    }

    .stat-row {
      grid-template-columns: 1fr;
    }

    .interface-header {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--spacing-sm);
    }
  }
</style>
