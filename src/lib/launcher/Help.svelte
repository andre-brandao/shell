<main>
  <div class="splash-container">
    <div class="logo-section">
      <div class="logo-wrapper">
        <img src="/favicon.png" alt="LOGO" class="logo" />
      </div>
      <h1 class="app-title">Help Menu</h1>
      <p class="app-subtitle">System Application Launcher</p>
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
