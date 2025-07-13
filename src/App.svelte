<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { page } from '$app/stores';
  import { fade } from 'svelte/transition';
  import { slide } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
  
  // Import components
  import Sidebar from '$lib/components/Sidebar.svelte';
  import StatusBar from '$lib/components/StatusBar.svelte';
  import Toast from '$lib/components/ui/Toast.svelte';
  import CommandPalette from '$lib/components/command-palette/CommandPalette.svelte';
  
  // Import stores
  import { activeNotebook, notebooks, activeCell, isSidebarCollapsed, toast } from '$lib/stores';
  import { getContext, setContext } from 'svelte';
  
  // App state
  let showCommandPalette = false;
  let isMobileMenuOpen = false;
  let isInitialized = false;
  
  // Theme handling
  let theme = 'dark';
  
  // Handle keyboard shortcuts
  function handleKeyDown(event: KeyboardEvent) {
    // Toggle command palette with Cmd/Ctrl + K
    if ((event.metaKey || event.ctrlKey) && event.key === 'k') {
      event.preventDefault();
      showCommandPalette = !showCommandPalette;
    }
    
    // Close command palette with Escape
    if (event.key === 'Escape' && showCommandPalette) {
      event.preventDefault();
      showCommandPalette = false;
    }
  }
  
  // Initialize app
  async function initialize() {
    if (isInitialized) return;
    
    // Load initial data
    try {
      // TODO: Load notebooks and other initial data
      // const response = await fetch('/api/notebooks');
      // const data = await response.json();
      // notebooks.set(data);
    } catch (error) {
      console.error('Failed to initialize app:', error);
      toast.error('Failed to load initial data. Please refresh the page.');
    } finally {
      isInitialized = true;
    }
  }
  
  // Set up event listeners
  onMount(() => {
    window.addEventListener('keydown', handleKeyDown);
    initialize();
    
    // Set up context for child components
    setContext('app', {
      theme,
      isMobile: window.innerWidth < 768,
      isCommandPaletteOpen: () => showCommandPalette
    });
  });
  
  // Clean up
  onDestroy(() => {
    window.removeEventListener('keydown', handleKeyDown);
  });
  
  // Toggle sidebar
  function toggleSidebar() {
    isSidebarCollapsed.update(collapsed => !collapsed);
  }
  
  // Handle command palette actions
  function handleCommandSelect(command: { id: string; data?: any }) {
    switch (command.id) {
      case 'new-notebook':
        // TODO: Create new notebook
        break;
      case 'open-settings':
        // TODO: Open settings
        break;
      // Add more commands as needed
    }
    showCommandPalette = false;
  }
</script>

<svelte:head>
  <title>OpenMind IDE</title>
  <meta name="description" content="An open-source ML notebook IDE" />
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@300;400;500;600;700&family=JetBrains+Mono:wght@400;500;600&display=swap" rel="stylesheet">
</svelte:head>

