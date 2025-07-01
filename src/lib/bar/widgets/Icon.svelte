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
  <span class="icon">  </span>
</button>

<style>
  .icon-widget {
    display: flex;
    align-items: center;
  }

  .icon {
    font-size: 12px;
  }
</style>
