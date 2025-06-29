<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Clock from "./Clock.svelte";

  // let name = $state("");
  // let greetMsg = $state("");

  // async function greet(event: Event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }
</script>

<main class="container">
  <div class="left">lefty</div>
  <div class="center">
    <!-- {new Date().getDate()} -->
    <Clock />
  </div>

  <div class="right">temp right widget</div>
</main>

<style>
  /* <!-- style for a top bar of an operating system --> */
  .container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 25px;
    background-color: #333;
    color: white;
    padding: 0 0;
    margin: 0;
    /* remove scroll */
    overflow: hidden;
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
