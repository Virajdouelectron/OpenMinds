<script lang="ts">
  import { Shield, AlertTriangle, CheckCircle, TrendingUp, BarChart3, Eye, Download, RefreshCw } from 'lucide-svelte'
  
  let selectedDataset = null
  let profilingResults = null
  let isAnalyzing = false
  let analysisProgress = 0
  
  const datasets = [
    { id: 1, name: 'customer_data.csv', rows: 10000, columns: 15, size: '2.4 MB' },
    { id: 2, name: 'sales_transactions.csv', rows: 50000, columns: 8, size: '8.1 MB' },
    { id: 3, name: 'product_reviews.csv', rows: 25000, columns: 12, size: '5.2 MB' }
  ]
  
  function selectDataset(dataset) {
    selectedDataset = dataset
    startAnalysis()
  }
  
  function startAnalysis() {
    isAnalyzing = true
    analysisProgress = 0
    profilingResults = null
    
    const interval = setInterval(() => {
      analysisProgress += Math.random() * 20
      if (analysisProgress >= 100) {
        analysisProgress = 100
        isAnalyzing = false
        generateProfilingResults()
        clearInterval(interval)
      }
    }, 500)
  }
  
  function generateProfilingResults() {
    profilingResults = {
      overview: {
        totalRows: selectedDataset.rows,
        totalColumns: selectedDataset.columns,
        memoryUsage: selectedDataset.size,
        duplicateRows: Math.floor(selectedDataset.rows * 0.02),
        missingValues: Math.floor(selectedDataset.rows * selectedDataset.columns * 0.05)
      },
      dataQuality: {
        score: 87,
        issues: [
          { type: 'missing_values', severity: 'medium', count: 245, description: 'Missing values in age and income columns' },
          { type: 'duplicates', severity: 'low', count: 12, description: 'Duplicate customer records found' },
          { type: 'outliers', severity: 'high', count: 89, description: 'Extreme outliers in transaction amounts' },
          { type: 'inconsistent_format', severity: 'medium', count: 156, description: 'Inconsistent date formats' }
        ]
      },
      bias: {
        overallScore: 72,
        checks: [
          { name: 'Gender Representation', score: 85, status: 'good', details: 'Balanced gender distribution (52% F, 48% M)' },
          { name: 'Age Distribution', score: 45, status: 'warning', details: 'Skewed towards younger demographics (18-35)' },
          { name: 'Geographic Coverage', score: 78, status: 'good', details: 'Good coverage across regions' },
          { name: 'Income Levels', score: 60, status: 'warning', details: 'Under-representation of high-income groups' }
        ]
      },
      distribution: {
        numerical: [
          { column: 'age', mean: 34.2, std: 12.8, skewness: 0.45, kurtosis: -0.12, outliers: 23 },
          { column: 'income', mean: 65000, std: 25000, skewness: 1.2, kurtosis: 2.1, outliers: 45 },
          { column: 'transaction_amount', mean: 127.50, std: 89.30, skewness: 2.8, kurtosis: 8.9, outliers: 89 }
        ],
        categorical: [
          { column: 'gender', categories: { 'Female': 5200, 'Male': 4800 }, entropy: 0.99 },
          { column: 'region', categories: { 'North': 2500, 'South': 2800, 'East': 2200, 'West': 2500 }, entropy: 1.38 },
          { column: 'product_category', categories: { 'Electronics': 3000, 'Clothing': 2500, 'Books': 2000, 'Home': 2500 }, entropy: 1.95 }
        ]
      },
      correlations: [
        { feature1: 'age', feature2: 'income', correlation: 0.67, strength: 'strong' },
        { feature1: 'income', feature2: 'transaction_amount', correlation: 0.45, strength: 'moderate' },
        { feature1: 'age', feature2: 'product_category', correlation: 0.23, strength: 'weak' }
      ],
      recommendations: [
        { type: 'data_cleaning', priority: 'high', action: 'Handle missing values in age and income columns using median imputation' },
        { type: 'bias_mitigation', priority: 'medium', action: 'Consider stratified sampling to balance age distribution' },
        { type: 'outlier_treatment', priority: 'high', action: 'Investigate and potentially cap extreme transaction amounts' },
        { type: 'feature_engineering', priority: 'low', action: 'Create age groups to reduce granularity and improve model stability' }
      ]
    }
  }
  
  function getScoreColor(score) {
    if (score >= 80) return 'text-secondary-400'
    if (score >= 60) return 'text-yellow-400'
    return 'text-red-400'
  }
  
  function getSeverityColor(severity) {
    switch (severity) {
      case 'high': return 'text-red-400'
      case 'medium': return 'text-yellow-400'
      case 'low': return 'text-secondary-400'
      default: return 'text-dark-400'
    }
  }
  
  function getStatusIcon(status) {
    switch (status) {
      case 'good': return CheckCircle
      case 'warning': return AlertTriangle
      case 'error': return AlertTriangle
      default: return CheckCircle
    }
  }
  
  function exportReport() {
    if (!profilingResults) return
    
    const report = {
      dataset: selectedDataset.name,
      timestamp: new Date().toISOString(),
      results: profilingResults
    }
    
    const blob = new Blob([JSON.stringify(report, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `data-profile-${selectedDataset.name}.json`
    a.click()
    URL.revokeObjectURL(url)
  }
</script>

<div class="flex h-full">
  <!-- Dataset Selection -->
  <div class="w-80 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <h2 class="text-lg font-semibold mb-4 flex items-center">
        <Shield size={20} class="text-cyan-400 mr-2" />
        Smart Data Profiler
      </h2>
      
      <p class="text-sm text-dark-400 mb-4">
        Analyze data quality, detect bias, and identify patterns automatically.
      </p>
    </div>
    
    <div class="flex-1 overflow-auto p-4">
      <h3 class="font-medium mb-3">Select Dataset</h3>
      
      <div class="space-y-2">
        {#each datasets as dataset}
          <button
            class="w-full p-3 rounded-lg border border-dark-600 hover:border-cyan-500 transition-colors text-left {selectedDataset?.id === dataset.id ? 'border-cyan-500 bg-dark-700' : ''}"
            on:click={() => selectDataset(dataset)}
          >
            <div class="font-medium text-sm mb-1">{dataset.name}</div>
            <div class="text-xs text-dark-400 space-y-1">
              <p>{dataset.rows.toLocaleString()} rows • {dataset.columns} columns</p>
              <p>Size: {dataset.size}</p>
            </div>
          </button>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Analysis Results -->
  <div class="flex-1 flex flex-col">
    {#if selectedDataset}
      <!-- Header -->
      <div class="p-4 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold">Data Profile: {selectedDataset.name}</h3>
            <p class="text-sm text-dark-400">Comprehensive data quality and bias analysis</p>
          </div>
          
          <div class="flex space-x-2">
            <button 
              class="btn-secondary"
              on:click={() => startAnalysis()}
              disabled={isAnalyzing}
            >
              <RefreshCw size={16} class="mr-2" />
              Re-analyze
            </button>
            <button 
              class="btn-primary"
              on:click={exportReport}
              disabled={!profilingResults}
            >
              <Download size={16} class="mr-2" />
              Export Report
            </button>
          </div>
        </div>
      </div>
      
      <div class="flex-1 overflow-auto p-4">
        {#if isAnalyzing}
          <!-- Analysis Progress -->
          <div class="panel p-6 mb-6">
            <div class="flex items-center mb-4">
              <Shield size={20} class="text-cyan-400 mr-2" />
              <span class="font-medium">Analyzing dataset...</span>
            </div>
            
            <div class="mb-4">
              <div class="flex justify-between text-sm mb-1">
                <span>Progress</span>
                <span>{analysisProgress.toFixed(0)}%</span>
              </div>
              <div class="w-full bg-dark-700 rounded-full h-2">
                <div 
                  class="bg-cyan-500 h-2 rounded-full transition-all duration-300"
                  style="width: {analysisProgress}%"
                ></div>
              </div>
            </div>
            
            <div class="text-sm text-dark-400">
              Running comprehensive analysis including quality checks, bias detection, and pattern recognition...
            </div>
          </div>
        {:else if profilingResults}
          <!-- Results -->
          <div class="space-y-6">
            <!-- Overview -->
            <div class="panel p-4">
              <h4 class="font-medium mb-3 flex items-center">
                <BarChart3 size={16} class="mr-2" />
                Dataset Overview
              </h4>
              
              <div class="grid grid-cols-2 md:grid-cols-4 gap-4">
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-cyan-400">{profilingResults.overview.totalRows.toLocaleString()}</div>
                  <div class="text-xs text-dark-400">Total Rows</div>
                </div>
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-cyan-400">{profilingResults.overview.totalColumns}</div>
                  <div class="text-xs text-dark-400">Columns</div>
                </div>
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-red-400">{profilingResults.overview.duplicateRows}</div>
                  <div class="text-xs text-dark-400">Duplicates</div>
                </div>
                <div class="text-center p-3 bg-dark-700 rounded">
                  <div class="text-lg font-semibold text-yellow-400">{profilingResults.overview.missingValues}</div>
                  <div class="text-xs text-dark-400">Missing Values</div>
                </div>
              </div>
            </div>
            
            <!-- Data Quality Score -->
            <div class="panel p-4">
              <h4 class="font-medium mb-3 flex items-center">
                <CheckCircle size={16} class="mr-2" />
                Data Quality Score
              </h4>
              
              <div class="flex items-center mb-4">
                <div class="text-3xl font-bold {getScoreColor(profilingResults.dataQuality.score)} mr-4">
                  {profilingResults.dataQuality.score}/100
                </div>
                <div class="flex-1">
                  <div class="w-full bg-dark-700 rounded-full h-3">
                    <div 
                      class="bg-gradient-to-r from-red-500 via-yellow-500 to-secondary-500 h-3 rounded-full"
                      style="width: {profilingResults.dataQuality.score}%"
                    ></div>
                  </div>
                </div>
              </div>
              
              <div class="space-y-2">
                {#each profilingResults.dataQuality.issues as issue}
                  <div class="flex items-center justify-between p-2 bg-dark-700 rounded">
                    <div class="flex items-center">
                      <AlertTriangle size={14} class="{getSeverityColor(issue.severity)} mr-2" />
                      <span class="text-sm">{issue.description}</span>
                    </div>
                    <span class="text-xs text-dark-400">{issue.count} issues</span>
                  </div>
                {/each}
              </div>
            </div>
            
            <!-- Bias Analysis -->
            <div class="panel p-4">
              <h4 class="font-medium mb-3 flex items-center">
                <Shield size={16} class="mr-2" />
                Bias Analysis
              </h4>
              
              <div class="mb-4">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-sm">Overall Bias Score</span>
                  <span class="font-semibold {getScoreColor(profilingResults.bias.overallScore)}">{profilingResults.bias.overallScore}/100</span>
                </div>
                <div class="w-full bg-dark-700 rounded-full h-2">
                  <div 
                    class="bg-gradient-to-r from-red-500 via-yellow-500 to-secondary-500 h-2 rounded-full"
                    style="width: {profilingResults.bias.overallScore}%"
                  ></div>
                </div>
              </div>
              
              <div class="space-y-3">
                {#each profilingResults.bias.checks as check}
                  <div class="p-3 bg-dark-700 rounded">
                    <div class="flex items-center justify-between mb-2">
                      <div class="flex items-center">
                        <svelte:component this={getStatusIcon(check.status)} size={14} class="mr-2 {check.status === 'good' ? 'text-secondary-400' : 'text-yellow-400'}" />
                        <span class="font-medium text-sm">{check.name}</span>
                      </div>
                      <span class="text-sm {getScoreColor(check.score)}">{check.score}/100</span>
                    </div>
                    <p class="text-xs text-dark-400">{check.details}</p>
                  </div>
                {/each}
              </div>
            </div>
            
            <!-- Recommendations -->
            <div class="panel p-4">
              <h4 class="font-medium mb-3 flex items-center">
                <TrendingUp size={16} class="mr-2" />
                Recommendations
              </h4>
              
              <div class="space-y-3">
                {#each profilingResults.recommendations as rec}
                  <div class="p-3 bg-dark-700 rounded">
                    <div class="flex items-center justify-between mb-2">
                      <span class="font-medium text-sm capitalize">{rec.type.replace('_', ' ')}</span>
                      <span class="px-2 py-1 text-xs rounded {rec.priority === 'high' ? 'bg-red-600' : rec.priority === 'medium' ? 'bg-yellow-600' : 'bg-secondary-600'}">
                        {rec.priority} priority
                      </span>
                    </div>
                    <p class="text-sm text-dark-300">{rec.action}</p>
                  </div>
                {/each}
              </div>
            </div>
          </div>
        {/if}
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <Shield size={48} class="text-dark-600 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-dark-300 mb-2">Smart Data Profiling</h3>
          <p class="text-dark-500 mb-4">Select a dataset to start comprehensive analysis</p>
          <div class="text-sm text-dark-400">
            <p>✓ Data quality assessment</p>
            <p>✓ Bias detection</p>
            <p>✓ Pattern recognition</p>
            <p>✓ Automated recommendations</p>
          </div>
        </div>
      </div>
    {/if}
  </div>
</div>