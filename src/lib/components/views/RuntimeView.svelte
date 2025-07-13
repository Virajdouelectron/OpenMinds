<script lang="ts">
  import { Server, Cpu, Zap, Monitor, Settings, Play, Square, RefreshCw, Cloud, HardDrive } from 'lucide-svelte'
  
  let selectedRuntime = 'local'
  let runtimeStatus = 'idle'
  let cpuUsage = 15
  let memoryUsage = 32
  let gpuUsage = 0
  let activeJobs = []
  
  const runtimeOptions = [
    {
      id: 'local',
      name: 'Local Runtime',
      type: 'local',
      icon: HardDrive,
      specs: {
        cpu: 'Intel i7-10700K (8 cores)',
        memory: '32 GB DDR4',
        gpu: 'None',
        storage: '1 TB SSD'
      },
      status: 'available',
      cost: 'Free'
    },
    {
      id: 'cloud-cpu',
      name: 'Cloud CPU (Standard)',
      type: 'cloud',
      icon: Cloud,
      specs: {
        cpu: '4 vCPUs (Intel Xeon)',
        memory: '16 GB',
        gpu: 'None',
        storage: '100 GB SSD'
      },
      status: 'available',
      cost: '$0.50/hour'
    },
    {
      id: 'cloud-gpu-t4',
      name: 'Cloud GPU (T4)',
      type: 'cloud',
      icon: Zap,
      specs: {
        cpu: '4 vCPUs',
        memory: '16 GB',
        gpu: 'NVIDIA Tesla T4 (16 GB)',
        storage: '100 GB SSD'
      },
      status: 'available',
      cost: '$1.20/hour'
    },
    {
      id: 'cloud-gpu-v100',
      name: 'Cloud GPU (V100)',
      type: 'cloud',
      icon: Zap,
      specs: {
        cpu: '8 vCPUs',
        memory: '32 GB',
        gpu: 'NVIDIA Tesla V100 (32 GB)',
        storage: '200 GB SSD'
      },
      status: 'available',
      cost: '$2.50/hour'
    },
    {
      id: 'cloud-gpu-a100',
      name: 'Cloud GPU (A100)',
      type: 'cloud',
      icon: Zap,
      specs: {
        cpu: '12 vCPUs',
        memory: '64 GB',
        gpu: 'NVIDIA A100 (80 GB)',
        storage: '500 GB SSD'
      },
      status: 'available',
      cost: '$4.00/hour'
    }
  ]
  
  let currentRuntime = runtimeOptions[0]
  
  function selectRuntime(runtime) {
    currentRuntime = runtime
    selectedRuntime = runtime.id
    
    // Simulate runtime switch
    runtimeStatus = 'switching'
    setTimeout(() => {
      runtimeStatus = 'ready'
      // Update usage based on runtime type
      if (runtime.specs.gpu !== 'None') {
        gpuUsage = Math.floor(Math.random() * 30)
      } else {
        gpuUsage = 0
      }
    }, 2000)
  }
  
  function startRuntime() {
    runtimeStatus = 'starting'
    setTimeout(() => {
      runtimeStatus = 'running'
      // Simulate some usage
      cpuUsage = Math.floor(Math.random() * 40 + 20)
      memoryUsage = Math.floor(Math.random() * 30 + 25)
      if (currentRuntime.specs.gpu !== 'None') {
        gpuUsage = Math.floor(Math.random() * 50 + 10)
      }
    }, 3000)
  }
  
  function stopRuntime() {
    runtimeStatus = 'stopping'
    setTimeout(() => {
      runtimeStatus = 'idle'
      cpuUsage = 5
      memoryUsage = 10
      gpuUsage = 0
      activeJobs = []
    }, 1500)
  }
  
  function getStatusColor(status) {
    switch (status) {
      case 'running': return 'text-secondary-400'
      case 'ready': return 'text-blue-400'
      case 'starting': case 'stopping': case 'switching': return 'text-yellow-400'
      case 'idle': return 'text-dark-400'
      case 'error': return 'text-red-400'
      default: return 'text-dark-400'
    }
  }
  
  function getUsageColor(usage) {
    if (usage > 80) return 'text-red-400'
    if (usage > 60) return 'text-yellow-400'
    return 'text-secondary-400'
  }
  
  // Simulate real-time updates
  setInterval(() => {
    if (runtimeStatus === 'running') {
      cpuUsage = Math.max(5, cpuUsage + (Math.random() - 0.5) * 10)
      memoryUsage = Math.max(10, memoryUsage + (Math.random() - 0.5) * 8)
      if (currentRuntime.specs.gpu !== 'None') {
        gpuUsage = Math.max(0, gpuUsage + (Math.random() - 0.5) * 15)
      }
    }
  }, 2000)
