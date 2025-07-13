<script lang="ts">
  import { Upload, Eye, Trash2, Download, Database, BarChart3 } from 'lucide-svelte'
  
  let datasets = [
    { 
      id: 1, 
      name: 'sales_data.csv', 
      size: '2.4 MB', 
      rows: 15420, 
      columns: 8,
      uploadDate: '2024-01-15'
    },
    { 
      id: 2, 
      name: 'customer_data.csv', 
      size: '1.8 MB', 
      rows: 8930, 
      columns: 12,
      uploadDate: '2024-01-14'
    }
  ]
  
  let selectedDataset = null
  let previewData = []
  let stats = null
  
  function selectDataset(dataset) {
    selectedDataset = dataset
    // Simulate loading preview data
    previewData = [
      { id: 1, name: 'John Doe', age: 28, salary: 50000, department: 'Engineering' },
      { id: 2, name: 'Jane Smith', age: 32, salary: 65000, department: 'Marketing' },
      { id: 3, name: 'Bob Johnson', age: 45, salary: 75000, department: 'Sales' }
    ]
    
    stats = {
      nullValues: 245,
      duplicates: 12,
      numericColumns: 4,
      categoricalColumns: 4
    }
  }
  
  function handleFileUpload(event) {
    const file = event.target.files[0]
    if (file) {
      const newDataset = {
        id: Date.now(),
        name: file.name,
        size: (file.size / 1024 / 1024).toFixed(1) + ' MB',
        rows: Math.floor(Math.random() * 10000) + 1000,
        columns: Math.floor(Math.random() * 15) + 5,
        uploadDate: new Date().toISOString().split('T')[0]
      }
      datasets = [...datasets, newDataset]
    }
  }
  
  function deleteDataset(id) {
    datasets = datasets.filter(d => d.id !== id)
    if (selectedDataset?.id === id) {
      selectedDataset = null
      previewData = []
      stats = null
    }
  }
</script>

<div class="flex h-full">
  <!-- Dataset List -->
  <div class="w-80 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold">Datasets</h2>
        <label class="btn-primary p-2 cursor-pointer">
          <Upload size={16} />
          <input type="file" class="hidden" accept=".csv,.json,.xlsx" on:change={handleFileUpload} />
        </label>
      </div>
      
      <div class="space-y-2">
        {#each datasets as dataset}
          <div class="p-3 rounded-lg border border-dark-600 hover:border-secondary-500 transition-colors {selectedDataset?.id === dataset.id ? 'border-secondary-500 bg-dark-700' : ''}">
            <button
              class="w-full text-left"
              on:click={() => selectDataset(dataset)}
            >
              <div class="flex items-center">
                <Database size={16} class="text-secondary-400 mr-2" />
                <span class="font-medium text-sm">{dataset.name}</span>
              </div>
              <div class="text-xs text-dark-400 mt-1">
                <p>{dataset.size} • {dataset.rows} rows • {dataset.columns} cols</p>
                <p>Uploaded: {dataset.uploadDate}</p>
              </div>
            </button>
            
            <div class="flex items-center justify-end space-x-1 mt-2">
              <button 
                class="p-1 hover:bg-dark-600 rounded"
                on:click={() => selectDataset(dataset)}
              >
                <Eye size={14} class="text-dark-400" />
              </button>
              <button class="p-1 hover:bg-dark-600 rounded">
                <Download size={14} class="text-dark-400" />
              </button>
              <button 
                class="p-1 hover:bg-red-600 rounded"
                on:click={() => deleteDataset(dataset.id)}
              >
                <Trash2 size={14} class="text-red-400" />
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Dataset Preview -->
  <div class="flex-1 flex flex-col">
    {#if selectedDataset}
      <!-- Header -->
      <div class="p-4 border-b border-dark-700">
        <h3 class="text-lg font-semibold mb-2">{selectedDataset.name}</h3>
        <div class="flex items-center space-x-4 text-sm text-dark-400">
          <span>{selectedDataset.rows} rows</span>
          <span>{selectedDataset.columns} columns</span>
          <span>{selectedDataset.size}</span>
        </div>
      </div>
      
      <div class="flex-1 flex">
        <!-- Data Preview -->
        <div class="flex-1 p-4">
          <h4 class="font-medium mb-3 flex items-center">
            <Eye size={16} class="mr-2" />
            Data Preview
          </h4>
          
          <div class="panel p-4 overflow-auto">
            <table class="w-full text-sm">
              <thead>
                <tr class="border-b border-dark-600">
                  {#if previewData.length > 0}
                    {#each Object.keys(previewData[0]) as column}
                      <th class="text-left p-2 font-medium text-dark-300">{column}</th>
                    {/each}
                  {/if}
                </tr>
              </thead>
              <tbody>
                {#each previewData as row}
                  <tr class="border-b border-dark-700">
                    {#each Object.values(row) as value}
                      <td class="p-2 text-dark-100">{value}</td>
                    {/each}
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
        
        <!-- Statistics Panel -->
        <div class="w-80 p-4 border-l border-dark-700">
          <h4 class="font-medium mb-3 flex items-center">
            <BarChart3 size={16} class="mr-2" />
            Statistics
          </h4>
          
          {#if stats}
            <div class="space-y-3">
              <div class="panel p-3">
                <div class="text-xs text-dark-400 mb-1">Data Quality</div>
                <div class="space-y-2">
                  <div class="flex justify-between">
                    <span class="text-sm">Null Values</span>
                    <span class="text-sm text-red-400">{stats.nullValues}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-sm">Duplicates</span>
                    <span class="text-sm text-yellow-400">{stats.duplicates}</span>
                  </div>
                </div>
              </div>
              
              <div class="panel p-3">
                <div class="text-xs text-dark-400 mb-1">Column Types</div>
                <div class="space-y-2">
                  <div class="flex justify-between">
                    <span class="text-sm">Numeric</span>
                    <span class="text-sm text-secondary-400">{stats.numericColumns}</span>
                  </div>
                  <div class="flex justify-between">
                    <span class="text-sm">Categorical</span>
                    <span class="text-sm text-accent-400">{stats.categoricalColumns}</span>
                  </div>
                </div>
              </div>
              
              <button class="w-full btn-primary">
                Generate Report
              </button>
            </div>
          {/if}
        </div>
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <Database size={48} class="text-dark-600 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-dark-300 mb-2">No dataset selected</h3>
          <p class="text-dark-500 mb-4">Select a dataset from the sidebar or upload a new one</p>
          
          <label class="btn-primary cursor-pointer">
            <Upload size={16} class="mr-2" />
            Upload Dataset
            <input type="file" class="hidden" accept=".csv,.json,.xlsx" on:change={handleFileUpload} />
          </label>
        </div>
      </div>
    {/if}
  </div>
</div>