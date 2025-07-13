<script lang="ts">
  import { Blocks, Plus, Play, Save, Download, Trash2, Settings, Eye, Copy } from 'lucide-svelte'
  
  let selectedComponent = null
  let canvasComponents = []
  let connections = []
  let draggedComponent = null
  let canvasElement
  
  const componentLibrary = [
    {
      category: 'Data Input',
      components: [
        { id: 'csv-input', name: 'CSV Input', icon: '📄', color: 'bg-blue-500', description: 'Load data from CSV file' },
        { id: 'database-input', name: 'Database Input', icon: '🗄️', color: 'bg-blue-600', description: 'Connect to database' },
        { id: 'api-input', name: 'API Input', icon: '🌐', color: 'bg-blue-700', description: 'Fetch data from API' }
      ]
    },
    {
      category: 'Preprocessing',
      components: [
        { id: 'scaler', name: 'Standard Scaler', icon: '📏', color: 'bg-green-500', description: 'Normalize features' },
        { id: 'encoder', name: 'Label Encoder', icon: '🏷️', color: 'bg-green-600', description: 'Encode categorical data' },
        { id: 'imputer', name: 'Missing Value Imputer', icon: '🔧', color: 'bg-green-700', description: 'Handle missing values' },
        { id: 'feature-selector', name: 'Feature Selector', icon: '🎯', color: 'bg-green-800', description: 'Select important features' }
      ]
    },
    {
      category: 'Models',
      components: [
        { id: 'linear-regression', name: 'Linear Regression', icon: '📈', color: 'bg-purple-500', description: 'Linear regression model' },
        { id: 'random-forest', name: 'Random Forest', icon: '🌳', color: 'bg-purple-600', description: 'Ensemble tree model' },
        { id: 'neural-network', name: 'Neural Network', icon: '🧠', color: 'bg-purple-700', description: 'Deep learning model' },
        { id: 'svm', name: 'SVM', icon: '⚡', color: 'bg-purple-800', description: 'Support Vector Machine' }
      ]
    },
    {
      category: 'Evaluation',
      components: [
        { id: 'train-test-split', name: 'Train/Test Split', icon: '✂️', color: 'bg-orange-500', description: 'Split data for training' },
        { id: 'cross-validation', name: 'Cross Validation', icon: '🔄', color: 'bg-orange-600', description: 'K-fold validation' },
        { id: 'metrics', name: 'Metrics', icon: '📊', color: 'bg-orange-700', description: 'Calculate performance metrics' }
      ]
    },
    {
      category: 'Output',
      components: [
        { id: 'predictions', name: 'Predictions', icon: '🎯', color: 'bg-red-500', description: 'Model predictions' },
        { id: 'model-export', name: 'Model Export', icon: '💾', color: 'bg-red-600', description: 'Save trained model' },
        { id: 'visualization', name: 'Visualization', icon: '📈', color: 'bg-red-700', description: 'Create charts and plots' }
      ]
    }
  ]
  
  let selectedCategory = 'Data Input'
  
  function addComponentToCanvas(component) {
    const newComponent = {
      id: Date.now(),
      type: component.id,
      name: component.name,
      icon: component.icon,
      color: component.color,
      x: Math.random() * 400 + 50,
      y: Math.random() * 300 + 50,
      width: 150,
      height: 80,
      parameters: getDefaultParameters(component.id)
    }
    
    canvasComponents = [...canvasComponents, newComponent]
  }
  
  function getDefaultParameters(componentType) {
    switch (componentType) {
      case 'scaler':
        return { method: 'standard', with_mean: true, with_std: true }
      case 'random-forest':
        return { n_estimators: 100, max_depth: 10, random_state: 42 }
      case 'neural-network':
        return { hidden_layers: [64, 32], activation: 'relu', epochs: 100 }
      case 'train-test-split':
        return { test_size: 0.2, random_state: 42, stratify: true }
      default:
        return {}
    }
  }
  
  function selectComponent(component) {
    selectedComponent = component
  }
  
  function deleteComponent(componentId) {
    canvasComponents = canvasComponents.filter(c => c.id !== componentId)
    connections = connections.filter(conn => 
      conn.from !== componentId && conn.to !== componentId
    )
    if (selectedComponent?.id === componentId) {
      selectedComponent = null
    }
  }
  
  function duplicateComponent(component) {
    const newComponent = {
      ...component,
      id: Date.now(),
      x: component.x + 20,
      y: component.y + 20
    }
    canvasComponents = [...canvasComponents, newComponent]
  }
  
  function generateCode() {
    let code = `# Generated ML Pipeline\nimport pandas as pd\nimport numpy as np\nfrom sklearn.model_selection import train_test_split\nfrom sklearn.preprocessing import StandardScaler, LabelEncoder\nfrom sklearn.ensemble import RandomForestClassifier\nfrom sklearn.linear_model import LinearRegression\nfrom sklearn.neural_network import MLPClassifier\nfrom sklearn.svm import SVC\nfrom sklearn.metrics import accuracy_score, classification_report\n\n`
    
    // Sort components by execution order (simplified)
    const sortedComponents = [...canvasComponents].sort((a, b) => a.x - b.x)
    
    sortedComponents.forEach(component => {
      switch (component.type) {
        case 'csv-input':
          code += `# Load data\ndf = pd.read_csv('data.csv')\nX = df.drop('target', axis=1)\ny = df['target']\n\n`
          break
        case 'scaler':
          code += `# Scale features\nscaler = StandardScaler()\nX_scaled = scaler.fit_transform(X)\n\n`
          break
        case 'train-test-split':
          code += `# Split data\nX_train, X_test, y_train, y_test = train_test_split(\n    X_scaled, y, test_size=${component.parameters.test_size}, random_state=${component.parameters.random_state}\n)\n\n`
          break
        case 'random-forest':
          code += `# Train Random Forest\nrf_model = RandomForestClassifier(\n    n_estimators=${component.parameters.n_estimators},\n    max_depth=${component.parameters.max_depth},\n    random_state=${component.parameters.random_state}\n)\nrf_model.fit(X_train, y_train)\n\n`
          break
        case 'metrics':
          code += `# Evaluate model\ny_pred = rf_model.predict(X_test)\naccuracy = accuracy_score(y_test, y_pred)\nprint(f"Accuracy: {accuracy:.3f}")\nprint(classification_report(y_test, y_pred))\n\n`
          break
      }
    })
    
    return code
  }
  
  function runPipeline() {
    const code = generateCode()
    console.log('Generated Code:', code)
    // In a real implementation, this would execute the pipeline
    alert('Pipeline execution started! Check the console for generated code.')
  }
  
  function savePipeline() {
    const pipeline = {
      components: canvasComponents,
      connections: connections,
      metadata: {
        name: 'ML Pipeline',
        created: new Date().toISOString(),
        version: '1.0'
      }
    }
    
    const blob = new Blob([JSON.stringify(pipeline, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'ml-pipeline.json'
    a.click()
    URL.revokeObjectURL(url)
  }
</script>

<div class="flex h-full">
  <!-- Component Library -->
  <div class="w-80 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-lg font-semibold mb-4 flex items-center">
        <Blocks size={20} class="text-pink-400 mr-2" />
        Component Library
      </h2>
      
      <select bind:value={selectedCategory} class="input-field w-full">
        {#each componentLibrary as category}
          <option value={category.category}>{category.category}</option>
        {/each}
      </select>
    </div>
    
    <div class="flex-1 overflow-auto p-2">
      {#each componentLibrary as category}
        {#if category.category === selectedCategory}
          <div class="space-y-2">
            {#each category.components as component}
              <div
                class="p-3 rounded-lg border border-dark-600 hover:border-pink-500 cursor-pointer transition-colors"
                draggable="true"
                on:dragstart={() => draggedComponent = component}
                on:click={() => addComponentToCanvas(component)}
              >
                <div class="flex items-center mb-2">
                  <div class="w-8 h-8 {component.color} rounded flex items-center justify-center text-white mr-3">
                    {component.icon}
                  </div>
                  <span class="font-medium text-sm">{component.name}</span>
                </div>
                <p class="text-xs text-dark-400">{component.description}</p>
              </div>
            {/each}
          </div>
        {/if}
      {/each}
    </div>
  </div>
  
  <!-- Canvas -->
  <div class="flex-1 flex flex-col">
    <!-- Toolbar -->
    <div class="p-4 border-b border-dark-700 flex items-center justify-between">
      <h3 class="text-lg font-semibold">Visual Pipeline Builder</h3>
      
      <div class="flex space-x-2">
        <button class="btn-secondary">
          <Eye size={16} class="mr-2" />
          Preview
        </button>
        <button class="btn-secondary" on:click={savePipeline}>
          <Save size={16} class="mr-2" />
          Save
        </button>
        <button class="btn-primary" on:click={runPipeline}>
          <Play size={16} class="mr-2" />
          Run Pipeline
        </button>
      </div>
    </div>
    
    <!-- Canvas Area -->
    <div 
      bind:this={canvasElement}
      class="flex-1 relative bg-dark-800 overflow-auto"
      on:dragover={(e) => e.preventDefault()}
      on:drop={(e) => {
        e.preventDefault()
        if (draggedComponent) {
          addComponentToCanvas(draggedComponent)
          draggedComponent = null
        }
      }}
    >
      <!-- Grid Background -->
      <div class="absolute inset-0 opacity-10" style="background-image: radial-gradient(circle, #64748b 1px, transparent 1px); background-size: 20px 20px;"></div>
      
      <!-- Components -->
      {#each canvasComponents as component}
        <div
          class="absolute border-2 rounded-lg cursor-move {selectedComponent?.id === component.id ? 'border-pink-500' : 'border-dark-600'}"
          style="left: {component.x}px; top: {component.y}px; width: {component.width}px; height: {component.height}px;"
          on:click={() => selectComponent(component)}
        >
          <div class="h-full {component.color} rounded-lg p-3 text-white">
            <div class="flex items-center justify-between mb-2">
              <div class="flex items-center">
                <span class="mr-2">{component.icon}</span>
                <span class="text-sm font-medium">{component.name}</span>
              </div>
              
              <div class="flex space-x-1">
                <button 
                  class="p-1 hover:bg-black hover:bg-opacity-20 rounded"
                  on:click|stopPropagation={() => duplicateComponent(component)}
                >
                  <Copy size={12} />
                </button>
                <button 
                  class="p-1 hover:bg-black hover:bg-opacity-20 rounded"
                  on:click|stopPropagation={() => deleteComponent(component.id)}
                >
                  <Trash2 size={12} />
                </button>
              </div>
            </div>
            
            <!-- Connection Points -->
            <div class="absolute -left-2 top-1/2 w-4 h-4 bg-white rounded-full border-2 border-dark-600"></div>
            <div class="absolute -right-2 top-1/2 w-4 h-4 bg-white rounded-full border-2 border-dark-600"></div>
          </div>
        </div>
      {/each}
      
      <!-- Empty State -->
      {#if canvasComponents.length === 0}
        <div class="absolute inset-0 flex items-center justify-center">
          <div class="text-center">
            <Blocks size={48} class="text-dark-600 mx-auto mb-4" />
            <h3 class="text-lg font-medium text-dark-300 mb-2">Start Building Your Pipeline</h3>
            <p class="text-dark-500 mb-4">Drag components from the library or click to add them</p>
            <button class="btn-primary" on:click={() => addComponentToCanvas(componentLibrary[0].components[0])}>
              <Plus size={16} class="mr-2" />
              Add First Component
            </button>
          </div>
        </div>
      {/if}
    </div>
  </div>
  
  <!-- Properties Panel -->
  <div class="w-80 panel border-l border-dark-700 p-4">
    {#if selectedComponent}
      <h3 class="font-medium mb-4">Component Properties</h3>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Name</label>
          <input 
            type="text" 
            bind:value={selectedComponent.name}
            class="input-field w-full"
          />
        </div>
        
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Parameters</label>
          <div class="space-y-3">
            {#each Object.entries(selectedComponent.parameters) as [key, value]}
              <div>
                <label class="block text-xs text-dark-400 mb-1">{key}</label>
                {#if typeof value === 'boolean'}
                  <input 
                    type="checkbox" 
                    bind:checked={selectedComponent.parameters[key]}
                    class="mr-2"
                  />
                {:else if typeof value === 'number'}
                  <input 
                    type="number" 
                    bind:value={selectedComponent.parameters[key]}
                    class="input-field w-full"
                  />
                {:else}
                  <input 
                    type="text" 
                    bind:value={selectedComponent.parameters[key]}
                    class="input-field w-full"
                  />
                {/if}
              </div>
            {/each}
          </div>
        </div>
        
        <button class="btn-secondary w-full">
          <Settings size={16} class="mr-2" />
          Advanced Settings
        </button>
      </div>
    {:else}
      <div class="text-center py-8">
        <Settings size={32} class="text-dark-600 mx-auto mb-2" />
        <p class="text-sm text-dark-400">Select a component to edit properties</p>
      </div>
    {/if}
  </div>
</div>