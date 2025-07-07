<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";

  interface Props {
    style: "transparent" | "filled" | "ghost" | "outiline";
  }

  let {
    children,
    style = "outiline",
    ...rest
  }: HTMLButtonAttributes & Props = $props();
</script>

<button {...rest}>
  {@render children?.()}
</button>
