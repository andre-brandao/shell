<script lang="ts">
  let settings = $state({
    theme: "dark",
    autoLaunch: true,
    showIcons: false,
  });
</script>

<div class="settings-page">
  <h1>Settings</h1>

  <div class="settings-group">
    <h2>Appearance</h2>
    <div class="setting-item">
      <label>
        <input type="checkbox" bind:checked={settings.showIcons} />
        Show app icons
      </label>
    </div>
  </div>

  <div class="settings-group">
    <h2>Behavior</h2>
    <div class="setting-item">
      <label>
        <input type="checkbox" bind:checked={settings.autoLaunch} />
        Auto-launch on startup
      </label>
    </div>
  </div>
</div>

<style>
  .settings-page {
    background-color: #1c1c1e;
    min-height: calc(100vh - 60px);
  }

  h1 {
    margin: 0;
    padding: 20px 16px 16px;
    color: #ffffff;
    font-size: 22px;
    font-weight: 600;
    background-color: #2c2c2e;
    border-bottom: 1px solid #3a3a3c;
  }

  .settings-group {
    margin-bottom: 0;
  }

  h2 {
    color: #8e8e93;
    font-size: 13px;
    font-weight: 500;
    margin: 0;
    padding: 16px 16px 8px;
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .setting-item {
    background-color: #2c2c2e;
    border-bottom: 1px solid #3a3a3c;
    padding: 12px 16px;
    transition: background-color 0.2s ease;
  }

  .setting-item:hover {
    background-color: #3a3a3c;
  }

  label {
    display: flex;
    align-items: center;
    gap: 12px;
    cursor: pointer;
    color: #ffffff;
    font-size: 14px;
    font-weight: 400;
  }

  input[type="checkbox"] {
    width: 16px;
    height: 16px;
    accent-color: #007aff;
    cursor: pointer;
  }
</style>
