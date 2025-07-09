<script lang="ts">
  import { goto } from "$app/navigation";
  import { page } from "$app/state";
  import Icon from "$lib/bar/widgets/Icon.svelte";

  let { children } = $props();

  let isOnMainPage = $derived(page.route.id === "/app");

  let extensions = [
    {
      label: "git",
      path: "/app/gitskyline",
    },
    {
      label: "battery",
      path: "/app/battery",
    },
    {
      path: "/app/disks",
      label: "disk",
    },
    {
      path: "/app/settings",
      label: "settings",
    },
    {
      path: "/app/network",
      label: "net",
    },
  ];
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
      {#each extensions as item, i (item)}
        <button class="nav-btn config-btn" onclick={() => goto(item.path)}>
          {item.label}
        </button>
      {/each}
      <!-- <button
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
      </button> -->

      <Icon />
    </div>
  </nav>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--base-100);
    color: var(--base-fg);
    font-family: var(--font-family-sans);
    font-size: var(--font-size-sm);
    line-height: 1.4;
  }

  main {
    min-height: 100vh;
    background-color: var(--base-100);
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
    background-color: var(--base-200);
    border-top: 1px solid var(--base-300);
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
    border: 1px solid var(--secondary-bg);
    color: var(--base-fg);
    padding: 6px 12px;
    border-radius: var(--border-radius);
    cursor: pointer;
    font-family: inherit;
    font-size: var(--font-size-xs);
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .nav-btn:hover:not(:disabled) {
    background-color: var(--secondary-bg);
    border-color: var(--hover-bg);
  }

  .nav-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
    color: var(--disabled-fg);
  }
</style>
