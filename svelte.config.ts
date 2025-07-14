import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import { mdsvex } from 'mdsvex';
import type { Config } from '@sveltejs/kit';

const config: Config = {
  // Enable Svelte and MDsveX preprocessing
  extensions: ['.svelte', '.svx', '.md'],
  
  // Preprocessors for Svelte files
  preprocess: [
    vitePreprocess(),
    mdsvex({
      extensions: ['.svx', '.md'],
      layout: {
        _: './src/lib/components/markdown/Layout.svelte'
      }
    })
  ],
  
  // Vite-specific settings
  vitePlugin: {
    experimental: {
      inspector: {
        // Toggle hold to bypass event blocking
        toggleKeyCombo: 'meta-shift',
        // Show/hide the inspector
        showToggleButton: 'always',
        // Position of the inspector
        toggleButtonPos: 'bottom-right'
      }
    }
  }
};
