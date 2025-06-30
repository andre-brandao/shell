<script>
  import Icon from "$lib/widgets/Icon.svelte";
  import { onMount } from "svelte";

  let searchQuery = $state("");
  let searchInput;

  // Sample apps data
  let apps = $state([
    { id: 1, name: "Messages", icon: "💬", category: "Communication" },
    { id: 2, name: "Calendar", icon: "📅", category: "Productivity" },
    { id: 3, name: "Photos", icon: "📸", category: "Media" },
    { id: 4, name: "Music", icon: "🎵", category: "Media" },
    { id: 5, name: "Notes", icon: "📝", category: "Productivity" },
    { id: 6, name: "Calculator", icon: "🧮", category: "Utilities" },
    { id: 7, name: "Weather", icon: "🌤️", category: "Utilities" },
    { id: 8, name: "Maps", icon: "🗺️", category: "Navigation" },
    { id: 9, name: "Safari", icon: "🌐", category: "Internet" },
    { id: 10, name: "Mail", icon: "✉️", category: "Communication" },
    { id: 11, name: "Settings", icon: "⚙️", category: "System" },
    { id: 12, name: "Camera", icon: "📷", category: "Media" },
    { id: 13, name: "Files", icon: "📁", category: "System" },
    { id: 14, name: "Terminal", icon: "💻", category: "Development" },
    { id: 15, name: "Code Editor", icon: "⌨️", category: "Development" },
    { id: 16, name: "Spotify", icon: "🎧", category: "Media" },
  ]);

  // Computed property for filtered apps
  let filteredApps = $derived(
    apps.filter(
      (app) =>
        app.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        app.category.toLowerCase().includes(searchQuery.toLowerCase()),
    ),
  );

  function launchApp(app) {
    console.log(`Launching ${app.name}`);
    // In a real app, this would launch the application
    // You can add actual app launching logic here
  }

  function clearSearch() {
    searchQuery = "";
    searchInput?.focus();
  }

  function handleKeydown(event) {
    if (event.key === "Escape") {
      searchQuery = "";
    }
  }

  onMount(() => {
    searchInput?.focus();
    document.addEventListener("keydown", handleKeydown);
    return () => {
      document.removeEventListener("keydown", handleKeydown);
    };
  });
</script>

