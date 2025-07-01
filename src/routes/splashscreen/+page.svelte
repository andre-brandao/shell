<script>
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  let { data } = $props();
  onMount(() => {
    // @ts-expect-error it is a Tauri window
    data.win?.show().then(() => data.win?.setFocus());
    goto("/app");
  });
</script>

<main>
  <div class="cent">
    <img src="/favicon.png" alt="LOGO" />
    <h2>Deds Shell</h2>
  </div>
</main>

<style>
  /* <!-- give me some default style with the div on the center --> */
  main {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #f0f0f0;
  }
  .cent {
    text-align: center;
  }
  img {
    width: 100px;
    height: 100px;
    margin-bottom: 20px;
  }
  h2 {
    font-size: 24px;
    color: #333;
  }
</style>
