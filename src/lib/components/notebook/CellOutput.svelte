<script lang="ts">
  import { onMount } from 'svelte';
  import { fade, slide } from 'svelte/transition';
  import { cellExecutions } from '$lib/stores/cells';
  import type { CellExecutionResult } from '$lib/stores/cells';
  
  export let cellId: string;
  
  let outputs: CellExecutionResult[] = [];
  let isCollapsed = false;
  
  // Subscribe to cell execution results
  $: {
    cellExecutions.subscribe(executions => {
      outputs = executions[cellId] || [];
    });
  }
  
  function toggleCollapse() {
    isCollapsed = !isCollapsed;
  }
  
  function clearOutput() {
    cellExecutions.clearCellOutput(cellId);
  }
  
  function formatExecutionTime(time: number): string {
    if (time < 1000) {
      return `${time.toFixed(0)}ms`;
    } else {
      return `${(time / 1000).toFixed(2)}s`;
    }
  }
  
  function renderOutput(output: any) {
    if (!output) return '';
    
    switch (output.type) {
      case 'text':
        return output.value;
      case 'html':
        return output.value;
      case 'json':
        return JSON.stringify(output.value, null, 2);
      case 'table':
        return renderTable(output.data);
      case 'plot':
        return renderPlot(output);
      default:
        return JSON.stringify(output, null, 2);
    }
  }
  
  function renderTable(data: any[]): string {
    if (!Array.isArray(data) || data.length === 0) return '';
    
    const headers = Object.keys(data[0]);
    let html = '<table class="output-table"><thead><tr>';
    
    headers.forEach(header => {
      html += `<th>${header}</th>`;
    });
    
    html += '</tr></thead><tbody>';
    
    data.forEach(row => {
      html += '<tr>';
      headers.forEach(header => {
        html += `<td>${row[header] || ''}</td>`;
      });
      html += '</tr>';
    });
    
    html += '</tbody></table>';
    return html;
  }
  
  function renderPlot(plotData: any): string {
    // In a real implementation, this would render using Plotly.js or similar
    return `<div class="plot-placeholder">
      <div class="plot-info">
        <h4>${plotData.layout?.title || 'Plot'}</h4>
        <p>X: ${plotData.layout?.xaxis?.title || 'X Axis'}</p>
        <p>Y: ${plotData.layout?.yaxis?.title || 'Y Axis'}</p>
        <p>Data points: ${plotData.data?.x?.length || 0}</p>
      </div>
    </div>`;
  }
</script>

{#if outputs.length > 0}
  <div class="cell-output border-t border-surface-700" transition:slide={{ duration: 200 }}>
    <!-- Output Header -->
    <div class="output-header flex items-center justify-between px-4 py-2 bg-surface-800">
      <div class="flex items-center space-x-3">
        <button
          class="flex items-center space-x-2 text-sm text-surface-300 hover:text-surface-100 transition-colors"
          on:click={toggleCollapse}
        >
          <svg 
            class="w-4 h-4 transform transition-transform {isCollapsed ? '-rotate-90' : ''}" 
            fill="none" 
            stroke="currentColor" 
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
          </svg>
          <span>Output ({outputs.length})</span>
        </button>
        
        {#if outputs.length > 0}
          <div class="text-xs text-surface-400">
            Last execution: {formatExecutionTime(outputs[outputs.length - 1].executionTime)}
          </div>
        {/if}
      </div>
      
      <button
        class="p-1 hover:bg-surface-700 rounded transition-colors"
        on:click={clearOutput}
        title="Clear output"
      >
        <svg class="w-4 h-4 text-surface-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
        </svg>
      </button>
    </div>
    
    <!-- Output Content -->
    {#if !isCollapsed}
      <div class="output-content" transition:slide={{ duration: 200 }}>
        {#each outputs as result, index}
          <div class="output-item border-b border-surface-700 last:border-b-0">
            {#if result.error}
              <!-- Error Output -->
              <div class="error-output p-4 bg-red-900/20 border-l-4 border-red-500">
                <div class="flex items-center space-x-2 mb-2">
                  <svg class="w-4 h-4 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  <span class="text-sm font-medium text-red-400">Error</span>
                  <span class="text-xs text-surface-400">({formatExecutionTime(result.executionTime)})</span>
                </div>
                <pre class="text-sm text-red-300 whitespace-pre-wrap font-mono">{result.error}</pre>
              </div>
            {:else if result.output}
              <!-- Success Output -->
              <div class="success-output p-4">
                <div class="flex items-center space-x-2 mb-2">
                  <svg class="w-4 h-4 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                  </svg>
                  <span class="text-sm font-medium text-green-400">Output</span>
                  <span class="text-xs text-surface-400">({formatExecutionTime(result.executionTime)})</span>
                </div>
                
                <div class="output-data">
                  {#if result.output.type === 'text'}
                    <pre class="text-sm text-surface-200 whitespace-pre-wrap font-mono">{result.output.value}</pre>
                  {:else if result.output.type === 'html'}
                    <div class="html-output">{@html result.output.value}</div>
                  {:else if result.output.type === 'table'}
                    <div class="table-output overflow-x-auto">
                      {@html renderTable(result.output.data)}
                    </div>
                  {:else if result.output.type === 'plot'}
                    <div class="plot-output">
                      {@html renderPlot(result.output)}
                    </div>
                  {:else}
                    <pre class="text-sm text-surface-200 whitespace-pre-wrap font-mono">{JSON.stringify(result.output, null, 2)}</pre>
                  {/if}
                </div>
              </div>
            {/if}
          </div>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .cell-output {
    background: theme('colors.surface.950');
  }
  
  .output-content {
    max-height: 400px;
    overflow-y: auto;
  }
  
  :global(.output-table) {
    @apply w-full border-collapse border border-surface-600 text-sm;
  }
  
  :global(.output-table th) {
    @apply bg-surface-800 border border-surface-600 px-3 py-2 text-left font-medium text-surface-200;
  }
  
  :global(.output-table td) {
    @apply border border-surface-600 px-3 py-2 text-surface-300;
  }
  
  :global(.output-table tr:nth-child(even)) {
    @apply bg-surface-900;
  }
  
  .plot-placeholder {
    @apply bg-surface-800 border border-surface-600 rounded-lg p-6 text-center;
  }
  
  .plot-info h4 {
    @apply text-lg font-medium text-surface-200 mb-2;
  }
  
  .plot-info p {
    @apply text-sm text-surface-400 mb-1;
  }
  
  .html-output {
    @apply text-surface-200;
  }
  
  .html-output h1, .html-output h2, .html-output h3, .html-output h4, .html-output h5, .html-output h6 {
    @apply font-bold text-surface-100 mb-2;
  }
  
  .html-output p {
    @apply mb-2;
  }
  
  .html-output ul, .html-output ol {
    @apply ml-4 mb-2;
  }
  
  .html-output li {
    @apply mb-1;
  }
</style>