<script lang="ts">
  import { Brain, Play, Settings, TrendingUp, Download } from 'lucide-svelte'
  
  let models = [
    { 
      id: 1, 
      name: 'Sales Prediction Model', 
      algorithm: 'Random Forest',
      accuracy: 0.94,
      status: 'trained',
      createdDate: '2024-01-15'
    },
    { 
      id: 2, 
      name: 'Customer Segmentation', 
      algorithm: 'K-Means',
      accuracy: 0.87,
      status: 'training',
      createdDate: '2024-01-14'
    }
  ]
  
  let selectedAlgorithm = 'linear_regression'
  let hyperparameters = {
    learning_rate: 0.001,
    epochs: 100,
    batch_size: 32,
    test_size: 0.2
  }
  
  let trainingStatus = 'idle'
  let trainingProgress = 0
  let metrics = null
  
  const algorithms = [
    { value: 'linear_regression', label: 'Linear Regression' },
    { value: 'random_forest', label: 'Random Forest' },
    { value: 'svm', label: 'Support Vector Machine' },
    { value: 'neural_network', label: 'Neural Network' },
    { value: 'kmeans', label: 'K-Means Clustering' },
    { value: 'logistic_regression', label: 'Logistic Regression' }
  ]
  
  function startTraining() {
    trainingStatus = 'training'
    trainingProgress = 0
    
    const interval = setInterval(() => {
      trainingProgress += Math.random() * 10
      if (trainingProgress >= 100) {
        trainingProgress = 100
        trainingStatus = 'completed'
        metrics = {
          accuracy: 0.92,
          precision: 0.89,
          recall: 0.91,
          f1_score: 0.90
        }
        clearInterval(interval)
      }
    }, 500)
  }
  
  function deployModel(model) {
    console.log('Deploying model:', model.name)
  }
</script>

<div class="flex h-full">
  <!-- Model List -->
  <div class="w-80 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-lg font-semibold mb-4">Models</h2>
      
      <div class="space-y-2">
        {#each models as model}
          <div class="p-3 rounded-lg border border-dark-600 hover:border-accent-500 transition-colors">
            <div class="flex items-center mb-2">
              <Brain size={16} class="text-accent-400 mr-2" />
              <span class="font-medium text-sm">{model.name}</span>
            </div>
            
            <div class="text-xs text-dark-400 space-y-1">
              <p>Algorithm: {model.algorithm}</p>
              <p>Accuracy: {(model.accuracy * 100).toFixed(1)}%</p>
              <p>Status: <span class="capitalize {model.status === 'trained' ? 'text-secondary-400' : 'text-yellow-400'}">{model.status}</span></p>
            </div>
            
            <div class="flex items-center justify-end space-x-1 mt-2">
              <button class="p-1 hover:bg-dark-600 rounded">
                <TrendingUp size={14} class="text-dark-400" />
              </button>
              <button class="p-1 hover:bg-dark-600 rounded" on:click={() => deployModel(model)}>
                <Download size={14} class="text-dark-400" />
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Model Trainer -->
  <div class="flex-1 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-dark-700">
      <h3 class="text-lg font-semibold">Model Trainer</h3>
      <p class="text-sm text-dark-400">Configure and train machine learning models</p>
    </div>
    
    <div class="flex-1 overflow-auto p-4">
      <div class="max-w-2xl space-y-6">
        <!-- Algorithm Selection -->
        <div class="panel p-4">
          <h4 class="font-medium mb-3 flex items-center">
            <Settings size={16} class="mr-2" />
            Algorithm Selection
          </h4>
          
          <select 
            bind:value={selectedAlgorithm}
            class="input-field w-full"
          >
            {#each algorithms as algo}
              <option value={algo.value}>{algo.label}</option>
            {/each}
          </select>
        </div>
        
        <!-- Hyperparameters -->
        <div class="panel p-4">
          <h4 class="font-medium mb-3">Hyperparameters</h4>
          
          <div class="grid grid-cols-2 gap-4">
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Learning Rate</label>
              <input 
                type="number" 
                bind:value={hyperparameters.learning_rate}
                step="0.001"
                class="input-field w-full"
              />
            </div>
            
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Epochs</label>
              <input 
                type="number" 
                bind:value={hyperparameters.epochs}
                class="input-field w-full"
              />
            </div>
            
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Batch Size</label>
              <input 
                type="number" 
                bind:value={hyperparameters.batch_size}
                class="input-field w-full"
              />
            </div>
            
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Test Size</label>
              <input 
                type="number" 
                bind:value={hyperparameters.test_size}
                step="0.1"
                min="0.1"
                max="0.5"
                class="input-field w-full"
              />
            </div>
          </div>
        </div>
        
        <!-- Training Controls -->
        <div class="panel p-4">
          <div class="flex items-center justify-between mb-4">
            <h4 class="font-medium">Training</h4>
            <button 
              class="btn-primary"
              disabled={trainingStatus === 'training'}
              on:click={startTraining}
            >
              <Play size={16} class="mr-2" />
              {trainingStatus === 'training' ? 'Training...' : 'Start Training'}
            </button>
          </div>
          
          {#if trainingStatus === 'training'}
            <div class="mb-4">
              <div class="flex justify-between text-sm mb-1">
                <span>Progress</span>
                <span>{trainingProgress.toFixed(0)}%</span>
              </div>
              <div class="w-full bg-dark-700 rounded-full h-2">
                <div 
                  class="bg-primary-500 h-2 rounded-full transition-all duration-300"
                  style="width: {trainingProgress}%"
                ></div>
              </div>
            </div>
          {/if}
          
          {#if metrics}
            <div class="grid grid-cols-2 gap-4">
              <div class="text-center p-3 bg-dark-700 rounded">
                <div class="text-lg font-semibold text-secondary-400">{(metrics.accuracy * 100).toFixed(1)}%</div>
                <div class="text-xs text-dark-400">Accuracy</div>
              </div>
              <div class="text-center p-3 bg-dark-700 rounded">
                <div class="text-lg font-semibold text-secondary-400">{(metrics.precision * 100).toFixed(1)}%</div>
                <div class="text-xs text-dark-400">Precision</div>
              </div>
              <div class="text-center p-3 bg-dark-700 rounded">
                <div class="text-lg font-semibold text-secondary-400">{(metrics.recall * 100).toFixed(1)}%</div>
                <div class="text-xs text-dark-400">Recall</div>
              </div>
              <div class="text-center p-3 bg-dark-700 rounded">
                <div class="text-lg font-semibold text-secondary-400">{(metrics.f1_score * 100).toFixed(1)}%</div>
                <div class="text-xs text-dark-400">F1 Score</div>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
</div>