<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  import { dndzone } from 'svelte-dnd-action';
  import MonacoEditor from '../editor/MonacoEditor.svelte';
  import CellOutput from './CellOutput.svelte';
  import MarkdownRenderer from './MarkdownRenderer.svelte';
  import type { Cell, Notebook } from '$lib/stores/notebooks';
  import { cellExecutions } from '$lib/stores/cells';
  
  export let notebook: Notebook;
  export let activeCell: string | null = null;
  export let readOnly = false;
  
  const dispatch = createEventDispatcher<{
    cellChange: { cellId: string; content: string };
    cellExecute: { cellId: string };
    cellAdd: { type: 'code' | 'markdown'; afterCellId?: string };
    cellDelete: { cellId: string };
    cellMove: { cellId: string; direction: 'up' | 'down' };
    cellTypeChange: { cellId: string; type: 'code' | 'markdown' };
    notebookSave: void;
  }>();
  
  let cells = notebook.cells;
  let dragDisabled = true;
  
  // Handle drag and drop reordering
  function handleDndConsider(e: CustomEvent) {
    cells = e.detail.items;
    dragDisabled = false;
  }
  
  function handleDndFinalize(e: CustomEvent) {
    cells = e.detail.items;
    dragDisabled = true;
    // Update the notebook with new cell order
    notebook.cells = cells;
  }
  
  // Cell management functions
  function addCell(type: 'code' | 'markdown', afterCellId?: string) {
    dispatch('cellAdd', { type, afterCellId });
  }
  
  function deleteCell(cellId: string) {
    if (cells.length <= 1) return; // Don't delete the last cell
    dispatch('cellDelete', { cellId });
  }
  
  function executeCell(cellId: string) {
    dispatch('cellExecute', { cellId });
  }
  
  function changeCellType(cellId: string, type: 'code' | 'markdown') {
    dispatch('cellTypeChange', { cellId, type });
  }
  
  function handleCellChange(cellId: string, content: string) {
    dispatch('cellChange', { cellId, content });
  }
  
  function moveCell(cellId: string, direction: 'up' | 'down') {
    dispatch('cellMove', { cellId, direction });
  }
  
  function focusCell(cellId: string) {
    activeCell = cellId;
  }
  
  // Keyboard shortcuts
  function handleKeyDown(event: KeyboardEvent) {
    if (!activeCell) return;
    
    const currentIndex = cells.findIndex(cell => cell.id === activeCell);
    
    if (event.key === 'Escape') {
      activeCell = null;
      return;
    }
    
    // Only handle shortcuts when not in an input
    if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
      return;
    }
    
    if (event.ctrlKey || event.metaKey) {
      switch (event.key) {
        case 's':
          event.preventDefault();
          dispatch('notebookSave');
          break;
        case 'Enter':
          event.preventDefault();
          executeCell(activeCell);
          break;
      }
    } else if (event.shiftKey) {
      switch (event.key) {
        case 'Enter':
          event.preventDefault();
          executeCell(activeCell);
          break;
      }
    } else {
      switch (event.key) {
        case 'ArrowUp':
          if (currentIndex > 0) {
            event.preventDefault();
            activeCell = cells[currentIndex - 1].id;
          }
          break;
        case 'ArrowDown':
          if (currentIndex < cells.length - 1) {
            event.preventDefault();
            activeCell = cells[currentIndex + 1].id;
          }
          break;
        case 'a':
          event.preventDefault();
          addCell('code', activeCell);
          break;
        case 'b':
          event.preventDefault();
          addCell('code', activeCell);
          break;
        case 'm':
          event.preventDefault();
          changeCellType(activeCell, 'markdown');
          break;
        case 'y':
          event.preventDefault();
          changeCellType(activeCell, 'code');
          break;
        case 'd':
          if (event.repeat) {
            event.preventDefault();
            deleteCell(activeCell);
          }
          break;
      }
    }
  }
  
  onMount(() => {
    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  });
  
  // Update cells when notebook changes
  $: cells = notebook.cells;
</script>

