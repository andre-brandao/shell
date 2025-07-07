<script lang="ts">
  import { launchApp, type NhSearchResult } from "./helpers";

  type Props = {
    channel: string;
    app: NhSearchResult;
  };

  let { app, channel }: Props = $props();
</script>

<div class="app-item">
  <div class="app-content">
    <div class="app-header">
      <span class="app-name">{app.package_pname}</span>
      <span class="app-version">{app.package_pversion}</span>
    </div>
    <div class="app-attr-name">{app.package_attr_name}</div>
    {#if app.package_description}
      <div class="app-description">{app.package_description}</div>
    {/if}
    <div class="app-meta">
      {#if app.package_homepage?.length > 0}
        <button
          class="app-homepage"
          onclick={() => open(app.package_homepage[0])}
        >
          🔗 Homepage
        </button>
      {/if}
      <button
        class="app-source"
        onclick={() => {
          let posInfo = app.package_position.split(":");
          let fileLoc = `${posInfo[0]}#L${posInfo[1]}`;
          open(`https://github.com/NixOS/nixpkgs/blob/${channel}/${fileLoc}`);
        }}
      >
        📦 Source
      </button>
      {#if app.package_license_set?.length > 0}
        <span class="app-license">{app.package_license_set[0]}</span>
      {/if}
    </div>
  </div>
  <button class="run-button" onclick={() => launchApp(app)}>
    <svg
      width="16"
      height="16"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path d="M8 5V19L19 12L8 5Z" fill="currentColor" />
    </svg>
    Run
  </button>
</div>

<style>
  .app-item {
    background: transparent;
    border: 1px solid #2c2c2e;
    color: #ffffff;
    padding: 16px;
    font-family: inherit;
    font-size: 14px;
    display: flex;
    justify-content: space-between;
    align-items: stretch;
    gap: 16px;
    width: 100%;
    max-width: 100%;
    border-radius: 8px;
    transition: all 0.2s ease;
    box-sizing: border-box;
    min-height: 130px;
  }

  .app-item:hover {
    background-color: #1c1c1e;
    border-color: #3a3a3c;
  }

  .app-content {
    display: flex;
    flex-direction: column;
    gap: 8px;
    flex: 1;
    min-width: 0;
    overflow: hidden;
    justify-content: flex-start;
  }

  .app-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
    min-width: 0;
    flex-shrink: 0;
  }

  .app-name {
    font-weight: 600;
    color: #ffffff;
    font-size: 16px;
    min-width: 0;
    flex: 1;
    word-break: break-word;
    line-height: 1.2;
  }

  .app-version {
    font-size: 12px;
    color: #8e8e93;
    font-weight: 400;
    background: #2c2c2e;
    padding: 2px 6px;
    border-radius: 4px;
    flex-shrink: 0;
    white-space: nowrap;
    align-self: flex-start;
  }

  .app-attr-name {
    font-size: 12px;
    color: #64d2ff;
    font-weight: 500;
    font-family: "SF Mono", "Monaco", "Inconsolata", "Fira Code", monospace;
    word-break: break-all;
    line-height: 1.3;
    flex-shrink: 0;
  }

  .app-description {
    font-size: 13px;
    color: #a1a1a6;
    font-weight: 400;
    line-height: 1.4;
    overflow: hidden;
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    max-height: calc(1.4em * 2);
    flex-grow: 1;
    min-height: calc(1.4em * 2);
  }

  .app-meta {
    display: flex;
    gap: 12px;
    align-items: center;
    font-size: 11px;
    color: #8e8e93;
    overflow: hidden;
    flex-wrap: wrap;
    flex-shrink: 0;
    margin-top: auto;
    padding-top: 4px;
  }

  .app-homepage {
    background: none;
    border: none;
    color: #64d2ff;
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    padding: 0;
    font-size: inherit;
    font-family: inherit;
    text-decoration: underline;
    transition: color 0.2s ease;
  }

  .app-homepage:hover {
    color: #5ac8fa;
  }

  .app-source {
    background: none;
    border: none;
    color: #64d2ff;
    font-weight: 500;
    white-space: nowrap;
    cursor: pointer;
    padding: 0;
    font-size: inherit;
    font-family: inherit;
    text-decoration: underline;
    transition: color 0.2s ease;
  }

  .app-source:hover {
    color: #5ac8fa;
  }

  .app-license {
    color: #8e8e93;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .run-button {
    background: #007aff;
    border: none;
    color: white;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 12px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 4px;
    flex-shrink: 0;
    white-space: nowrap;
    align-self: flex-start;
    height: 32px;
    min-width: 64px;
    justify-content: center;
  }

  .run-button:hover {
    background: #0056b3;
  }

  .run-button:active {
    background: #004494;
    transform: scale(0.98);
  }
</style>
