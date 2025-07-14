<script lang="ts">
  import { onMount } from 'svelte';
  
  export let content: string;
  
  let renderedHtml = '';
  
  // Simple markdown parser (in a real app, you'd use a library like marked or remark)
  function parseMarkdown(markdown: string): string {
    if (!markdown) return '';
    
    let html = markdown
      // Headers
      .replace(/^### (.*$)/gim, '<h3>$1</h3>')
      .replace(/^## (.*$)/gim, '<h2>$1</h2>')
      .replace(/^# (.*$)/gim, '<h1>$1</h1>')
      
      // Bold
      .replace(/\*\*(.*)\*\*/gim, '<strong>$1</strong>')
      .replace(/__(.*?)__/gim, '<strong>$1</strong>')
      
      // Italic
      .replace(/\*(.*)\*/gim, '<em>$1</em>')
      .replace(/_(.*?)_/gim, '<em>$1</em>')
      
      // Code blocks
      .replace(/```([\s\S]*?)```/gim, '<pre><code>$1</code></pre>')
      
      // Inline code
      .replace(/`(.*?)`/gim, '<code>$1</code>')
      
      // Links
      .replace(/\[([^\]]+)\]\(([^)]+)\)/gim, '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>')
      
      // Images
      .replace(/!\[([^\]]*)\]\(([^)]+)\)/gim, '<img alt="$1" src="$2" />')
      
      // Unordered lists
      .replace(/^\* (.+)$/gim, '<li>$1</li>')
      .replace(/(<li>.*<\/li>)/s, '<ul>$1</ul>')
      
      // Ordered lists
      .replace(/^\d+\. (.+)$/gim, '<li>$1</li>')
      
      // Blockquotes
      .replace(/^> (.+)$/gim, '<blockquote>$1</blockquote>')
      
      // Horizontal rules
      .replace(/^---$/gim, '<hr>')
      
      // Line breaks
      .replace(/\n\n/gim, '</p><p>')
      .replace(/\n/gim, '<br>');
    
    // Wrap in paragraphs if not already wrapped
    if (!html.startsWith('<')) {
      html = '<p>' + html + '</p>';
    }
    
    return html;
  }
  
  $: renderedHtml = parseMarkdown(content);
</script>

<div class="markdown-renderer p-4 min-h-[100px]">
  {#if content.trim()}
    <div class="markdown-content">
      {@html renderedHtml}
    </div>
  {:else}
    <div class="empty-markdown text-surface-500 italic text-center py-8">
      <svg class="w-8 h-8 mx-auto mb-2 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
      </svg>
      Click to edit this markdown cell
    </div>
  {/if}
</div>

<style>
  .markdown-content {
    @apply text-surface-200 leading-relaxed;
  }
  
  .markdown-content :global(h1) {
    @apply text-2xl font-bold text-surface-100 mb-4 mt-6 first:mt-0;
  }
  
  .markdown-content :global(h2) {
    @apply text-xl font-bold text-surface-100 mb-3 mt-5 first:mt-0;
  }
  
  .markdown-content :global(h3) {
    @apply text-lg font-bold text-surface-100 mb-2 mt-4 first:mt-0;
  }
  
  .markdown-content :global(p) {
    @apply mb-4 last:mb-0;
  }
  
  .markdown-content :global(strong) {
    @apply font-bold text-surface-100;
  }
  
  .markdown-content :global(em) {
    @apply italic;
  }
  
  .markdown-content :global(code) {
    @apply bg-surface-800 text-pink-400 px-1.5 py-0.5 rounded text-sm font-mono;
  }
  
  .markdown-content :global(pre) {
    @apply bg-surface-900 border border-surface-700 rounded-lg p-4 mb-4 overflow-x-auto;
  }
  
  .markdown-content :global(pre code) {
    @apply bg-transparent text-surface-200 p-0;
  }
  
  .markdown-content :global(a) {
    @apply text-primary-400 hover:text-primary-300 underline transition-colors;
  }
  
  .markdown-content :global(img) {
    @apply max-w-full h-auto rounded-lg shadow-md my-4;
  }
  
  .markdown-content :global(ul) {
    @apply list-disc list-inside mb-4 space-y-1;
  }
  
  .markdown-content :global(ol) {
    @apply list-decimal list-inside mb-4 space-y-1;
  }
  
  .markdown-content :global(li) {
    @apply text-surface-200;
  }
  
  .markdown-content :global(blockquote) {
    @apply border-l-4 border-surface-600 pl-4 italic text-surface-400 my-4;
  }
  
  .markdown-content :global(hr) {
    @apply border-surface-600 my-6;
  }
  
  .markdown-content :global(table) {
    @apply w-full border-collapse border border-surface-600 mb-4;
  }
  
  .markdown-content :global(th) {
    @apply bg-surface-800 border border-surface-600 px-3 py-2 text-left font-medium text-surface-200;
  }
  
  .markdown-content :global(td) {
    @apply border border-surface-600 px-3 py-2 text-surface-300;
  }
</style>