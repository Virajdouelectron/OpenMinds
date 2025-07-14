<script lang="ts">
  import { onMount, onDestroy, createEventDispatcher } from 'svelte';
  import { fade } from 'svelte/transition';
  import loader from '@monaco-editor/loader';
  import type { editor } from 'monaco-editor';
  
  export let value = '';
  export let language = 'python';
  export let theme = 'vs-dark';
  export let readOnly = false;
  export let height = '400px';
  export let minimap = false;
  export let lineNumbers: 'on' | 'off' | 'relative' | 'interval' = 'on';
  export let wordWrap: 'on' | 'off' | 'wordWrapColumn' | 'bounded' = 'on';
  export let fontSize = 14;
  export let tabSize = 4;
  export let insertSpaces = true;
  export let autoIndent: 'none' | 'keep' | 'brackets' | 'advanced' | 'full' = 'advanced';
  export let formatOnPaste = true;
  export let formatOnType = true;
  
  const dispatch = createEventDispatcher<{
    change: { value: string };
    save: { value: string };
    focus: void;
    blur: void;
  }>();
  
  let container: HTMLElement;
  let monacoEditor: editor.IStandaloneCodeEditor | null = null;
  let isLoading = true;
  let loadError: string | null = null;
  
  // Monaco editor configuration
  const editorOptions: editor.IStandaloneEditorConstructionOptions = {
    value,
    language,
    theme,
    readOnly,
    minimap: { enabled: minimap },
    lineNumbers,
    wordWrap,
    fontSize,
    tabSize,
    insertSpaces,
    autoIndent,
    formatOnPaste,
    formatOnType,
    scrollBeyondLastLine: false,
    automaticLayout: true,
    contextmenu: true,
    mouseWheelZoom: true,
    smoothScrolling: true,
    cursorBlinking: 'smooth',
    cursorSmoothCaretAnimation: 'on',
    renderLineHighlight: 'gutter',
    selectOnLineNumbers: true,
    glyphMargin: true,
    folding: true,
    foldingHighlight: true,
    showFoldingControls: 'mouseover',
    unfoldOnClickAfterEndOfLine: true,
    bracketPairColorization: { enabled: true },
    guides: {
      bracketPairs: true,
      bracketPairsHorizontal: true,
      highlightActiveBracketPair: true,
      indentation: true,
    },
    suggest: {
      showKeywords: true,
      showSnippets: true,
      showFunctions: true,
      showConstructors: true,
      showFields: true,
      showVariables: true,
      showClasses: true,
      showStructs: true,
      showInterfaces: true,
      showModules: true,
      showProperties: true,
      showEvents: true,
      showOperators: true,
      showUnits: true,
      showValues: true,
      showConstants: true,
      showEnums: true,
      showEnumMembers: true,
      showColors: true,
      showFiles: true,
      showReferences: true,
      showFolders: true,
      showTypeParameters: true,
    },
    quickSuggestions: {
      other: true,
      comments: false,
      strings: false,
    },
    parameterHints: { enabled: true },
    autoClosingBrackets: 'always',
    autoClosingQuotes: 'always',
    autoSurround: 'languageDefined',
    codeLens: true,
    colorDecorators: true,
    dragAndDrop: true,
    find: {
      addExtraSpaceOnTop: false,
      autoFindInSelection: 'never',
      seedSearchStringFromSelection: 'always',
    },
    links: true,
    matchBrackets: 'always',
    renderControlCharacters: false,
    renderWhitespace: 'selection',
    rulers: [],
    scrollbar: {
      vertical: 'auto',
      horizontal: 'auto',
      arrowSize: 11,
      useShadows: true,
      verticalHasArrows: false,
      horizontalHasArrows: false,
      verticalScrollbarSize: 14,
      horizontalScrollbarSize: 14,
    },
  };
  
  onMount(async () => {
    try {
      // Configure Monaco loader
      loader.config({
        paths: {
          vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.45.0/min/vs'
        }
      });
      
      const monaco = await loader.init();
      
      // Configure custom themes
      monaco.editor.defineTheme('openmind-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [
          { token: 'comment', foreground: '6A9955', fontStyle: 'italic' },
          { token: 'keyword', foreground: '569CD6', fontStyle: 'bold' },
          { token: 'string', foreground: 'CE9178' },
          { token: 'number', foreground: 'B5CEA8' },
          { token: 'regexp', foreground: 'D16969' },
          { token: 'operator', foreground: 'D4D4D4' },
          { token: 'namespace', foreground: '4EC9B0' },
          { token: 'type', foreground: '4EC9B0' },
          { token: 'struct', foreground: '86C691' },
          { token: 'class', foreground: '4EC9B0' },
          { token: 'interface', foreground: 'B8D7A3' },
          { token: 'enum', foreground: 'B8D7A3' },
          { token: 'typeParameter', foreground: 'B8D7A3' },
          { token: 'function', foreground: 'DCDCAA' },
          { token: 'method', foreground: 'DCDCAA' },
          { token: 'decorator', foreground: 'C586C0' },
          { token: 'macro', foreground: 'C586C0' },
          { token: 'variable', foreground: '9CDCFE' },
          { token: 'variable.readonly', foreground: '4FC1FF' },
          { token: 'parameter', foreground: '9CDCFE' },
          { token: 'property', foreground: '9CDCFE' },
          { token: 'enumMember', foreground: 'B8D7A3' },
          { token: 'event', foreground: 'DCDCAA' },
        ],
        colors: {
          'editor.background': '#0f172a',
          'editor.foreground': '#f1f5f9',
          'editor.lineHighlightBackground': '#1e293b',
          'editor.selectionBackground': '#3b82f6',
          'editor.selectionHighlightBackground': '#3b82f640',
          'editor.findMatchBackground': '#f59e0b80',
          'editor.findMatchHighlightBackground': '#f59e0b40',
          'editorCursor.foreground': '#f1f5f9',
          'editorWhitespace.foreground': '#475569',
          'editorIndentGuide.background': '#334155',
          'editorIndentGuide.activeBackground': '#475569',
          'editorLineNumber.foreground': '#64748b',
          'editorLineNumber.activeForeground': '#f1f5f9',
          'editorBracketMatch.background': '#3b82f640',
          'editorBracketMatch.border': '#3b82f6',
          'editorError.foreground': '#ef4444',
          'editorWarning.foreground': '#f59e0b',
          'editorInfo.foreground': '#3b82f6',
          'editorHint.foreground': '#10b981',
          'editorGutter.background': '#0f172a',
          'editorGutter.modifiedBackground': '#f59e0b',
          'editorGutter.addedBackground': '#10b981',
          'editorGutter.deletedBackground': '#ef4444',
          'scrollbar.shadow': '#00000050',
          'scrollbarSlider.background': '#47556950',
          'scrollbarSlider.hoverBackground': '#47556970',
          'scrollbarSlider.activeBackground': '#475569',
        }
      });
      
      // Create the editor
      monacoEditor = monaco.editor.create(container, {
        ...editorOptions,
        theme: theme === 'vs-dark' ? 'openmind-dark' : theme,
      });
      
      // Set up event listeners
      monacoEditor.onDidChangeModelContent(() => {
        const newValue = monacoEditor?.getValue() || '';
        if (newValue !== value) {
          value = newValue;
          dispatch('change', { value: newValue });
        }
      });
      
      monacoEditor.onDidFocusEditorText(() => {
        dispatch('focus');
      });
      
      monacoEditor.onDidBlurEditorText(() => {
        dispatch('blur');
      });
      
      // Add keyboard shortcuts
      monacoEditor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () => {
        dispatch('save', { value: monacoEditor?.getValue() || '' });
      });
      
      // Configure Python language features
      if (language === 'python') {
        monaco.languages.registerCompletionItemProvider('python', {
          provideCompletionItems: (model, position) => {
            const suggestions = [
              {
                label: 'import pandas as pd',
                kind: monaco.languages.CompletionItemKind.Snippet,
                insertText: 'import pandas as pd',
                documentation: 'Import pandas library'
              },
              {
                label: 'import numpy as np',
                kind: monaco.languages.CompletionItemKind.Snippet,
                insertText: 'import numpy as np',
                documentation: 'Import numpy library'
              },
              {
                label: 'import matplotlib.pyplot as plt',
                kind: monaco.languages.CompletionItemKind.Snippet,
                insertText: 'import matplotlib.pyplot as plt',
                documentation: 'Import matplotlib for plotting'
              },
              {
                label: 'from sklearn',
                kind: monaco.languages.CompletionItemKind.Snippet,
                insertText: 'from sklearn.${1:module} import ${2:class}',
                insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
                documentation: 'Import from scikit-learn'
              },
            ];
            return { suggestions };
          }
        });
      }
      
      isLoading = false;
    } catch (error) {
      console.error('Failed to initialize Monaco Editor:', error);
      loadError = error instanceof Error ? error.message : 'Unknown error';
      isLoading = false;
    }
  });
  
  onDestroy(() => {
    monacoEditor?.dispose();
  });
  
  // Update editor when props change
  $: if (monacoEditor && monacoEditor.getValue() !== value) {
    monacoEditor.setValue(value);
  }
  
  $: if (monacoEditor) {
    const model = monacoEditor.getModel();
    if (model && model.getLanguageId() !== language) {
      monaco.editor.setModelLanguage(model, language);
    }
  }
  
  $: if (monacoEditor) {
    monacoEditor.updateOptions({
      readOnly,
      fontSize,
      tabSize,
      insertSpaces,
      wordWrap,
      lineNumbers,
      minimap: { enabled: minimap },
    });
  }
  
  // Public methods
  export function focus() {
    monacoEditor?.focus();
  }
  
  export function getSelection() {
    return monacoEditor?.getSelection();
  }
  
  export function setSelection(selection: any) {
    monacoEditor?.setSelection(selection);
  }
  
  export function insertText(text: string) {
    const selection = monacoEditor?.getSelection();
    if (selection) {
      monacoEditor?.executeEdits('', [{
        range: selection,
        text,
        forceMoveMarkers: true,
      }]);
    }
  }
  
  export function formatDocument() {
    monacoEditor?.getAction('editor.action.formatDocument')?.run();
  }
