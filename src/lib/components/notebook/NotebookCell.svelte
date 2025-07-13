<script lang="ts">
  import { createEventDispatcher } from 'svelte'
  import { Play, Trash2, Type, Code } from 'lucide-svelte'
  import CodeEditor from '../editor/CodeEditor.svelte'
  
  export let cell
  
  const dispatch = createEventDispatcher()
  
  function executeCell() {
    dispatch('execute')
  }
  
  function deleteCell() {
    dispatch('delete')
  }
  
  function toggleCellType() {
    cell.type = cell.type === 'code' ? 'markdown' : 'code'
  }
</script>

<div class="panel p-0 overflow-hidden">
  <!-- Cell Header -->
  <div class="flex items-center justify-between p-2 bg-dark-700 border-b border-dark-600">
    <div class="flex items-center space-x-2">
      <button 
        class="p-1 hover:bg-dark-600 rounded text-xs"
        on:click={toggleCellType}
      >
        {#if cell.type === 'code'}
          <Code size={14} class="text-primary-400" />
        {:else}
          <Type size={14} class="text-accent-400" />
        {/if}
      </button>
      <span class="text-xs text-dark-400 capitalize">{cell.type}</span>
    </div>
    
    <div class="flex items-center space-x-1">
      {#if cell.type === 'code'}
        <button 
          class="p-1 hover:bg-dark-600 rounded"
          on:click={executeCell}
        >
          <Play size={14} class="text-secondary-400" />
        </button>
      {/if}
      <button 
        class="p-1 hover:bg-red-600 rounded"
        on:click={deleteCell}
      >
        <Trash2 size={14} class="text-red-400" />
      </button>
    </div>
  </div>
  
  <!-- Cell Content -->
  <div class="relative">
    {#if cell.type === 'code'}
      <CodeEditor 
        bind:value={cell.content}
        language="python"
        height="150px"
      />
    {:else}
      <textarea 
        bind:value={cell.content}
        class="w-full p-4 bg-dark-800 text-dark-100 border-none resize-none focus:outline-none"
        placeholder="Enter markdown content..."
        rows="6"
      ></textarea>
    {/if}
  </div>
  
  <!-- Cell Output -->
  {#if cell.output && cell.type === 'code'}
    <div class="border-t border-dark-600 p-4 bg-dark-800">
      <div class="text-xs text-dark-400 mb-2">Output:</div>
      <pre class="text-sm text-secondary-400 whitespace-pre-wrap">{cell.output}</pre>
    </div>
  {/if}
</div>