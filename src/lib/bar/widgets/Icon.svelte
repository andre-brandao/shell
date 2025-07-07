<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  function toggle() {
    invoke<any>("toggle_launcher")
      .then((e) => {
        console.log(e);
      })
      .catch(console.error);
  }
</script>

<button
  class="icon-widget"
  aria-label="Toggle Launcher"
  onclick={() => {
    toggle();
  }}
>
  <span class="icon">
    <img src="/nix-snowflake-colours.svg" alt="" height="20px" />
  </span>
</button>

<style>
  button {
    background: transparent;
    border: none;
    color: #ffffff;
    padding: 0;
    cursor: pointer;
    font-family: inherit;
    font-size: 12px;
    display: flex;
    align-items: center;
    gap: 4px;
  }
  .icon-widget {
    display: flex;
    align-items: center;
  }

  .icon {
    font-size: 12px;
  }

  .icon-widget:hover {
    transform: scale(0.9);
  }
</style>
