<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Clock from "./Clock.svelte";
  import { platform, hostname, locale } from "@tauri-apps/plugin-os";

  // let name = $state("");
  // let greetMsg = $state("");

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main class="container">
  <div class="left">
    {#await hostname()}
      Loading hostname...
    {:then host}
      Hostname: {host}
    {:catch error}
      Error fetching hostname: {error.message}
    {/await}
  </div>
  <div class="center">
    <Clock />
  </div>

  <div class="right">temp right widget</div>
</main>

<style>
  /* <!-- style for a top bar of an operating system --> */
  .container {
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    height: 30px;
    background-color: #333;
    color: white;
    padding: 0 5px;
    margin: 0 0 0 0;
    overflow: hidden;
    /* remove scroll */
    white-space: nowrap;

    font-family: Arial, sans-serif;
    font-size: 14px;
    font-weight: normal;
    text-align: center;
  }
  .left {
    flex: 1;
    text-align: left;
  }
  .center {
    flex: 1;
    text-align: center;
  }
  .right {
    flex: 1;
    text-align: right;
  }
</style>
