<script lang="ts">
  import { Plus, Play, Save, FileText } from 'lucide-svelte'
  import CodeEditor from '../editor/CodeEditor.svelte'
  import NotebookCell from '../notebook/NotebookCell.svelte'
  
  let notebooks = [
    { id: 1, name: 'Data Analysis.ipynb', lastModified: '2 hours ago' },
    { id: 2, name: 'Model Training.ipynb', lastModified: '1 day ago' },
    { id: 3, name: 'Exploratory Analysis.ipynb', lastModified: '3 days ago' }
  ]
  
  let activeNotebook = null
  let cells = [
    { id: 1, type: 'code', content: 'import pandas as pd\nimport numpy as np\n\n# Load dataset\ndf = pd.read_csv("data.csv")\nprint(df.head())', output: '' },
    { id: 2, type: 'markdown', content: '# Data Analysis\n\nThis notebook demonstrates basic data analysis techniques.', output: '' },
    { id: 3, type: 'code', content: '# Basic statistics\nprint(df.describe())', output: '' }
  ]
  
  function openNotebook(notebook) {
    activeNotebook = notebook
  }
  
  function addCell(type = 'code') {
    const newCell = {
      id: Date.now(),
      type,
      content: '',
      output: ''
    }
    cells = [...cells, newCell]
  }
  
  function executeCell(cellId) {
    // Simulate cell execution
    const cellIndex = cells.findIndex(c => c.id === cellId)
    if (cellIndex !== -1) {
      cells[cellIndex].output = 'Executed successfully'
      cells = [...cells]
    }
  }
  
  function deleteCell(cellId) {
    cells = cells.filter(c => c.id !== cellId)
  }
</script>

<div class="flex h-full">
  <!-- Notebook List -->
  <div class="w-80 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">Notebooks</h2>
        <button class="btn-primary p-2">
          <Plus size={16} />
        </button>
      </div>
      
      <div class="space-y-2">
        {#each notebooks as notebook}
          <button
            class="w-full p-3 rounded-lg border border-dark-600 hover:border-primary-500 transition-colors text-left {activeNotebook?.id === notebook.id ? 'border-primary-500 bg-dark-700' : ''}"
            on:click={() => openNotebook(notebook)}
          >
            <div class="flex items-center">
              <FileText size={16} class="text-primary-400 mr-2" />
              <span class="font-medium text-sm">{notebook.name}</span>
            </div>
            <p class="text-xs text-dark-400 mt-1">{notebook.lastModified}</p>
          </button>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Notebook Editor -->
  <div class="flex-1 flex flex-col">
    {#if activeNotebook}
      <!-- Toolbar -->
      <div class="p-4 border-b border-dark-700 flex items-center space-x-2">
        <button class="btn-primary">
          <Save size={16} class="mr-2" />
          Save
        </button>
        <button class="btn-secondary">
          <Play size={16} class="mr-2" />
          Run All
        </button>
        <div class="flex space-x-2 ml-auto">
          <button class="btn-secondary" on:click={() => addCell('code')}>+ Code</button>
          <button class="btn-secondary" on:click={() => addCell('markdown')}>+ Markdown</button>
        </div>
      </div>
      
      <!-- Cells -->
      <div class="flex-1 overflow-auto p-4 space-y-4">
        {#each cells as cell}
          <NotebookCell 
            {cell} 
            on:execute={() => executeCell(cell.id)}
            on:delete={() => deleteCell(cell.id)}
          />
        {/each}
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <FileText size={48} class="text-dark-600 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-dark-300 mb-2">No notebook selected</h3>
          <p class="text-dark-500">Select a notebook from the sidebar or create a new one</p>
        </div>
      </div>
    {/if}
  </div>
</div>