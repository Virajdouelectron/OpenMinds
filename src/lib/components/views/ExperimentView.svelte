<script lang="ts">
  import { BarChart3, Play, Trash2, Eye, Download, Filter, Search } from 'lucide-svelte'
  
  let experiments = [
    {
      id: 1,
      name: 'Sales Prediction v1',
      status: 'completed',
      algorithm: 'Random Forest',
      dataset: 'sales_data.csv',
      accuracy: 0.94,
      precision: 0.92,
      recall: 0.91,
      f1Score: 0.915,
      startTime: '2024-01-15 10:30:00',
      duration: '2.3 min',
      parameters: {
        n_estimators: 100,
        max_depth: 10,
        learning_rate: 0.1
      }
    },
    {
      id: 2,
      name: 'Customer Churn Analysis',
      status: 'running',
      algorithm: 'XGBoost',
      dataset: 'customer_data.csv',
      accuracy: null,
      precision: null,
      recall: null,
      f1Score: null,
      startTime: '2024-01-15 11:45:00',
      duration: null,
      parameters: {
        n_estimators: 200,
        max_depth: 8,
        learning_rate: 0.05
      }
    },
    {
      id: 3,
      name: 'Housing Price Prediction',
      status: 'failed',
      algorithm: 'Linear Regression',
      dataset: 'housing_prices.csv',
      accuracy: null,
      precision: null,
      recall: null,
      f1Score: null,
      startTime: '2024-01-15 09:15:00',
      duration: '0.5 min',
      parameters: {
        alpha: 1.0,
        fit_intercept: true
      }
    }
  ]
  
  let selectedExperiment = null
  let searchTerm = ''
  let statusFilter = 'all'
  
  $: filteredExperiments = experiments.filter(exp => {
    const matchesSearch = exp.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         exp.algorithm.toLowerCase().includes(searchTerm.toLowerCase())
    const matchesStatus = statusFilter === 'all' || exp.status === statusFilter
    return matchesSearch && matchesStatus
  })
  
  function selectExperiment(experiment) {
    selectedExperiment = experiment
  }
  
  function deleteExperiment(id) {
    experiments = experiments.filter(exp => exp.id !== id)
    if (selectedExperiment?.id === id) {
      selectedExperiment = null
    }
  }
  
  function getStatusColor(status) {
    switch (status) {
      case 'completed': return 'text-secondary-400'
      case 'running': return 'text-yellow-400'
      case 'failed': return 'text-red-400'
      default: return 'text-dark-400'
    }
  }
  
  function getStatusIcon(status) {
    switch (status) {
      case 'completed': return '✓'
      case 'running': return '⟳'
      case 'failed': return '✗'
      default: return '?'
    }
  }
</script>

