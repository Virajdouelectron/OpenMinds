<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { slide, fade } from 'svelte/transition';
  
  export let modelType: 'classification' | 'regression' | 'clustering' | 'neural_network' = 'classification';
  export let config: Record<string, any> = {};
  export let isTraining = false;
  export let trainingProgress = 0;
  export let validationMetrics: Record<string, number> | null = null;
  
  const dispatch = createEventDispatcher<{
    configChange: { config: Record<string, any> };
    startTraining: { config: Record<string, any> };
    stopTraining: void;
    exportModel: void;
    loadModel: { file: File };
  }>();
  
  // Model configurations for different types
  const modelConfigs = {
    classification: {
      algorithm: 'random_forest',
      algorithms: [
        { value: 'random_forest', label: 'Random Forest', params: ['n_estimators', 'max_depth', 'min_samples_split'] },
        { value: 'logistic_regression', label: 'Logistic Regression', params: ['C', 'max_iter', 'solver'] },
        { value: 'svm', label: 'Support Vector Machine', params: ['C', 'kernel', 'gamma'] },
        { value: 'gradient_boosting', label: 'Gradient Boosting', params: ['n_estimators', 'learning_rate', 'max_depth'] },
        { value: 'naive_bayes', label: 'Naive Bayes', params: ['alpha'] },
      ],
      defaultParams: {
        random_forest: { n_estimators: 100, max_depth: 10, min_samples_split: 2 },
        logistic_regression: { C: 1.0, max_iter: 1000, solver: 'lbfgs' },
        svm: { C: 1.0, kernel: 'rbf', gamma: 'scale' },
        gradient_boosting: { n_estimators: 100, learning_rate: 0.1, max_depth: 3 },
        naive_bayes: { alpha: 1.0 },
      }
    },
    regression: {
      algorithm: 'linear_regression',
      algorithms: [
        { value: 'linear_regression', label: 'Linear Regression', params: ['fit_intercept', 'normalize'] },
        { value: 'ridge', label: 'Ridge Regression', params: ['alpha', 'fit_intercept'] },
        { value: 'lasso', label: 'Lasso Regression', params: ['alpha', 'max_iter'] },
        { value: 'random_forest', label: 'Random Forest', params: ['n_estimators', 'max_depth', 'min_samples_split'] },
        { value: 'svr', label: 'Support Vector Regression', params: ['C', 'kernel', 'gamma'] },
      ],
      defaultParams: {
        linear_regression: { fit_intercept: true, normalize: false },
        ridge: { alpha: 1.0, fit_intercept: true },
        lasso: { alpha: 1.0, max_iter: 1000 },
        random_forest: { n_estimators: 100, max_depth: 10, min_samples_split: 2 },
        svr: { C: 1.0, kernel: 'rbf', gamma: 'scale' },
      }
    },
    clustering: {
      algorithm: 'kmeans',
      algorithms: [
        { value: 'kmeans', label: 'K-Means', params: ['n_clusters', 'init', 'max_iter'] },
        { value: 'dbscan', label: 'DBSCAN', params: ['eps', 'min_samples'] },
        { value: 'hierarchical', label: 'Hierarchical', params: ['n_clusters', 'linkage'] },
        { value: 'gaussian_mixture', label: 'Gaussian Mixture', params: ['n_components', 'covariance_type'] },
      ],
      defaultParams: {
        kmeans: { n_clusters: 3, init: 'k-means++', max_iter: 300 },
        dbscan: { eps: 0.5, min_samples: 5 },
        hierarchical: { n_clusters: 3, linkage: 'ward' },
        gaussian_mixture: { n_components: 3, covariance_type: 'full' },
      }
    },
    neural_network: {
      algorithm: 'mlp',
      algorithms: [
        { value: 'mlp', label: 'Multi-Layer Perceptron', params: ['hidden_layer_sizes', 'activation', 'learning_rate_init'] },
        { value: 'cnn', label: 'Convolutional Neural Network', params: ['filters', 'kernel_size', 'pool_size'] },
        { value: 'rnn', label: 'Recurrent Neural Network', params: ['units', 'dropout', 'recurrent_dropout'] },
      ],
      defaultParams: {
        mlp: { hidden_layer_sizes: [100, 50], activation: 'relu', learning_rate_init: 0.001 },
        cnn: { filters: [32, 64], kernel_size: 3, pool_size: 2 },
        rnn: { units: 50, dropout: 0.2, recurrent_dropout: 0.2 },
      }
    }
  };
  
  let currentConfig = modelConfigs[modelType];
  let selectedAlgorithm = config.algorithm || currentConfig.algorithm;
  let parameters = { ...currentConfig.defaultParams[selectedAlgorithm], ...config };
  let showAdvanced = false;
  let fileInput: HTMLInputElement;
  
  // Update config when model type changes
  $: {
    currentConfig = modelConfigs[modelType];
    if (!currentConfig.algorithms.find(a => a.value === selectedAlgorithm)) {
      selectedAlgorithm = currentConfig.algorithm;
      parameters = { ...currentConfig.defaultParams[selectedAlgorithm] };
      updateConfig();
    }
  }
  
  // Update parameters when algorithm changes
  $: {
    if (selectedAlgorithm && currentConfig.defaultParams[selectedAlgorithm]) {
      parameters = { 
        ...currentConfig.defaultParams[selectedAlgorithm], 
        ...parameters 
      };
      updateConfig();
    }
  }
  
  function updateConfig() {
    const newConfig = {
      algorithm: selectedAlgorithm,
      ...parameters,
      modelType
    };
    dispatch('configChange', { config: newConfig });
  }
  
  function handleParameterChange(param: string, value: any) {
    parameters[param] = value;
    updateConfig();
  }
  
  function startTraining() {
    dispatch('startTraining', { config: { algorithm: selectedAlgorithm, ...parameters, modelType } });
  }
  
  function stopTraining() {
    dispatch('stopTraining');
  }
  
  function exportModel() {
    dispatch('exportModel');
  }
  
  function handleFileLoad(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files && target.files[0]) {
      dispatch('loadModel', { file: target.files[0] });
    }
  }
  
  function getParameterInput(param: string, value: any) {
    // Determine input type based on parameter name and value
    if (typeof value === 'boolean') {
      return 'checkbox';
    } else if (typeof value === 'number') {
      return 'number';
    } else if (Array.isArray(value)) {
      return 'array';
    } else if (param.includes('solver') || param.includes('kernel') || param.includes('init') || param.includes('linkage') || param.includes('activation')) {
      return 'select';
    } else {
      return 'text';
    }
  }
  
  function getSelectOptions(param: string) {
    const options = {
      solver: ['lbfgs', 'liblinear', 'newton-cg', 'sag', 'saga'],
      kernel: ['linear', 'poly', 'rbf', 'sigmoid'],
      init: ['k-means++', 'random'],
      linkage: ['ward', 'complete', 'average', 'single'],
      activation: ['relu', 'tanh', 'logistic', 'identity'],
      covariance_type: ['full', 'tied', 'diag', 'spherical']
    };
    return options[param] || [];
  }
