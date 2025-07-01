<script lang="ts">
  import Clock from "./widgets/Clock.svelte";
  import Workspaces from "./widgets/Workspaces.svelte";
  import Hostname from "./widgets/Hostname.svelte";
  import ActiveClient from "./widgets/ActiveClient.svelte";
  import Battery from "./widgets/Battery.svelte";
  import SysInfo from "./widgets/SysInfo.svelte";
  import Disks from "./widgets/Disks.svelte";
  import Icon from "./widgets/Icon.svelte";
</script>

<main class="container">
  <div class="section left">
    <Icon />
    <SysInfo />
    <Disks />
    <Workspaces />
    <div class="widget"></div>
  </div>

  <div class="section center">
    <div class="widget">
      <Clock />
    </div>
  </div>

  <div class="section right">
    <div class="widget">
      <ActiveClient />

      <Battery />
    </div>
  </div>
</main>

<style>
  .container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 30px;
    background-color: #333;
    color: white;
    padding: 0 10px;
    margin: 0;
    overflow: hidden;
    white-space: nowrap;
    font-family: Arial, sans-serif;
    font-size: 14px;
    font-weight: normal;
  }

  .section {
    display: flex;
    align-items: center;
    height: 100%;
  }

  .left {
    flex: 1;
    justify-content: flex-start;
    gap: 10px;
  }

  .center {
    flex: 1;
    justify-content: center;
    gap: 10px;
  }

  .right {
    flex: 1;
    justify-content: flex-end;
    gap: 10px;
  }

  .widget {
    padding: 0 8px;
    height: 100%;
    display: flex;
    align-items: center;
  }

  /* Optional: Add hover effect for widgets */
  .widget:hover {
    background-color: #444;
  }
</style>