</script>

<div class="flex h-full">
  <!-- Runtime Selection -->
  <div class="w-96 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-lg font-semibold mb-4 flex items-center">
        <Server size={20} class="text-indigo-400 mr-2" />
        Runtime Environment
      </h2>
      
      <p class="text-sm text-dark-400 mb-4">
        Choose your compute environment for training and inference.
      </p>
    </div>
    
    <div class="flex-1 overflow-auto p-4">
      <div class="space-y-3">
        {#each runtimeOptions as runtime}
          <button
            class="w-full p-4 rounded-lg border border-dark-600 hover:border-indigo-500 transition-colors text-left {selectedRuntime === runtime.id ? 'border-indigo-500 bg-dark-700' : ''}"
            on:click={() => selectRuntime(runtime)}
          >
            <div class="flex items-center justify-between mb-3">
              <div class="flex items-center">
                <svelte:component this={runtime.icon} size={20} class="text-indigo-400 mr-2" />
                <span class="font-medium">{runtime.name}</span>
              </div>
              <span class="text-xs px-2 py-1 bg-indigo-600 rounded">{runtime.cost}</span>
            </div>
            
            <div class="text-xs text-dark-400 space-y-1">
              <p><strong>CPU:</strong> {runtime.specs.cpu}</p>
              <p><strong>Memory:</strong> {runtime.specs.memory}</p>
              <p><strong>GPU:</strong> {runtime.specs.gpu}</p>
              <p><strong>Storage:</strong> {runtime.specs.storage}</p>
            </div>
            
            <div class="mt-2 flex items-center justify-between">
              <span class="text-xs capitalize {runtime.status === 'available' ? 'text-secondary-400' : 'text-yellow-400'}">
                {runtime.status}
              </span>
              {#if runtime.type === 'cloud'}
                <span class="text-xs text-dark-500">Cloud</span>
              {:else}
                <span class="text-xs text-dark-500">Local</span>
              {/if}
            </div>
          </button>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Runtime Dashboard -->
  <div class="flex-1 flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-dark-700">
      <div class="flex items-center justify-between">
        <div>
          <h3 class="text-lg font-semibold">{currentRuntime.name}</h3>
          <p class="text-sm text-dark-400">
            Status: <span class="{getStatusColor(runtimeStatus)} capitalize">{runtimeStatus}</span>
          </p>
        </div>
        
        <div class="flex space-x-2">
          {#if runtimeStatus === 'idle' || runtimeStatus === 'ready'}
            <button class="btn-primary" on:click={startRuntime}>
              <Play size={16} class="mr-2" />
              Start Runtime
            </button>
          {:else if runtimeStatus === 'running'}
            <button class="btn-secondary" on:click={stopRuntime}>
              <Square size={16} class="mr-2" />
              Stop Runtime
            </button>
          {/if}
          
          <button class="btn-secondary">
            <Settings size={16} class="mr-2" />
            Configure
          </button>
        </div>
      </div>
    </div>
    
    <div class="flex-1 overflow-auto p-4">
      <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <!-- Resource Usage -->
        <div class="panel p-4">
          <h4 class="font-medium mb-4 flex items-center">
            <Monitor size={16} class="mr-2" />
            Resource Usage
          </h4>
          
          <div class="space-y-4">
            <!-- CPU Usage -->
            <div>
              <div class="flex justify-between text-sm mb-1">
                <span>CPU Usage</span>
                <span class="{getUsageColor(cpuUsage)}">{cpuUsage.toFixed(1)}%</span>
              </div>
              <div class="w-full bg-dark-700 rounded-full h-2">
                <div 
                  class="bg-gradient-to-r from-secondary-500 to-primary-500 h-2 rounded-full transition-all duration-500"
                  style="width: {cpuUsage}%"
                ></div>
              </div>
            </div>
            
            <!-- Memory Usage -->
            <div>
              <div class="flex justify-between text-sm mb-1">
                <span>Memory Usage</span>
                <span class="{getUsageColor(memoryUsage)}">{memoryUsage.toFixed(1)}%</span>
              </div>
              <div class="w-full bg-dark-700 rounded-full h-2">
                <div 
                  class="bg-gradient-to-r from-blue-500 to-purple-500 h-2 rounded-full transition-all duration-500"
                  style="width: {memoryUsage}%"
                ></div>
              </div>
            </div>
            
            <!-- GPU Usage -->
            {#if currentRuntime.specs.gpu !== 'None'}
              <div>
                <div class="flex justify-between text-sm mb-1">
                  <span>GPU Usage</span>
                  <span class="{getUsageColor(gpuUsage)}">{gpuUsage.toFixed(1)}%</span>
                </div>
                <div class="w-full bg-dark-700 rounded-full h-2">
                  <div 
                    class="bg-gradient-to-r from-yellow-500 to-red-500 h-2 rounded-full transition-all duration-500"
                    style="width: {gpuUsage}%"
                  ></div>
                </div>
              </div>
            {/if}
          </div>
        </div>
        
        <!-- Runtime Specifications -->
        <div class="panel p-4">
          <h4 class="font-medium mb-4 flex items-center">
            <Cpu size={16} class="mr-2" />
            Specifications
          </h4>
          
          <div class="space-y-3">
            <div class="flex justify-between">
              <span class="text-dark-400">CPU:</span>
              <span class="text-sm">{currentRuntime.specs.cpu}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">Memory:</span>
              <span class="text-sm">{currentRuntime.specs.memory}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">GPU:</span>
              <span class="text-sm">{currentRuntime.specs.gpu}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">Storage:</span>
              <span class="text-sm">{currentRuntime.specs.storage}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">Cost:</span>
              <span class="text-sm font-medium">{currentRuntime.cost}</span>
            </div>
          </div>
        </div>
        
        <!-- Active Jobs -->
        <div class="panel p-4">
          <h4 class="font-medium mb-4">Active Jobs</h4>
          
          {#if activeJobs.length > 0}
            <div class="space-y-2">
              {#each activeJobs as job}
                <div class="p-3 bg-dark-700 rounded">
                  <div class="flex items-center justify-between mb-2">
                    <span class="font-medium text-sm">{job.name}</span>
                    <span class="text-xs text-secondary-400">{job.status}</span>
                  </div>
                  <div class="w-full bg-dark-600 rounded-full h-1">
                    <div 
                      class="bg-secondary-500 h-1 rounded-full"
                      style="width: {job.progress}%"
                    ></div>
                  </div>
                </div>
              {/each}
            </div>
          {:else}
            <div class="text-center py-8">
              <Server size={32} class="text-dark-600 mx-auto mb-2" />
              <p class="text-sm text-dark-400">No active jobs</p>
            </div>
          {/if}
        </div>
        
        <!-- Environment Variables -->
        <div class="panel p-4">
          <h4 class="font-medium mb-4">Environment</h4>
          
          <div class="space-y-2 text-sm">
            <div class="flex justify-between">
              <span class="text-dark-400">Python:</span>
              <span>3.9.7</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">CUDA:</span>
              <span>{currentRuntime.specs.gpu !== 'None' ? '11.8' : 'Not available'}</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">PyTorch:</span>
              <span>2.0.1</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">TensorFlow:</span>
              <span>2.13.0</span>
            </div>
            <div class="flex justify-between">
              <span class="text-dark-400">Scikit-learn:</span>
              <span>1.3.0</span>
            </div>
          </div>
          
          <button class="btn-secondary w-full mt-4">
            <RefreshCw size={16} class="mr-2" />
            Update Packages
          </button>
        </div>
      </div>
    </div>
  </div>
</div>