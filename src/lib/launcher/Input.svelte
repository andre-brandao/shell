<script lang="ts">
  import { appState } from "$lib/launcher/launcher.svelte";

  type Props = {
    onChange?: (value: string) => void | Promise<void>;
    onEnterPresed?: () => void | Promise<void>;
  };

  let { onChange, onEnterPresed }: Props = $props();
</script>

<div
  class="search-container"
  role="searchbox"
  tabindex="0"
  onkeydown={(e) => {
    if (e.key === "Enter") {
      onEnterPresed?.();
    }
  }}
  onclick={() => appState.inputRef?.focus()}
>
  <div class="input-wrapper">
    <input
      bind:value={appState.input}
      bind:this={appState.inputRef}
      type="text"
      placeholder="Search apps..."
      class="search-input"
      onkeydown={() => onChange?.(appState.search)}
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
</div>

<style>
  .search-container {
    padding: 10px;
    border-bottom: 1px solid #3a3a3c;
    cursor: text;
  }

  .input-wrapper {
    position: relative;
    display: flex;
    align-items: center;
    min-height: 40px;
  }

  .search-input {
    flex: 1;
    /* height: 48px; */
    padding: 0;
    padding-right: 32px;
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
    right: 0;
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
    width: 24px;
    height: 24px;
  }

  .clear-btn:hover {
    background-color: rgba(255, 255, 255, 0.1);
    color: #ffffff;
  }
</style>
