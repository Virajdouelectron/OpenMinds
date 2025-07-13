<script lang="ts">
  import { onMount, onDestroy } from 'svelte'
  import * as monaco from 'monaco-editor'
  import loader from '@monaco-editor/loader'
  
  export let value = ''
  export let language = 'python'
  export let readOnly = false
  export let height = '300px'
  
  let container: HTMLElement
  let editor: monaco.editor.IStandaloneCodeEditor
  
  onMount(async () => {
    // Configure Monaco loader
    loader.config({
      paths: {
        vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.43.0/min/vs'
      }
    })
    
    const monacoInstance = await loader.init()
    
    // Configure theme
    monacoInstance.editor.defineTheme('custom-dark', {
      base: 'vs-dark',
      inherit: true,
      rules: [
        { token: 'comment', foreground: '6A9955' },
        { token: 'keyword', foreground: '569CD6' },
        { token: 'string', foreground: 'CE9178' },
        { token: 'number', foreground: 'B5CEA8' }
      ],
      colors: {
        'editor.background': '#1e293b',
        'editor.foreground': '#f1f5f9',
        'editor.lineHighlightBackground': '#334155',
        'editor.selectionBackground': '#3b82f6',
        'editorCursor.foreground': '#f1f5f9',
        'editorWhitespace.foreground': '#475569'
      }
    })
    
    // Create editor
    editor = monacoInstance.editor.create(container, {
      value,
      language,
      theme: 'custom-dark',
      readOnly,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      fontSize: 14,
      lineNumbers: 'on',
      glyphMargin: false,
      folding: true,
      lineDecorationsWidth: 0,
      lineNumbersMinChars: 3,
      automaticLayout: true
    })
    
    // Listen for content changes
    editor.onDidChangeModelContent(() => {
      value = editor.getValue()
    })
  })
  
  onDestroy(() => {
    if (editor) {
      editor.dispose()
    }
  })
  
  // Update editor when value changes externally
  $: if (editor && editor.getValue() !== value) {
    editor.setValue(value)
  }
</script>

<div bind:this={container} style="height: {height};" class="rounded-lg overflow-hidden border border-dark-600"></div>