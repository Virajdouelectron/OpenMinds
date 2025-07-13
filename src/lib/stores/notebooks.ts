import { writable } from 'svelte/store';
import { v4 as uuidv4 } from 'uuid';

export interface Cell {
  id: string;
  type: 'code' | 'markdown';
  content: string;
  output?: any;
  isExecuting?: boolean;
  metadata?: Record<string, any>;
}

export interface Notebook {
  id: string;
  name: string;
  cells: Cell[];
  createdAt: Date;
  updatedAt: Date;
  metadata?: {
    description?: string;
    tags?: string[];
    [key: string]: any;
  };
}

function createNotebookStore() {
  const { subscribe, set, update } = writable<Notebook[]>([]);
  
  // Initialize with a default notebook if empty
  function initialize() {
    update(notebooks => {
      if (notebooks.length === 0) {
        return [
          createNewNotebook('Welcome to OpenMind')
        ];
      }
      return notebooks;
    });
  }

  function createNewNotebook(name: string = 'Untitled Notebook'): Notebook {
    const now = new Date();
    const notebook: Notebook = {
      id: uuidv4(),
      name,
      cells: [
        {
          id: uuidv4(),
          type: 'markdown',
          content: '# Welcome to OpenMind\n\nThis is a new notebook. You can start by adding code or markdown cells.',
        },
        {
          id: uuidv4(),
          type: 'code',
          content: '# Your first code cell\nprint("Hello, OpenMind!")',
        },
      ],
      createdAt: now,
      updatedAt: now,
    };
    
    // Add the new notebook to the store
    update(notebooks => [...notebooks, notebook]);
    
    return notebook;
  }

  function getNotebookById(id: string): Notebook | undefined {
    let result: Notebook | undefined;
    subscribe(notebooks => {
      result = notebooks.find(n => n.id === id);
    })();
    return result;
  }

  function updateNotebook(id: string, updates: Partial<Notebook>) {
    update(notebooks =>
      notebooks.map(notebook =>
        notebook.id === id
          ? { ...notebook, ...updates, updatedAt: new Date() }
          : notebook
      )
    );
  }

  function deleteNotebook(id: string) {
    update(notebooks => notebooks.filter(notebook => notebook.id !== id));
  }

  function addCell(notebookId: string, type: 'code' | 'markdown', content: string = '') {
    const newCell: Cell = {
      id: uuidv4(),
      type,
      content,
    };

    update(notebooks =>
      notebooks.map(notebook => {
        if (notebook.id === notebookId) {
          return {
            ...notebook,
            cells: [...notebook.cells, newCell],
            updatedAt: new Date(),
          };
        }
        return notebook;
      })
    );

    return newCell.id;
  }

  function updateCell(notebookId: string, cellId: string, updates: Partial<Cell>) {
    update(notebooks =>
      notebooks.map(notebook => {
        if (notebook.id === notebookId) {
          return {
            ...notebook,
            cells: notebook.cells.map(cell =>
              cell.id === cellId ? { ...cell, ...updates } : cell
            ),
            updatedAt: new Date(),
          };
        }
        return notebook;
      })
    );
  }

  function deleteCell(notebookId: string, cellId: string) {
    update(notebooks =>
      notebooks.map(notebook => {
        if (notebook.id === notebookId) {
          return {
            ...notebook,
            cells: notebook.cells.filter(cell => cell.id !== cellId),
            updatedAt: new Date(),
          };
        }
        return notebook;
      })
    );
  }

  function moveCell(notebookId: string, cellId: string, direction: 'up' | 'down') {
    update(notebooks =>
      notebooks.map(notebook => {
        if (notebook.id === notebookId) {
          const cells = [...notebook.cells];
          const index = cells.findIndex(cell => cell.id === cellId);
          
          if (index === -1) return notebook;
          
          const newIndex = direction === 'up' ? index - 1 : index + 1;
          
          if (newIndex >= 0 && newIndex < cells.length) {
            [cells[index], cells[newIndex]] = [cells[newIndex], cells[index]];
            return {
              ...notebook,
              cells,
              updatedAt: new Date(),
            };
          }
        }
        return notebook;
      })
    );
  }

  return {
    subscribe,
    set,
    update,
    initialize,
    createNewNotebook,
    getNotebookById,
    updateNotebook,
    deleteNotebook,
    addCell,
    updateCell,
    deleteCell,
    moveCell,
  };
}

export const notebooks = createNotebookStore();
export const activeNotebookId = writable<string | null>(null);

export const activeNotebook = {
  subscribe: (run: (value: Notebook | null) => void) => {
    return activeNotebookId.subscribe(id => {
      if (!id) {
        run(null);
        return;
      }
      
      const unsubscribe = notebooks.subscribe(notebooks => {
        const notebook = notebooks.find(n => n.id === id) || null;
        run(notebook);
      });
      
      return unsubscribe;
    });
  },
};

// Initialize the store when the module is loaded
notebooks.initialize();
