<script lang="ts">
  import { Zap, Upload, Play, Download, TrendingUp, CheckCircle, Clock, AlertCircle } from 'lucide-svelte'
  
  let selectedDataset = null
  let targetColumn = ''
  let problemType = 'classification'
  let timeLimit = 30
  let modelQuality = 'medium'
  let wizardStep = 1
  let isRunning = false
  let progress = 0
  let recommendations = []
  
  const datasets = [
    { id: 1, name: 'sales_data.csv', columns: ['date', 'product', 'sales', 'region', 'category'] },
    { id: 2, name: 'customer_data.csv', columns: ['age', 'income', 'gender', 'churn', 'satisfaction'] },
    { id: 3, name: 'housing_prices.csv', columns: ['bedrooms', 'bathrooms', 'sqft', 'price', 'location'] }
  ]
  
  const problemTypes = [
    { value: 'classification', label: 'Classification', description: 'Predict categories or classes' },
    { value: 'regression', label: 'Regression', description: 'Predict continuous values' },
    { value: 'clustering', label: 'Clustering', description: 'Find hidden patterns in data' }
  ]
  
  const qualityLevels = [
    { value: 'fast', label: 'Fast', description: 'Quick results, lower accuracy' },
    { value: 'medium', label: 'Medium', description: 'Balanced speed and accuracy' },
    { value: 'high', label: 'High', description: 'Best accuracy, longer training' }
  ]
  
  function nextStep() {
    if (wizardStep < 4) wizardStep++
  }
  
  function prevStep() {
    if (wizardStep > 1) wizardStep--
  }
  
  function startAutoML() {
    isRunning = true
    progress = 0
    wizardStep = 4
    
    const interval = setInterval(() => {
      progress += Math.random() * 15
      if (progress >= 100) {
        progress = 100
        isRunning = false
        generateRecommendations()
        clearInterval(interval)
      }
    }, 1000)
  }
  
  function generateRecommendations() {
    recommendations = [
      {
        id: 1,
        algorithm: 'Random Forest',
        accuracy: 0.94,
        precision: 0.92,
        recall: 0.91,
        f1Score: 0.915,
        trainingTime: '2.3 min',
        complexity: 'Medium',
        recommended: true
      },
      {
        id: 2,
        algorithm: 'XGBoost',
        accuracy: 0.92,
        precision: 0.90,
        recall: 0.93,
        f1Score: 0.915,
        trainingTime: '4.1 min',
        complexity: 'High',
        recommended: false
      },
      {
        id: 3,
        algorithm: 'Logistic Regression',
        accuracy: 0.87,
        precision: 0.85,
        recall: 0.89,
        f1Score: 0.87,
        trainingTime: '0.8 min',
        complexity: 'Low',
        recommended: false
      }
    ]
  }
  
  function deployModel(model) {
    console.log('Deploying model:', model.algorithm)
  }
</script>

