import { writable } from 'svelte/store';

export const isSidebarCollapsed = writable(false);
export const activeView = writable('notebooks');
export const isDarkMode = writable(true);

export const toggleDarkMode = () => {
  isDarkMode.update(mode => {
    const newMode = !mode;
    // Update the theme class on the document element
    if (newMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
    return newMode;
  });
};

// Initialize dark mode based on user preference
if (typeof window !== 'undefined') {
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  isDarkMode.set(prefersDark);
  
  // Apply the initial theme
  if (prefersDark) {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
  
  // Listen for system theme changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', e => {
    isDarkMode.set(e.matches);
  });
}

export const isMobileMenuOpen = writable(false);
export const isCommandPaletteOpen = writable(false);

// Export a function to toggle the command palette
export const toggleCommandPalette = () => {
  isCommandPalette.update(open => !open);
};

// Export a function to close the command palette
export const closeCommandPalette = () => {
  isCommandPalette.set(false);
};

// Export a function to open the command palette
export const openCommandPalette = () => {
  isCommandPalette.set(true);
};
