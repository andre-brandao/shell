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
  onclick={() => {
    toggle();
  }}
>
  <img src="/tauri.svg" height="24" width="24" alt="logo" />
</button>
