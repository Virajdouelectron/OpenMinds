<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';
  
  export let data: any[] = [];
  export let type: 'bar' | 'line' | 'scatter' | 'pie' | 'histogram' | 'heatmap' = 'bar';
  export let title = '';
  export let xAxis = '';
  export let yAxis = '';
  export let width = '100%';
  export let height = '400px';
  export let theme: 'light' | 'dark' = 'dark';
  export let interactive = true;
  export let showLegend = true;
  export let colors: string[] = [];
  
  let container: HTMLElement;
  let plotlyLoaded = false;
  let plotlyError: string | null = null;
  
  // Default color schemes
  const defaultColors = {
    dark: ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#84cc16', '#f97316'],
    light: ['#1d4ed8', '#059669', '#d97706', '#dc2626', '#7c3aed', '#0891b2', '#65a30d', '#ea580c']
  };
  
  onMount(async () => {
    try {
      // Load Plotly.js from CDN
      if (!window.Plotly) {
        const script = document.createElement('script');
        script.src = 'https://cdn.plot.ly/plotly-2.26.0.min.js';
        script.onload = () => {
          plotlyLoaded = true;
          renderPlot();
        };
        script.onerror = () => {
          plotlyError = 'Failed to load Plotly.js';
        };
        document.head.appendChild(script);
      } else {
        plotlyLoaded = true;
        renderPlot();
      }
    } catch (error) {
      plotlyError = error instanceof Error ? error.message : 'Unknown error';
    }
  });
  
  onDestroy(() => {
    if (container && window.Plotly) {
      window.Plotly.purge(container);
    }
  });
  
  function renderPlot() {
    if (!plotlyLoaded || !container || !data.length) return;
    
    const plotData = generatePlotData();
    const layout = generateLayout();
    const config = generateConfig();
    
    window.Plotly.newPlot(container, plotData, layout, config);
  }
  
  function generatePlotData() {
    const colorScheme = colors.length > 0 ? colors : defaultColors[theme];
    
    switch (type) {
      case 'bar':
        return [{
          x: data.map(d => d[xAxis]),
          y: data.map(d => d[yAxis]),
          type: 'bar',
          marker: { color: colorScheme[0] },
          name: yAxis
        }];
        
      case 'line':
        return [{
          x: data.map(d => d[xAxis]),
          y: data.map(d => d[yAxis]),
          type: 'scatter',
          mode: 'lines+markers',
          line: { color: colorScheme[0] },
          marker: { color: colorScheme[0] },
          name: yAxis
        }];
        
      case 'scatter':
        return [{
          x: data.map(d => d[xAxis]),
          y: data.map(d => d[yAxis]),
          type: 'scatter',
          mode: 'markers',
          marker: { 
            color: colorScheme[0],
            size: 8,
            opacity: 0.7
          },
          name: `${yAxis} vs ${xAxis}`
        }];
        
      case 'pie':
        const pieData = data.reduce((acc, d) => {
          const key = d[xAxis];
          acc[key] = (acc[key] || 0) + (d[yAxis] || 1);
          return acc;
        }, {});
        
        return [{
          labels: Object.keys(pieData),
          values: Object.values(pieData),
          type: 'pie',
          marker: { colors: colorScheme },
          textinfo: 'label+percent',
          textposition: 'outside'
        }];
        
      case 'histogram':
        return [{
          x: data.map(d => d[xAxis]),
          type: 'histogram',
          marker: { color: colorScheme[0] },
          name: xAxis
        }];
        
      case 'heatmap':
        // Assuming data is in matrix format or can be converted
        const matrix = generateHeatmapMatrix();
        return [{
          z: matrix.values,
          x: matrix.xLabels,
          y: matrix.yLabels,
          type: 'heatmap',
          colorscale: theme === 'dark' ? 'Viridis' : 'Blues',
          showscale: true
        }];
        
      default:
        return [];
    }
  }
  
  function generateHeatmapMatrix() {
    // Simple correlation matrix generation
    const numericColumns = Object.keys(data[0]).filter(key => 
      typeof data[0][key] === 'number'
    );
    
    const matrix = numericColumns.map(col1 => 
      numericColumns.map(col2 => {
        if (col1 === col2) return 1;
        return calculateCorrelation(
          data.map(d => d[col1]),
          data.map(d => d[col2])
        );
      })
    );
    
    return {
      values: matrix,
      xLabels: numericColumns,
      yLabels: numericColumns
    };
  }
  
  function calculateCorrelation(x: number[], y: number[]): number {
    const n = x.length;
    const sumX = x.reduce((a, b) => a + b, 0);
    const sumY = y.reduce((a, b) => a + b, 0);
    const sumXY = x.reduce((sum, xi, i) => sum + xi * y[i], 0);
    const sumX2 = x.reduce((sum, xi) => sum + xi * xi, 0);
    const sumY2 = y.reduce((sum, yi) => sum + yi * yi, 0);
    
    const numerator = n * sumXY - sumX * sumY;
    const denominator = Math.sqrt((n * sumX2 - sumX * sumX) * (n * sumY2 - sumY * sumY));
    
    return denominator === 0 ? 0 : numerator / denominator;
  }
  
  function generateLayout() {
    const isDark = theme === 'dark';
    
    return {
      title: {
        text: title,
        font: {
          color: isDark ? '#f1f5f9' : '#1e293b',
          size: 16
        }
      },
      xaxis: {
        title: xAxis,
        color: isDark ? '#94a3b8' : '#64748b',
        gridcolor: isDark ? '#334155' : '#e2e8f0',
        zerolinecolor: isDark ? '#475569' : '#cbd5e1'
      },
      yaxis: {
        title: yAxis,
        color: isDark ? '#94a3b8' : '#64748b',
        gridcolor: isDark ? '#334155' : '#e2e8f0',
        zerolinecolor: isDark ? '#475569' : '#cbd5e1'
      },
      plot_bgcolor: isDark ? '#0f172a' : '#ffffff',
      paper_bgcolor: isDark ? '#0f172a' : '#ffffff',
      font: {
        color: isDark ? '#f1f5f9' : '#1e293b'
      },
      showlegend: showLegend && type !== 'pie',
      legend: {
        bgcolor: isDark ? '#1e293b' : '#f8fafc',
        bordercolor: isDark ? '#475569' : '#cbd5e1',
        borderwidth: 1
      },
      margin: { t: 50, r: 50, b: 50, l: 50 },
      autosize: true
    };
  }
  
  function generateConfig() {
    return {
      responsive: true,
      displayModeBar: interactive,
      modeBarButtonsToRemove: ['pan2d', 'lasso2d', 'select2d'],
      displaylogo: false,
      toImageButtonOptions: {
        format: 'png',
        filename: title || 'plot',
        height: 500,
        width: 700,
        scale: 1
      }
    };
  }
  
  // Re-render when data or settings change
  $: if (plotlyLoaded && container) {
    renderPlot();
  }
  
  // Export functions
  export function exportPlot(format: 'png' | 'svg' | 'pdf' = 'png') {
    if (!container || !window.Plotly) return;
    
    window.Plotly.downloadImage(container, {
      format,
      width: 800,
      height: 600,
      filename: title || 'plot'
    });
  }
  
  export function getPlotData() {
    return generatePlotData();
  }
  
  export function getPlotLayout() {
    return generateLayout();
  }
