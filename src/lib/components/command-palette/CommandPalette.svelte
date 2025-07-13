<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { fade } from 'svelte/transition';
  import { flip } from 'svelte/animate';
  import { createEventDispatcher } from 'svelte';
  import { notebooks } from '$lib/stores/notebooks';
  import { toast } from '$lib/stores/toast';
  
  export let onSelect: (command: { id: string; data?: any }) => void = () => {};
  
  const dispatch = createEventDispatcher<{ select: { id: string; data?: any } }>();
  
  // Available commands
  const commands = [
    {
      id: 'new-notebook',
      title: 'New Notebook',
      description: 'Create a new notebook',
      icon: 'M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z',
      category: 'Notebook',
      action: () => {
        const notebook = notebooks.createNewNotebook('Untitled Notebook');
        toast.success('Created new notebook');
        return { id: 'new-notebook', data: { notebookId: notebook.id } };
      },
    },
    {
      id: 'save-notebook',
      title: 'Save Notebook',
      description: 'Save the current notebook',
      icon: 'M8 7H5a2 2 0 00-2 2v9a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-3m-1 4l-3 3m0 0l-3-3m3 3V4',
      category: 'Notebook',
      action: () => {
        toast.info('Notebook saved');
        return { id: 'save-notebook' };
      },
    },
    {
      id: 'toggle-theme',
      title: 'Toggle Theme',
      description: 'Switch between light and dark theme',
      icon: 'M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z',
      category: 'View',
      action: () => {
        // This would be handled by the theme store
        return { id: 'toggle-theme' };
      },
    },
    {
      id: 'open-settings',
      title: 'Open Settings',
      description: 'Open application settings',
      icon: 'M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z',
      category: 'Preferences',
      action: () => {
        return { id: 'open-settings' };
      },
    },
  ];
  
  // Group commands by category
  const groupedCommands = $derived({
    Notebook: commands.filter(cmd => cmd.category === 'Notebook'),
    View: commands.filter(cmd => cmd.category === 'View'),
    Preferences: commands.filter(cmd => cmd.category === 'Preferences'),
  });
  
  // State
  let query = '';
  let selectedIndex = 0;
  let isOpen = true;
  let filteredCommands = $derived(
    query === ''
      ? commands
      : commands.filter(cmd => 
          cmd.title.toLowerCase().includes(query.toLowerCase()) ||
          cmd.description.toLowerCase().includes(query.toLowerCase())
        )
  );
  
  // Handle keyboard navigation
  function handleKeyDown(event: KeyboardEvent) {
    if (event.key === 'ArrowDown') {
      event.preventDefault();
      selectedIndex = Math.min(selectedIndex + 1, filteredCommands.length - 1);
    } else if (event.key === 'ArrowUp') {
      event.preventDefault();
      selectedIndex = Math.max(selectedIndex - 1, 0);
    } else if (event.key === 'Enter') {
      event.preventDefault();
      if (filteredCommands[selectedIndex]) {
        executeCommand(filteredCommands[selectedIndex]);
      }
    } else if (event.key === 'Escape') {
      event.preventDefault();
      close();
    }
  }
  
  // Execute a command
  function executeCommand(command: typeof commands[number]) {
    const result = command.action();
    onSelect(result);
    dispatch('select', result);
    close();
  }
  
  // Close the command palette
  function close() {
    isOpen = false;
    // Small delay to allow the close animation to play
    setTimeout(() => {
      dispatch('close');
    }, 150);
  }
  
  // Focus the input when mounted
  let inputElement: HTMLInputElement;
  onMount(() => {
    inputElement?.focus();
    document.body.style.overflow = 'hidden';
    return () => {
      document.body.style.overflow = '';
    };
  });
  
  // Reset selected index when query changes
  $effect(() => {
    if (filteredCommands) {
      selectedIndex = Math.min(selectedIndex, filteredCommands.length - 1);
    }
  });
</script>

<div 
  class="fixed inset-0 z-50 overflow-hidden"
  on:keydown={handleKeyDown}
  on:click|self={close}
  role="dialog"
  aria-modal="true"
  tabindex="-1"
  transition:fade={{ duration: 150 }}