<div class="app-container {theme} bg-background text-foreground min-h-screen flex flex-col">
  <!-- Mobile header -->
  <header class="md:hidden bg-surface-900 border-b border-surface-800 p-4 flex items-center justify-between">
    <div class="flex items-center space-x-2">
      <button 
        on:click={() => isMobileMenuOpen = !isMobileMenuOpen}
        class="p-2 rounded-md text-surface-400 hover:text-surface-200 hover:bg-surface-800"
        aria-label="Toggle menu"
      >
        <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16m-7 6h7" />
        </svg>
      </button>
      <h1 class="text-xl font-semibold text-white">OpenMind</h1>
    </div>
    
    <button 
      on:click={() => showCommandPalette = true}
      class="flex items-center space-x-2 bg-surface-800 hover:bg-surface-700 text-surface-300 px-3 py-2 rounded-md text-sm font-medium transition-colors"
    >
      <span class="hidden sm:inline">Search or run a command...</span>
      <span class="text-xs bg-surface-700 px-2 py-1 rounded">⌘K</span>
    </button>
  </header>
  
  <div class="flex flex-1 overflow-hidden">
    <!-- Sidebar -->
    <aside 
      class="fixed md:static inset-y-0 left-0 z-40 w-64 bg-surface-900 border-r border-surface-800 transform transition-transform duration-300 ease-in-out md:translate-x-0 {isMobileMenuOpen ? 'translate-x-0' : '-translate-x-full'}"
      class:translate-x-0={isMobileMenuOpen}
    >
      <div class="h-full flex flex-col">
        <div class="p-4 border-b border-surface-800">
          <div class="flex items-center space-x-2">
            <div class="w-8 h-8 rounded-md bg-primary-600 flex items-center justify-center text-white font-bold">
              <span>OM</span>
            </div>
            <h2 class="text-lg font-semibold text-white">OpenMind</h2>
          </div>
        </div>
        
        <div class="flex-1 overflow-y-auto">
          <Sidebar />
        </div>
        
        <div class="p-4 border-t border-surface-800">
          <!-- User menu or other sidebar footer content -->
        </div>
      </div>
    </aside>
    
    <!-- Main content -->
    <main class="flex-1 flex flex-col overflow-hidden bg-surface-950">
      <div class="flex-1 overflow-auto">
        <slot />
      </div>
      
      <StatusBar />
    </main>
  </div>
  
  {/* Command Palette */}
  {#if showCommandPalette}
    <div 
      class="fixed inset-0 bg-black/50 z-50 flex items-start justify-center p-4 pt-20"
      on:click|self={() => showCommandPalette = false}
      transition:fade={{ duration: 150 }}
    >
      <div 
        class="w-full max-w-2xl bg-surface-900 rounded-lg shadow-xl border border-surface-800 overflow-hidden"
        transition:slide={{ duration: 150, easing: quintOut }}
      >
        <CommandPalette on:select={handleCommandSelect} />
      </div>
    </div>
  {/if}
  
  {/* Toast Notifications */}
  <div class="fixed bottom-4 right-4 z-50 space-y-2">
    {#each $toast as item (item.id)}
      <Toast 
        id={item.id}
        type={item.type} 
        message={item.message}
        duration={item.duration}
      />
    {/each}
  </div>
  
  {/* Mobile overlay */}
  {#if isMobileMenuOpen}
    <div 
      class="md:hidden fixed inset-0 bg-black/50 z-30"
      on:click={() => isMobileMenuOpen = false}
      transition:fade={{ duration: 200 }}
    />
  {/if}
</div>

<style global>
  :root {
    --font-sans: 'Inter', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    --font-mono: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', 'Menlo', 'Monaco', 'Consolas', monospace;
  }
  
  body {
    @apply font-sans text-base text-foreground bg-background antialiased;
    font-feature-settings: 'rlig' 1, 'calt' 1;
  }
  
  /* Custom scrollbar */
  ::-webkit-scrollbar {
    @apply w-2 h-2;
  }
  
  ::-webkit-scrollbar-track {
    @apply bg-transparent;
  }
  
  ::-webkit-scrollbar-thumb {
    @apply bg-surface-600 rounded-full hover:bg-surface-500 transition-colors;
  }
  
  /* Monaco Editor overrides */
  .monaco-editor {
    --vscode-editor-background: theme('colors.surface.950');
    --vscode-editor-foreground: theme('colors.surface.100');
    --vscode-editor-lineHighlightBackground: theme('colors.surface.800');
    --vscode-editor-lineNumbers-foreground: theme('colors.surface.500');
    --vscode-editor-selectionBackground: theme('colors.primary.500/0.3');
  }
  
  /* Tiptap editor overrides */
  .ProseMirror {
    @apply min-h-[300px] p-4 focus:outline-none;
  }
  
  .ProseMirror p.is-editor-empty:first-child::before {
    @apply text-surface-500 float-left h-0 pointer-events-none;
    content: attr(data-placeholder);
  }
</style>