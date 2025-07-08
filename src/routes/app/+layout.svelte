<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import Icon from "$lib/bar/widgets/Icon.svelte";

  let { children } = $props();

  let isOnMainPage = $derived(page.route.id === "/app");
</script>

<main>
  <div class="content">
    {@render children()}
  </div>

  <nav class="nav-bar">
    <div class="nav-left">
      <button
        class="nav-btn back-btn"
        disabled={isOnMainPage}
        onclick={() => goto("/app")}
      >
        {isOnMainPage ? "_" : "← back"}
      </button>
    </div>
    <div class="nav-right">
      <button
        class="nav-btn config-btn"
        onclick={() => goto("/app/gitskyline")}
      >
        git
      </button>
      <button class="nav-btn config-btn" onclick={() => goto("/app/battery")}>
        battery
      </button>
      <button class="nav-btn config-btn" onclick={() => goto("/app/disks")}>
        disks
      </button>
      <button class="nav-btn config-btn" onclick={() => goto("/app/settings")}>
        config
      </button>

      <Icon />
    </div>
  </nav>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background-color: #1c1c1e;
    color: #ffffff;
    font-family:
      -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
    font-size: 14px;
    line-height: 1.4;
  }

  main {
    min-height: 100vh;
    background-color: #1c1c1e;
    display: flex;
    flex-direction: column;
  }

  .content {
    flex: 1;
    padding: 0;
    padding-bottom: 60px; /* Add padding to prevent content from being hidden behind the navbar */
    overflow-y: auto;
  }

  .nav-bar {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: #2c2c2e;
    border-top: 1px solid #3a3a3c; /* Changed from border-bottom to border-top */
    z-index: 10;
  }

  .nav-left,
  .nav-right {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
  }

  .nav-btn {
    background: transparent;
    border: 1px solid #48484a;
    color: #ffffff;
    padding: 6px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-family: inherit;
    font-size: 13px;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .nav-btn:hover:not(:disabled) {
    background-color: #48484a;
    border-color: #5a5a5c;
  }

  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    color: #8e8e93;
  }
</style>
