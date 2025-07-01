<script>
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";

  let { data } = $props();

  onMount(() => {
    data.win?.show().then(() => {
      data.win?.setFocus();
      goto("/app");
    });
  });
</script>

<main>
  <div class="splash-container">
    <div class="logo-section">
      <div class="logo-wrapper">
        <img src="/favicon.png" alt="LOGO" class="logo" />
      </div>
      <h1 class="app-title">Deds Shell</h1>
      <p class="app-subtitle">System Application Launcher</p>
    </div>

    <div class="action-section">
      <a href="/app" class="launch-button">
        <span class="button-text">Launch Application</span>
        <span class="button-shortcut">⏎</span>
      </a>

      <div class="loading-indicator">
        <div class="loading-dots">
          <span></span>
          <span></span>
          <span></span>
        </div>
        <span class="loading-text">Initializing...</span>
      </div>
    </div>
  </div>
</main>

<style>
  main {
    min-height: 100vh;
    background: linear-gradient(135deg, #1c1c1e 0%, #2c2c2e 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    font-family:
      -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .splash-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 40px;
    max-width: 400px;
    width: 100%;
    text-align: center;
  }

  .logo-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 16px;
  }

  .logo-wrapper {
    width: 80px;
    height: 80px;
    background: linear-gradient(135deg, #3a3a3c 0%, #48484a 100%);
    border-radius: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
    box-shadow:
      0 8px 32px rgba(0, 0, 0, 0.3),
      0 2px 8px rgba(0, 0, 0, 0.2),
      inset 0 1px 0 rgba(255, 255, 255, 0.1);
    position: relative;
    overflow: hidden;
  }

  .logo-wrapper::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.1) 0%,
      transparent 50%
    );
    border-radius: 20px;
  }

  .logo {
    width: 48px;
    height: 48px;
    object-fit: contain;
    z-index: 1;
  }

  .app-title {
    margin: 0;
    font-size: 28px;
    font-weight: 600;
    color: #ffffff;
    letter-spacing: -0.5px;
  }

  .app-subtitle {
    margin: 0;
    font-size: 14px;
    font-weight: 500;
    color: #8e8e93;
    letter-spacing: 0.2px;
  }

  .action-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 24px;
    width: 100%;
  }

  .launch-button {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 12px 24px;
    background: linear-gradient(135deg, #007aff 0%, #0056cc 100%);
    color: #ffffff;
    text-decoration: none;
    border-radius: 12px;
    font-size: 16px;
    font-weight: 600;
    min-width: 200px;
    box-shadow:
      0 4px 16px rgba(0, 122, 255, 0.3),
      0 2px 4px rgba(0, 0, 0, 0.1);
    transition: all 0.3s ease;
    position: relative;
    overflow: hidden;
  }

  .launch-button::before {
    content: "";
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(
      135deg,
      rgba(255, 255, 255, 0.2) 0%,
      transparent 50%
    );
    opacity: 0;
    transition: opacity 0.3s ease;
  }

  .launch-button:hover {
    transform: translateY(-2px);
    box-shadow:
      0 8px 24px rgba(0, 122, 255, 0.4),
      0 4px 8px rgba(0, 0, 0, 0.2);
  }

  .launch-button:hover::before {
    opacity: 1;
  }

  .launch-button:active {
    transform: translateY(0);
    box-shadow:
      0 2px 8px rgba(0, 122, 255, 0.3),
      0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .button-text {
    flex: 1;
    text-align: left;
  }

  .button-shortcut {
    font-size: 14px;
    opacity: 0.8;
    background: rgba(255, 255, 255, 0.2);
    padding: 2px 6px;
    border-radius: 4px;
    font-weight: 500;
  }

  .loading-indicator {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
  }

  .loading-dots {
    display: flex;
    gap: 6px;
  }

  .loading-dots span {
    width: 6px;
    height: 6px;
    background-color: #8e8e93;
    border-radius: 50%;
    animation: loading-pulse 1.5s ease-in-out infinite;
  }

  .loading-dots span:nth-child(2) {
    animation-delay: 0.2s;
  }

  .loading-dots span:nth-child(3) {
    animation-delay: 0.4s;
  }

  .loading-text {
    font-size: 12px;
    color: #8e8e93;
    font-weight: 500;
  }

  @keyframes loading-pulse {
    0%,
    80%,
    100% {
      opacity: 0.3;
      transform: scale(1);
    }
    40% {
      opacity: 1;
      transform: scale(1.2);
    }
  }

  /* Responsive adjustments */
  @media (max-width: 480px) {
    .splash-container {
      gap: 32px;
    }

    .app-title {
      font-size: 24px;
    }

    .launch-button {
      min-width: 180px;
      font-size: 15px;
    }
  }
</style>
