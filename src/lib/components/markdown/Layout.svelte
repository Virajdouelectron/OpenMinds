<script lang="ts">
  import '../styles/markdown.css';
  
  export let title = 'Untitled';
  export let date: Date | null = null;
  export let readingTime: string | null = null;
  export let description: string | null = null;
  export let coverImage: string | null = null;
  export let tags: string[] = [];
  
  $: formattedDate = date ? new Date(date).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'long',
    day: 'numeric'
  }) : null;
</script>

<article class="prose dark:prose-invert max-w-none">
  {#if coverImage}
    <img 
      src={coverImage} 
      alt={title} 
      class="w-full h-64 object-cover rounded-lg mb-8"
    />
  {/if}
  
  <header class="mb-12">
    <h1 class="text-4xl font-bold mb-4">{title}</h1>
    
    <div class="flex items-center text-sm text-gray-600 dark:text-gray-400 space-x-4">
      {#if formattedDate}
        <div class="flex items-center">
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
          </svg>
          <span>{formattedDate}</span>
        </div>
      {/if}
      
      {#if readingTime}
        <div class="flex items-center">
          <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <span>{readingTime}</span>
        </div>
      {/if}
    </div>
    
    {#if description}
      <p class="mt-4 text-lg text-gray-600 dark:text-gray-300">
        {description}
      </p>
    {/if}
    
    {#if tags.length > 0}
      <div class="flex flex-wrap gap-2 mt-4">
        {#each tags as tag}
          <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200">
            {tag}
          </span>
        {/each}
      </div>
    {/if}
  </header>
  
  <div class="markdown-content">
    <slot />
  </div>
</article>

<style>
  /* Add any custom markdown styles here */
  .markdown-content {
    @apply text-gray-800 dark:text-gray-200;
  }
  
  .markdown-content h2 {
    @apply text-2xl font-bold mt-8 mb-4;
  }
  
  .markdown-content h3 {
    @apply text-xl font-bold mt-6 mb-3;
  }
  
  .markdown-content p {
    @apply my-4 leading-relaxed;
  }
  
  .markdown-content a {
    @apply text-blue-600 dark:text-blue-400 hover:underline;
  }
  
  .markdown-content ul, .markdown-content ol {
    @apply my-4 pl-6;
  }
  
  .markdown-content ul {
    @apply list-disc;
  }
  
  .markdown-content ol {
    @apply list-decimal;
  }
  
  .markdown-content li {
    @apply my-2;
  }
  
  .markdown-content code {
    @apply bg-gray-100 dark:bg-gray-800 rounded px-1.5 py-0.5 text-sm font-mono;
  }
  
  .markdown-content pre {
    @apply bg-gray-100 dark:bg-gray-800 rounded-lg p-4 my-4 overflow-x-auto;
  }
  
  .markdown-content pre code {
    @apply bg-transparent p-0;
  }
  
  .markdown-content blockquote {
    @apply border-l-4 border-gray-300 dark:border-gray-600 pl-4 italic text-gray-600 dark:text-gray-400 my-4;
  }
  
  .markdown-content table {
    @apply min-w-full divide-y divide-gray-300 dark:divide-gray-700 my-6;
  }
  
  .markdown-content th {
    @apply px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-400 uppercase tracking-wider;
  }
  
  .markdown-content td {
    @apply px-6 py-4 whitespace-nowrap text-sm text-gray-900 dark:text-gray-100;
  }
  
  .markdown-content tr {
    @apply border-b border-gray-200 dark:border-gray-700;
  }
</style>
