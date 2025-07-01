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

<div
  class="icon-widget"
  aria-label="Toggle Launcher"
  onclick={() => {
    toggle();
  }}
>
  <span class="icon">  </span>
</div>

<style>
  .icon-widget {
    display: flex;
    align-items: center;
    cursor: pointer;
  }

  .icon {
    font-size: 12px;
  }
</style>
