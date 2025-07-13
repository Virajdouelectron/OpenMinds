import { writable } from 'svelte/store';

type ToastType = 'success' | 'error' | 'warning' | 'info';

interface Toast {
  id: string;
  type: ToastType;
  message: string;
  duration?: number;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  let toastId = 0;

  function addToast(
    type: ToastType,
    message: string,
    duration: number = 5000
  ) {
    const id = `toast-${toastId++}`;
    
    // Add the toast
    update((toasts) => [...toasts, { id, type, message, duration }]);
    
    // Auto-remove the toast after duration
    if (duration > 0) {
      setTimeout(() => {
        removeToast(id);
      }, duration);
    }
    
    return id;
  }

  function removeToast(id: string) {
    update((toasts) => toasts.filter((t) => t.id !== id));
  }

  function success(message: string, duration?: number) {
    return addToast('success', message, duration);
  }

  function error(message: string, duration?: number) {
    return addToast('error', message, duration || 10000); // Longer duration for errors
  }

  function warning(message: string, duration?: number) {
    return addToast('warning', message, duration);
  }

  function info(message: string, duration?: number) {
    return addToast('info', message, duration);
  }

  return {
    subscribe,
    add: addToast,
    remove: removeToast,
    success,
    error,
    warning,
    info,
  };
}

export const toast = createToastStore();
