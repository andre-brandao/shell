<script lang="ts">
  import { activeClient } from "$lib/state.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen, type UnlistenFn, type Event } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  // let activeClient = $state<string>("");

  // let unlisten: UnlistenFn;

  // onMount(async () => {
  //   unlisten = await listen<{ class: string; title: string } | null>(
  //     "active-window-changed",
  //     (e) => {
  //       console.log("active-window-changed", e);
  //       if (!e.payload) {
  //         activeClient = "";
  //         return;
  //       }
  //       activeClient = `${e.payload.title}`;
  //     },
  //   );
  // });
  // onDestroy(() => {
  //   unlisten();
  // });
</script>

{activeClient.title}
