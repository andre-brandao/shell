<script lang="ts">
  import "../styles.css";
  import type { LayoutProps } from "./$types";

  let { children, data }: LayoutProps = $props();
</script>

{@render children()}
