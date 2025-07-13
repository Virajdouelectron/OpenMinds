<script lang="ts">
  import { Terminal, Play, Trash2, Download, Upload } from 'lucide-svelte'
  import { onMount } from 'svelte'
  
  let terminalOutput = []
  let currentCommand = ''
  let commandHistory = []
  let historyIndex = -1
  let terminalElement
  
  const welcomeMessage = `
OpenMinds ML IDE Terminal v1.0.0
Type 'help' for available commands.

Available integrations:
- GitHub: git clone, push, pull
- Hugging Face: huggingface-hub commands
- Python: pip, python, jupyter
- System: ls, cd, mkdir, rm, etc.

`
  
  onMount(() => {
    terminalOutput = [
      { type: 'system', content: welcomeMessage, timestamp: new Date() }
    ]
  })
  
  function executeCommand() {
    if (!currentCommand.trim()) return
    
    // Add command to history
    commandHistory.unshift(currentCommand)
    historyIndex = -1
    
    // Add command to output
    terminalOutput = [...terminalOutput, {
      type: 'command',
      content: `$ ${currentCommand}`,
      timestamp: new Date()
    }]
    
    // Simulate command execution
    setTimeout(() => {
      const response = processCommand(currentCommand.trim())
      terminalOutput = [...terminalOutput, {
        type: 'output',
        content: response,
        timestamp: new Date()
      }]
      scrollToBottom()
    }, 100)
    
    currentCommand = ''
    scrollToBottom()
  }
  
  function processCommand(cmd) {
    const [command, ...args] = cmd.split(' ')
    
    switch (command.toLowerCase()) {
      case 'help':
        return `Available commands:
  help                    - Show this help message
  ls                      - List files and directories
  cd <directory>          - Change directory
  pwd                     - Print working directory
  mkdir <directory>       - Create directory
  rm <file>              - Remove file
  cat <file>             - Display file contents
  
  Python & ML:
  python <script>         - Run Python script
  pip install <package>   - Install Python package
  jupyter notebook        - Start Jupyter notebook
  
  Git & GitHub:
  git clone <repo>        - Clone repository
  git status              - Show git status
  git add <file>          - Add file to staging
  git commit -m "msg"     - Commit changes
  git push                - Push to remote
  git pull                - Pull from remote
  
  Hugging Face:
  huggingface-cli login   - Login to Hugging Face
  huggingface-cli upload  - Upload model/dataset
  huggingface-cli download - Download model/dataset
  
  clear                   - Clear terminal`
      
      case 'ls':
        return `notebooks/
datasets/
models/
experiments/
.git/
requirements.txt
README.md`
      
      case 'pwd':
        return '/home/project'
      
      case 'cd':
        return args.length > 0 ? `Changed directory to: ${args[0]}` : 'Usage: cd <directory>'
      
      case 'mkdir':
        return args.length > 0 ? `Created directory: ${args[0]}` : 'Usage: mkdir <directory>'
      
      case 'git':
        return handleGitCommand(args)
      
      case 'python':
        return args.length > 0 ? `Executing Python script: ${args[0]}...` : 'Python 3.9.7 (default, Jan 15 2024, 10:30:00)'
      
      case 'pip':
        if (args[0] === 'install' && args[1]) {
          return `Installing ${args[1]}...
Collecting ${args[1]}
Successfully installed ${args[1]}`
        }
        return 'Usage: pip install <package>'
      
      case 'jupyter':
        if (args[0] === 'notebook') {
          return 'Starting Jupyter notebook server...\nNotebook server is running at: http://localhost:8888'
        }
        return 'Usage: jupyter notebook'
      
      case 'huggingface-cli':
        return handleHuggingFaceCommand(args)
      
      case 'clear':
        terminalOutput = []
        return ''
      
      case 'cat':
        return args.length > 0 ? `Contents of ${args[0]}:\n[File contents would be displayed here]` : 'Usage: cat <file>'
      
      case 'rm':
        return args.length > 0 ? `Removed: ${args[0]}` : 'Usage: rm <file>'
      
      default:
        return `Command not found: ${command}. Type 'help' for available commands.`
    }
  }
  
  function handleGitCommand(args) {
    const [subcommand, ...subargs] = args
    
    switch (subcommand) {
      case 'clone':
        return subargs.length > 0 ? 
          `Cloning repository: ${subargs[0]}...\nRepository cloned successfully.` :
          'Usage: git clone <repository-url>'
      
      case 'status':
        return `On branch main
Your branch is up to date with 'origin/main'.

Changes not staged for commit:
  modified:   notebooks/analysis.ipynb
  modified:   datasets/data.csv

Untracked files:
  models/new_model.pkl`
      
      case 'add':
        return subargs.length > 0 ? `Added ${subargs[0]} to staging area` : 'Usage: git add <file>'
      
      case 'commit':
        if (subargs[0] === '-m' && subargs[1]) {
          return `[main ${Math.random().toString(36).substr(2, 7)}] ${subargs.slice(1).join(' ')}\n 2 files changed, 15 insertions(+), 3 deletions(-)`
        }
        return 'Usage: git commit -m "commit message"'
      
      case 'push':
        return `Pushing to origin main...
Everything up-to-date`
      
      case 'pull':
        return `From github.com:user/repo
 * branch            main       -> FETCH_HEAD
Already up to date.`
      
      default:
        return `Unknown git command: ${subcommand}`
    }
  }
  
  function handleHuggingFaceCommand(args) {
    const [subcommand, ...subargs] = args
    
    switch (subcommand) {
      case 'login':
        return `Login to Hugging Face Hub
Token: [Enter your token]
Login successful!`
      
      case 'upload':
        return subargs.length > 0 ? 
          `Uploading ${subargs[0]} to Hugging Face Hub...
Upload completed successfully.` :
          'Usage: huggingface-cli upload <model-or-dataset>'
      
      case 'download':
        return subargs.length > 0 ? 
          `Downloading ${subargs[0]} from Hugging Face Hub...
Download completed successfully.` :
          'Usage: huggingface-cli download <model-or-dataset>'
      
      default:
        return `Unknown huggingface-cli command: ${subcommand}`
    }
  }
  
  function handleKeyDown(event) {
    if (event.key === 'Enter') {
      executeCommand()
    } else if (event.key === 'ArrowUp') {
      event.preventDefault()
      if (historyIndex < commandHistory.length - 1) {
        historyIndex++
        currentCommand = commandHistory[historyIndex] || ''
      }
    } else if (event.key === 'ArrowDown') {
      event.preventDefault()
      if (historyIndex > 0) {
        historyIndex--
        currentCommand = commandHistory[historyIndex] || ''
      } else if (historyIndex === 0) {
        historyIndex = -1
        currentCommand = ''
      }
    }
  }
  
  function scrollToBottom() {
    setTimeout(() => {
      if (terminalElement) {
        terminalElement.scrollTop = terminalElement.scrollHeight
      }
    }, 10)
  }
  
  function clearTerminal() {
    terminalOutput = []
  }
  
  function downloadLogs() {
    const logs = terminalOutput.map(entry => 
      `[${entry.timestamp.toLocaleTimeString()}] ${entry.content}`
    ).join('\n')
    
    const blob = new Blob([logs], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = 'terminal-logs.txt'
    a.click()
    URL.revokeObjectURL(url)
  }
</script>

<div class="flex h-full flex-col">
  <!-- Header -->
  <div class="p-4 border-b border-dark-700 flex items-center justify-between">
    <h2 class="text-lg font-semibold flex items-center">
      <Terminal size={20} class="text-green-400 mr-2" />
      Terminal
    </h2>
    
    <div class="flex space-x-2">
      <button class="btn-secondary p-2" on:click={clearTerminal}>
        <Trash2 size={16} />
      </button>
      <button class="btn-secondary p-2" on:click={downloadLogs}>
        <Download size={16} />
      </button>
    </div>
  </div>
  
  <!-- Terminal Output -->
  <div 
    bind:this={terminalElement}
    class="flex-1 overflow-auto p-4 bg-dark-900 font-mono text-sm"
  >
    {#each terminalOutput as entry}
      <div class="mb-1 {entry.type === 'command' ? 'text-green-400' : entry.type === 'system' ? 'text-blue-400' : 'text-dark-100'}">
        <pre class="whitespace-pre-wrap">{entry.content}</pre>
      </div>
    {/each}
    
    <!-- Command Input -->
    <div class="flex items-center text-green-400">
      <span class="mr-2">$</span>
      <input
        type="text"
        bind:value={currentCommand}
        on:keydown={handleKeyDown}
        class="flex-1 bg-transparent border-none outline-none text-dark-100"
        placeholder="Enter command..."
        autocomplete="off"
      />
    </div>
  </div>
  
  <!-- Quick Commands -->
  <div class="p-4 border-t border-dark-700">
    <div class="text-xs text-dark-400 mb-2">Quick Commands:</div>
    <div class="flex flex-wrap gap-2">
      <button 
        class="px-2 py-1 bg-dark-700 hover:bg-dark-600 rounded text-xs"
        on:click={() => currentCommand = 'git status'}
      >
        git status
      </button>
      <button 
        class="px-2 py-1 bg-dark-700 hover:bg-dark-600 rounded text-xs"
        on:click={() => currentCommand = 'ls -la'}
      >
        ls -la
      </button>
      <button 
        class="px-2 py-1 bg-dark-700 hover:bg-dark-600 rounded text-xs"
        on:click={() => currentCommand = 'pip list'}
      >
        pip list
      </button>
      <button 
        class="px-2 py-1 bg-dark-700 hover:bg-dark-600 rounded text-xs"
        on:click={() => currentCommand = 'jupyter notebook'}
      >
        jupyter notebook
      </button>
      <button 
        class="px-2 py-1 bg-dark-700 hover:bg-dark-600 rounded text-xs"
        on:click={() => currentCommand = 'huggingface-cli login'}
      >
        hf login
      </button>
    </div>
  </div>
</div>