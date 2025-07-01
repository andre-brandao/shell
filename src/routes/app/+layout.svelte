<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";

  let { children } = $props();

  let isOnMainPage = $derived(page.route.id === "/app");
</script>

<main>
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
      <button class="nav-btn config-btn" onclick={() => goto("/app/settings")}>
        config
      </button>
    </div>
  </nav>

  <div class="content">
    {@render children()}
  </div>
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
  }

  .nav-bar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background-color: #2c2c2e;
    border-bottom: 1px solid #3a3a3c;
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

  .content {
    padding: 0;
  }
</style>
