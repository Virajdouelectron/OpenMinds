<script lang="ts">
  import { Rocket, Globe, Code, Settings, Play, Eye, Download, Copy, ExternalLink, CheckCircle } from 'lucide-svelte'
  
  let selectedModel = null
  let deploymentType = 'api'
  let deploymentStatus = 'idle'
  let deploymentUrl = ''
  let isDeploying = false
  let deploymentProgress = 0
  
  const models = [
    {
      id: 1,
      name: 'Sales Prediction Model',
      algorithm: 'Random Forest',
      accuracy: 0.94,
      size: '2.3 MB',
      lastTrained: '2024-01-15',
      status: 'ready'
    },
    {
      id: 2,
      name: 'Customer Churn Model',
      algorithm: 'XGBoost',
      accuracy: 0.89,
      size: '5.1 MB',
      lastTrained: '2024-01-14',
      status: 'ready'
    },
    {
      id: 3,
      name: 'Image Classifier',
      algorithm: 'CNN',
      accuracy: 0.96,
      size: '45.2 MB',
      lastTrained: '2024-01-13',
      status: 'ready'
    }
  ]
  
  const deploymentTypes = [
    {
      id: 'api',
      name: 'REST API',
      description: 'Deploy as a REST API endpoint for programmatic access',
      icon: Code,
      features: ['JSON input/output', 'Authentication', 'Rate limiting', 'Monitoring']
    },
    {
      id: 'webapp',
      name: 'Web Application',
      description: 'Create a simple web interface for model interaction',
      icon: Globe,
      features: ['User-friendly interface', 'File upload', 'Real-time predictions', 'Result visualization']
    },
    {
      id: 'batch',
      name: 'Batch Processing',
      description: 'Process large datasets in batches',
      icon: Settings,
      features: ['Bulk processing', 'Scheduled runs', 'Progress tracking', 'Result export']
    }
  ]
  
  let deploymentConfig = {
    name: '',
    description: '',
    version: '1.0.0',
    environment: 'production',
    scaling: 'auto',
    memory: '512MB',
    timeout: 30,
    authentication: true,
    monitoring: true
  }
  
  let deployedApps = [
    {
      id: 1,
      name: 'Sales Predictor API',
      model: 'Sales Prediction Model',
      type: 'api',
      url: 'https://api.openminds.ai/sales-predictor',
      status: 'running',
      requests: 1247,
      uptime: '99.9%',
      deployedAt: '2024-01-10'
    },
    {
      id: 2,
      name: 'Churn Analysis Dashboard',
      model: 'Customer Churn Model',
      type: 'webapp',
      url: 'https://churn-analysis.openminds.ai',
      status: 'running',
      requests: 89,
      uptime: '100%',
      deployedAt: '2024-01-12'
    }
  ]
  
  function selectModel(model) {
    selectedModel = model
    deploymentConfig.name = `${model.name} API`
    deploymentConfig.description = `Deployed ${model.algorithm} model for predictions`
  }
  
  function startDeployment() {
    if (!selectedModel) return
    
    isDeploying = true
    deploymentProgress = 0
    deploymentStatus = 'deploying'
    
    const interval = setInterval(() => {
      deploymentProgress += Math.random() * 15
      if (deploymentProgress >= 100) {
        deploymentProgress = 100
        isDeploying = false
        deploymentStatus = 'deployed'
        deploymentUrl = `https://api.openminds.ai/${selectedModel.name.toLowerCase().replace(/\s+/g, '-')}`
        
        // Add to deployed apps
        const newApp = {
          id: Date.now(),
          name: deploymentConfig.name,
          model: selectedModel.name,
          type: deploymentType,
          url: deploymentUrl,
          status: 'running',
          requests: 0,
          uptime: '100%',
          deployedAt: new Date().toISOString().split('T')[0]
        }
        deployedApps = [...deployedApps, newApp]
        
        clearInterval(interval)
      }
    }, 800)
  }
  
  function copyToClipboard(text) {
    navigator.clipboard.writeText(text)
  }
  
  function getStatusColor(status) {
    switch (status) {
      case 'running': return 'text-secondary-400'
      case 'deploying': return 'text-yellow-400'
      case 'stopped': return 'text-red-400'
      case 'error': return 'text-red-400'
      default: return 'text-dark-400'
    }
  }
  
  function generateApiCode() {
    if (!deploymentUrl) return ''
    
    return `# Python example
import requests
import json

url = "${deploymentUrl}/predict"
headers = {
    "Content-Type": "application/json",
    "Authorization": "Bearer YOUR_API_KEY"
}

data = {
    "features": [1.2, 3.4, 5.6, 7.8]
}

response = requests.post(url, headers=headers, json=data)
result = response.json()
print(f"Prediction: {result['prediction']}")

# JavaScript example
const response = await fetch('${deploymentUrl}/predict', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
        'Authorization': 'Bearer YOUR_API_KEY'
    },
    body: JSON.stringify({
        features: [1.2, 3.4, 5.6, 7.8]
    })
});

const result = await response.json();
console.log('Prediction:', result.prediction);`
  }
