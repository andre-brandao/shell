<script lang="ts">
  // import {
  //   assetToPath,
  //   onEventShowMenu,
  //   showMenu,
  //   type ContextMenu,
  // } from "$lib/utils/context-menu";
  import { invoke } from "@tauri-apps/api/core";

  let result_binds = $state("");
</script>

<main class="container">
  <div class="test-c">
    <h1>Teste Binds</h1>
    <button
      onclick={async () => {
        const res = await invoke("hypr_clients");
        console.log(res);
        result_binds = JSON.stringify(res, null, 2);
      }}
    >
      Teste
    </button>

    <pre>
      {result_binds}
    </pre>
  </div>
</main>

<style>
  /* <!-- styel it --> */

  .container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    /* min-height: 100vh; */
    background-color: #282c34;
    color: white;
    padding: 20px;
    overflow: scroll;
    max-height: 100vh;
  }

  .test-c {
    margin-bottom: 20px;
    padding: 20px;
    border: 1px solid #61dafb;
    border-radius: 8px;
    text-align: center;
    width: 80%;
    max-width: 600px;
  }

  h1 {
    font-size: 2em;
    margin-bottom: 10px;
  }

  button {
    background-color: #61dafb;
    color: #282c34;
    border: none;
    padding: 10px 20px;
    border-radius: 5px;
    cursor: pointer;
    font-size: 1em;
    transition: background-color 0.3s ease;
  }

  button:hover {
    background-color: #47a3c6;
  }

  pre {
    background-color: #333;
    color: #f0f0f0;
    padding: 10px;
    border-radius: 5px;
    overflow-x: auto;
    white-space: pre-wrap;
    word-wrap: break-word;
    text-align: left;
  }
</style>