<div class="launcher">
  <!-- Header with Search -->
  <div class="launcher-header">
    <div class="search-container">
      <input
        bind:this={searchInput}
        bind:value={searchQuery}
        type="text"
        placeholder="Search applications..."
        class="search-input"
      />
      <span class="search-icon">🔍</span>
      <Icon />
      {#if searchQuery}
        <button class="clear-button" onclick={clearSearch}>✕</button>
      {/if}
    </div>
    <button class="action-button">
      <span class="action-icon">⚙️</span>
    </button>
  </div>

  <!-- Apps Grid Container -->
  <div class="launcher-content">
    {#if filteredApps.length > 0}
      <div class="apps-grid">
        {#each filteredApps as app (app.id)}
          <button class="app-item" onclick={() => launchApp(app)}>
            <div class="app-icon">{app.icon}</div>
            <div class="app-name">{app.name}</div>
            <div class="app-category">{app.category}</div>
          </button>
        {/each}
      </div>
    {:else}
      <div class="no-results">
        <div class="no-results-icon">🔍</div>
        <div class="no-results-text">No applications found</div>
        <div class="no-results-subtitle">Try adjusting your search terms</div>
      </div>
    {/if}
  </div>

  <!-- Footer -->
  <div class="launcher-footer">
    <div class="app-count">
      {filteredApps.length} of {apps.length} applications
    </div>
  </div>
</div>

<style>
  .launcher {
    width: 100%;
    height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    flex-direction: column;
    font-family:
      -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }

  .launcher-header {
    display: flex;
    align-items: center;
    gap: 15px;
    padding: 20px 25px;
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(20px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.2);
  }

  .search-container {
    position: relative;
    flex: 1;
  }

  .search-input {
    width: 100%;
    padding: 16px 50px 16px 20px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 15px;
    font-size: 16px;
    background: rgba(255, 255, 255, 0.9);
    outline: none;
    transition: all 0.3s ease;
    box-sizing: border-box;
  }

  .search-input:focus {
    border-color: rgba(255, 255, 255, 0.8);
    background: white;
    box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
  }

  .search-input::placeholder {
    color: #999;
  }

  .search-icon {
    position: absolute;
    right: 20px;
    top: 50%;
    transform: translateY(-50%);
    font-size: 18px;
    color: #666;
    pointer-events: none;
  }

  .clear-button {
    position: absolute;
    right: 50px;
    top: 50%;
    transform: translateY(-50%);
    width: 24px;
    height: 24px;
    border: none;
    background: #ff6b6b;
    color: white;
    border-radius: 50%;
    cursor: pointer;
    font-size: 12px;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
  }

  .clear-button:hover {
    background: #ff5252;
    transform: translateY(-50%) scale(1.1);
  }

  .action-button {
    width: 50px;
    height: 50px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-radius: 15px;
    background: rgba(255, 255, 255, 0.2);
    backdrop-filter: blur(20px);
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .action-button:hover {
    background: rgba(255, 255, 255, 0.3);
    border-color: rgba(255, 255, 255, 0.5);
    transform: translateY(-2px);
  }

  .action-icon {
    font-size: 20px;
  }

  .launcher-content {
    flex: 1;
    padding: 30px 25px;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .apps-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(140px, 1fr));
    gap: 20px;
    width: 100%;
    max-width: 1200px;
    max-height: 100%;
    overflow-y: auto;
    padding: 10px;
  }

  .app-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 25px 15px;
    background: rgba(255, 255, 255, 0.15);
    backdrop-filter: blur(20px);
    border: 2px solid rgba(255, 255, 255, 0.2);
    border-radius: 20px;
    cursor: pointer;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    min-height: 140px;
    justify-content: center;
    color: white;
  }

  .app-item:hover {
    background: rgba(255, 255, 255, 0.25);
    border-color: rgba(255, 255, 255, 0.4);
    transform: translateY(-5px);
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.2);
  }

  .app-item:active {
    transform: translateY(-2px);
  }

  .app-icon {
    font-size: 42px;
    margin-bottom: 12px;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
  }

  .app-name {
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 6px;
    text-align: center;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .app-category {
    font-size: 13px;
    opacity: 0.8;
    text-align: center;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .no-results {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    color: rgba(255, 255, 255, 0.8);
    text-align: center;
  }

  .no-results-icon {
    font-size: 64px;
    margin-bottom: 20px;
    opacity: 0.6;
  }

  .no-results-text {
    font-size: 24px;
    font-weight: 600;
    margin-bottom: 8px;
    text-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  }

  .no-results-subtitle {
    font-size: 16px;
    opacity: 0.7;
  }

  .launcher-footer {
    padding: 15px 25px;
    background: rgba(0, 0, 0, 0.1);
    backdrop-filter: blur(20px);
    border-top: 1px solid rgba(255, 255, 255, 0.1);
    text-align: center;
  }

  .app-count {
    color: rgba(255, 255, 255, 0.8);
    font-size: 14px;
    font-weight: 500;
  }

  /* Custom Scrollbar */
  .apps-grid::-webkit-scrollbar {
    width: 8px;
  }

  .apps-grid::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.1);
    border-radius: 4px;
  }

  .apps-grid::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.3);
    border-radius: 4px;
  }

  .apps-grid::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.4);
  }

  /* Responsive Design */
  @media (max-width: 768px) {
    .launcher-header {
      padding: 15px 20px;
      gap: 10px;
    }

    .search-input {
      padding: 14px 45px 14px 16px;
      font-size: 16px;
    }

    .action-button {
      width: 45px;
      height: 45px;
    }

    .launcher-content {
      padding: 20px 15px;
    }

    .apps-grid {
      grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
      gap: 15px;
    }

    .app-item {
      padding: 20px 12px;
      min-height: 120px;
    }

    .app-icon {
      font-size: 36px;
      margin-bottom: 10px;
    }

    .app-name {
      font-size: 14px;
    }

    .app-category {
      font-size: 12px;
    }

    .no-results-icon {
      font-size: 48px;
    }

    .no-results-text {
      font-size: 20px;
    }

    .no-results-subtitle {
      font-size: 14px;
    }
  }

  @media (max-width: 480px) {
    .apps-grid {
      grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
      gap: 12px;
    }

    .app-item {
      padding: 16px 10px;
      min-height: 100px;
    }

    .app-icon {
      font-size: 32px;
    }

    .app-name {
      font-size: 13px;
    }

    .app-category {
      font-size: 11px;
    }
  }
</style>
