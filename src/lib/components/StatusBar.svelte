<script lang="ts">
  import { onMount } from 'svelte'
  
  let kernelStatus = 'idle'
  let memoryUsage = '45%'
  let cpuUsage = '12%'
  
  onMount(() => {
    // Simulate status updates
    const interval = setInterval(() => {
      memoryUsage = Math.floor(Math.random() * 60 + 20) + '%'
      cpuUsage = Math.floor(Math.random() * 30 + 5) + '%'
    }, 3000)
    
    return () => clearInterval(interval)
  })
</script>

<div class="h-6 bg-dark-800 border-t border-dark-700 flex items-center justify-between px-4 text-xs text-dark-400">
  <div class="flex items-center space-x-4">
    <span class="flex items-center">
      <div class="w-2 h-2 rounded-full bg-secondary-500 mr-2"></div>
      Kernel: {kernelStatus}
    </span>
    <span>Python 3.9.7</span>
  </div>
  
  <div class="flex items-center space-x-4">
    <span>Memory: {memoryUsage}</span>
    <span>CPU: {cpuUsage}</span>
    <span>Ready</span>
  </div>
</div>