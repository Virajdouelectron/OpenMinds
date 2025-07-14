<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { fade, scale } from 'svelte/transition';
  import { toast } from '$lib/stores/toast';
  
  export let accept = '*/*';
  export let multiple = false;
  export let maxSize = 100 * 1024 * 1024; // 100MB default
  export let disabled = false;
  export let dragAndDrop = true;
  export let showPreview = true;
  
  const dispatch = createEventDispatcher<{
    upload: { files: File[] };
    error: { message: string };
  }>();
  
  let isDragOver = false;
  let fileInput: HTMLInputElement;
  let uploadedFiles: File[] = [];
  let isUploading = false;
  
  // Supported file types for preview
  const imageTypes = ['image/jpeg', 'image/png', 'image/gif', 'image/webp', 'image/svg+xml'];
  const textTypes = ['text/plain', 'text/csv', 'application/json'];
  const dataTypes = ['text/csv', 'application/json', 'application/vnd.ms-excel', 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'];
  
  function handleFileSelect(event: Event) {
    const target = event.target as HTMLInputElement;
    if (target.files) {
      processFiles(Array.from(target.files));
    }
  }
  
  function handleDrop(event: DragEvent) {
    event.preventDefault();
    isDragOver = false;
    
    if (disabled || !event.dataTransfer) return;
    
    const files = Array.from(event.dataTransfer.files);
    processFiles(files);
  }
  
  function handleDragOver(event: DragEvent) {
    event.preventDefault();
    if (!disabled) {
      isDragOver = true;
    }
  }
  
  function handleDragLeave(event: DragEvent) {
    event.preventDefault();
    isDragOver = false;
  }
  
  function processFiles(files: File[]) {
    if (disabled) return;
    
    // Validate files
    const validFiles: File[] = [];
    const errors: string[] = [];
    
    for (const file of files) {
      // Check file size
      if (file.size > maxSize) {
        errors.push(`${file.name} is too large (max ${formatFileSize(maxSize)})`);
        continue;
      }
      
      // Check file type if accept is specified
      if (accept !== '*/*' && !isFileTypeAccepted(file, accept)) {
        errors.push(`${file.name} is not an accepted file type`);
        continue;
      }
      
      validFiles.push(file);
    }
    
    // Show errors
    if (errors.length > 0) {
      errors.forEach(error => toast.error(error));
      dispatch('error', { message: errors.join(', ') });
    }
    
    // Process valid files
    if (validFiles.length > 0) {
      if (!multiple && validFiles.length > 1) {
        toast.error('Only one file is allowed');
        return;
      }
      
      uploadedFiles = multiple ? [...uploadedFiles, ...validFiles] : validFiles;
      uploadFiles(validFiles);
    }
  }
  
  async function uploadFiles(files: File[]) {
    isUploading = true;
    
    try {
      // Simulate upload delay
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      dispatch('upload', { files });
      toast.success(`Successfully uploaded ${files.length} file${files.length > 1 ? 's' : ''}`);
    } catch (error) {
      const message = error instanceof Error ? error.message : 'Upload failed';
      toast.error(message);
      dispatch('error', { message });
    } finally {
      isUploading = false;
    }
  }
  
  function removeFile(index: number) {
    uploadedFiles = uploadedFiles.filter((_, i) => i !== index);
  }
  
  function clearFiles() {
    uploadedFiles = [];
    if (fileInput) {
      fileInput.value = '';
    }
  }
  
  function formatFileSize(bytes: number): string {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }
  
  function isFileTypeAccepted(file: File, acceptString: string): boolean {
    const acceptedTypes = acceptString.split(',').map(type => type.trim());
    
    return acceptedTypes.some(type => {
      if (type === '*/*') return true;
      if (type.endsWith('/*')) {
        const category = type.slice(0, -2);
        return file.type.startsWith(category);
      }
      return file.type === type || file.name.toLowerCase().endsWith(type.toLowerCase());
    });
  }
  
  function getFileIcon(file: File): string {
    if (imageTypes.includes(file.type)) {
      return '🖼️';
    } else if (textTypes.includes(file.type)) {
      return '📄';
    } else if (dataTypes.includes(file.type)) {
      return '📊';
    } else if (file.type.includes('pdf')) {
      return '📕';
    } else if (file.type.includes('zip') || file.type.includes('archive')) {
      return '📦';
    } else {
      return '📎';
    }
  }
  
  function createPreviewUrl(file: File): string | null {
    if (imageTypes.includes(file.type)) {
      return URL.createObjectURL(file);
    }
    return null;
  }
</script>

<div class="file-upload-container">
  <!-- Drop Zone -->
  <div
    class="drop-zone border-2 border-dashed rounded-lg p-8 text-center transition-all duration-200 {isDragOver ? 'border-primary-500 bg-primary-500/10' : 'border-surface-600 hover:border-surface-500'} {disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}"
    class:drag-over={isDragOver}
    on:drop={handleDrop}
    on:dragover={handleDragOver}
    on:dragleave={handleDragLeave}
    on:click={() => !disabled && fileInput?.click()}
    role="button"
    tabindex="0"
  >
    <input
      bind:this={fileInput}
      type="file"
      {accept}
      {multiple}
      {disabled}
      on:change={handleFileSelect}
      class="hidden"
    />
    
    <div class="upload-content">
      {#if isUploading}
        <div class="uploading-state" transition:scale={{ duration: 200 }}>
          <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-primary-500 mx-auto mb-4"></div>
          <p class="text-lg font-medium text-surface-200">Uploading files...</p>
          <p class="text-sm text-surface-400">Please wait while we process your files</p>
        </div>
      {:else if isDragOver}
        <div class="drag-state" transition:scale={{ duration: 200 }}>
          <svg class="w-16 h-16 mx-auto mb-4 text-primary-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
          </svg>
          <p class="text-lg font-medium text-primary-400">Drop files here</p>
          <p class="text-sm text-surface-400">Release to upload</p>
        </div>
      {:else}
        <div class="default-state">
          <svg class="w-16 h-16 mx-auto mb-4 text-surface-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12" />
          </svg>
          <p class="text-lg font-medium text-surface-200 mb-2">
            {dragAndDrop ? 'Drag and drop files here' : 'Click to select files'}
          </p>
          <p class="text-sm text-surface-400 mb-4">
            {dragAndDrop ? 'or click to browse' : 'Choose files from your computer'}
          </p>
          
          <div class="upload-info text-xs text-surface-500 space-y-1">
            {#if accept !== '*/*'}
              <p>Accepted types: {accept}</p>
            {/if}
            <p>Maximum file size: {formatFileSize(maxSize)}</p>
            {#if multiple}
              <p>Multiple files allowed</p>
            {/if}
          </div>
        </div>
      {/if}
    </div>
  </div>
  
  <!-- File List -->
  {#if uploadedFiles.length > 0 && showPreview}
    <div class="file-list mt-6" transition:fade={{ duration: 200 }}>
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-medium text-surface-200">
          Uploaded Files ({uploadedFiles.length})
        </h3>
        <button
          class="btn-secondary text-sm"
          on:click={clearFiles}
          disabled={isUploading}
        >
          Clear All
        </button>
      </div>
      
      <div class="space-y-3">
        {#each uploadedFiles as file, index}
          <div 
            class="file-item flex items-center space-x-4 p-4 bg-surface-800 rounded-lg border border-surface-700"
            transition:slide={{ duration: 200 }}
          >
            <!-- File Preview -->
            <div class="file-preview flex-shrink-0">
              {#if createPreviewUrl(file)}
                <img 
                  src={createPreviewUrl(file)} 
                  alt={file.name}
                  class="w-12 h-12 object-cover rounded"
                />
              {:else}
                <div class="w-12 h-12 bg-surface-700 rounded flex items-center justify-center text-2xl">
                  {getFileIcon(file)}
                </div>
              {/if}
            </div>
            
            <!-- File Info -->
            <div class="file-info flex-1 min-w-0">
              <p class="text-sm font-medium text-surface-200 truncate">{file.name}</p>
              <div class="flex items-center space-x-4 text-xs text-surface-400 mt-1">
                <span>{formatFileSize(file.size)}</span>
                <span>{file.type || 'Unknown type'}</span>
                <span>Modified: {new Date(file.lastModified).toLocaleDateString()}</span>
              </div>
            </div>
            
            <!-- Actions -->
            <div class="file-actions flex items-center space-x-2">
              <button
                class="p-2 hover:bg-surface-700 rounded transition-colors"
                on:click={() => removeFile(index)}
                disabled={isUploading}
                title="Remove file"
              >
                <svg class="w-4 h-4 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  .drop-zone {
    min-height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  
  .drop-zone.drag-over {
    transform: scale(1.02);
  }
  
  .upload-content {
    pointer-events: none;
  }
  
  .file-item:hover {
    background-color: theme('colors.surface.700');
  }
</style>