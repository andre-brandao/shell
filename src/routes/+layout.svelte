<script lang="ts">
  import { onMount } from "svelte";
  import type { LayoutProps } from "./$types";
  import { init } from "$lib/state.svelte";
  // let unlisten: undefined | (() => void) = $state();

  $effect(() => {
    const deinit = init();

    // return () => {
    //   deinit.then((f) => f());
    // };
  });

  let { children, data }: LayoutProps = $props();
</script>

{@render children()}