</script>

<div class="data-visualization" style="width: {width}; height: {height};">
  {#if plotlyError}
    <div 
      class="error-state flex items-center justify-center h-full bg-surface-900 border border-red-500 rounded-lg"
      transition:fade={{ duration: 200 }}
    >
      <div class="text-center">
        <svg class="w-12 h-12 text-red-400 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
        </svg>
        <p class="text-red-400 font-medium">Visualization Error</p>
        <p class="text-surface-400 text-sm mt-1">{plotlyError}</p>
      </div>
    </div>
  {:else if !plotlyLoaded}
    <div 
      class="loading-state flex items-center justify-center h-full bg-surface-900 border border-surface-700 rounded-lg"
      transition:fade={{ duration: 200 }}
    >
      <div class="text-center">
        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-primary-500 mx-auto mb-4"></div>
        <p class="text-surface-300">Loading visualization...</p>
      </div>
    </div>
  {:else if !data.length}
    <div 
      class="empty-state flex items-center justify-center h-full bg-surface-900 border border-surface-700 rounded-lg"
      transition:fade={{ duration: 200 }}
    >
      <div class="text-center">
        <svg class="w-12 h-12 text-surface-500 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
        </svg>
        <p class="text-surface-400">No data to visualize</p>
        <p class="text-surface-500 text-sm mt-1">Upload data or run a cell to generate visualizations</p>
      </div>
    </div>
  {:else}
    <div 
      bind:this={container} 
      class="plot-container w-full h-full rounded-lg overflow-hidden border border-surface-700"
      transition:fade={{ duration: 200 }}
    />
  {/if}
</div>

<style>
  .data-visualization {
    position: relative;
  }
  
  .plot-container {
    background: theme('colors.surface.900');
  }
  
  :global(.plotly .modebar) {
    background: theme('colors.surface.800') !important;
    border: 1px solid theme('colors.surface.600') !important;
  }
  
  :global(.plotly .modebar-btn) {
    color: theme('colors.surface.300') !important;
  }
  
  :global(.plotly .modebar-btn:hover) {
    background: theme('colors.surface.700') !important;
    color: theme('colors.surface.100') !important;
  }
  
  :global(.plotly .modebar-btn.active) {
    background: theme('colors.primary.600') !important;
    color: white !important;
  }
</style>