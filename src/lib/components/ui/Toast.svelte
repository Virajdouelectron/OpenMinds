<script lang="ts">
  import { fade, fly } from 'svelte/transition';
  import { quintOut } from 'svelte/easing';
  
  export let id: string;
  export let type: 'success' | 'error' | 'warning' | 'info' = 'info';
  export let message: string;
  export let duration: number = 5000;
  export let onDismiss: (id: string) => void = () => {};
  
  // Auto-dismiss after duration if duration is positive
  import { onMount } from 'svelte';
  
  onMount(() => {
    if (duration > 0) {
      const timer = setTimeout(() => {
        onDismiss(id);
      }, duration);
      
      return () => clearTimeout(timer);
    }
  });
  
  // Get the appropriate icon and colors based on the toast type
  const getToastConfig = () => {
    switch (type) {
      case 'success':
        return {
          icon: 'M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z',
          bg: 'bg-green-100 dark:bg-green-900',
          text: 'text-green-800 dark:text-green-200',
          border: 'border-green-200 dark:border-green-800',
        };
      case 'error':
        return {
          icon: 'M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z',
          bg: 'bg-red-100 dark:bg-red-900',
          text: 'text-red-800 dark:text-red-200',
          border: 'border-red-200 dark:border-red-800',
        };
      case 'warning':
        return {
          icon: 'M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z',
          bg: 'bg-yellow-100 dark:bg-yellow-900',
          text: 'text-yellow-800 dark:text-yellow-200',
          border: 'border-yellow-200 dark:border-yellow-800',
        };
      case 'info':
      default:
        return {
          icon: 'M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h2a1 1 0 100-2v-3a1 1 0 00-1-1H9z',
          bg: 'bg-blue-100 dark:bg-blue-900',
          text: 'text-blue-800 dark:text-blue-200',
          border: 'border-blue-200 dark:border-blue-800',
        };
    }
  };
  
  const { icon, bg, text, border } = $derived(getToastConfig());
</script>

<div
  class="toast-container relative p-4 mb-2 rounded-lg shadow-lg ${bg} ${text} ${border} border max-w-sm w-full pointer-events-auto"
  role="alert"
  in:fade={{ duration: 150 }}
  out:fly={{ y: 20, duration: 150, easing: quintOut }}
>
  <div class="flex items-start">
    <div class="flex-shrink-0 pt-0.5">
      <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
        <path fill-rule="evenodd" d="{icon}" clip-rule="evenodd" />
      </svg>
    </div>
    <div class="ml-3 w-0 flex-1 pt-0.5">
      <p class="text-sm font-medium">
        {message}
      </p>
    </div>
    <div class="ml-4 flex-shrink-0 flex">
      <button
        class="inline-flex rounded-md ${text} hover:opacity-75 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-${type}-100 focus:ring-${type}-500"
        on:click={() => onDismiss(id)}
      >
        <span class="sr-only">Close</span>
        <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor">
          <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
        </svg>
      </button>
    </div>
  </div>
  
  {#if duration > 0}
    <div class="absolute bottom-0 left-0 right-0 h-1 bg-black/10 dark:bg-white/10 rounded-b overflow-hidden">
      <div
        class="h-full ${type === 'success' ? 'bg-green-500' : type === 'error' ? 'bg-red-500' : type === 'warning' ? 'bg-yellow-500' : 'bg-blue-500'}"
        style={`animation: progress ${duration}ms linear;`}
      />
    </div>
  {/if}
</div>

<style>
  @keyframes progress {
    from { width: 100%; }
    to { width: 0%; }
  }
  
  .toast-container {
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
  }
</style>
