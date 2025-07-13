<script lang="ts">
  import { GitBranch, GitCommit, GitMerge, Upload, Download, Eye, Plus, Trash2 } from 'lucide-svelte'
  
  let repositories = [
    {
      id: 1,
      name: 'ml-experiments',
      url: 'https://github.com/user/ml-experiments',
      branch: 'main',
      status: 'clean',
      lastCommit: '2 hours ago',
      commits: 15
    },
    {
      id: 2,
      name: 'data-analysis',
      url: 'https://github.com/user/data-analysis',
      branch: 'feature/new-model',
      status: 'modified',
      lastCommit: '1 day ago',
      commits: 8
    }
  ]
  
  let selectedRepo = repositories[0]
  let commitMessage = ''
  let newRepoUrl = ''
  let showCloneDialog = false
  
  let fileChanges = [
    {
      file: 'notebooks/sales_analysis.ipynb',
      status: 'modified',
      additions: 15,
      deletions: 3
    },
    {
      file: 'datasets/customer_data.csv',
      status: 'added',
      additions: 1000,
      deletions: 0
    },
    {
      file: 'models/random_forest_model.pkl',
      status: 'deleted',
      additions: 0,
      deletions: 1
    }
  ]
  
  let commitHistory = [
    {
      id: 'a1b2c3d',
      message: 'Add customer segmentation analysis',
      author: 'John Doe',
      date: '2024-01-15 14:30:00',
      files: 3
    },
    {
      id: 'e4f5g6h',
      message: 'Update sales prediction model',
      author: 'Jane Smith',
      date: '2024-01-15 10:15:00',
      files: 2
    },
    {
      id: 'i7j8k9l',
      message: 'Initial project setup',
      author: 'John Doe',
      date: '2024-01-14 16:45:00',
      files: 5
    }
  ]
  
  function selectRepository(repo) {
    selectedRepo = repo
  }
  
  function commitChanges() {
    if (!commitMessage.trim()) return
    
    // Add new commit to history
    const newCommit = {
      id: Math.random().toString(36).substr(2, 7),
      message: commitMessage,
      author: 'Current User',
      date: new Date().toISOString(),
      files: fileChanges.length
    }
    
    commitHistory = [newCommit, ...commitHistory]
    fileChanges = []
    commitMessage = ''
    
    // Update repo status
    selectedRepo.status = 'clean'
    selectedRepo.lastCommit = 'just now'
    selectedRepo.commits++
  }
  
  function cloneRepository() {
    if (!newRepoUrl.trim()) return
    
    const repoName = newRepoUrl.split('/').pop()?.replace('.git', '') || 'new-repo'
    const newRepo = {
      id: Date.now(),
      name: repoName,
      url: newRepoUrl,
      branch: 'main',
      status: 'clean',
      lastCommit: 'just cloned',
      commits: 0
    }
    
    repositories = [...repositories, newRepo]
    newRepoUrl = ''
    showCloneDialog = false
  }
  
  function getStatusColor(status) {
    switch (status) {
      case 'clean': return 'text-secondary-400'
      case 'modified': return 'text-yellow-400'
      case 'conflict': return 'text-red-400'
      default: return 'text-dark-400'
    }
  }
  
  function getFileStatusColor(status) {
    switch (status) {
      case 'added': return 'text-secondary-400'
      case 'modified': return 'text-yellow-400'
      case 'deleted': return 'text-red-400'
      default: return 'text-dark-400'
    }
  }
  
  function getFileStatusIcon(status) {
    switch (status) {
      case 'added': return '+'
      case 'modified': return 'M'
      case 'deleted': return '-'
      default: return '?'
    }
  }
</script>