</script>

<div class="monaco-editor-container relative" style="height: {height};">
  {#if isLoading}
    <div 
      class="absolute inset-0 flex items-center justify-center bg-surface-900 rounded-lg"
      transition:fade={{ duration: 200 }}
    >
      <div class="flex items-center space-x-3">
        <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-primary-500"></div>
        <span class="text-surface-300">Loading editor...</span>
      </div>
    </div>
  {:else if loadError}
    <div 
      class="absolute inset-0 flex items-center justify-center bg-surface-900 rounded-lg border border-error-500"
      transition:fade={{ duration: 200 }}
    >
      <div class="text-center">
        <div class="text-error-400 mb-2">
          <svg class="w-8 h-8 mx-auto" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </div>
        <p class="text-surface-300 text-sm">Failed to load editor</p>
        <p class="text-surface-500 text-xs mt-1">{loadError}</p>
      </div>
    </div>
  {/if}
  
  <div 
    bind:this={container} 
    class="w-full h-full rounded-lg overflow-hidden border border-surface-700"
    class:opacity-0={isLoading || loadError}
    class:opacity-100={!isLoading && !loadError}
  />
</div>

<style>
  .monaco-editor-container {
    transition: opacity 0.2s ease-in-out;
  }
  
  :global(.monaco-editor) {
    font-family: 'JetBrains Mono', 'Fira Code', 'Cascadia Code', monospace !important;
  }
  
  :global(.monaco-editor .margin) {
    background-color: #0f172a !important;
  }
  
  :global(.monaco-editor .monaco-editor-background) {
    background-color: #0f172a !important;
  }
  
  :global(.monaco-editor .current-line) {
    background-color: #1e293b !important;
  }
  
  :global(.monaco-editor .line-numbers) {
    color: #64748b !important;
  }
  
  :global(.monaco-editor .current-line .line-numbers) {
    color: #f1f5f9 !important;
  }
</style>