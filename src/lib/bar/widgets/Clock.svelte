<script lang="ts">
  let time = $state("");
  let intervalId: number | undefined = $state();

  function updateTime() {
    const now = new Date();
    time = now.toLocaleTimeString("pt-BR", {
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    });
  }
  $effect(() => {
    updateTime();
    intervalId = setInterval(updateTime, 1000);
    return () => {
      clearInterval(intervalId);
    };
  });
</script>

<span class="clock">{time}</span>

<style>
  .clock {
    font-variant-numeric: tabular-nums;
    font-weight: 600;
    color: #ffffff;
    font-size: 12px;
  }
</style>
