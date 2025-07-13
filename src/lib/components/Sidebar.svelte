<script lang="ts">
  import { FileText, Database, Brain, Settings, ChevronLeft, ChevronRight, Zap, BarChart3, Terminal, GitBranch, Bot, Blocks, Shield, Server, Rocket } from 'lucide-svelte'
  
  export let activeView: string
  export let collapsed: boolean
  
  const menuItems = [
    { id: 'notebooks', label: 'Notebooks', icon: FileText, color: 'text-primary-400' },
    { id: 'datasets', label: 'Datasets', icon: Database, color: 'text-secondary-400' },
    { id: 'models', label: 'Models', icon: Brain, color: 'text-accent-400' },
    { id: 'automl', label: 'AutoML', icon: Zap, color: 'text-yellow-400' },
    { id: 'experiments', label: 'Experiments', icon: BarChart3, color: 'text-purple-400' },
    { id: 'ai-copilot', label: 'AI Copilot', icon: Bot, color: 'text-blue-400' },
    { id: 'visual-builder', label: 'Visual Builder', icon: Blocks, color: 'text-pink-400' },
    { id: 'data-profiler', label: 'Data Profiler', icon: Shield, color: 'text-cyan-400' },
    { id: 'runtime', label: 'Runtime', icon: Server, color: 'text-indigo-400' },
    { id: 'deployment', label: 'Deployment', icon: Rocket, color: 'text-red-400' },
    { id: 'terminal', label: 'Terminal', icon: Terminal, color: 'text-green-400' },
    { id: 'version-control', label: 'Version Control', icon: GitBranch, color: 'text-orange-400' },
    { id: 'settings', label: 'Settings', icon: Settings, color: 'text-dark-400' }
  ]
  
  function setActiveView(id: string) {
    activeView = id
  }
  
  function toggleSidebar() {
    collapsed = !collapsed
  }
</script>

<div class="h-full panel border-r border-dark-700 flex flex-col">
  <!-- Header -->
  <div class="p-4 border-b border-dark-700 flex items-center justify-between">
    {#if !collapsed}
      <h1 class="text-lg font-bold text-primary-400">OpenMinds</h1>
    {/if}
    <button 
      on:click={toggleSidebar}
      class="p-1 hover:bg-dark-700 rounded transition-colors"
    >
      {#if collapsed}
        <ChevronRight size={16} />
      {:else}
        <ChevronLeft size={16} />
      {/if}
    </button>
  </div>
  
  <!-- Navigation -->
  <nav class="flex-1 p-2">
    {#each menuItems as item}
      <button
        class="w-full flex items-center p-3 rounded-lg transition-colors mb-1 {activeView === item.id ? 'bg-dark-700 border-l-2 border-primary-500' : 'hover:bg-dark-700'}"
        on:click={() => setActiveView(item.id)}
      >
        <svelte:component this={item.icon} size={20} class={item.color} />
        {#if !collapsed}
          <span class="ml-3 text-sm font-medium">{item.label}</span>
        {/if}
      </button>
    {/each}
  </nav>
  
  <!-- Footer -->
  <div class="p-4 border-t border-dark-700">
    {#if !collapsed}
      <div class="text-xs text-dark-400">
        <p>Version 1.0.0</p>
        <p>Phase 1 MVP</p>
      </div>
    {/if}
  </div>
</div>