</script>

<div class="model-config-panel bg-surface-900 border border-surface-700 rounded-lg">
  <!-- Header -->
  <div class="panel-header p-4 border-b border-surface-700">
    <div class="flex items-center justify-between">
      <h3 class="text-lg font-semibold text-surface-100 flex items-center">
        <svg class="w-5 h-5 mr-2 text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z" />
        </svg>
        Model Configuration
      </h3>
      
      <div class="flex items-center space-x-2">
        <select 
          bind:value={modelType}
          class="input-field text-sm"
          disabled={isTraining}
        >
          <option value="classification">Classification</option>
          <option value="regression">Regression</option>
          <option value="clustering">Clustering</option>
          <option value="neural_network">Neural Network</option>
        </select>
      </div>
    </div>
  </div>
  
  <!-- Configuration Form -->
  <div class="panel-content p-4 space-y-6">
    <!-- Algorithm Selection -->
    <div class="config-section">
      <label class="block text-sm font-medium text-surface-300 mb-2">Algorithm</label>
      <select 
        bind:value={selectedAlgorithm}
        class="input-field w-full"
        disabled={isTraining}
      >
        {#each currentConfig.algorithms as algorithm}
          <option value={algorithm.value}>{algorithm.label}</option>
        {/each}
      </select>
    </div>
    
    <!-- Parameters -->
    <div class="config-section">
      <div class="flex items-center justify-between mb-3">
        <label class="text-sm font-medium text-surface-300">Parameters</label>
        <button
          class="text-xs text-primary-400 hover:text-primary-300 transition-colors"
          on:click={() => showAdvanced = !showAdvanced}
        >
          {showAdvanced ? 'Hide' : 'Show'} Advanced
        </button>
      </div>
      
      <div class="space-y-4">
        {#each currentConfig.algorithms.find(a => a.value === selectedAlgorithm)?.params || [] as param}
          <div class="parameter-input">
            <label class="block text-sm text-surface-400 mb-1 capitalize">
              {param.replace(/_/g, ' ')}
            </label>
            
            {#if getParameterInput(param, parameters[param]) === 'checkbox'}
              <label class="flex items-center">
                <input
                  type="checkbox"
                  checked={parameters[param]}
                  on:change={(e) => handleParameterChange(param, e.target.checked)}
                  disabled={isTraining}
                  class="mr-2"
                />
                <span class="text-sm text-surface-300">Enable {param.replace(/_/g, ' ')}</span>
              </label>
            {:else if getParameterInput(param, parameters[param]) === 'number'}
              <input
                type="number"
                value={parameters[param]}
                on:input={(e) => handleParameterChange(param, parseFloat(e.target.value) || 0)}
                disabled={isTraining}
                class="input-field w-full"
                step={param.includes('rate') || param.includes('alpha') || param.includes('eps') ? '0.001' : '1'}
                min="0"
              />
            {:else if getParameterInput(param, parameters[param]) === 'select'}
              <select
                value={parameters[param]}
                on:change={(e) => handleParameterChange(param, e.target.value)}
                disabled={isTraining}
                class="input-field w-full"
              >
                {#each getSelectOptions(param) as option}
                  <option value={option}>{option}</option>
                {/each}
              </select>
            {:else if getParameterInput(param, parameters[param]) === 'array'}
              <input
                type="text"
                value={Array.isArray(parameters[param]) ? parameters[param].join(', ') : parameters[param]}
                on:input={(e) => {
                  const value = e.target.value.split(',').map(v => parseInt(v.trim())).filter(v => !isNaN(v));
                  handleParameterChange(param, value);
                }}
                disabled={isTraining}
                class="input-field w-full"
                placeholder="e.g., 100, 50, 25"
              />
            {:else}
              <input
                type="text"
                value={parameters[param]}
                on:input={(e) => handleParameterChange(param, e.target.value)}
                disabled={isTraining}
                class="input-field w-full"
              />
            {/if}
          </div>
        {/each}
      </div>
    </div>
    
    <!-- Training Progress -->
    {#if isTraining}
      <div class="training-progress" transition:slide={{ duration: 200 }}>
        <div class="flex items-center justify-between mb-2">
          <span class="text-sm font-medium text-surface-300">Training Progress</span>
          <span class="text-sm text-surface-400">{trainingProgress.toFixed(1)}%</span>
        </div>
        <div class="w-full bg-surface-700 rounded-full h-2">
          <div 
            class="bg-primary-500 h-2 rounded-full transition-all duration-300"
            style="width: {trainingProgress}%"
          ></div>
        </div>
      </div>
    {/if}
    
    <!-- Validation Metrics -->
    {#if validationMetrics}
      <div class="validation-metrics" transition:fade={{ duration: 200 }}>
        <h4 class="text-sm font-medium text-surface-300 mb-3">Validation Metrics</h4>
        <div class="grid grid-cols-2 gap-3">
          {#each Object.entries(validationMetrics) as [metric, value]}
            <div class="metric-item bg-surface-800 p-3 rounded">
              <div class="text-xs text-surface-400 uppercase tracking-wide">{metric}</div>
              <div class="text-lg font-semibold text-surface-100">
                {typeof value === 'number' ? value.toFixed(4) : value}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
  
  <!-- Actions -->
  <div class="panel-actions p-4 border-t border-surface-700 space-y-3">
    <!-- Training Controls -->
    <div class="flex space-x-2">
      {#if isTraining}
        <button
          class="btn-secondary flex-1"
          on:click={stopTraining}
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 10h6v4H9z" />
          </svg>
          Stop Training
        </button>
      {:else}
        <button
          class="btn-primary flex-1"
          on:click={startTraining}
        >
          <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h1m4 0h1m-6 4h1m4 0h1m2-7a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
          Start Training
        </button>
      {/if}
    </div>
    
    <!-- Model Management -->
    <div class="flex space-x-2">
      <button
        class="btn-secondary flex-1"
        on:click={exportModel}
        disabled={isTraining}
      >
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
        </svg>
        Export Model
      </button>
      
      <button
        class="btn-secondary flex-1"
        on:click={() => fileInput?.click()}
        disabled={isTraining}
      >
        <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
        </svg>
        Load Model
      </button>
      
      <input
        bind:this={fileInput}
        type="file"
        accept=".pkl,.joblib,.h5,.json"
        on:change={handleFileLoad}
        class="hidden"
      />
    </div>
  </div>
</div>

<style>
  .parameter-input {
    transition: opacity 0.2s ease-in-out;
  }
  
  .parameter-input:hover {
    opacity: 0.9;
  }
  
  .metric-item {
    transition: transform 0.2s ease-in-out;
  }
  
  .metric-item:hover {
    transform: translateY(-1px);
  }
</style>