</script>

<div class="flex h-full">
  <!-- Model Selection & Config -->
  <div class="w-96 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-lg font-semibold mb-4 flex items-center">
        <Rocket size={20} class="text-red-400 mr-2" />
        Model Deployment
      </h2>
      
      <p class="text-sm text-dark-400 mb-4">
        Deploy your trained models as APIs or web applications with one click.
      </p>
    </div>
    
    <div class="flex-1 overflow-auto p-4 space-y-6">
      <!-- Model Selection -->
      <div>
        <h3 class="font-medium mb-3">Select Model</h3>
        <div class="space-y-2">
          {#each models as model}
            <button
              class="w-full p-3 rounded-lg border border-dark-600 hover:border-red-500 transition-colors text-left {selectedModel?.id === model.id ? 'border-red-500 bg-dark-700' : ''}"
              on:click={() => selectModel(model)}
            >
              <div class="font-medium text-sm mb-1">{model.name}</div>
              <div class="text-xs text-dark-400 space-y-1">
                <p>Algorithm: {model.algorithm}</p>
                <p>Accuracy: {(model.accuracy * 100).toFixed(1)}%</p>
                <p>Size: {model.size}</p>
              </div>
            </button>
          {/each}
        </div>
      </div>
      
      <!-- Deployment Type -->
      <div>
        <h3 class="font-medium mb-3">Deployment Type</h3>
        <div class="space-y-2">
          {#each deploymentTypes as type}
            <label class="flex items-start p-3 rounded-lg border border-dark-600 hover:border-red-500 cursor-pointer {deploymentType === type.id ? 'border-red-500 bg-dark-700' : ''}">
              <input 
                type="radio" 
                bind:group={deploymentType} 
                value={type.id}
                class="mt-1 mr-3"
              />
              <div class="flex-1">
                <div class="flex items-center mb-1">
                  <svelte:component this={type.icon} size={16} class="mr-2" />
                  <span class="font-medium text-sm">{type.name}</span>
                </div>
                <p class="text-xs text-dark-400 mb-2">{type.description}</p>
                <div class="flex flex-wrap gap-1">
                  {#each type.features as feature}
                    <span class="text-xs px-2 py-1 bg-dark-600 rounded">{feature}</span>
                  {/each}
                </div>
              </div>
            </label>
          {/each}
        </div>
      </div>
      
      <!-- Configuration -->
      {#if selectedModel}
        <div>
          <h3 class="font-medium mb-3">Configuration</h3>
          <div class="space-y-3">
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Name</label>
              <input 
                type="text" 
                bind:value={deploymentConfig.name}
                class="input-field w-full"
              />
            </div>
            
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Description</label>
              <textarea 
                bind:value={deploymentConfig.description}
                class="input-field w-full h-20 resize-none"
              ></textarea>
            </div>
            
            <div class="grid grid-cols-2 gap-2">
              <div>
                <label class="block text-sm font-medium text-dark-300 mb-1">Memory</label>
                <select bind:value={deploymentConfig.memory} class="input-field w-full">
                  <option value="256MB">256MB</option>
                  <option value="512MB">512MB</option>
                  <option value="1GB">1GB</option>
                  <option value="2GB">2GB</option>
                </select>
              </div>
              
              <div>
                <label class="block text-sm font-medium text-dark-300 mb-1">Timeout (s)</label>
                <input 
                  type="number" 
                  bind:value={deploymentConfig.timeout}
                  class="input-field w-full"
                />
              </div>
            </div>
            
            <div class="space-y-2">
              <label class="flex items-center">
                <input 
                  type="checkbox" 
                  bind:checked={deploymentConfig.authentication}
                  class="mr-2"
                />
                <span class="text-sm">Enable Authentication</span>
              </label>
              
              <label class="flex items-center">
                <input 
                  type="checkbox" 
                  bind:checked={deploymentConfig.monitoring}
                  class="mr-2"
                />
                <span class="text-sm">Enable Monitoring</span>
              </label>
            </div>
          </div>
        </div>
      {/if}
    </div>
    
    <!-- Deploy Button -->
    <div class="p-4 border-t border-dark-700">
      <button 
        class="btn-primary w-full"
        disabled={!selectedModel || isDeploying}
        on:click={startDeployment}
      >
        <Rocket size={16} class="mr-2" />
        {isDeploying ? 'Deploying...' : 'Deploy Model'}
      </button>
    </div>
  </div>
  
  <!-- Deployment Dashboard -->
  <div class="flex-1 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-dark-700">
      <h3 class="text-lg font-semibold">Deployment Dashboard</h3>
      <p class="text-sm text-dark-400">Monitor and manage your deployed models</p>
    </div>
    
    <div class="flex-1 overflow-auto p-4">
      {#if isDeploying}
        <!-- Deployment Progress -->
        <div class="panel p-6 mb-6">
          <div class="flex items-center mb-4">
            <Rocket size={20} class="text-red-400 mr-2" />
            <span class="font-medium">Deploying {selectedModel.name}...</span>
          </div>
          
          <div class="mb-4">
            <div class="flex justify-between text-sm mb-1">
              <span>Progress</span>
              <span>{deploymentProgress.toFixed(0)}%</span>
            </div>
            <div class="w-full bg-dark-700 rounded-full h-2">
              <div 
                class="bg-red-500 h-2 rounded-full transition-all duration-300"
                style="width: {deploymentProgress}%"
              ></div>
            </div>
          </div>
          
          <div class="text-sm text-dark-400">
            Building container, configuring endpoints, and starting services...
          </div>
        </div>
      {/if}
      
      {#if deploymentStatus === 'deployed' && deploymentUrl}
        <!-- Deployment Success -->
        <div class="panel p-6 mb-6 border-secondary-500">
          <div class="flex items-center mb-4">
            <CheckCircle size={20} class="text-secondary-400 mr-2" />
            <span class="font-medium">Deployment Successful!</span>
          </div>
          
          <div class="space-y-3">
            <div>
              <label class="block text-sm font-medium text-dark-300 mb-1">Endpoint URL</label>
              <div class="flex items-center space-x-2">
                <input 
                  type="text" 
                  value={deploymentUrl}
                  readonly
                  class="input-field flex-1"
                />
                <button 
                  class="btn-secondary p-2"
                  on:click={() => copyToClipboard(deploymentUrl)}
                >
                  <Copy size={16} />
                </button>
                <button class="btn-primary p-2">
                  <ExternalLink size={16} />
                </button>
              </div>
            </div>
            
            <div class="grid grid-cols-2 gap-4">
              <button class="btn-secondary">
                <Eye size={16} class="mr-2" />
                Test API
              </button>
              <button class="btn-secondary">
                <Download size={16} class="mr-2" />
                Get API Key
              </button>
            </div>
          </div>
        </div>
        
        <!-- API Documentation -->
        <div class="panel p-4 mb-6">
          <h4 class="font-medium mb-3">API Usage Example</h4>
          <pre class="bg-dark-900 p-3 rounded text-sm overflow-x-auto text-dark-300">{generateApiCode()}</pre>
        </div>
      {/if}
      
      <!-- Deployed Applications -->
      <div class="panel p-4">
        <h4 class="font-medium mb-4">Deployed Applications</h4>
        
        {#if deployedApps.length > 0}
          <div class="space-y-3">
            {#each deployedApps as app}
              <div class="p-4 bg-dark-700 rounded-lg">
                <div class="flex items-center justify-between mb-3">
                  <div>
                    <h5 class="font-medium">{app.name}</h5>
                    <p class="text-sm text-dark-400">Model: {app.model}</p>
                  </div>
                  
                  <div class="flex items-center space-x-2">
                    <span class="px-2 py-1 text-xs rounded {app.status === 'running' ? 'bg-secondary-600' : 'bg-red-600'}">
                      {app.status}
                    </span>
                    <button class="p-1 hover:bg-dark-600 rounded">
                      <ExternalLink size={14} class="text-dark-400" />
                    </button>
                  </div>
                </div>
                
                <div class="grid grid-cols-3 gap-4 text-sm">
                  <div>
                    <div class="text-dark-400">Requests</div>
                    <div class="font-medium">{app.requests.toLocaleString()}</div>
                  </div>
                  <div>
                    <div class="text-dark-400">Uptime</div>
                    <div class="font-medium">{app.uptime}</div>
                  </div>
                  <div>
                    <div class="text-dark-400">Deployed</div>
                    <div class="font-medium">{app.deployedAt}</div>
                  </div>
                </div>
                
                <div class="flex items-center justify-between mt-3 pt-3 border-t border-dark-600">
                  <div class="flex items-center text-sm text-dark-400">
                    <Globe size={14} class="mr-1" />
                    {app.url}
                  </div>
                  
                  <div class="flex space-x-2">
                    <button class="btn-secondary text-xs px-2 py-1">Monitor</button>
                    <button class="btn-secondary text-xs px-2 py-1">Logs</button>
                    <button class="btn-secondary text-xs px-2 py-1">Settings</button>
                  </div>
                </div>
              </div>
            {/each}
          </div>
        {:else}
          <div class="text-center py-8">
            <Rocket size={32} class="text-dark-600 mx-auto mb-2" />
            <p class="text-sm text-dark-400">No deployed applications yet</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
</div>