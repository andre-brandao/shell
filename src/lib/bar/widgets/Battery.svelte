<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  let battery = $state<number>(0);

  let isCharging = $state<boolean>(false);

  let status = $derived.by(() => {
    if (battery > 75) return "High";
    if (battery > 50) return "Medium";
    if (battery > 25) return "Low";
    return "Critical";
  });

  function getBat() {
    invoke<number>("get_battery_sys")
      .then((e) => {
        const bat = (e as number) || 0;
        battery = bat;
      })
      .catch(console.error);
  }

  $effect(() => {
    let timeOutId: number | null = null;
    getBat();
    timeOutId = setTimeout(getBat, 10000);
    return () => clearTimeout(timeOutId);
  });
</script>

{battery}
