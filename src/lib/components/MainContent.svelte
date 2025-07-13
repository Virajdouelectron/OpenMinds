<script lang="ts">
  import NotebookView from './views/NotebookView.svelte'
  import DatasetView from './views/DatasetView.svelte'
  import ModelView from './views/ModelView.svelte'
  import AutoMLView from './views/AutoMLView.svelte'
  import ExperimentView from './views/ExperimentView.svelte'
  import AICopilotView from './views/AICopilotView.svelte'
  import VisualBuilderView from './views/VisualBuilderView.svelte'
  import DataProfilerView from './views/DataProfilerView.svelte'
  import RuntimeView from './views/RuntimeView.svelte'
  import DeploymentView from './views/DeploymentView.svelte'
  import TerminalView from './views/TerminalView.svelte'
  import VersionControlView from './views/VersionControlView.svelte'
  import SettingsView from './views/SettingsView.svelte'
  
  export let activeView: string
</script>

<div class="flex-1 flex flex-col overflow-hidden">
  {#if activeView === 'notebooks'}
    <NotebookView />
  {:else if activeView === 'datasets'}
    <DatasetView />
  {:else if activeView === 'models'}
    <ModelView />
  {:else if activeView === 'automl'}
    <AutoMLView />
  {:else if activeView === 'experiments'}
    <ExperimentView />
  {:else if activeView === 'ai-copilot'}
    <AICopilotView />
  {:else if activeView === 'visual-builder'}
    <VisualBuilderView />
  {:else if activeView === 'data-profiler'}
    <DataProfilerView />
  {:else if activeView === 'runtime'}
    <RuntimeView />
  {:else if activeView === 'deployment'}
    <DeploymentView />
  {:else if activeView === 'terminal'}
    <TerminalView />
  {:else if activeView === 'version-control'}
    <VersionControlView />
  {:else if activeView === 'settings'}
    <SettingsView />
  {/if}
</div>