<div class="flex h-full">
  <!-- Experiment List -->
  <div class="w-96 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-lg font-semibold mb-4 flex items-center">
        <BarChart3 size={20} class="text-purple-400 mr-2" />
        Experiments
      </h2>
      
      <!-- Search and Filter -->
      <div class="space-y-3">
        <div class="relative">
          <Search size={16} class="absolute left-3 top-1/2 transform -translate-y-1/2 text-dark-400" />
          <input 
            type="text" 
            placeholder="Search experiments..."
            bind:value={searchTerm}
            class="input-field w-full pl-10"
          />
        </div>
        
        <select bind:value={statusFilter} class="input-field w-full">
          <option value="all">All Status</option>
          <option value="completed">Completed</option>
          <option value="running">Running</option>
          <option value="failed">Failed</option>
        </select>
      </div>
    </div>
    
    <div class="flex-1 overflow-auto p-2">
      <div class="space-y-2">
        {#each filteredExperiments as experiment}
          <button
            class="w-full p-3 rounded-lg border border-dark-600 hover:border-purple-500 transition-colors text-left {selectedExperiment?.id === experiment.id ? 'border-purple-500 bg-dark-700' : ''}"
            on:click={() => selectExperiment(experiment)}
          >
            <div class="flex items-center justify-between mb-2">
              <span class="font-medium text-sm">{experiment.name}</span>
              <span class="{getStatusColor(experiment.status)} text-xs">
                {getStatusIcon(experiment.status)}
              </span>
            </div>
            
            <div class="text-xs text-dark-400 space-y-1">
              <p>Algorithm: {experiment.algorithm}</p>
              <p>Dataset: {experiment.dataset}</p>
              <p>Started: {new Date(experiment.startTime).toLocaleString()}</p>
              {#if experiment.accuracy}
                <p class="text-secondary-400">Accuracy: {(experiment.accuracy * 100).toFixed(1)}%</p>
              {/if}
            </div>
            
            <div class="flex items-center justify-end space-x-1 mt-2">
              <button 
                class="p-1 hover:bg-dark-600 rounded"
                on:click|stopPropagation={() => selectExperiment(experiment)}
              >
                <Eye size={12} class="text-dark-400" />
              </button>
              <button class="p-1 hover:bg-dark-600 rounded">
                <Download size={12} class="text-dark-400" />
              </button>
              <button 
                class="p-1 hover:bg-red-600 rounded"
                on:click|stopPropagation={() => deleteExperiment(experiment.id)}
              >
                <Trash2 size={12} class="text-red-400" />
              </button>
            </div>
          </button>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Experiment Details -->
  <div class="flex-1 flex flex-col">
    {#if selectedExperiment}
      <!-- Header -->
      <div class="p-4 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold">{selectedExperiment.name}</h3>
            <p class="text-sm text-dark-400">
              Status: <span class="{getStatusColor(selectedExperiment.status)} capitalize">{selectedExperiment.status}</span>
            </p>
          </div>
          
          <div class="flex space-x-2">
            <button class="btn-secondary">
              <Play size={16} class="mr-2" />
              Rerun
            </button>
            <button class="btn-primary">
              <Download size={16} class="mr-2" />
              Export
            </button>
          </div>
        </div>
      </div>
      
      <div class="flex-1 overflow-auto p-4">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
          <!-- Experiment Info -->
          <div class="panel p-4">
            <h4 class="font-medium mb-3">Experiment Details</h4>
            <div class="space-y-2 text-sm">
              <div class="flex justify-between">
                <span class="text-dark-400">Algorithm:</span>
                <span>{selectedExperiment.algorithm}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-dark-400">Dataset:</span>
                <span>{selectedExperiment.dataset}</span>
              </div>
              <div class="flex justify-between">
                <span class="text-dark-400">Start Time:</span>
                <span>{new Date(selectedExperiment.startTime).toLocaleString()}</span>
              </div>
              {#if selectedExperiment.duration}
                <div class="flex justify-between">
                  <span class="text-dark-400">Duration:</span>
                  <span>{selectedExperiment.duration}</span>
                </div>
              {/if}
            </div>
          </div>
          
          <!-- Hyperparameters -->
          <div class="panel p-4">
            <h4 class="font-medium mb-3">Hyperparameters</h4>
            <div class="space-y-2 text-sm">
              {#each Object.entries(selectedExperiment.parameters) as [key, value]}
                <div class="flex justify-between">
                  <span class="text-dark-400">{key}:</span>
                  <span>{value}</span>
                </div>
              {/each}
            </div>
          </div>
          
          <!-- Metrics -->
          {#if selectedExperiment.accuracy}
            <div class="panel p-4 lg:col-span-2">
              <h4 class="font-medium mb-3">Performance Metrics</h4>
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-secondary-400">
                    {(selectedExperiment.accuracy * 100).toFixed(1)}%
                  </div>
                  <div class="text-xs text-dark-400">Accuracy</div>
                </div>
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-secondary-400">
                    {(selectedExperiment.precision * 100).toFixed(1)}%
                  </div>
                  <div class="text-xs text-dark-400">Precision</div>
                </div>
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-secondary-400">
                    {(selectedExperiment.recall * 100).toFixed(1)}%
                  </div>
                  <div class="text-xs text-dark-400">Recall</div>
                </div>
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-secondary-400">
                    {(selectedExperiment.f1Score * 100).toFixed(1)}%
                  </div>
                  <div class="text-xs text-dark-400">F1 Score</div>
                </div>
              </div>
            </div>
          {/if}
          
          <!-- Logs -->
          <div class="panel p-4 lg:col-span-2">
            <h4 class="font-medium mb-3">Training Logs</h4>
            <div class="bg-dark-900 p-3 rounded text-xs font-mono text-dark-300 h-32 overflow-auto">
              <div>2024-01-15 10:30:15 - Starting experiment: {selectedExperiment.name}</div>
              <div>2024-01-15 10:30:16 - Loading dataset: {selectedExperiment.dataset}</div>
              <div>2024-01-15 10:30:17 - Preprocessing data...</div>
              <div>2024-01-15 10:30:20 - Training {selectedExperiment.algorithm} model...</div>
              {#if selectedExperiment.status === 'completed'}
                <div>2024-01-15 10:32:45 - Training completed successfully</div>
                <div>2024-01-15 10:32:46 - Model evaluation completed</div>
              {:else if selectedExperiment.status === 'running'}
                <div>2024-01-15 11:45:30 - Training in progress... (epoch 45/100)</div>
              {:else if selectedExperiment.status === 'failed'}
                <div class="text-red-400">2024-01-15 09:15:30 - Error: Insufficient data for training</div>
              {/if}
            </div>
          </div>
        </div>
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <BarChart3 size={48} class="text-dark-600 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-dark-300 mb-2">No experiment selected</h3>
          <p class="text-dark-500">Select an experiment from the sidebar to view details</p>
        </div>
      </div>
    {/if}
  </div>
</div>