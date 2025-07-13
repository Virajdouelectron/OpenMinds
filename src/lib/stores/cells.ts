import { writable } from 'svelte/store';
import { v4 as uuidv4 } from 'uuid';
import type { Cell } from './notebooks';

export interface CellExecutionResult {
  id: string;
  cellId: string;
  output: any;
  error: string | null;
  executionTime: number;
  timestamp: Date;
}

function createCellStore() {
  const { subscribe, set, update } = writable<Record<string, CellExecutionResult[]>>({});
  
  function addExecutionResult(cellId: string, result: Omit<CellExecutionResult, 'id' | 'cellId' | 'timestamp'>) {
    const executionResult: CellExecutionResult = {
      id: uuidv4(),
      cellId,
      ...result,
      timestamp: new Date(),
    };
    
    update(executions => ({
      ...executions,
      [cellId]: [...(executions[cellId] || []), executionResult],
    }));
    
    return executionResult;
  }
  
  function clearCellOutput(cellId: string) {
    update(executions => {
      const newExecutions = { ...executions };
      delete newExecutions[cellId];
      return newExecutions;
    });
  }
  
  function clearAllOutputs() {
    set({});
  }
  
  function getCellOutputs(cellId: string): CellExecutionResult[] {
    let result: CellExecutionResult[] = [];
    subscribe(executions => {
      result = executions[cellId] || [];
    })();
    return result;
  }
  
  // Simulate cell execution (in a real app, this would call the Python kernel)
  async function executeCell(cell: Cell): Promise<CellExecutionResult> {
    const startTime = performance.now();
    
    try {
      // In a real implementation, this would send the code to a Python kernel
      // For now, we'll simulate a simple execution
      let output: any;
      let error: string | null = null;
      
      if (cell.type === 'code') {
        try {
          // Simple evaluation for demonstration
          // In a real app, this would be handled by a Python kernel
          if (cell.content.includes('error')) {
            throw new Error('Simulated error in code execution');
          }
          
          // Mock different output types
          if (cell.content.includes('plot')) {
            output = {
              type: 'plot',
              data: {
                // Mock plot data
                x: [1, 2, 3, 4, 5],
                y: [1, 4, 9, 16, 25],
                type: 'scatter',
                mode: 'lines+markers',
                marker: { color: 'blue' },
              },
              layout: {
                title: 'Sample Plot',
                xaxis: { title: 'X Axis' },
                yaxis: { title: 'Y Axis' },
              },
            };
          } else if (cell.content.includes('table')) {
            output = {
              type: 'table',
              data: [
                { name: 'Alice', age: 25, city: 'New York' },
                { name: 'Bob', age: 30, city: 'San Francisco' },
                { name: 'Charlie', age: 35, city: 'Seattle' },
              ],
            };
          } else if (cell.content.includes('print')) {
            output = {
              type: 'text',
              value: 'Hello from OpenMind!\nThis is a multi-line output.',
            };
          } else {
            output = {
              type: 'text',
              value: 'Execution completed successfully.',
            };
          }
        } catch (e) {
          error = e instanceof Error ? e.message : 'Unknown error occurred';
        }
      } else {
        // For markdown cells, just return success
        output = { type: 'text', value: 'Markdown rendered successfully' };
      }
      
      const executionTime = performance.now() - startTime;
      
      const result = addExecutionResult(cell.id, {
        output,
        error,
        executionTime,
      });
      
      return result;
    } catch (error) {
      const executionTime = performance.now() - startTime;
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      
      const result = addExecutionResult(cell.id, {
        output: null,
        error: errorMessage,
        executionTime,
      });
      
      return result;
    }
  }
  
  return {
    subscribe,
    addExecutionResult,
    clearCellOutput,
    clearAllOutputs,
    getCellOutputs,
    executeCell,
  };
}

export const cellExecutions = createCellStore();

export const activeCellId = writable<string | null>(null);

export const activeCell = {
  subscribe: (run: (value: { cell: Cell | null; notebookId: string | null }) => void) => {
    return activeCellId.subscribe(cellId => {
      if (!cellId) {
        run({ cell: null, notebookId: null });
        return;
      }
      
      // In a real implementation, we would find which notebook contains this cell
      // For now, we'll just return the cell from the first notebook that contains it
      import('./notebooks').then(({ notebooks }) => {
        notebooks.subscribe(notebooksList => {
          for (const notebook of notebooksList) {
            const cell = notebook.cells.find(c => c.id === cellId);
            if (cell) {
              run({ cell, notebookId: notebook.id });
              return;
            }
          }
          run({ cell: null, notebookId: null });
        });
      });
    });
  },
};
