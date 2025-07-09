<script lang="ts">
  import Clock from "./widgets/Clock.svelte";
  import Workspaces from "./widgets/Workspaces.svelte";
  import Hostname from "./widgets/Hostname.svelte";
  import ActiveClient from "./widgets/ActiveClient.svelte";
  import Battery from "./widgets/Battery.svelte";
  import SysInfo from "./widgets/SysInfoIcon.svelte";
  import Disks from "./widgets/Disks.svelte";
  import Icon from "./widgets/Icon.svelte";
</script>

<main class="container">
  <div class="section left">
    <Icon />
    <SysInfo />
    <Disks />
    <Workspaces />
  </div>

  <div class="section center">
    <Clock />
  </div>

  <div class="section right">
    <ActiveClient />
    <Battery />
  </div>
</main>

<style>
  .container {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    height: 30px;
    background-color: var(--base-200);
    color: var(--base-fg);
    padding: 0 12px;
    margin: 0;
    overflow: hidden;
    white-space: nowrap;
    font-family: var(--font-family-sans);
    font-size: var(--font-size-xs);
    font-weight: 500;
    border-bottom: 1px solid var(--base-300);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    position: relative;
  }

  .container::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      180deg,
      rgba(255, 255, 255, 0.05) 0%,
      rgba(255, 255, 255, 0.02) 100%
    );
    pointer-events: none;
  }

  .section {
    display: flex;
    align-items: center;
    height: 100%;
    z-index: 1;
  }

  .left {
    flex: 1;
    justify-content: flex-start;
    gap: 8px;
  }

  .center {
    flex: 1;
    justify-content: center;
    gap: 8px;
  }

  .right {
    flex: 1;
    justify-content: flex-end;
    gap: 8px;
  }

  /* Add subtle separators between sections */
  /* .center::before, */
  .right::before {
    content: "";
    width: 1px;
    height: 16px;
    background-color: var(--secondary-bg);
    margin-right: 8px;
  }

  /* .center::before {
    margin-left: 8px;
  } */
</style>