<div class="flex h-full">
  <!-- Repository List -->
  <div class="w-80 panel border-r border-dark-700 flex flex-col">
    <div class="p-4 border-b border-dark-700">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-lg font-semibold flex items-center">
          <GitBranch size={20} class="text-orange-400 mr-2" />
          Repositories
        </h2>
        <button 
          class="btn-primary p-2"
          on:click={() => showCloneDialog = true}
        >
          <Plus size={16} />
        </button>
      </div>
      
      <div class="space-y-2">
        {#each repositories as repo}
          <button
            class="w-full p-3 rounded-lg border border-dark-600 hover:border-orange-500 transition-colors text-left {selectedRepo?.id === repo.id ? 'border-orange-500 bg-dark-700' : ''}"
            on:click={() => selectRepository(repo)}
          >
            <div class="flex items-center justify-between mb-1">
              <span class="font-medium text-sm">{repo.name}</span>
              <span class="{getStatusColor(repo.status)} text-xs capitalize">
                {repo.status}
              </span>
            </div>
            
            <div class="text-xs text-dark-400 space-y-1">
              <p>Branch: {repo.branch}</p>
              <p>Last commit: {repo.lastCommit}</p>
              <p>{repo.commits} commits</p>
            </div>
          </button>
        {/each}
      </div>
    </div>
  </div>
  
  <!-- Version Control Details -->
  <div class="flex-1 flex flex-col">
    {#if selectedRepo}
      <!-- Header -->
      <div class="p-4 border-b border-dark-700">
        <div class="flex items-center justify-between">
          <div>
            <h3 class="text-lg font-semibold">{selectedRepo.name}</h3>
            <p class="text-sm text-dark-400">
              {selectedRepo.url} • Branch: {selectedRepo.branch}
            </p>
          </div>
          
          <div class="flex space-x-2">
            <button class="btn-secondary">
              <Download size={16} class="mr-2" />
              Pull
            </button>
            <button class="btn-primary">
              <Upload size={16} class="mr-2" />
              Push
            </button>
          </div>
        </div>
      </div>
      
      <div class="flex-1 overflow-auto">
        <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 p-4">
          <!-- Changes -->
          <div class="panel p-4">
            <div class="flex items-center justify-between mb-4">
              <h4 class="font-medium">Changes</h4>
              <span class="text-xs text-dark-400">{fileChanges.length} files</span>
            </div>
            
            {#if fileChanges.length > 0}
              <div class="space-y-2 mb-4">
                {#each fileChanges as change}
                  <div class="flex items-center justify-between p-2 bg-dark-700 rounded">
                    <div class="flex items-center">
                      <span class="{getFileStatusColor(change.status)} text-xs font-mono w-4">
                        {getFileStatusIcon(change.status)}
                      </span>
                      <span class="text-sm ml-2">{change.file}</span>
                    </div>
                    <div class="text-xs text-dark-400">
                      {#if change.additions > 0}
                        <span class="text-secondary-400">+{change.additions}</span>
                      {/if}
                      {#if change.deletions > 0}
                        <span class="text-red-400 ml-1">-{change.deletions}</span>
                      {/if}
                    </div>
                  </div>
                {/each}
              </div>
              
              <div class="space-y-3">
                <textarea
                  bind:value={commitMessage}
                  placeholder="Enter commit message..."
                  class="input-field w-full h-20 resize-none"
                ></textarea>
                
                <button 
                  class="btn-primary w-full"
                  disabled={!commitMessage.trim()}
                  on:click={commitChanges}
                >
                  <GitCommit size={16} class="mr-2" />
                  Commit Changes
                </button>
              </div>
            {:else}
              <div class="text-center py-8">
                <GitBranch size={32} class="text-dark-600 mx-auto mb-2" />
                <p class="text-sm text-dark-400">No changes to commit</p>
              </div>
            {/if}
          </div>
          
          <!-- Commit History -->
          <div class="panel p-4">
            <h4 class="font-medium mb-4">Recent Commits</h4>
            
            <div class="space-y-3">
              {#each commitHistory as commit}
                <div class="p-3 bg-dark-700 rounded">
                  <div class="flex items-start justify-between mb-2">
                    <div class="flex-1">
                      <p class="text-sm font-medium">{commit.message}</p>
                      <p class="text-xs text-dark-400 mt-1">
                        {commit.author} • {new Date(commit.date).toLocaleString()}
                      </p>
                    </div>
                    <span class="text-xs text-dark-500 font-mono">{commit.id}</span>
                  </div>
                  
                  <div class="flex items-center justify-between">
                    <span class="text-xs text-dark-400">{commit.files} files changed</span>
                    <button class="p-1 hover:bg-dark-600 rounded">
                      <Eye size={12} class="text-dark-400" />
                    </button>
                  </div>
                </div>
              {/each}
            </div>
          </div>
          
          <!-- Branches -->
          <div class="panel p-4">
            <h4 class="font-medium mb-4">Branches</h4>
            
            <div class="space-y-2">
              <div class="flex items-center justify-between p-2 bg-dark-700 rounded">
                <div class="flex items-center">
                  <GitBranch size={14} class="text-orange-400 mr-2" />
                  <span class="text-sm">main</span>
                  <span class="ml-2 px-2 py-1 bg-orange-500 text-dark-900 text-xs rounded">current</span>
                </div>
                <button class="p-1 hover:bg-dark-600 rounded">
                  <GitMerge size={12} class="text-dark-400" />
                </button>
              </div>
              
              <div class="flex items-center justify-between p-2 hover:bg-dark-700 rounded">
                <div class="flex items-center">
                  <GitBranch size={14} class="text-dark-400 mr-2" />
                  <span class="text-sm">feature/automl</span>
                </div>
                <button class="p-1 hover:bg-dark-600 rounded">
                  <GitMerge size={12} class="text-dark-400" />
                </button>
              </div>
              
              <div class="flex items-center justify-between p-2 hover:bg-dark-700 rounded">
                <div class="flex items-center">
                  <GitBranch size={14} class="text-dark-400 mr-2" />
                  <span class="text-sm">feature/experiments</span>
                </div>
                <button class="p-1 hover:bg-dark-600 rounded">
                  <GitMerge size={12} class="text-dark-400" />
                </button>
              </div>
            </div>
            
            <button class="btn-secondary w-full mt-3">
              <Plus size={16} class="mr-2" />
              New Branch
            </button>
          </div>
          
          <!-- Remote Integrations -->
          <div class="panel p-4">
            <h4 class="font-medium mb-4">Integrations</h4>
            
            <div class="space-y-3">
              <div class="p-3 bg-dark-700 rounded">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-sm font-medium">GitHub</span>
                  <span class="text-xs text-secondary-400">Connected</span>
                </div>
                <p class="text-xs text-dark-400">Sync with GitHub repositories</p>
                <button class="btn-secondary w-full mt-2">Manage</button>
              </div>
              
              <div class="p-3 bg-dark-700 rounded">
                <div class="flex items-center justify-between mb-2">
                  <span class="text-sm font-medium">Hugging Face</span>
                  <span class="text-xs text-yellow-400">Setup Required</span>
                </div>
                <p class="text-xs text-dark-400">Share models and datasets</p>
                <button class="btn-primary w-full mt-2">Connect</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    {:else}
      <div class="flex-1 flex items-center justify-center">
        <div class="text-center">
          <GitBranch size={48} class="text-dark-600 mx-auto mb-4" />
          <h3 class="text-lg font-medium text-dark-300 mb-2">No repository selected</h3>
          <p class="text-dark-500">Select a repository from the sidebar or clone a new one</p>
        </div>
      </div>
    {/if}
  </div>
</div>

<!-- Clone Repository Dialog -->
{#if showCloneDialog}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="panel p-6 w-96">
      <h3 class="text-lg font-semibold mb-4">Clone Repository</h3>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-medium text-dark-300 mb-2">Repository URL</label>
          <input 
            type="text" 
            bind:value={newRepoUrl}
            placeholder="https://github.com/user/repo.git"
            class="input-field w-full"
          />
        </div>
        
        <div class="flex space-x-3">
          <button 
            class="btn-secondary flex-1"
            on:click={() => showCloneDialog = false}
          >
            Cancel
          </button>
          <button 
            class="btn-primary flex-1"
            disabled={!newRepoUrl.trim()}
            on:click={cloneRepository}
          >
            Clone
          </button>
        </div>
      </div>
    </div>
  </div>
{/if}