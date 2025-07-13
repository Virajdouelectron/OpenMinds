-- Create notebooks table
CREATE TABLE IF NOT EXISTS notebooks (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);

-- Create cells table
CREATE TABLE IF NOT EXISTS cells (
    id TEXT PRIMARY KEY,
    notebook_id TEXT NOT NULL,
    cell_type TEXT NOT NULL, -- 'code' or 'markdown'
    content TEXT NOT NULL,
    output TEXT,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (notebook_id) REFERENCES notebooks(id) ON DELETE CASCADE
);

-- Create datasets table
CREATE TABLE IF NOT EXISTS datasets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    size INTEGER NOT NULL,
    created_at DATETIME NOT NULL
);

-- Create models table
CREATE TABLE IF NOT EXISTS models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    metrics TEXT, -- JSON string of metrics
    dataset_id TEXT,
    created_at DATETIME NOT NULL,
    FOREIGN KEY (dataset_id) REFERENCES datasets(id) ON DELETE SET NULL
);

-- Create indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_cells_notebook_id ON cells(notebook_id);
CREATE INDEX IF NOT EXISTS idx_models_dataset_id ON models(dataset_id);