<div class="flex h-full">
  <!-- AutoML Wizard -->
  <div class="flex-1 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-xl font-semibold flex items-center">
        <Zap size={24} class="text-yellow-400 mr-2" />
        AutoML Wizard
      </h2>
      <p class="text-sm text-dark-400 mt-1">Automatically find the best machine learning model for your data</p>
    </div>
    
    <div class="flex-1 overflow-auto p-6">
      {#if wizardStep === 1}
        <!-- Step 1: Dataset Selection -->
        <div class="max-w-2xl">
          <div class="mb-6">
            <h3 class="text-lg font-medium mb-2">Step 1: Select Dataset</h3>
            <p class="text-sm text-dark-400">Choose the dataset you want to analyze</p>
          </div>
          
          <div class="space-y-3 mb-6">
            {#each datasets as dataset}
              <button
                class="w-full p-4 rounded-lg border border-dark-600 hover:border-yellow-500 transition-colors text-left {selectedDataset?.id === dataset.id ? 'border-yellow-500 bg-dark-700' : ''}"
                on:click={() => selectedDataset = dataset}
              >
                <div class="font-medium">{dataset.name}</div>
                <div class="text-sm text-dark-400 mt-1">
                  Columns: {dataset.columns.join(', ')}
                </div>
              </button>
            {/each}
          </div>
          
          <div class="flex justify-end">
            <button 
              class="btn-primary"
              disabled={!selectedDataset}
              on:click={nextStep}
            >
              Next Step
            </button>
          </div>
        </div>
        
      {:else if wizardStep === 2}
        <!-- Step 2: Problem Configuration -->
        <div class="max-w-2xl">
          <div class="mb-6">
            <h3 class="text-lg font-medium mb-2">Step 2: Configure Problem</h3>
            <p class="text-sm text-dark-400">Define what you want to predict</p>
          </div>
          
          <div class="space-y-6">
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-2">Target Column</label>
              <select bind:value={targetColumn} class="input-field w-full">
                <option value="">Select target column...</option>
                {#each selectedDataset?.columns || [] as column}
                  <option value={column}>{column}</option>
                {/each}
              </select>
            </div>
            
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-3">Problem Type</label>
              <div class="space-y-3">
                {#each problemTypes as type}
                  <label class="flex items-start p-3 rounded-lg border border-dark-600 hover:border-yellow-500 cursor-pointer {problemType === type.value ? 'border-yellow-500 bg-dark-700' : ''}">
                    <input 
                      type="radio" 
                      bind:group={problemType} 
                      value={type.value}
                      class="mt-1 mr-3"
                    />
                    <div>
                      <div class="font-medium">{type.label}</div>
                      <div class="text-sm text-dark-400">{type.description}</div>
                    </div>
                  </label>
                {/each}
              </div>
            </div>
          </div>
          
          <div class="flex justify-between mt-6">
            <button class="btn-secondary" on:click={prevStep}>Previous</button>
            <button 
              class="btn-primary"
              disabled={!targetColumn || !problemType}
              on:click={nextStep}
            >
              Next Step
            </button>
          </div>
        </div>
        
      {:else if wizardStep === 3}
        <!-- Step 3: Training Configuration -->
        <div class="max-w-2xl">
          <div class="mb-6">
            <h3 class="text-lg font-medium mb-2">Step 3: Training Settings</h3>
            <p class="text-sm text-dark-400">Configure how AutoML should train your models</p>
          </div>
          
          <div class="space-y-6">
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-2">Time Limit (minutes)</label>
              <input 
                type="range" 
                min="5" 
                max="120" 
                bind:value={timeLimit}
                class="w-full"
              />
              <div class="text-sm text-dark-400 mt-1">{timeLimit} minutes</div>
            </div>
            
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-3">Model Quality</label>
              <div class="space-y-3">
                {#each qualityLevels as quality}
                  <label class="flex items-start p-3 rounded-lg border border-dark-600 hover:border-yellow-500 cursor-pointer {modelQuality === quality.value ? 'border-yellow-500 bg-dark-700' : ''}">
                    <input 
                      type="radio" 
                      bind:group={modelQuality} 
                      value={quality.value}
                      class="mt-1 mr-3"
                    />
                    <div>
                      <div class="font-medium">{quality.label}</div>
                      <div class="text-sm text-dark-400">{quality.description}</div>
                    </div>
                  </label>
                {/each}
              </div>
            </div>
          </div>
          
          <div class="flex justify-between mt-6">
            <button class="btn-secondary" on:click={prevStep}>Previous</button>
            <button class="btn-primary" on:click={startAutoML}>
              <Play size={16} class="mr-2" />
              Start AutoML
            </button>
          </div>
        </div>
        
      {:else if wizardStep === 4}
        <!-- Step 4: Results -->
        <div class="max-w-4xl">
          <div class="mb-6">
            <h3 class="text-lg font-medium mb-2">AutoML Results</h3>
            <p class="text-sm text-dark-400">Best models found for your dataset</p>
          </div>
          
          {#if isRunning}
            <div class="panel p-6 mb-6">
              <div class="flex items-center mb-4">
                <Clock size={20} class="text-yellow-400 mr-2" />
                <span class="font-medium">Training in progress...</span>
              </div>
              
              <div class="mb-4">
                <div class="flex justify-between text-sm mb-1">
                  <span>Progress</span>
                  <span>{progress.toFixed(0)}%</span>
                </div>
                <div class="w-full bg-dark-700 rounded-full h-2">
                  <div 
                    class="bg-yellow-500 h-2 rounded-full transition-all duration-300"
                    style="width: {progress}%"
                  ></div>
                </div>
              </div>
              
              <div class="text-sm text-dark-400">
                Testing different algorithms and hyperparameters...
              </div>
            </div>
          {:else if recommendations.length > 0}
            <div class="space-y-4">
              {#each recommendations as model}
                <div class="panel p-4 {model.recommended ? 'border-yellow-500' : ''}">
                  <div class="flex items-center justify-between mb-3">
                    <div class="flex items-center">
                      <h4 class="font-medium text-lg">{model.algorithm}</h4>
                      {#if model.recommended}
                        <span class="ml-2 px-2 py-1 bg-yellow-500 text-dark-900 text-xs rounded-full font-medium">
                          Recommended
                        </span>
                      {/if}
                    </div>
                    
                    <div class="flex space-x-2">
                      <button class="btn-secondary">
                        <TrendingUp size={16} class="mr-2" />
                        Details
                      </button>
                      <button class="btn-primary" on:click={() => deployModel(model)}>
                        <Download size={16} class="mr-2" />
                        Deploy
                      </button>
                    </div>
                  </div>
                  
                  <div class="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
                    <div>
                      <div class="text-dark-400">Accuracy</div>
                      <div class="font-medium text-secondary-400">{(model.accuracy * 100).toFixed(1)}%</div>
                    </div>
                    <div>
                      <div class="text-dark-400">Precision</div>
                      <div class="font-medium">{(model.precision * 100).toFixed(1)}%</div>
                    </div>
                    <div>
                      <div class="text-dark-400">Training Time</div>
                      <div class="font-medium">{model.trainingTime}</div>
                    </div>
                    <div>
                      <div class="text-dark-400">Complexity</div>
                      <div class="font-medium">{model.complexity}</div>
                    </div>
                  </div>
                </div>
              {/each}
            </div>
            
            <div class="flex justify-between mt-6">
              <button class="btn-secondary" on:click={() => wizardStep = 1}>
                Start New Analysis
              </button>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  </div>
</div>