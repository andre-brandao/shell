<script lang="ts">
  import { appState } from "$lib/launcher/launcher.svelte";
</script>

<div
  class="search-container"
  role="searchbox"
  tabindex="0"
  onkeydown={(e) => {
    // if (e.key === "Enter") {
    //   appState.command = "search";
    // } else if (e.key === "Escape") {
    //   appState.command = "help";
    // }
  }}
  onclick={() => appState.inputRef?.focus()}
>
  <input
    bind:value={appState.input}
    bind:this={appState.inputRef}
    type="text"
    placeholder="Search apps..."
    class="search-input"
  />
  {#if appState.input}
    <button
      class="clear-btn"
      aria-label="Clear search"
      onclick={() => appState.clearInput()}
    >
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <line x1="18" y1="6" x2="6" y2="18"></line>
        <line x1="6" y1="6" x2="18" y2="18"></line>
      </svg>
    </button>
  {/if}
</div>

<style>
  .search-container {
    position: relative;
    padding: 16px 16px 16px 16px;
    border-bottom: 1px solid #3a3a3c;
    cursor: text;
  }

  .search-input {
    width: 100%;
    padding: 0;
    color: white;
    font-size: 16px;
    background-color: transparent;
    border: none;
    outline: none;
    cursor: text;
  }

  .search-input:focus {
    outline: none;
    background-color: transparent;
  }

  .search-input::placeholder {
    color: #8e8e93;
  }

  .clear-btn {
    position: absolute;
    /* right: 50px; */
    top: 50%;
    transform: translateY(-50%);
    background: transparent;
    border: none;
    color: #8e8e93;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .clear-btn:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }
</style>