<div class="notebook-renderer h-full flex flex-col">
  <!-- Notebook Header -->
  <div class="notebook-header flex items-center justify-between p-4 border-b border-surface-700 bg-surface-900">
    <div class="flex items-center space-x-3">
      <div class="flex items-center space-x-2">
        <svg class="w-6 h-6 text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        <h1 class="text-xl font-semibold text-surface-100">{notebook.name}</h1>
      </div>
      <div class="flex items-center space-x-2 text-sm text-surface-400">
        <span>{cells.length} cells</span>
        <span>•</span>
        <span>Last saved: {notebook.updatedAt.toLocaleTimeString()}</span>
      </div>
    </div>
    
    <div class="flex items-center space-x-2">
      <button
        class="btn-secondary"
        on:click={() => dispatch('notebookSave')}
        title="Save notebook (Ctrl+S)"
      >
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4" />
        </svg>
        Save
      </button>
      
      <div class="flex items-center space-x-1 bg-surface-800 rounded-lg p-1">
        <button
          class="px-3 py-1 text-sm rounded hover:bg-surface-700 transition-colors"
          on:click={() => addCell('code')}
          title="Add code cell (A)"
        >
          + Code
        </button>
        <button
          class="px-3 py-1 text-sm rounded hover:bg-surface-700 transition-colors"
          on:click={() => addCell('markdown')}
          title="Add markdown cell (M)"
        >
          + Markdown
        </button>
      </div>
    </div>
  </div>
  
  <!-- Cells Container -->
  <div 
    class="cells-container flex-1 overflow-auto p-4"
    use:dndzone={{ items: cells, dragDisabled, dropTargetStyle: {} }}
    on:consider={handleDndConsider}
    on:finalize={handleDndFinalize}
  >
    {#each cells as cell (cell.id)}
      <div 
        class="cell-wrapper mb-4"
        animate:flip={{ duration: 300 }}
        transition:slide={{ duration: 200 }}
      >
        <div 
          class="cell border border-surface-700 rounded-lg overflow-hidden transition-all duration-200 {activeCell === cell.id ? 'ring-2 ring-primary-500 border-primary-500' : 'hover:border-surface-600'}"
          on:click={() => focusCell(cell.id)}
          role="button"
          tabindex="0"
        >
          <!-- Cell Header -->
          <div class="cell-header flex items-center justify-between px-4 py-2 bg-surface-800 border-b border-surface-700">
            <div class="flex items-center space-x-3">
              <div class="flex items-center space-x-2">
                <div class="w-2 h-2 rounded-full {cell.type === 'code' ? 'bg-blue-400' : 'bg-green-400'}"></div>
                <span class="text-sm font-medium text-surface-300 capitalize">{cell.type}</span>
              </div>
              
              {#if cell.isExecuting}
                <div class="flex items-center space-x-2 text-yellow-400">
                  <div class="animate-spin rounded-full h-3 w-3 border-b-2 border-yellow-400"></div>
                  <span class="text-xs">Executing...</span>
                </div>
              {/if}
            </div>
            
            <div class="flex items-center space-x-1">
              <!-- Cell Type Toggle -->
              <button
                class="p-1 hover:bg-surface-700 rounded transition-colors"
                on:click|stopPropagation={() => changeCellType(cell.id, cell.type === 'code' ? 'markdown' : 'code')}
                title="Toggle cell type (Y/M)"
              >
                <svg class="w-4 h-4 text-surface-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
                </svg>
              </button>
              
              <!-- Execute Cell -->
              {#if cell.type === 'code'}
                <button
                  class="p-1 hover:bg-surface-700 rounded transition-colors"
                  on:click|stopPropagation={() => executeCell(cell.id)}
                  disabled={cell.isExecuting}
                  title="Execute cell (Shift+Enter)"
                >
                  <svg class="w-4 h-4 text-surface-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m2-7a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                </button>
              {/if}
              
              <!-- Move Cell Up -->
              <button
                class="p-1 hover:bg-surface-700 rounded transition-colors"
                on:click|stopPropagation={() => moveCell(cell.id, 'up')}
                disabled={cells.findIndex(c => c.id === cell.id) === 0}
                title="Move cell up"
              >
                <svg class="w-4 h-4 text-surface-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 15l7-7 7 7" />
                </svg>
              </button>
              
              <!-- Move Cell Down -->
              <button
                class="p-1 hover:bg-surface-700 rounded transition-colors"
                on:click|stopPropagation={() => moveCell(cell.id, 'down')}
                disabled={cells.findIndex(c => c.id === cell.id) === cells.length - 1}
                title="Move cell down"
              >
                <svg class="w-4 h-4 text-surface-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
                </svg>
              </button>
              
              <!-- Delete Cell -->
              <button
                class="p-1 hover:bg-red-600 rounded transition-colors"
                on:click|stopPropagation={() => deleteCell(cell.id)}
                disabled={cells.length <= 1}
                title="Delete cell (DD)"
              >
                <svg class="w-4 h-4 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
          
          <!-- Cell Content -->
          <div class="cell-content">
            {#if cell.type === 'code'}
              <MonacoEditor
                bind:value={cell.content}
                language="python"
                height="auto"
                readOnly={readOnly || cell.isExecuting}
                on:change={(e) => handleCellChange(cell.id, e.detail.value)}
                on:save={() => dispatch('notebookSave')}
              />
            {:else}
              <div class="markdown-cell">
                {#if activeCell === cell.id}
                  <MonacoEditor
                    bind:value={cell.content}
                    language="markdown"
                    height="auto"
                    readOnly={readOnly}
                    on:change={(e) => handleCellChange(cell.id, e.detail.value)}
                    on:blur={() => activeCell = null}
                  />
                {:else}
                  <MarkdownRenderer content={cell.content} />
                {/if}
              </div>
            {/if}
          </div>
          
          <!-- Cell Output -->
          {#if cell.type === 'code'}
            <CellOutput cellId={cell.id} />
          {/if}
        </div>
      </div>
    {/each}
    
    <!-- Add Cell Button -->
    <div class="add-cell-container flex justify-center mt-4">
      <div class="flex items-center space-x-2">
        <button
          class="flex items-center space-x-2 px-4 py-2 bg-surface-800 hover:bg-surface-700 border border-surface-600 rounded-lg transition-colors"
          on:click={() => addCell('code')}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span class="text-sm">Add Code Cell</span>
        </button>
        
        <button
          class="flex items-center space-x-2 px-4 py-2 bg-surface-800 hover:bg-surface-700 border border-surface-600 rounded-lg transition-colors"
          on:click={() => addCell('markdown')}
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
          </svg>
          <span class="text-sm">Add Markdown Cell</span>
        </button>
      </div>
    </div>
  </div>
</div>

<!-- Keyboard Shortcuts Help -->
{#if activeCell}
  <div class="fixed bottom-4 right-4 bg-surface-800 border border-surface-600 rounded-lg p-3 text-xs text-surface-300 max-w-xs" transition:fade>
    <div class="font-medium mb-2">Keyboard Shortcuts</div>
    <div class="space-y-1">
      <div><kbd class="bg-surface-700 px-1 rounded">Shift+Enter</kbd> Execute cell</div>
      <div><kbd class="bg-surface-700 px-1 rounded">A</kbd> Add cell above</div>
      <div><kbd class="bg-surface-700 px-1 rounded">B</kbd> Add cell below</div>
      <div><kbd class="bg-surface-700 px-1 rounded">M</kbd> Change to markdown</div>
      <div><kbd class="bg-surface-700 px-1 rounded">Y</kbd> Change to code</div>
      <div><kbd class="bg-surface-700 px-1 rounded">DD</kbd> Delete cell</div>
      <div><kbd class="bg-surface-700 px-1 rounded">Ctrl+S</kbd> Save notebook</div>
    </div>
  </div>
{/if}

<style>
  .cell-wrapper {
    position: relative;
  }
  
  .cell {
    background: theme('colors.surface.900');
  }
  
  .cell:hover {
    box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  }
  
  .cell.active {
    box-shadow: 0 0 0 2px theme('colors.primary.500');
  }
  
  kbd {
    font-family: inherit;
    font-size: 0.75rem;
    padding: 0.125rem 0.25rem;
    border-radius: 0.25rem;
  }
</style>