>
  <div class="absolute inset-0 bg-black/50 backdrop-blur-sm" />
  
  <div 
    class="fixed inset-0 flex items-center justify-center p-4"
    style="pointer-events: none;"
  >
    <div 
      class="w-full max-w-xl bg-surface-900 border border-surface-800 rounded-xl shadow-2xl overflow-hidden transform transition-all duration-150 ease-out"
      style="pointer-events: auto; max-height: 80vh;"
      in:fade={{ duration: 150 }}
    >
      <div class="relative">
        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
          <svg class="h-5 w-5 text-surface-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z" clip-rule="evenodd" />
          </svg>
        </div>
        <input
          bind:this={inputElement}
          type="text"
          class="block w-full pl-10 pr-4 py-4 bg-transparent border-0 text-surface-100 placeholder-surface-400 focus:ring-0 focus:outline-none text-sm"
          placeholder="Type a command or search..."
          bind:value={query}
          autocomplete="off"
          autocorrect="off"
          spellcheck="false"
        />
        <div class="absolute inset-y-0 right-0 flex items-center pr-3">
          <kbd class="inline-flex items-center px-2 py-1 rounded border border-surface-700 text-xs text-surface-300">
            Esc
          </kbd>
        </div>
      </div>
      
      <div class="border-t border-surface-800">
        {#if filteredCommands.length > 0}
          <div class="py-2 max-h-[60vh] overflow-y-auto">
            {#each Object.entries(
              filteredCommands.reduce((acc, cmd) => {
                if (!acc[cmd.category]) acc[cmd.category] = [];
                acc[cmd.category].push(cmd);
                return acc;
              }, {})
            ) as [string, typeof commands]}
              {#if $groupedCommands[group[0]]?.length > 0}
                <div class="px-4 py-2">
                  <div class="text-xs font-medium text-surface-400 px-2 py-1">
                    {group[0]}
                  </div>
                  <div class="mt-1">
                    {#each group[1] as command, i}
                      <button
                        class={`w-full text-left px-4 py-2 rounded-md flex items-center space-x-3 transition-colors ${selectedIndex === commands.findIndex(c => c.id === command.id) ? 'bg-surface-800' : 'hover:bg-surface-800'}`}
                        on:click|stopPropagation={() => executeCommand(command)}
                        on:mouseenter={() => selectedIndex = commands.findIndex(c => c.id === command.id)}
                      >
                        <div class={`p-1.5 rounded-md ${selectedIndex === commands.findIndex(c => c.id === command.id) ? 'bg-primary-500/10 text-primary-400' : 'text-surface-400'}`}>
                          <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
                            <path fill-rule="evenodd" d={command.icon} clip-rule="evenodd" />
                          </svg>
                        </div>
                        <div class="flex-1 min-w-0">
                          <div class="text-sm font-medium text-surface-100 truncate">
                            {command.title}
                          </div>
                          <div class="text-xs text-surface-400 truncate">
                            {command.description}
                          </div>
                        </div>
                        <div class="text-xs text-surface-500">
                          {#if command.shortcut}
                            {command.shortcut.split('+').map(key => (
                              <kbd class="inline-flex items-center px-1.5 py-0.5 rounded border border-surface-700 text-xs font-mono">
                                {key}
                              </kbd>
                            ))}
                          {/if}
                        </div>
                      </button>
                    {/each}
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        {:else}
          <div class="py-8 text-center">
            <svg class="mx-auto h-12 w-12 text-surface-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <h3 class="mt-2 text-sm font-medium text-surface-100">No commands found</h3>
            <p class="mt-1 text-sm text-surface-400">
              No commands match your search. Try a different term.
            </p>
          </div>
        {/if}
      </div>
      
      <div class="bg-surface-800/50 px-4 py-2.5 border-t border-surface-800 flex items-center justify-between text-xs">
        <div class="flex items-center space-x-4">
          <div class="flex items-center space-x-1">
            <kbd class="inline-flex items-center px-1.5 py-0.5 rounded border border-surface-700 text-xs font-mono">↑</kbd>
            <kbd class="inline-flex items-center px-1.5 py-0.5 rounded border border-surface-700 text-xs font-mono">↓</kbd>
            <span class="text-surface-400">to navigate</span>
          </div>
          <div class="flex items-center space-x-1">
            <kbd class="inline-flex items-center px-1.5 py-0.5 rounded border border-surface-700 text-xs font-mono">↵</kbd>
            <span class="text-surface-400">to select</span>
          </div>
        </div>
        <div class="text-surface-400">
          <span class="hidden sm:inline">Press</span>
          <kbd class="inline-flex items-center px-1.5 py-0.5 rounded border border-surface-700 text-xs font-mono ml-1">esc</kbd>
          <span class="hidden sm:inline">to close</span>
        </div>
      </div>
    </div>
  </div>
</div>
