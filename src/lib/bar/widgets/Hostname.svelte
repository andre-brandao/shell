<script>
  import { platform, hostname, locale } from "@tauri-apps/plugin-os";
</script>

{#await hostname()}
  <span>Loading...</span>
{:then host}
  <span>{host}</span>
{:catch error}
  <span>Error: {error}</span>
{/await}
