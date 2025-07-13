<script lang="ts">
  import Sidebar from './lib/components/Sidebar.svelte'
  import MainContent from './lib/components/MainContent.svelte'
  import StatusBar from './lib/components/StatusBar.svelte'
  
  let activeView = 'notebooks'
  let sidebarCollapsed = false
</script>

<div class="flex h-screen bg-dark-900 text-dark-100">
  <!-- Sidebar -->
  <div class="flex-shrink-0 {sidebarCollapsed ? 'w-16' : 'w-64'} transition-all duration-300">
    <Sidebar bind:activeView bind:collapsed={sidebarCollapsed} />
  </div>
  
  <!-- Main Content Area -->
  <div class="flex-1 flex flex-col min-w-0">
    <MainContent {activeView} />
    <StatusBar />
  </div>
</div>