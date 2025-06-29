<script lang="ts">
  let time = $state("");
  let intervalId: number | undefined = $state();

  function updateTime() {
    const now = new Date();
    time = now.toLocaleTimeString();
  }
  $effect(() => {
    updateTime();
    intervalId = setInterval(updateTime, 1000);
    return () => {
      clearInterval(intervalId);
    };
  });
</script>

{time}
