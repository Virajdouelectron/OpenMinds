<script lang="ts">
  import { Settings, Palette, Code, Server } from 'lucide-svelte'
  
  let theme = 'dark'
  let fontSize = 14
  let autoSave = true
  let showLineNumbers = true
  let kernelPath = '/usr/local/bin/python'
  let maxMemory = 8192
</script>

<div class="flex-1 overflow-auto p-6">
  <div class="max-w-2xl">
    <h2 class="text-2xl font-semibold mb-6">Settings</h2>
    
    <!-- Appearance -->
    <div class="panel p-6 mb-6">
      <h3 class="text-lg font-medium mb-4 flex items-center">
        <Palette size={20} class="mr-2" />
        Appearance
      </h3>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Theme</label>
          <select bind:value={theme} class="input-field">
            <option value="dark">Dark</option>
            <option value="light">Light</option>
            <option value="auto">Auto</option>
          </select>
        </div>
        
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Font Size</label>
          <input type="range" min="10" max="20" bind:value={fontSize} class="w-full" />
          <div class="text-sm text-dark-400">{fontSize}px</div>
        </div>
      </div>
    </div>
    
    <!-- Editor -->
    <div class="panel p-6 mb-6">
      <h3 class="text-lg font-medium mb-4 flex items-center">
        <Code size={20} class="mr-2" />
        Editor
      </h3>
      
      <div class="space-y-4">
        <div class="flex items-center">
          <input type="checkbox" bind:checked={autoSave} class="mr-2" />
          <label class="text-sm">Auto-save</label>
        </div>
        
        <div class="flex items-center">
          <input type="checkbox" bind:checked={showLineNumbers} class="mr-2" />
          <label class="text-sm">Show line numbers</label>
        </div>
      </div>
    </div>
    
    <!-- Runtime -->
    <div class="panel p-6">
      <h3 class="text-lg font-medium mb-4 flex items-center">
        <Server size={20} class="mr-2" />
        Runtime
      </h3>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Python Kernel Path</label>
          <input type="text" bind:value={kernelPath} class="input-field w-full" />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Max Memory (MB)</label>
          <input type="number" bind:value={maxMemory} class="input-field" />
        </div>
      </div>
    </div>
  </div>
</div>